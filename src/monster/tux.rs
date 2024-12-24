use parking_lot::MutexGuard;
use proc::{monster_derive, monster_function_macro};
use rand::{thread_rng, Rng};
use raylib::prelude::*;
use std::time::SystemTime;

use super::{Monster, MonsterName, DEFAULT_AI_LEVEL};
use crate::{config::config, enums::Room, texture_rect, textures::Textures};

#[monster_derive]
pub struct Tux {
    pub time_since_entered_hallway: SystemTime,
    pub time_since_last_attempt: SystemTime,
    pub can_move: bool,
    pub moved_to_hallway_at: SystemTime,
    pub checked_camera: Option<SystemTime>,
}

impl Tux {
    pub fn new() -> Self {
        Self {
            name: MonsterName::Tux,
            room: Room::Room1,

            next_room: Room::None,
            ai_level: DEFAULT_AI_LEVEL,
            active: false,
            entered_from_left: false,
            entered_from_right: false,
            progress_to_hallway: 1,

            last_scared_at: SystemTime::now(),
            timer_until_office: SystemTime::now(),
            move_timer: 0,
            time_since_entered_hallway: SystemTime::now(),
            time_since_last_attempt: SystemTime::now(),
            time_in_room: SystemTime::now(),
            can_move: true,

            moved_to_hallway_at: SystemTime::now(),
            checked_camera: None,

            move_after_timer: true,
        }
    }
}

impl Monster for Tux {
    monster_function_macro!();

    fn draw(
        &mut self,
        textures: &mut Textures,
        rl: &mut RaylibDrawHandle,
        x_offset: f32,
        y_offset: f32,
        width_offset: f32,
        height_offset: f32,
    ) {
        match self.room {
            Room::Room3 | Room::Room5 => {
                if let None = self.checked_camera {
                    self.checked_camera = Some(SystemTime::now());
                }
                if let Some(t) = self.get_texture(textures) {
                    let checked_camera = self.checked_camera.unwrap();
                    let mo = checked_camera.elapsed().unwrap().as_secs_f32();
                    rl.draw_texture_pro(
                        &*t,
                        texture_rect!(t),
                        Rectangle::new(
                            (0.0 + (config().width() / 2) as f32) - (mo * 2400.0),
                            (config().height() / 2) as f32 - (mo * 2000.0),
                            t.width as f32 + config().width() as f32 * width_offset * (mo * 4.0),
                            t.height as f32 + config().height() as f32 * height_offset * (mo * 4.0),
                        ),
                        Vector2::new(t.width as f32 / 2.0, t.height as f32 / 2.0),
                        0.0,
                        Color::WHITE,
                    );
                }
            }
            _ => {
                if let Some(t) = self.get_texture(textures) {
                    rl.draw_texture_pro(
                        &*t,
                        texture_rect!(t),
                        Rectangle::new(
                            x_offset,
                            y_offset,
                            config().width() as f32 * width_offset,
                            config().height() as f32 * height_offset,
                        ),
                        Vector2::new(0.0, 0.0),
                        0.0,
                        Color::WHITE,
                    );
                }
            }
        }
    }
    fn get_texture<'a>(&'a self, textures: &'a mut Textures) -> Option<MutexGuard<Texture2D>> {
        match self.room {
            Room::Room1 => {
                if self.active {
                    Some(textures.tux.tuxawake())
                } else {
                    Some(textures.tux.tuxidle())
                }
            }
            Room::Room3 | Room::Room5 => Some(textures.tux.slidingtux()),
            _ => None,
        }
    }

    fn begin_move_timer(&mut self) {
        if !self.can_move {
            self.set_move_timer(0);
        } else {
            self.set_move_timer(5);
        }
    }
    fn next(&mut self) {
        if !self.can_move {
            return;
        }
        match self.room {
            Room::Room1 => {
                self.time_since_entered_hallway = SystemTime::now();
                self.time_since_last_attempt = SystemTime::now();
                match thread_rng().gen_range(0..2) as u64 {
                    0 => self.set_room(Room::Room3),
                    _ => self.set_room(Room::Room5),
                }
                self.moved_to_hallway_at = SystemTime::now();
            }
            Room::Room3 | Room::Room5 => {
                if self.moved_to_hallway_at.elapsed().unwrap().as_secs() <= 10 {
                    if let Some(c) = self.checked_camera {
                        if c.elapsed().unwrap().as_secs() <= 2 {
                            return;
                        }
                    } else {
                        return;
                    }
                }
                match self.room {
                    Room::Room3 => {
                        self.set_entered_from_left(true);
                    }
                    Room::Room5 => {
                        self.set_entered_from_right(true);
                    }
                    _ => {}
                }
                self.begin_move_timer();
                self.set_timer_until_office(SystemTime::now());
                self.set_room(Room::Office);
                self.can_move = false;
            }
            _ => {}
        }
    }

    fn step(&mut self) {
        if let Some(c) = self.checked_camera {
            if c.elapsed().unwrap().as_secs() >= 1 {
                self.next();
            }
        }

        self._step();
    }

    fn room_after_office(&mut self) -> Room {
        Room::Room1
    }
    // Tux instakills.
    fn taint_percent(&self) -> f32 {
        9999.0
    }
}
