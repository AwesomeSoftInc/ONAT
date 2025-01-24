use std::{collections::HashMap, fs::File, io::Read, path::PathBuf, sync::Arc};

use parking_lot::Mutex;
use piper_rs::synth::PiperSpeechSynthesizer;
use proc::audio_generate;
use rand::{thread_rng, Rng};
use sdl2::mixer::{Channel, Chunk, AUDIO_F32};

use crate::config::config;

static CUR_AUDIO_LOAD: Mutex<String> = Mutex::new(String::new());

pub fn audio_load_status() -> String {
    CUR_AUDIO_LOAD.lock().clone()
}

fn audio_set_status(val: &str) {
    *CUR_AUDIO_LOAD.lock() = String::from(val)
}

pub fn audio_init(frequency: i32) -> Result<(), Box<dyn std::error::Error>> {
    sdl2::mixer::close_audio();
    sdl2::mixer::open_audio(frequency, AUDIO_F32, 2, 1024)?;
    sdl2::mixer::allocate_channels(1024);
    Ok(())
}

fn file_get_value(file: &mut File, val: &str) -> Result<String, Box<dyn std::error::Error>> {
    let mut buf = String::new();
    file.read_to_string(&mut buf)?;

    // get all the lines starting with the key
    let name_lines = buf
        .lines()
        .filter(|f| f.to_lowercase().replace(" ", "").starts_with(val))
        .map(|f| f.to_string())
        .collect::<Vec<_>>();

    // Get the first one we found and seperate each end of the equals side.
    let parts = name_lines.first().unwrap().split("=").collect::<Vec<_>>();

    let name = parts.last().unwrap().replace("\"", "");

    Ok(name)
}

fn os_name() -> Result<String, Box<dyn std::error::Error>> {
    #[cfg(target_os = "windows")]
    {
        Ok("Windows".to_string())
    }
    #[cfg(target_os = "linux")]
    {
        if let Ok(mut file) = File::open("/etc/os-release") {
            return Ok(file_get_value(&mut file, "name=")?);
        } else if let Ok(mut file) = File::open("/etc/lsb-release") {
            return Ok(file_get_value(&mut file, "distrib_id=")?);
        } else {
            return Ok("some shitfuck version of Linux that nobody's ever heard of".to_string());
        }
    }
}

fn tts_fetch() -> Result<Vec<(String, usize, Sound)>, Box<dyn std::error::Error>> {
    let model = piper_rs::from_config_path(&PathBuf::new().join("tts").join("bonzi.json"))?;
    if let Some(speakers) = model.get_speakers()? {
        if let Some(first_key) = speakers.keys().collect::<Vec<_>>().first() {
            if let Some(err) = model.set_speaker(**first_key) {
                return Err(Box::new(err));
            };
        }
    };

    let synth = Arc::new(PiperSpeechSynthesizer::new(model)?);

    let crime = format!("For the crime of using {},", os_name()?);

    let sentences = vec!["Well!".to_string(),"Hello there!".to_string(),"I don't believe we've been properly introduced.".to_string(),"I am Bonzi.".to_string(),"Tux wants you to play a little game!".to_string(),crime,"we have locked you in this office.".to_string(),"Tux's followers will be coming shortly to deal with you.".to_string(),"When they come in, they will corrupt your PC\nuntil they are able to attack.".to_string(),"You have doors you can shut on them,\nbut they will open on their own and be jammed for a bit".to_string(),"that is, unless you are wise with shutting them,".to_string(),"as if they run into it, they will unjam it".to_string(),"before walking back to their post.".to_string(),"You have 6 hours to fend them off.".to_string(),"A word of advice?".to_string(),"Keep track of them on the cameras to shut\nthe doors before they can start corrupting anything.".to_string(),"Have fun!".to_string()];

    let s = Box::leak(Box::new(sentences.clone()));
    let mut outs = Vec::new();

    let bonzi_dir = PathBuf::new().join("audio").join("bonzi");
    let _ = std::fs::create_dir(bonzi_dir.clone());

    let mut i = 0;
    for f in s {
        let st = f.to_string();
        let synth = synth.clone();
        let file = bonzi_dir.clone().join(format!("{}.ogg", i));
        if !std::fs::exists(file.clone())? {
            audio_set_status(format!("Generating TTS #{}", i).as_str());
            synth
                .synthesize_to_file(&file, f.to_string(), None)
                .unwrap();
        }
        let chunk = Chunk::from_file(file)?;
        outs.push((
            st,
            (unsafe { *chunk.raw }).alen as usize,
            Sound::from_chunk(chunk)?,
        ));
        i += 1;
    }

    Ok(outs)
}

