use std::{
    fs::File,
    io::{BufReader, Read, Seek, SeekFrom},
    path::Path,
    sync::{Arc, OnceLock},
};

use alto::{Alto, Buffer, Context, ContextAttrs, Mono, Source, SourceState, StaticSource, Stereo};
use parking_lot::Mutex;

pub const AUDIO_F32: u16 = 0x8120;

const MAX_VOLUME: i32 = 128;

struct Mixer {
    ctx: Context,
    frequency: i32,
}

static MIXER: OnceLock<Mixer> = OnceLock::new();

struct Slot {
    src: Option<StaticSource>,
    volume: i32,
    panning: Option<(u8, u8)>,
}

impl Slot {
    fn new() -> Self {
        Self {
            src: None,
            volume: MAX_VOLUME,
            panning: None,
        }
    }

    fn is_playing(&self) -> bool {
        self.src
            .as_ref()
            .map_or(false, |s| s.state() == SourceState::Playing)
    }
}

static SLOTS: Mutex<Vec<Slot>> = Mutex::new(Vec::new());

const DEFAULT_CHANNELS: usize = 8;

fn mixer() -> Result<&'static Mixer, String> {
    MIXER
        .get()
        .ok_or_else(|| "audio not opened (call open_audio first)".to_string())
}

pub fn open_audio(
    frequency: i32,
    _format: u16,
    _channels: i32,
    _chunksize: i32,
) -> Result<(), String> {
    if MIXER.get().is_some() {
        return Ok(());
    }
    let alto = Alto::load_default()
        .or_else(|_| {
            #[cfg(target_os = "linux")]
            {
                Alto::load("libopenal.so.1")
            }
            #[cfg(target_os = "windows")]
            {
                Alto::load("OpenAL32.dll")
            }
        })
        .map_err(|e| match e {
            alto::AltoError::Io(io) => format!("failed to load OpenAL: {}", io),
            e => format!("failed to load OpenAL: {:?}", e),
        })?;
    let dev = alto
        .open(None)
        .map_err(|e| format!("failed to open audio device: {}", e))?;
    let ctx = dev
        .new_context(Some(ContextAttrs {
            frequency: Some(frequency),
            mono_sources: Some(512),
            stereo_sources: Some(512),
            ..Default::default()
        }))
        .or_else(|_| dev.new_context(None))
        .map_err(|e| format!("failed to create OpenAL context: {}", e))?;
    let _ = MIXER.set(Mixer { ctx, frequency });

    let mut slots = SLOTS.lock();
    if slots.is_empty() {
        slots.resize_with(DEFAULT_CHANNELS, Slot::new);
    }
    Ok(())
}

pub fn close_audio() {
    for slot in SLOTS.lock().iter_mut() {
        slot.src = None;
    }
}

pub fn allocate_channels(numchans: i32) -> i32 {
    SLOTS
        .lock()
        .resize_with(numchans.max(0) as usize, Slot::new);
    numchans
}

fn apply_panning(src: &mut StaticSource, left: u8, right: u8) -> Result<(), String> {
    let (l, r) = (left as f32, right as f32);
    let x = if l + r > 0.0 { (r - l) / (l + r) } else { 0.0 };
    src.set_relative(true);
    src.set_position([x, 0.0, -(1.0 - x * x).max(0.0).sqrt()])
        .map_err(|e| e.to_string())
}

pub struct Chunk {
    buffer: Arc<Buffer>,
    pub alen: u32,
}

