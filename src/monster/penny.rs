use parking_lot::MutexGuard;
use proc::{monster_derive, monster_function_macro};
use raylib::texture::Texture2D;
use std::time::SystemTime;

use super::{
    HallwayMonster, Monster, MonsterName, DEFAULT_AI_LEVEL, MONSTER_TIME_OFFICE_WAIT_THING,
};
use crate::{enums::Room, textures::Textures};

#[monster_derive]
pub struct Penny {
    door_shut: bool,
}

impl Penny {
    pub fn new() -> Self {
        Self {
            name: MonsterName::Penny,
            room: Room::Room2,
            next_room: Room::None,
            ai_level: DEFAULT_AI_LEVEL,
            active: true,
            entered_from_left: false,
            entered_from_right: false,
            door_shut: false,
            progress_to_hallway: 0,
            last_scared_at: SystemTime::now(),
            timer_until_office: SystemTime::now(),
            move_timer: 0,
            time_in_room: SystemTime::now(),
            move_after_timer: true,
        }
    }
}

impl Monster for Penny {
    monster_function_macro!();
    fn get_texture<'a>(&'a self, textures: &'a mut Textures) -> Option<MutexGuard<Texture2D>> {
        if self.active {
            match self.room {
                Room::Room2 => match self.progress_to_hallway {
                    0 => Some(textures.penny.cam2stage1()),
                    1 => Some(textures.penny.cam2stage2()),
                    _ => None,
                },
                Room::Room3 => match self.progress_to_hallway {
                    0 => Some(textures.penny.cam3stage1()),
                    1 => Some(textures.penny.cam3stage2()),
                    _ => None,
                },
                Room::Office => {
                    if self.timer_until_office().elapsed().unwrap().as_secs()
                        >= MONSTER_TIME_OFFICE_WAIT_THING
                    {
                        Some(textures.penny.pennydoor())
                    } else {
                        None
                    }
                }
                _ => None,
            }
        } else {
            None
        }
    }

    fn _next(&mut self) -> Room {
        HallwayMonster::_next(self)
    }
    fn next(&mut self) {
        HallwayMonster::next(self)
    }

    fn end_move_timer(&mut self) {
        HallwayMonster::end_move_timer(self);
    }

    fn room_after_office(&mut self) -> Room {
        Room::Room2
    }
}

impl HallwayMonster for Penny {
    fn hallway_room(&self) -> Room {
        Room::Room3
    }

    fn set_door(&mut self) {
        self.set_entered_from_left(true);
    }
}
