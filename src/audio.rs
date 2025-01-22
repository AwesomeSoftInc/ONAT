use std::collections::HashMap;

use proc::audio_generate;
use rand::{thread_rng, Rng};
use sdl2::mixer::{Channel, Chunk, AUDIO_F32};

use crate::config::config;

fn audio_init() -> Result<(), Box<dyn std::error::Error>> {
    sdl2::mixer::open_audio(44000, AUDIO_F32, 2, 1024)?;
    sdl2::mixer::allocate_channels(1024);
    Ok(())
}

pub struct Sound {
    path: String,
    chunk: Chunk,
    channels: HashMap<usize, Channel>,
}

impl Sound {
    pub fn from_file(path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let chunk = Chunk::from_file(path)?;

        Ok(Self {
            path: path.to_string(),
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