impl Chunk {
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Chunk, String> {
        let path = path.as_ref();
        let m = mixer()?;
        let err = |e: &dyn std::fmt::Display| format!("couldn't load {:?}: {}", path, e);

        let mut file = File::open(path).map_err(|e| err(&e))?;
        let mut magic = [0u8; 4];
        file.read_exact(&mut magic).map_err(|e| err(&e))?;
        file.seek(SeekFrom::Start(0)).map_err(|e| err(&e))?;

        let (samples, channels, rate): (Vec<i16>, u16, u32) = match &magic {
            b"OggS" => {
                let mut ogg = lewton::inside_ogg::OggStreamReader::new(BufReader::new(file))
                    .map_err(|e| err(&e))?;
                let channels = ogg.ident_hdr.audio_channels as u16;
                let rate = ogg.ident_hdr.audio_sample_rate;
                let mut samples = Vec::new();
                while let Some(mut packet) = ogg.read_dec_packet_itl().map_err(|e| err(&e))? {
                    samples.append(&mut packet);
                }
                (samples, channels, rate)
            }
            b"RIFF" => {
                let mut wav = hound::WavReader::new(BufReader::new(file)).map_err(|e| err(&e))?;
                let spec = wav.spec();
                let samples = match spec.sample_format {
                    hound::SampleFormat::Int => {
                        let shift = spec.bits_per_sample as i32 - 16;
                        wav.samples::<i32>()
                            .map(|s| {
                                s.map(|v| {
                                    if shift >= 0 {
                                        (v >> shift) as i16
                                    } else {
                                        (v << -shift) as i16
                                    }
                                })
                            })
                            .collect::<Result<Vec<_>, _>>()
                    }
                    hound::SampleFormat::Float => wav
                        .samples::<f32>()
                        .map(|s| s.map(|v| (v.clamp(-1.0, 1.0) * i16::MAX as f32) as i16))
                        .collect::<Result<Vec<_>, _>>(),
                }
                .map_err(|e| err(&e))?;
                (samples, spec.channels, spec.sample_rate)
            }
            _ => {
                return Err(err(
                    &"unrecognized audio format (expected Ogg Vorbis or WAV)",
                ))
            }
        };

        let frames = samples.len() / channels.max(1) as usize;
        let buffer = match channels {
            1 => {
                let data: Vec<Mono<i16>> = samples.iter().map(|&s| Mono { center: s }).collect();
                m.ctx.new_buffer::<Mono<i16>, _>(data, rate as i32)
            }
            2 => {
                let data: Vec<Stereo<i16>> = samples
                    .chunks_exact(2)
                    .map(|s| Stereo {
                        left: s[0],
                        right: s[1],
                    })
                    .collect();
                m.ctx.new_buffer::<Stereo<i16>, _>(data, rate as i32)
            }
            n => return Err(format!("{:?}: unsupported channel count {}", path, n)),
        }
        .map_err(|e| e.to_string())?;

        let out_frames = frames as u64 * m.frequency as u64 / rate.max(1) as u64;
        Ok(Chunk {
            buffer: Arc::new(buffer),
            alen: (out_frames * 2 * 4) as u32,
        })
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Channel(usize);

pub struct AnyChannel;

impl Channel {
    pub fn all() -> AnyChannel {
        AnyChannel
    }

    pub fn halt(self) {
        if let Some(slot) = SLOTS.lock().get_mut(self.0) {
            slot.src = None;
        }
    }

    pub fn is_playing(self) -> bool {
        SLOTS.lock().get(self.0).map_or(false, Slot::is_playing)
    }

    pub fn set_volume(self, volume: i32) -> i32 {
        let mut slots = SLOTS.lock();
        let Some(slot) = slots.get_mut(self.0) else {
            return 0;
        };
        let prev = slot.volume;
        if volume >= 0 {
            slot.volume = volume.min(MAX_VOLUME);
            if let Some(src) = slot.src.as_mut() {
                let _ = src.set_gain(slot.volume as f32 / MAX_VOLUME as f32);
            }
        }
        prev
    }

    pub fn get_volume(self) -> i32 {
        SLOTS.lock().get(self.0).map_or(0, |s| s.volume)
    }

    pub fn set_panning(self, left: u8, right: u8) -> Result<(), String> {
        let mut slots = SLOTS.lock();
        let slot = slots
            .get_mut(self.0)
            .ok_or_else(|| format!("invalid channel {}", self.0))?;
        slot.panning = Some((left, right));
        match slot.src.as_mut() {
            Some(src) => apply_panning(src, left, right),
            None => Ok(()),
        }
    }
}

impl AnyChannel {
    pub fn play(&self, chunk: &Chunk, loops: i32) -> Result<Channel, String> {
        let m = mixer()?;

        let mut slots = SLOTS.lock();
        let idx = slots
            .iter()
            .position(|s| !s.is_playing())
            .ok_or_else(|| "No free channels available".to_string())?;
        let slot = &mut slots[idx];

        let mut src = m.ctx.new_static_source().map_err(|e| e.to_string())?;
        src.set_buffer(chunk.buffer.clone())
            .map_err(|e| e.to_string())?;
        src.set_looping(loops != 0);
        src.set_gain(slot.volume as f32 / MAX_VOLUME as f32)
            .map_err(|e| e.to_string())?;
        if let Some((l, r)) = slot.panning {
            apply_panning(&mut src, l, r)?;
        }
        src.play();

        slot.src = Some(src);
        Ok(Channel(idx))
    }
}
