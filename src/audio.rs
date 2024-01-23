use sdl2::{
    audio::AudioSpecDesired,
    mixer::{AudioFormat, Channel, Chunk, InitFlag, Music, Sdl2MixerContext, AUDIO_F32},
    sys::SDL_AudioFormat,
    AudioSubsystem, Sdl,
};

macro_rules! play {
    ($($val:tt).*,$($chunk:tt).*) => {
        $($val).* = Some(sdl2::mixer::Channel::all().play(&$($chunk).*, -1)?)
    };
}

/*pub fn play(&mut self, chunk: Chunk) -> Result<(), Box<dyn std::error::Error>> {
    self.left_channel = Some(sdl2::mixer::Channel::all().play(&chunk, -1)?);
    Ok(())
}
pub fn play_right(&mut self, chunk: Chunk) -> Result<(), Box<dyn std::error::Error>> {
    self.right_channel = Some(sdl2::mixer::Channel::all().play(&chunk, -1)?);
    Ok(())
} */

pub struct Audio {
    sdl: Sdl,
    audio: AudioSubsystem,
    mixer: Sdl2MixerContext,
    door: Chunk,
    left_channel: Option<Channel>,

    right_channel: Option<Channel>,
}
impl Audio {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let sdl = sdl2::init()?;
        let audio = sdl.audio()?;
        let mixer =
            sdl2::mixer::init(InitFlag::MP3 | InitFlag::FLAC | InitFlag::MOD | InitFlag::OGG)?;

        sdl2::mixer::open_audio(44000, AUDIO_F32, 2, 256)?;
        sdl2::mixer::allocate_channels(4);

        let door = sdl2::mixer::Chunk::from_file("./assets/door.mp3")?;
        Ok(Self {
            sdl,
            audio,
            mixer,
            door,
            left_channel: None,
            right_channel: None,
        })
    }

    pub fn play_door_left(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        play!(self.left_channel, self.door);
        Ok(())
    }
    pub fn play_door_right(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        play!(self.right_channel, self.door);
        Ok(())
    }
    pub fn step(&mut self, bg_offset_x: f32) -> Result<(), Box<dyn std::error::Error>> {
        let var_name = bg_offset_x / 3.0;
        let mut left = (255.0 - var_name) + 64.0;
        if left >= 255.0 {
            left = 255.0;
        }
        let mut right = var_name + 64.0;
        if right >= 255.0 {
            right = 255.0;
        }
        let left = left as u8;
        let right = right as u8;
        if let Some(ch) = self.left_channel {
            ch.set_panning(left, 0)?;
            if !ch.is_playing() {
                self.left_channel = None;
            }
        }
        if let Some(ch) = self.right_channel {
            ch.set_panning(0, right)?;
            if !ch.is_playing() {
                self.right_channel = None;
            }
        }

        Ok(())
    }
}
