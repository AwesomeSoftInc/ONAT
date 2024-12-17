use parking_lot::MutexGuard;
use proc::{monster_derive, monster_function_macro};
use raylib::texture::Texture2D;
use std::time::SystemTime;

use super::{Monster, MonsterName, DEFAULT_AI_LEVEL};
use crate::{audio::Audio, enums::Room, textures::Textures};

#[monster_derive]
pub struct Wilber {
    rage: f32,
    pub stage: u8,
    pub time_since_appeared: Option<SystemTime>,
}

impl Wilber {
    pub fn new() -> Self {
        Self {
            name: MonsterName::Wilber,
            room: Room::Room6,
            next_room: Room::None,
            ai_level: DEFAULT_AI_LEVEL,
            active: false,
            entered_from_left: false,
            entered_from_right: false,
            rage: 0.0,
            stage: 0,
            progress_to_hallway: 1,
            last_scared_at: SystemTime::now(),
            timer_until_office: SystemTime::now(),
            move_timer: 0,
            time_in_room: SystemTime::now(),
            time_since_appeared: None,
            move_after_timer: true,
        }
    }
    pub fn rage(&self) -> f32 {
        self.rage
    }
    pub fn set_rage(&mut self, val: f32) {
        self.rage = val;
    }
    pub fn rage_increment(&mut self, aud: &mut Audio) {
        if !self.active {
            return;
        }
        if self.rage < 100.0 {
            self.rage += 0.1;
        } else {
            if self.stage <= 1 {
                aud.play_wilbur(self.stage);
            }
            self.stage += 1;
            self.rage = 0.0;
        }
    }
    pub fn rage_decrement(&mut self) {
        if !self.active {
            return;
        }
        if self.rage > 0.0 {
            self.rage -= 1.0;
        }
    }
}

impl Monster for Wilber {
    monster_function_macro!();
    fn get_texture<'a>(&'a self, textures: &'a mut Textures) -> Option<MutexGuard<Texture2D>> {
        if self.active {
            match self.stage {
                0 => Some(textures.wilber.progress1()),
                1 => Some(textures.wilber.progress2()),
                2 => Some(textures.wilber.progress3()),
                _ => None,
            }
        } else {
            Some(textures.wilber.inactive())
        }
    }
}
