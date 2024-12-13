use rand::{rngs::ThreadRng, thread_rng, Rng};
use sdl2::mixer::{Channel, Chunk, AUDIO_F32};

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
    pub door: Chunk,
    pub fuck_you_tux: Chunk,
    pub thud: Chunk,
    pub noise: Chunk,
    pub wilber_appear: Chunk,
    pub tux_appear: Chunk,
    pub ambience_ominous: Vec<Chunk>,
    pub ambience_sinister: Vec<Chunk>,
    pub tainted_notes: Vec<Chunk>,
    pub plush: Chunk,

    pub thread_rng: ThreadRng,

    pub regular_jumpscare: Chunk,
    pub tux_jumpscare: Chunk,

    pub brownian_noise: Chunk,
    pub bells: Chunk,

    pub stinger: Chunk,
    pub jammed: Chunk,

    pub camera_flip: Chunk,

    pub wilburs: Vec<Chunk>,
    pub gopher_appear: Chunk,

    pub open_source_closed_casket: Chunk,

    pub revenant_party: Chunk,
    pub ambience_unused: Chunk,
    pub brownian_channel: Option<Channel>,
    pub title_channel: Option<Channel>,
    pub left_channel_door: Option<Channel>,
    pub right_channel_door: Option<Channel>,
    pub left_channel_thud: Option<Channel>,
    pub right_channel_thud: Option<Channel>,
    pub noise_channel: Option<Channel>,
    pub monster_appear_channel: Option<Channel>,
    pub ambient_channel: Option<Channel>,
    pub open_source_channel: Option<Channel>,
    pub plush_channel: Option<Channel>,
    pub jumpscare_channel: Option<Channel>,
    pub tainted_channels: Vec<Option<Channel>>,
    pub bells_channel: Option<Channel>,
    pub stinger_channel: Option<Channel>,
    pub jammed_channel: Option<Channel>,

    pub wilber_channel: Option<Channel>,
}
impl Audio {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        sdl2::mixer::open_audio(44000, AUDIO_F32, 2, 256)?;
        sdl2::mixer::allocate_channels(8);

        let door = sdl2::mixer::Chunk::from_file("./audio/door.ogg")?;
        let fuck_you_tux = sdl2::mixer::Chunk::from_file("./audio/fuck_you_tux.ogg")?;
        let thud = sdl2::mixer::Chunk::from_file("./audio/thud.ogg")?;
        let noise = sdl2::mixer::Chunk::from_file("./audio/noise.ogg")?;
        let wilber_appear = sdl2::mixer::Chunk::from_file("./audio/wilber_appear.ogg")?;
        let tux_appear = sdl2::mixer::Chunk::from_file("./audio/tux_appears.ogg")?;
        let gopher_appear = sdl2::mixer::Chunk::from_file("./audio/gopher.ogg")?;
        let open_source_closed_casket =
            sdl2::mixer::Chunk::from_file("./audio/open_source_closed_casket.ogg")?;
        let plush = sdl2::mixer::Chunk::from_file("./audio/plush.ogg")?;

        let regular_jumpscare = sdl2::mixer::Chunk::from_file("./audio/regular_jumpscare.ogg")?;
        let tux_jumpscare = sdl2::mixer::Chunk::from_file("./audio/tux_jumpscare.ogg")?;
        let revenant_party = sdl2::mixer::Chunk::from_file("./audio/revenant_party.ogg")?;
        let brownian_noise = sdl2::mixer::Chunk::from_file("./audio/brownian_noise.ogg")?;
        let ambience_ominous = vec![
            sdl2::mixer::Chunk::from_file("./audio/ominous_ambient_1.ogg")?,
            sdl2::mixer::Chunk::from_file("./audio/ominous_ambient_3.ogg")?,
        ];
        let ambience_unused = sdl2::mixer::Chunk::from_file("./audio/ominous_ambient_2.ogg")?;

        let ambience_sinister = vec![
            sdl2::mixer::Chunk::from_file("./audio/sinister_ambient_1.ogg")?,
            sdl2::mixer::Chunk::from_file("./audio/sinister_ambient_2.ogg")?,
            sdl2::mixer::Chunk::from_file("./audio/sinister_ambient_3.ogg")?,
        ];

