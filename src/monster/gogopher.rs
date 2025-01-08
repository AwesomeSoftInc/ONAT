use parking_lot::MutexGuard;
use proc::{monster_derive, monster_function_macro};
use raylib::{drawing::RaylibDrawHandle, texture::Texture2D};
use std::time::SystemTime;

use rand::{thread_rng, Rng};

use super::{Monster, MonsterName, DEFAULT_AI_LEVEL, MONSTER_TIME_OFFICE_WAIT_THING};
use crate::{config::config, enums::Room, textures::Textures};

const DUCT_THING: u16 = 1000;

#[monster_derive]
pub struct GoGopher {
    pub duct_timer: u16,
    pub duct_heat_timer: u16,
    pub appeared: SystemTime,
}

impl GoGopher {
    pub fn new() -> Self {
        Self {
            name: MonsterName::GoGopher,
            room: Room::None,

            next_room: Room::None,
            ai_level: DEFAULT_AI_LEVEL,
            active: false,
            entered_from_left: false,
            entered_from_right: false,
            duct_timer: 0,
            duct_heat_timer: 0,
            progress_to_hallway: 1,
            last_scared_at: SystemTime::now(),
            timer_until_office: SystemTime::now(),
            appeared: SystemTime::now(),
            move_timer: 0,
            time_in_room: SystemTime::now(),
            move_after_timer: true,
        }
    }
}

impl Monster for GoGopher {
    monster_function_macro!();

    fn get_texture<'a>(&'a self, textures: &'a mut Textures) -> Option<MutexGuard<Texture2D>> {
        match self.room {
            Room::Room4 => {
                if self.duct_timer > 1 && self.duct_timer <= (DUCT_THING / 2) {
                    Some(textures.gopher.gopher1())
                } else if self.duct_timer <= DUCT_THING {
                    Some(textures.gopher.gopher2())
                } else {
                    None
                }
            }
            Room::Office => {
                if self.timer_until_office().elapsed().unwrap().as_secs()
                    >= MONSTER_TIME_OFFICE_WAIT_THING
                {
                    Some(textures.gopher.office())
                } else {
                    None
                }
            }
            _ => None,
        }
    }
    fn draw(
        &mut self,
        textures: &mut Textures,
        rl: &mut RaylibDrawHandle,
        x_offset: f32,
        y_offset: f32,
        width_offset: f32,
        height_offset: f32,
    ) {
        if self.room == Room::Office {
            self._draw(textures, rl, x_offset + 75.0, -200.0, 1.5, 1.5);
        } else {
            self._draw(
                textures,
                rl,
                x_offset,
                y_offset,
                width_offset,
                height_offset,
            )
        }
    }
    fn try_move(&mut self) -> bool {
        false
    }
    fn step(&mut self) {
        self._step();
        if self.duct_heat_timer == 0 {
            match self.room {
                Room::None => {
                    let coin_flip = {
                        if config().night_2() {
                            thread_rng().gen_range(0..500)
                        } else {
                            thread_rng().gen_range(0..5000)
                        }
                    };
                    if coin_flip <= 1 {
                        self.set_room(Room::Room4)
                    }
                }
                Room::Room4 => {
                    self.duct_timer += 1;
                    if self.duct_timer >= DUCT_THING {
                        self.set_timer_until_office(SystemTime::now());

                        self.set_room(Room::Office);
                        self.set_last_scared_at(SystemTime::now());
                        self.appeared = SystemTime::now();
                    }
                    if self.duct_heat_timer >= (DUCT_THING / 2) {
                        self.set_room(Room::None);
                    }
                }
                Room::Office => {
                    if self.duct_heat_timer >= (DUCT_THING / 2) {
                        self.set_room(Room::None);
                        self.set_last_scared_at(SystemTime::now());
                    }
                }
                _ => {}
            }
        } else {
            self.duct_timer = 0;
            self.set_room(Room::None);
        }
    }
}
