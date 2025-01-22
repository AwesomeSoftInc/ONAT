use parking_lot::MutexGuard;
use raylib::prelude::*;
use std::time::{Duration, SystemTime};

use rand::{thread_rng, Rng};

use crate::{audio::Audio, config::config, enums::Room, texture_rect, textures::Textures};

pub mod beastie;
pub mod gogopher;
pub mod golden_tux;
pub mod nolok;
pub mod penny;
pub mod tux;
pub mod wilber;
use beastie::Beastie;
use gogopher::GoGopher;
use golden_tux::GoldenTux;
use nolok::Nolok;
use penny::Penny;
use tux::Tux;
use wilber::Wilber;

pub const MONSTER_TIME_OFFICE_WAIT_THING: u64 = 5;

pub const DEFAULT_AI_LEVEL: u8 = 2;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MonsterName {
    Penny,
    Beastie,
    Wilber,
    GoGopher,
    Tux,
    Nolok,
    GoldenTux,
    None,
}

pub trait Monster {
    fn id(&self) -> MonsterName;
    fn room(&self) -> Room;
    // fn next_room(&self) -> Room;
    fn ai_level(&self) -> u8;
    fn set_room(&mut self, room: Room);
    // fn set_next_room(&mut self, room: Room);
    fn active(&self) -> bool;
    fn activate(&mut self);
    fn deactivate(&mut self);
    fn entered_from_left(&self) -> bool;
    fn entered_from_right(&self) -> bool;
    fn set_entered_from_left(&mut self, res: bool);
    fn set_entered_from_right(&mut self, res: bool);
    fn last_scared_at(&self) -> SystemTime;
    fn set_last_scared_at(&mut self, time: SystemTime);
    fn move_timer(&self) -> u8;
    fn set_move_timer(&mut self, val: u8);
    fn timer_until_office(&self) -> SystemTime;
    fn set_timer_until_office(&mut self, val: SystemTime);
    fn time_in_room(&mut self) -> SystemTime;
    fn reset_time_in_room(&mut self);

    // fn move_after_timer(&mut self) -> bool;
    fn set_move_after_timer(&mut self, val: bool);

    fn begin_move_timer(&mut self) {
        self.set_move_timer(5);
    }

    fn _end_move_timer(&mut self) -> bool {
        if self.move_timer() >= 1 {
            self.set_move_timer(self.move_timer() - 1);
            if self.move_timer() == 0 {
                return true;
            }
        }
        false
    }
    fn end_move_timer(&mut self) {
        if self._end_move_timer() {
            self.reset_time_in_room();
            self.next();
        };
    }
    fn progress_to_hallway(&mut self) -> i8;

    fn get_texture<'a>(&'a self, _textures: &'a mut Textures) -> Option<MutexGuard<Texture2D>> {
        None
    }
    fn _draw(
        &mut self,
        textures: &mut Textures,
        rl: &mut RaylibDrawHandle,
        x_offset: f32,
        y_offset: f32,
        width_offset: f32,
        height_offset: f32,
    ) {
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
    fn draw(
        &mut self,
        textures: &mut Textures,
        rl: &mut RaylibDrawHandle,
        x_offset: f32,
        y_offset: f32,
        width_offset: f32,
        height_offset: f32,
    ) {
        self._draw(
            textures,
            rl,
            x_offset,
            y_offset,
            width_offset,
            height_offset,
        )
    }

    // fn name(&self) -> String {
    //     return format!("{:?}", self.id());
    // }

    fn taint_percent(&self) -> f32 {
        0.2
    }

    fn _try_move(&mut self) -> bool {
        let chance = thread_rng().gen_range(0..20);
        // if any of them are in the hallways, have them move in.
        if self.room() == Room::Room3 || self.room() == Room::Room5 {
            self.next();
        } else {
            if chance <= self.ai_level() {
                self.begin_move_timer();
                return true;
            }
        }
        false
    }
    fn try_move(&mut self) -> bool {
        self._try_move()
    }

    fn _step(&mut self) {
        self.end_move_timer();
    }
    fn step(&mut self) {
        self._step();
    }

    fn _next(&mut self) -> Room {
        match self.room().next() {
            crate::enums::RoomOption::Room(a) => a,
            crate::enums::RoomOption::Multiple(a) => {
                let rnd = thread_rng().gen_range(0..a.len());
                a.get(rnd).unwrap().clone()
            }
            crate::enums::RoomOption::None => Room::None,
        }
    }
    fn next(&mut self) {
        let r = self._next();
        self.set_room(r);
    }

    fn room_after_office(&mut self) -> Room {
        Room::random()
    }

    fn set_progress_to_hallway(&mut self, yeah: i8);
    fn goto_room_after_office(&mut self) -> Room {
        self.set_last_scared_at(SystemTime::now());
        self.set_progress_to_hallway(0);
        let rao = self.room_after_office();
        self.set_room(rao);
        self.room_after_office()
    }
}

trait HallwayMonster: Monster {
    fn hallway_room(&self) -> Room;
    fn set_door(&mut self);

    fn _next(&mut self) -> Room {
        match self.room().next() {
            crate::enums::RoomOption::Room(a) => a,
            crate::enums::RoomOption::Multiple(_) => self.hallway_room(),
            crate::enums::RoomOption::None => Room::None,
        }
    }