        let bells = sdl2::mixer::Chunk::from_file("./audio/bells.ogg")?;

        let stinger = sdl2::mixer::Chunk::from_file("./audio/stinger.ogg")?;
        let jammed = sdl2::mixer::Chunk::from_file("./audio/jammed.ogg")?;
        let camera_flip = sdl2::mixer::Chunk::from_file("./audio/camera_flip.ogg")?;

        let wilburs = vec![
            sdl2::mixer::Chunk::from_file("./audio/wilbur1.ogg")?,
            sdl2::mixer::Chunk::from_file("./audio/wilbur3.ogg")?,
            sdl2::mixer::Chunk::from_file("./audio/wilbur2.ogg")?,
        ];

        let mut tainted_notes = Vec::new();
        let mut tainted_channels = Vec::new();
        for i in 1..37 {
            tainted_notes.push(sdl2::mixer::Chunk::from_file(format!(
                "./audio/tainted/note{}.ogg",
                i
            ))?);
            tainted_channels.push(None);
        }
        let audio = Self {
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
            left_channel_door: None,
            right_channel_door: None,

            left_channel_thud: None,
            right_channel_thud: None,
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
            stinger,
            stinger_channel: None,
            jammed_channel: None,
            jammed,
            gopher_appear,
            open_source_closed_casket,
            camera_flip,
            wilburs,
            wilber_channel: None,
            ambience_unused,
            open_source_channel: None,
        };
        Ok(audio)
    }

    pub fn play_ambience(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        if let None = self.ambient_channel {
            if let None = self.open_source_channel {
                let chance_to_play = self.thread_rng.gen_range(1..1000);
                if chance_to_play <= 1 {
                    let chance = self.thread_rng.gen_range(1..2000);
                    let vec;
                    if chance <= 1 {
                        vec = &self.ambience_ominous;
                    } else {
                        vec = &self.ambience_sinister;
                    }
                    let chance = self.thread_rng.gen_range(1..vec.len());
                    let snd = vec.get(chance - 1).unwrap();
                    if false {
                        self.play_ambience_unused_channel()?;
                    } else {
                        play!(self.ambient_channel, snd);
                    }
                }
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
                Err(_er) => {}
            }
        };
        Ok(())
    }
    pub fn play_open_source_closed_casket(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        play!(self.open_source_channel, self.open_source_closed_casket);
        Ok(())
    }
    pub fn play_wilber_channel(&mut self, nth: usize) -> Result<(), Box<dyn std::error::Error>> {
        let wil = self.wilburs.get(nth).unwrap();
        play!(self.wilber_channel, wil);
        Ok(())
    }
    pub fn play_ambience_unused_channel(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        play!(self.ambient_channel, self.ambience_unused);
        Ok(())
    }
    pub fn play_camera_flip(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        sdl2::mixer::Channel::all().play(&self.camera_flip, 0)?;
        Ok(())
    }
    pub fn play_jammed(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        play!(self.jammed_channel, self.jammed);
        Ok(())
    }
    pub fn play_stinger(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        play!(self.stinger_channel, self.stinger);
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
    pub fn play_gopher(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        play!(self.monster_appear_channel, self.gopher_appear);

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
        play!(self.left_channel_door, self.door);
        Ok(())
    }
    pub fn play_door_right(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.right_halt();
        play!(self.right_channel_door, self.door);
        Ok(())
    }
    pub fn play_thud_left(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.left_halt();
        play!(self.left_channel_thud, self.thud);
        Ok(())
    }
    pub fn play_thud_right(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.right_halt();
        play!(self.right_channel_thud, self.thud);
        Ok(())
    }
    pub fn play_bells(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        play!(self.bells_channel, self.bells);
        Ok(())
    }

    pub fn left_halt(&mut self) {
        if let Some(ch) = self.left_channel_door {
            ch.halt();
        }
    }
    pub fn right_halt(&mut self) {
        if let Some(ch) = self.right_channel_door {
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