pub struct Sound {
    chunk: Chunk,
    channels: HashMap<usize, Channel>,
}

impl Sound {
    fn from_chunk(chunk: Chunk) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self {
            chunk,
            channels: HashMap::new(),
        })
    }
    pub fn from_file(path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        audio_set_status(format!("Loading {}", path).as_str());
        let chunk = Chunk::from_file(path)?;

        Ok(Self {
            chunk,
            channels: HashMap::new(),
        })
    }

    pub fn play(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.channels.insert(
            self.channels.len(),
            sdl2::mixer::Channel::all().play(&self.chunk, 0)?,
        );
        Ok(())
    }

    pub fn play_reserved(
        &mut self,
        idx: usize,
        left: u8,
        right: u8,
    ) -> Result<(), Box<dyn std::error::Error>> {
        if let None = self.channels.get(&idx) {
            if let Some(ch) = self
                .channels
                .insert(idx, sdl2::mixer::Channel::all().play(&self.chunk, 0)?)
            {
                ch.set_panning(left, right)?;
            };
        }
        Ok(())
    }

    pub fn play_loop_reserved(&mut self, idx: usize) -> Result<(), Box<dyn std::error::Error>> {
        if let None = self.channels.get(&idx) {
            self.channels
                .insert(idx, sdl2::mixer::Channel::all().play(&self.chunk, -1)?);
        }
        Ok(())
    }

    pub fn play_panned(&mut self, left: u8, right: u8) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(ch) = self.channels.insert(
            self.channels.len(),
            sdl2::mixer::Channel::all().play(&self.chunk, 0)?,
        ) {
            ch.set_panning(left, right)?;
        }
        Ok(())
    }

    pub fn play_loop(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.channels.insert(
            self.channels.len(),
            sdl2::mixer::Channel::all().play(&self.chunk, -1)?,
        );
        Ok(())
    }

    pub fn halt(&mut self) {
        let mut to_remove = vec![];
        for channel in self.channels.clone() {
            channel.1.halt();
            to_remove.push(channel.0);
        }
        for idx in to_remove {
            self.channels.remove(&idx);
        }
    }

    pub fn halt_if_not_playing(&mut self) {
        let mut to_remove = vec![];
        for channel in self.channels.clone() {
            if !channel.1.is_playing() {
                channel.1.halt();
                to_remove.push(channel.0);
            }
        }
        for idx in to_remove {
            self.channels.remove(&idx);
        }
    }

    pub fn is_playing(&mut self) -> bool {
        let mut playing = false;
        for ch in &mut self.channels {
            if ch.1.is_playing() {
                playing = true;
                break;
            }
        }
        return playing;
    }

    pub fn volume(&mut self) -> i32 {
        for ch in &mut self.channels {
            return ch.1.get_volume();
        }
        return 0;
    }
    pub fn set_volume(&mut self, volume: i32) -> () {
        for ch in &mut self.channels {
            ch.1.set_volume(volume);
        }
    }
}

audio_generate!();

