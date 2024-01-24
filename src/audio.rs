use std::time::SystemTime;

use rand::{rngs::ThreadRng, thread_rng, Rng};
use sdl2::{
    audio::AudioSpecDesired,
    mixer::{AudioFormat, Channel, Chunk, InitFlag, Music, Sdl2MixerContext, AUDIO_F32},
    sys::SDL_AudioFormat,
    AudioSubsystem, Sdl,
};

use crate::state::State;

macro_rules! play {
    ($($val:tt).*,$($chunk:tt).*) => {
        play!($($val).*,$($chunk).*,0)
    };
    ($($val:tt).*,$($chunk:tt).*, $num:literal) => {
        if let None = $($val).*  {
            $($val).* = Some(sdl2::mixer::Channel::all().play(&$($chunk).*, $num)?)
        }
    };
}

pub struct Audio {
    door: Chunk,
    fuck_you_tux: Chunk,
    thud: Chunk,
    noise: Chunk,
    wilber_appear: Chunk,
    tux_appear: Chunk,
    ambience_ominous: Vec<Chunk>,
    ambience_sinister: Vec<Chunk>,
    tainted_notes: Vec<Chunk>,
    plush: Chunk,

    thread_rng: ThreadRng,

    regular_jumpscare: Chunk,
    tux_jumpscare: Chunk,

    brownian_noise: Chunk,
    bells: Chunk,

    revenant_party: Chunk,
    brownian_channel: Option<Channel>,
    title_channel: Option<Channel>,
    left_channel: Option<Channel>,
    right_channel: Option<Channel>,
    noise_channel: Option<Channel>,
    monster_appear_channel: Option<Channel>,
    ambient_channel: Option<Channel>,
    plush_channel: Option<Channel>,
    jumpscare_channel: Option<Channel>,
    tainted_channels: Vec<Option<Channel>>,
    bells_channel: Option<Channel>,
}
impl Audio {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        sdl2::mixer::open_audio(44000, AUDIO_F32, 2, 256)?;
        sdl2::mixer::allocate_channels(8);

        let door = sdl2::mixer::Chunk::from_file("./assets/door.mp3")?;
        let fuck_you_tux = sdl2::mixer::Chunk::from_file("./assets/fuck_you_tux.mp3")?;
        let thud = sdl2::mixer::Chunk::from_file("./assets/thud.mp3")?;
        let noise = sdl2::mixer::Chunk::from_file("./assets/noise.mp3")?;
        let wilber_appear = sdl2::mixer::Chunk::from_file("./assets/wilber_appear.mp3")?;
        let tux_appear = sdl2::mixer::Chunk::from_file("./assets/tux_appears.mp3")?;
        let plush = sdl2::mixer::Chunk::from_file("./assets/plush.mp3")?;

        let regular_jumpscare = sdl2::mixer::Chunk::from_file("./assets/regular_jumpscare.mp3")?;
        let tux_jumpscare = sdl2::mixer::Chunk::from_file("./assets/tux_jumpscare.mp3")?;
        let revenant_party = sdl2::mixer::Chunk::from_file("./assets/revenant_party.mp3")?;
        let brownian_noise = sdl2::mixer::Chunk::from_file("./assets/brownian_noise.mp3")?;
        let ambience_ominous = vec![
            sdl2::mixer::Chunk::from_file("./assets/ominous_ambient_1.mp3")?,
            sdl2::mixer::Chunk::from_file("./assets/ominous_ambient_2.mp3")?,
            sdl2::mixer::Chunk::from_file("./assets/ominous_ambient_3.mp3")?,
        ];

        let ambience_sinister = vec![
            sdl2::mixer::Chunk::from_file("./assets/sinister_ambient_1.mp3")?,
            sdl2::mixer::Chunk::from_file("./assets/sinister_ambient_2.mp3")?,
            sdl2::mixer::Chunk::from_file("./assets/sinister_ambient_3.mp3")?,
        ];