    fn end_move_timer(&mut self) {
        if self._end_move_timer() {
            self.reset_time_in_room();
            HallwayMonster::next(self);
        }
    }
    fn next(&mut self) {
        match self.progress_to_hallway() {
            0 => {
                let p = self.progress_to_hallway();
                self.set_progress_to_hallway(p + 1);
                self.set_move_after_timer(false);
                self.reset_time_in_room();
            }
            1 => {
                let p = self.progress_to_hallway();
                self.set_progress_to_hallway(p + 1);
                self.set_move_after_timer(false);
                self.begin_move_timer();
                self.reset_time_in_room();
            }
            2 => {
                self.reset_time_in_room();
                self.set_move_after_timer(true);

                let n = HallwayMonster::_next(self);
                if n == Room::Office {
                    self.set_timer_until_office(SystemTime::now());
                    self.set_door();
                }
                self.set_room(n);
                self.set_progress_to_hallway(0);
            }
            _ => panic!(),
        }
    }
}

pub struct Gang {
    pub penny: Penny,
    pub beastie: Beastie,
    pub wilber: Wilber,
    pub gogopher: GoGopher,
    pub tux: Tux,
    pub nolok: Nolok,
    pub golden_tux: GoldenTux,

    since_last_move: SystemTime,

    moved: bool,
    pub one_am_checked: bool,
    pub two_am_checked: bool,
    pub three_am_checked: bool,
    pub four_am_checked: bool,
    pub five_am_checked: bool,
    pub tux_moved: bool,
    pub hour_offset: i64,
}

impl Gang {
    pub fn new() -> Self {
        Self {
            penny: Penny::new(),
            beastie: Beastie::new(),
            wilber: Wilber::new(),
            gogopher: GoGopher::new(),
            tux: Tux::new(),
            nolok: Nolok::new(),
            golden_tux: GoldenTux::new(),
            since_last_move: SystemTime::now(),

            moved: true,
            one_am_checked: false,
            two_am_checked: false,
            three_am_checked: false,
            four_am_checked: false,
            five_am_checked: false,
            tux_moved: false,
            hour_offset: 0,
        }
    }

    pub fn hours(&self, time: Duration) -> i64 {
        if config().on_tutorial() {
            0
        } else {
            self.hour_offset + (time.as_secs() / 200) as i64
        }
    }
    pub fn step(&mut self, time: Duration, aud: &mut Audio) -> bool {
        let hours = self.hours(time);
        self.penny.step();
        self.beastie.step();
        self.tux.step();
        if self.gogopher.active() {
            self.gogopher.step();
        }

        // every few seconds, generate a random number between 1 and 20, for each enemy. if the animatronic's current ai level is greater/equal to the number, the animatronic moves.
        if self.since_last_move.elapsed().unwrap().as_secs() >= 5 {
            self.since_last_move = SystemTime::now();
            if self.penny.active {
                self.penny.try_move();
            }
            if self.beastie.active {
                if self.beastie.last_scared_at().elapsed().unwrap().as_secs() >= 30 {
                    if self.beastie.room != Room::Office {
                        self.beastie.begin_move_timer();
                    } else {
                        self.beastie.set_last_scared_at(SystemTime::now());
                    }
                } else {
                    self.beastie.try_move();
                }
            }

            if self.tux.active {
                if self.tux.try_move() {
                    self.tux_moved = true;
                }
            }

            if self.nolok.active {
                self.nolok.try_move();
            }
        } else {
            self.moved = true;
        }

        // 1 AM
        if hours >= 1 && !self.one_am_checked {
            self.wilber.time_since_appeared = Some(SystemTime::now());
            self.wilber.activate();
            self.one_am_checked = true;
            self.ai_level_increase();
        }
        // 2 AM
        if hours >= 2 && !self.two_am_checked {
            self.gogopher.activate();
            self.two_am_checked = true;
            self.ai_level_increase();
        }
        // 3 AM
        if hours >= 3 && !self.three_am_checked {
            self.tux.activate();
            self.three_am_checked = true;
            self.ai_level_increase();
            self.tux.can_move = true;
        }
        if hours >= 4 && !self.four_am_checked {
            self.tux.can_move = true;
            self.four_am_checked = true;
        }
        if hours >= 5 && !self.five_am_checked {
            self.tux.can_move = true;
            self.tux.ai_level = 10;
            self.five_am_checked = true;
            if config().night_2() {
                if !aud.night2.is_playing() {
                    aud.night2.play().unwrap();
                }
            } else {
                if !aud.open_source_closed_casket.is_playing() {
                    aud.open_source_closed_casket.play().unwrap();
                }
            }
        }

        return hours == 6;
    }
    pub fn in_room(&mut self, room: Room) -> Vec<&mut dyn Monster> {
        let mut res: Vec<&mut dyn Monster> = vec![];

        if self.penny.room() == room {
            res.push(&mut self.penny);
        }
        if self.beastie.room() == room {
            res.push(&mut self.beastie);
        }
        if self.wilber.room() == room {
            res.push(&mut self.wilber);
        }
        if self.gogopher.room() == room {
            res.push(&mut self.gogopher);
        }
        if self.tux.room() == room {
            res.push(&mut self.tux);
        }
        if self.nolok.room() == room {
            res.push(&mut self.nolok);
        }
        if self.golden_tux.room() == room {
            res.push(&mut self.golden_tux);
        }

        res
    }

    fn ai_level_increase(&mut self) {
        if config().night_2() {
            return;
        }
        self.penny.ai_level += 2;
        self.beastie.ai_level += 3;
        // self.wilber.ai_level += 3;
        // self.gogopher.ai_level += 3;
        // self.tux.ai_level += 3;       // Tux's AI level does not increase naturally, it bumps at 5AM
        // self.nolok.ai_level += 3;     // Nolok is cut.
        // self.golden_tux.ai_level += 3;
    }
}