// Helper functions for certain audio.
impl Audio {
    pub fn play_wilbur(&mut self, stage: u8) -> Result<(), Box<dyn std::error::Error>> {
        let snd = match stage {
            0 => &mut self.wilbur1,
            1 => &mut self.wilbur2,
            _ => &mut self.wilbur3,
        };
        if !snd.is_playing() {
            snd.play()?
        }

        Ok(())
    }

    pub fn play_title(&mut self, has_won: bool) -> Result<(), Box<dyn std::error::Error>> {
        if has_won {
            if !self.revenant_party.is_playing() && !config().night_2() {
                self.revenant_party.play_loop()?;
            }
        } else {
            if !self.fuck_you_tux.is_playing() {
                self.fuck_you_tux.play_loop()?;
            }
        };
        Ok(())
    }

    pub fn title_volume(
        &mut self,
        has_won: bool,
        volume: f32,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let snd = if has_won {
            &mut self.revenant_party
        } else {
            &mut self.fuck_you_tux
        };
        snd.set_volume((volume * config().volume() as f32) as i32);
        Ok(())
    }

    pub fn halt_title(&mut self, has_won: bool) {
        if has_won {
            self.revenant_party.halt();
        } else {
            self.fuck_you_tux.halt();
        }
    }

    pub fn play_tainted(&mut self, note: usize) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(snd) = match note {
            0 => Some(&mut self.note1),
            1 => Some(&mut self.note2),
            2 => Some(&mut self.note3),
            3 => Some(&mut self.note4),
            4 => Some(&mut self.note5),
            5 => Some(&mut self.note6),
            6 => Some(&mut self.note7),
            7 => Some(&mut self.note8),
            8 => Some(&mut self.note9),
            9 => Some(&mut self.note10),
            10 => Some(&mut self.note11),
            11 => Some(&mut self.note12),
            12 => Some(&mut self.note13),
            13 => Some(&mut self.note14),
            14 => Some(&mut self.note15),
            15 => Some(&mut self.note16),
            16 => Some(&mut self.note17),
            17 => Some(&mut self.note18),
            18 => Some(&mut self.note19),
            19 => Some(&mut self.note20),
            20 => Some(&mut self.note21),
            21 => Some(&mut self.note22),
            22 => Some(&mut self.note23),
            23 => Some(&mut self.note24),
            24 => Some(&mut self.note25),
            25 => Some(&mut self.note26),
            26 => Some(&mut self.note27),
            27 => Some(&mut self.note28),
            28 => Some(&mut self.note29),
            29 => Some(&mut self.note30),
            30 => Some(&mut self.note31),
            31 => Some(&mut self.note32),
            32 => Some(&mut self.note33),
            33 => Some(&mut self.note34),
            34 => Some(&mut self.note35),
            35 => Some(&mut self.note36),
            36 => Some(&mut self.note37),
            _ => None,
        } {
            if !snd.is_playing() {
                snd.play()?;
            }
        }

        Ok(())
    }

    pub fn play_ambience(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let ambient_playing = self.ominous_ambient_1.is_playing()
            && self.ominous_ambient_3.is_playing()
            && self.sinister_ambient_1.is_playing()
            && self.sinister_ambient_2.is_playing()
            && self.sinister_ambient_3.is_playing();
        if !ambient_playing {
            let chance_to_play = thread_rng().gen_range(1..1000);
            if chance_to_play <= 1 {
                let chance = thread_rng().gen_range(1..2000);
                let mut vec;
                if chance <= 1 {
                    // ambience_ominous
                    vec = vec![&mut self.ominous_ambient_1, &mut self.ominous_ambient_3]
                } else {
                    // ambience_sinister
                    vec = vec![
                        &mut self.sinister_ambient_1,
                        &mut self.sinister_ambient_2,
                        &mut self.sinister_ambient_3,
                    ];
                }
                let chance = thread_rng().gen_range(1..vec.len());
                let snd = vec.get_mut(chance - 1).unwrap();
                snd.play()?;
            }
        }

        Ok(())
    }
}

/*

*/
