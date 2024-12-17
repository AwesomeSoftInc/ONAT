use proc::audio_generate;
use rand::{thread_rng, Rng};
use sdl2::mixer::{Channel, Chunk, AUDIO_F32};

pub struct Sound {
    path: String,
    chunk: Chunk,
    channel: Option<Channel>,
}

impl Sound {
    pub fn from_file(path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let chunk = Chunk::from_file(path)?;

        Ok(Self {
            path: path.to_string(),
            chunk,
            channel: None,
        })
    }

    // pub fn from_bytes(bytes: Box<[u8]>) -> Result<Self, Box<dyn std::error::Error>> {
    //     let chunk = Chunk::from_raw_buffer(bytes)?;

    //     Ok(Self {
    //         chunk,
    //         channel: None,
    //     })
    // }

    pub fn play(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        if let None = self.channel {
            self.channel = Some(sdl2::mixer::Channel::all().play(&self.chunk, 0)?);
        }
        Ok(())
    }

    pub fn play_panned(&mut self, left: u8, right: u8) -> Result<(), Box<dyn std::error::Error>> {
        if let None = self.channel {
            let ch = sdl2::mixer::Channel::all().play(&self.chunk, 0)?;
            ch.set_panning(left, right)?;
            self.channel = Some(ch);
        }
        Ok(())
    }

    pub fn play_loop(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        if let None = self.channel {
            self.channel = Some(sdl2::mixer::Channel::all().play(&self.chunk, -1)?);
        }
        Ok(())
    }

    pub fn halt(&mut self) {
        if let Some(ch) = self.channel {
            ch.halt();
        }
        self.channel = None;
    }

    pub fn halt_if_not_playing(&mut self) {
        if let Some(ch) = self.channel {
            if !ch.is_playing() {
                ch.halt();
                ch.pause();
                self.channel = None;
            }
        }
    }

    pub fn is_playing(&mut self) -> bool {
        if let Some(ch) = self.channel {
            return ch.is_playing();
        } else {
            return false;
        }
    }
}

audio_generate!();

// Helper functions for certain audio.
impl Audio {
    pub fn play_wilbur(&mut self, stage: u8) -> Result<(), Box<dyn std::error::Error>> {
        match stage {
            0 => self.wilbur1.play()?,
            1 => self.wilbur2.play()?,
            _ => self.wilbur3.play()?,
        }

        Ok(())
    }

    pub fn play_title(&mut self, has_won: bool) -> Result<(), Box<dyn std::error::Error>> {
        if has_won {
            self.revenant_party.play_loop()
        } else {
            self.fuck_you_tux.play_loop()
        }
    }

    pub fn halt_title(&mut self, has_won: bool) {
        if has_won {
            self.revenant_party.halt();
        } else {
            self.fuck_you_tux.halt();
        }
    }

    pub fn play_tainted(&mut self, note: usize) -> Result<(), Box<dyn std::error::Error>> {
        match note {
            0 => self.note1.play()?,
            1 => self.note2.play()?,
            2 => self.note3.play()?,
            3 => self.note4.play()?,
            4 => self.note5.play()?,
            5 => self.note6.play()?,
            6 => self.note7.play()?,
            7 => self.note8.play()?,
            8 => self.note9.play()?,
            9 => self.note10.play()?,
            10 => self.note11.play()?,
            11 => self.note12.play()?,
            12 => self.note13.play()?,
            13 => self.note14.play()?,
            14 => self.note15.play()?,
            15 => self.note16.play()?,
            16 => self.note17.play()?,
            17 => self.note18.play()?,
            18 => self.note19.play()?,
            19 => self.note20.play()?,
            20 => self.note21.play()?,
            21 => self.note22.play()?,
            22 => self.note23.play()?,
            23 => self.note24.play()?,
            24 => self.note25.play()?,
            25 => self.note26.play()?,
            26 => self.note27.play()?,
            27 => self.note28.play()?,
            28 => self.note29.play()?,
            29 => self.note30.play()?,
            30 => self.note31.play()?,
            31 => self.note32.play()?,
            32 => self.note33.play()?,
            33 => self.note34.play()?,
            34 => self.note35.play()?,
            35 => self.note36.play()?,
            36 => self.note37.play()?,
            _ => {}
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