        let bells = sdl2::mixer::Chunk::from_file("./assets/bells.flac")?;
        let mut tainted_notes = Vec::new();
        let mut tainted_channels = Vec::new();
        for i in 1..37 {
            tainted_notes.push(sdl2::mixer::Chunk::from_file(format!(
                "./assets/tainted/note{}.mp3",
                i
            ))?);
            tainted_channels.push(None);
        }
        Ok(Self {
            door,
            fuck_you_tux,
            thud,
            noise,
            wilber_appear,
            tux_appear,
            ambience_ominous,
            ambience_sinister,
            thread_rng: thread_rng(),
            title_channel: None,
            left_channel: None,
            right_channel: None,
            noise_channel: None,
            monster_appear_channel: None,
            ambient_channel: None,
            plush,
            plush_channel: None,
            regular_jumpscare,
            tux_jumpscare,
            jumpscare_channel: None,
            brownian_noise,
            brownian_channel: None,
            tainted_notes,
            tainted_channels,
            bells,
            bells_channel: None,
            revenant_party,
        })
    }

    pub fn play_ambience(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        if let None = self.ambient_channel {
            let chance_to_play = self.thread_rng.gen_range(1..1000);
            if chance_to_play <= 1 {
                let chance = self.thread_rng.gen_range(1..150);
                let vec;
                if chance <= 1 {
                    vec = &self.ambience_ominous;
                } else {
                    vec = &self.ambience_sinister;
                }
                let chance = self.thread_rng.gen_range(1..vec.len());
                let snd = vec.get(chance - 1).unwrap();
                play!(self.ambient_channel, snd);
            }
        };
        Ok(())
    }

    pub fn play_tainted(&mut self, mut note: usize) -> Result<(), Box<dyn std::error::Error>> {
        if note >= 36 {
            note = 0;
        }
        let snd = &self.tainted_notes.get(note).unwrap();
        if let None = self.tainted_channels[note] {
            let pl = sdl2::mixer::Channel::all().play(&snd, 0);
            match pl {
                Ok(a) => self.tainted_channels[note] = Some(a),
                Err(er) => {}
            }
        };
        Ok(())
    }

    pub fn play_brownian_noise(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        play!(self.brownian_channel, self.brownian_noise);
        Ok(())
    }

    pub fn play_regular_jumpscare(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        play!(self.jumpscare_channel, self.regular_jumpscare);
        Ok(())
    }

    pub fn play_tux_jumpscare(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        play!(self.jumpscare_channel, self.tux_jumpscare);
        Ok(())
    }
    pub fn play_plush(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        play!(self.plush_channel, self.plush);
        Ok(())
    }
    pub fn play_wilber(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        play!(self.monster_appear_channel, self.wilber_appear);
        Ok(())
    }
    pub fn play_tux(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        play!(self.monster_appear_channel, self.tux_appear);

        Ok(())
    }
    pub fn play_noise(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        play!(self.noise_channel, self.noise);

        Ok(())
    }
    pub fn play_title(&mut self, won: bool) -> Result<(), Box<dyn std::error::Error>> {
        if won {
            play!(self.title_channel, self.revenant_party, -1);
        } else {
            play!(self.title_channel, self.fuck_you_tux, -1);
        }
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
    pub fn play_bells(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        play!(self.bells_channel, self.bells);
        Ok(())
    }
    pub fn step(&mut self, state: &State) -> Result<(), Box<dyn std::error::Error>> {
        let var_name = state.bg_offset_x / 3.0;
        let mut left = 191.0 - var_name;
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
        if let Some(ch) = self.right_channel {
            ch.set_panning(0, right)?;
            if !ch.is_playing() {
                self.right_channel = None;
            }
        }

        if let Some(ch) = self.noise_channel {
            ch.set_volume(100);
            if !ch.is_playing() {
                self.noise_channel = None;
            }
        }
        if let Some(ch) = self.monster_appear_channel {
            if !ch.is_playing() {
                self.monster_appear_channel = None;
            }
        }
        if let Some(ch) = self.bells_channel {
            if !ch.is_playing() {
                self.bells_channel = None;
            }
        }
        if let Some(ch) = self.ambient_channel {
            if !ch.is_playing() {
                self.ambient_channel = None;
            }
        }
        if let Some(ch) = self.plush_channel {
            if !ch.is_playing() {
                self.plush_channel = None;
            }
        }
        if let Some(ch) = self.jumpscare_channel {
            if !ch.is_playing() {
                self.jumpscare_channel = None;
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
            ch.set_volume(volume);
            if !ch.is_playing() {
                ch.set_volume(100);
                self.title_channel = None;
            }
        }
        Ok(())
    }

    pub fn left_halt(&mut self) {
        if let Some(ch) = self.left_channel {
            ch.halt();
        }
    }
    pub fn right_halt(&mut self) {
        if let Some(ch) = self.right_channel {
            ch.halt();
        }
    }
    pub fn center_halt(&mut self) {
        if let Some(ch) = self.title_channel {
            ch.halt();
        }
    }
    pub fn noise_halt(&mut self) {
        if let Some(ch) = self.noise_channel {
            ch.halt();
        }
    }
    pub fn brownian_halt(&mut self) {
        if let Some(ch) = self.brownian_channel {
            ch.halt();
        }
    }
    pub fn halt(&mut self) {
        self.left_halt();
        self.center_halt();
        self.right_halt();
    }
}
