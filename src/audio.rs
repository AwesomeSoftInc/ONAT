use std::time::SystemTime;

use sdl2::{
    audio::AudioSpecDesired,
    mixer::{AudioFormat, Channel, Chunk, InitFlag, Music, Sdl2MixerContext, AUDIO_F32},
    sys::SDL_AudioFormat,
    AudioSubsystem, Sdl,
};

use crate::state::State;

macro_rules! play {
    ($($val:tt).*,$($chunk:tt).*) => {
        $($val).* = Some(sdl2::mixer::Channel::all().play(&$($chunk).*, 0)?)
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
    door: Chunk,
    fuck_you_tux: Chunk,
    thud: Chunk,
    noise: Chunk,
    title_channel: Option<Channel>,
    left_channel: Option<Channel>,
    right_channel: Option<Channel>,
    noise_channel: Option<Channel>,
}
impl Audio {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        sdl2::mixer::open_audio(44000, AUDIO_F32, 2, 256)?;
        sdl2::mixer::allocate_channels(8);

        let door = sdl2::mixer::Chunk::from_file("./assets/door.mp3")?;
        let fuck_you_tux = sdl2::mixer::Chunk::from_file("./assets/fuck_you_tux.mp3")?;
        let thud = sdl2::mixer::Chunk::from_file("./assets/thud.mp3")?;
        let noise = sdl2::mixer::Chunk::from_file("./assets/noise.mp3")?;

        Ok(Self {
            door,
            fuck_you_tux,
            thud,
            noise,
            title_channel: None,
            left_channel: None,
            right_channel: None,
            noise_channel: None,
        })
    }

    pub fn play_noise_if_not_already(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        if let None = self.noise_channel {
            play!(self.noise_channel, self.noise);
        };
        Ok(())
    }
    pub fn play_title_if_not_already(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        if let None = self.title_channel {
            play!(self.title_channel, self.fuck_you_tux);
        };
        Ok(())
    }
    pub fn play_door_left(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.left_halt();
        play!(self.left_channel, self.door);
        Ok(())
    }
    pub fn play_door_right(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.right_halt();
        play!(self.right_channel, self.door);
        Ok(())
    }
    pub fn play_thud_left(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.left_halt();
        play!(self.left_channel, self.thud);
        Ok(())
    }
    pub fn play_thud_right(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.right_halt();
        play!(self.right_channel, self.thud);
        Ok(())
    }
    pub fn step(
        &mut self,
        bg_offset_x: f32,
        state: &State,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let var_name = bg_offset_x / 3.0;
        let mut left = (191.0 - var_name);
        if left <= 64.0 {
            left = 64.0;
        }
        if left >= 191.0 {
            left = 191.0;
        }
        let mut right = var_name;
        if right <= 64.0 {
            right = 64.0;
        }
        if right >= 191.0 {
            right = 191.0;
        }
        let left = left as u8;
        let right = right as u8;
        if let Some(ch) = self.left_channel {
            ch.set_panning(left, 0)?;
            if !ch.is_playing() {
                self.left_channel = None;
            }
        }
        if let Some(ch) = self.title_channel {
            let mut volume = {
                if state.going_to_office_from_title {
                    (100.0 - (state.title_clicked.elapsed()?.as_millis() as f32 / (5000.0 / 100.0)))
                        as i32
                } else {
                    100
                }
            };
            if volume >= 100 {
                volume = 100;
            }
            println!("{}", volume);
            ch.set_volume(volume);
            if !ch.is_playing() {
                ch.set_volume(100);
                self.title_channel = None;
            }
        }
        if let Some(ch) = self.noise_channel {
            ch.set_volume(100);
            if !ch.is_playing() {
                self.noise_channel = None;
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

    pub fn left_halt(&mut self) {
        if let Some(ch) = self.left_channel {
            ch.halt();
            self.left_channel = None;
        }
    }
    pub fn right_halt(&mut self) {
        if let Some(ch) = self.right_channel {
            ch.halt();
            self.right_channel = None;
        }
    }
    pub fn center_halt(&mut self) {
        if let Some(ch) = self.title_channel {
            ch.halt();
            self.title_channel = None;
        }
    }
    pub fn noise_halt(&mut self) {
        if let Some(ch) = self.noise_channel {
            ch.halt();
            self.noise_channel = None;
        }
    }
    pub fn halt(&mut self) {
        self.left_halt();
        self.center_halt();
        self.right_halt();
    }
}
