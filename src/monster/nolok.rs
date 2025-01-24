use proc::{monster_derive, monster_function_macro};
use rand::{thread_rng, Rng};
use std::time::SystemTime;

use super::{Monster, MonsterName, DEFAULT_AI_LEVEL};
use crate::enums::Room;

#[monster_derive]
pub struct Nolok {}

impl Nolok {
    pub fn new() -> Self {
        Self {
            name: MonsterName::Nolok,
            room: Room::None,

            next_room: Room::None,
            ai_level: DEFAULT_AI_LEVEL,
            active: false,
            entered_from_left: false,
            entered_from_right: false,
            progress_to_hallway: 1,

            last_scared_at: SystemTime::now(),
            timer_until_office: SystemTime::now(),
            move_timer: 0,
            time_in_room: SystemTime::now(),

            move_after_timer: true,
            stinger_played: false,
        }
    }
}

impl Monster for Nolok {
    monster_function_macro!();

    fn try_move(&mut self) -> bool {
        match self.room() {
            Room::None => {
                let coin_flip = thread_rng().gen_range(0..1);
                if coin_flip <= 1 {
                    let coin_flip_2 = thread_rng().gen_range(0..2);
                    if coin_flip_2 == 0 {
                        self.set_room(Room::Room3);
                    } else {
                        self.set_room(Room::Room5);
                    }
                    return true;
                } else {
                    return false;
                }
            }
            Room::Room3 => {
                self.set_entered_from_left(true);
                self.set_room(Room::Office);
                self.set_last_scared_at(SystemTime::now());
                self.set_stinger_played(false);
                return true;
            }
            Room::Room5 => {
                self.set_entered_from_right(true);
                self.set_room(Room::Office);
                self.set_last_scared_at(SystemTime::now());
                self.set_stinger_played(false);
                return true;
            }
            _ => {
                return false;
            }
        }
    }
    fn room_after_office(&mut self) -> Room {
        Room::None
    }
}
