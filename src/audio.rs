use sdl2::{
    audio::AudioSpecDesired,
    mixer::{AudioFormat, Channel, Chunk, InitFlag, Music, Sdl2MixerContext, AUDIO_F32},
    sys::SDL_AudioFormat,
    AudioSubsystem, Sdl,
};

pub struct Audio {
    sdl: Sdl,
    audio: AudioSubsystem,
    mixer: Sdl2MixerContext,
    door: Chunk,
    door_channel: Option<Channel>,
}
impl Audio {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let sdl = sdl2::init()?;
        let audio = sdl.audio()?;
        let mixer =
            sdl2::mixer::init(InitFlag::MP3 | InitFlag::FLAC | InitFlag::MOD | InitFlag::OGG)?;

        sdl2::mixer::open_audio(44000, AUDIO_F32, 4, 256)?;
        sdl2::mixer::allocate_channels(4);

        let door = sdl2::mixer::Chunk::from_file("./assets/door.mp3")?;
        Ok(Self {
            sdl,
            audio,
            mixer,
            door,
            door_channel: None,
        })
    }
    pub fn play_door_sound(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.door_channel = Some(sdl2::mixer::Channel::all().play(&self.door, -1)?);
        Ok(())
    }
    pub fn step(&mut self, bg_offset_x: f32) -> Result<(), Box<dyn std::error::Error>> {
        let mut left = 160.0 + (bg_offset_x / 8.0);
        if left >= 255.0 {
            left = 255.0;
        }
        let left = left as u8;
        println!("{}", left);
        if let Some(mut ch) = self.door_channel {
            ch.set_panning(left, 0)?;
        }

        Ok(())
    }
}
