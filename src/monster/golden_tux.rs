use parking_lot::MutexGuard;
use proc::{monster_derive, monster_function_macro};
use raylib::texture::Texture2D;
use std::time::SystemTime;

use super::{Monster, MonsterName, DEFAULT_AI_LEVEL};
use crate::{enums::Room, textures::Textures};

#[monster_derive]
pub struct GoldenTux {
    pub appeared: SystemTime,
}

impl GoldenTux {
    pub fn new() -> Self {
        Self {
            name: MonsterName::GoldenTux,
            room: Room::Office,

            next_room: Room::None,
            ai_level: DEFAULT_AI_LEVEL,
            active: false,
            entered_from_left: false,
            entered_from_right: false,
            progress_to_hallway: 1,

            last_scared_at: SystemTime::now(),
            timer_until_office: SystemTime::now(),
            move_timer: 0,

            appeared: SystemTime::now(),
            time_in_room: SystemTime::now(),
            move_after_timer: true,
        }
    }
}

impl Monster for GoldenTux {
    monster_function_macro!();

    // Golden Tux has special rules.
    fn taint_percent(&self) -> f32 {
        0.0
    }

    fn get_texture<'a>(&'a self, textures: &'a mut Textures) -> Option<MutexGuard<Texture2D>> {
        if self.active {
            Some(textures.misc.golden_tux())
        } else {
            None
        }
    }
}
