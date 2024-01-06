use proc::{monster_derive, monster_function_macro};
use raylib::{
    color::Color,
    drawing::{RaylibDraw, RaylibDrawHandle, RaylibTextureMode},
    math::{Rectangle, Vector2},
    texture::Texture2D,
};
use std::time::{Duration, SystemTime};

use rand::{thread_rng, Rng};

use crate::{enums::Room, get_height, get_margin, get_width, texture_rect, textures::Textures};

pub const PENNY_START: bool = false;
pub const BEASTIE_START: bool = false;
pub const WILBER_START: bool = false;
pub const GO_GOPHER_START: bool = true;
pub const TUX_START: bool = false;
pub const NOLOK_START: bool = false;
pub const GOLDEN_TUX_START: bool = false;

pub const MONSTER_TIME_OFFICE_WAIT_THING: u64 = 5;

pub const DEFAULT_AI_LEVEL: u8 = 20;

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
    fn next_room(&self) -> Room;
    fn ai_level(&self) -> u8;
    fn set_room(&mut self, room: Room);
    fn set_next_room(&mut self, room: Room);
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

    fn move_after_timer(&mut self) -> bool;
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

    fn get_texture<'a>(&'a self, textures: &'a Textures) -> Option<&'a Texture2D> {
        None
    }
    fn _draw(
        &mut self,
        textures: &Textures,
        rl: &mut RaylibTextureMode<RaylibDrawHandle>,
        x_offset: f32,
        y_offset: f32,
        width_offset: f32,
        height_offset: f32,
    ) {
        if let Some(t) = self.get_texture(textures) {
            rl.draw_texture_pro(
                &t,
                texture_rect!(t),
                Rectangle::new(
                    x_offset,
                    y_offset,
                    get_width() as f32 * width_offset,
                    get_height() as f32 * height_offset,
                ),
                Vector2::new(0.0, 0.0),
                0.0,
                Color::WHITE,
            );
        }
    }
    fn draw(
        &mut self,
        textures: &Textures,
        rl: &mut RaylibTextureMode<RaylibDrawHandle>,
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

    fn name(&self) -> String {
        return format!("{:?}", self.id());
    }
    fn taint_percent(&self) -> f32 {
        0.2
    }

    fn _try_move(&mut self) {
        let chance = thread_rng().gen_range(0..20);
        // if any of them are in the hallways, have them move in.
        if self.room() == Room::Room3 || self.room() == Room::Room5 {
            self.next();
        } else {
            if chance <= self.ai_level() {
                self.begin_move_timer();
            }
        }
    }
    fn try_move(&mut self) {
        self._try_move();
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
                    self.set_entered_from_right(true);
                }
                self.set_room(n);
                self.set_progress_to_hallway(0);
            }
            _ => panic!(),
        }
    }
}

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
            active: PENNY_START,
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
    fn get_texture<'a>(&'a self, textures: &'a Textures) -> Option<&Texture2D> {
        if self.active {
            match self.room {
                Room::Room2 => match self.progress_to_hallway {
                    0 => Some(&textures.penny.cam2stage1),
                    1 => Some(&textures.penny.cam2stage2),
                    _ => None,
                },
                Room::Room3 => match self.progress_to_hallway {
                    0 => Some(&textures.penny.cam3stage1),
                    1 => Some(&textures.penny.cam3stage2),
                    _ => None,
                },
                Room::Office => {
                    if self.timer_until_office().elapsed().unwrap().as_secs()
                        >= MONSTER_TIME_OFFICE_WAIT_THING
                    {
                        Some(&textures.penny.pennydoor)
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
}

#[monster_derive]
pub struct Beastie {
    door_shut: bool,
}

impl Beastie {
    pub fn new() -> Self {
        Self {
            name: MonsterName::Beastie,
            room: Room::Room2,
            next_room: Room::None,
            ai_level: DEFAULT_AI_LEVEL,
            active: BEASTIE_START,
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

impl Monster for Beastie {
    monster_function_macro!();

    fn get_texture<'a>(&'a self, textures: &'a Textures) -> Option<&'a Texture2D> {
        if self.active {
            match self.room {
                Room::Room2 => match self.progress_to_hallway {
                    0 => Some(&textures.beastie.cam2stage1),
                    1 => Some(&textures.beastie.cam2stage2),
                    _ => None,
                },
                Room::Room5 => match self.progress_to_hallway {
                    0 => Some(&textures.beastie.cam5stage1),
                    1 => Some(&textures.beastie.cam5stage2),
                    _ => None,
                },
                Room::Office => {
                    if self.timer_until_office().elapsed().unwrap().as_secs()
                        >= MONSTER_TIME_OFFICE_WAIT_THING
                    {
                        Some(&textures.beastie.bsdatdoor)
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
    #[cfg(feature = "beastie_always_move")]
    fn try_move(&mut self) {
        self.next();
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

impl HallwayMonster for Beastie {
    fn hallway_room(&self) -> Room {
        Room::Room5
    }
}

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
            active: WILBER_START,
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
    pub fn rage_increment(&mut self) {
        if !self.active {
            return;
        }
        if self.rage < 100.0 {
            self.rage += 0.1;
        } else {
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
    fn get_texture<'a>(&'a self, textures: &'a Textures) -> Option<&'a Texture2D> {
        if self.active {
            match self.stage {
                0 => Some(&textures.wilber.progress1),
                1 => Some(&textures.wilber.progress2),
                2 => Some(&textures.wilber.progress3),
                _ => None,
            }
        } else {
            Some(&textures.wilber.inactive)
        }
    }
}

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
            active: GO_GOPHER_START,
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

const DUCT_THING: u16 = 10000;

impl Monster for GoGopher {
    monster_function_macro!();

    fn get_texture<'a>(&'a self, textures: &'a Textures) -> Option<&'a Texture2D> {
        match self.room {
            Room::Room4 => {
                if self.duct_timer > 1 && self.duct_timer <= (DUCT_THING / 2) {
                    Some(&textures.gopher.gopher1)
                } else if self.duct_timer <= DUCT_THING {
                    Some(&textures.gopher.gopher2)
                } else {
                    None
                }
            }
            Room::Office => {
                if self.timer_until_office().elapsed().unwrap().as_secs()
                    >= MONSTER_TIME_OFFICE_WAIT_THING
                {
                    Some(&textures.gopher.gopheroffice)
                } else {
                    None
                }
            }
            _ => None,
        }
    }
    fn draw(
        &mut self,
        textures: &Textures,
        rl: &mut RaylibTextureMode<RaylibDrawHandle>,
        x_offset: f32,
        y_offset: f32,
        width_offset: f32,
        height_offset: f32,
    ) {
        if self.room == Room::Office {
            self._draw(&textures, rl, x_offset + 75.0, -200.0, 1.6, 1.6);
        } else {
            self._draw(
                &textures,
                rl,
                x_offset,
                y_offset,
                width_offset,
                height_offset,
            )
        }
    }
    fn try_move(&mut self) {}
    fn step(&mut self) {
        self._step();
        if self.duct_heat_timer == 0 {
            match self.room {
                Room::None => {
                    let coin_flip = thread_rng().gen_range(0..5000);
                    if coin_flip <= 1 {
                        self.set_room(Room::Room4)
                    }
                }
                Room::Room4 => {
                    self.duct_timer += 1;
                    if self.duct_timer >= DUCT_THING {
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
            active: TUX_START,
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
        textures: &Textures,
        rl: &mut RaylibTextureMode<RaylibDrawHandle>,
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
                        &t,
                        texture_rect!(t),
                        Rectangle::new(
                            (get_margin() + (get_width() / 2) as f32) - (mo * 2400.0),
                            (get_height() / 2) as f32 - (mo * 2000.0),
                            t.width as f32 + get_width() as f32 * width_offset * (mo * 4.0),
                            t.height as f32 + get_height() as f32 * height_offset * (mo * 4.0),
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
                        &t,
                        texture_rect!(t),
                        Rectangle::new(
                            x_offset,
                            y_offset,
                            get_width() as f32 * width_offset,
                            get_height() as f32 * height_offset,
                        ),
                        Vector2::new(0.0, 0.0),
                        0.0,
                        Color::WHITE,
                    );
                }
            }
        }
    }
    fn get_texture<'a>(&'a self, textures: &'a Textures) -> Option<&'a Texture2D> {
        match self.room {
            Room::Room1 => {
                if self.active {
                    Some(&textures.tux.awake)
                } else {
                    Some(&textures.tux.inactive)
                }
            }
            Room::Room3 | Room::Room5 => Some(&textures.tux.slide),
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
                match thread_rng().gen_range(0..1) as u64 {
                    0 => self.set_room(Room::Room3),
                    _ => self.set_room(Room::Room5),
                }
                self.moved_to_hallway_at = SystemTime::now();
            }
            Room::Room3 | Room::Room5 => {
                if self.moved_to_hallway_at.elapsed().unwrap().as_secs() <= 10 {
                    if let Some(c) = self.checked_camera {
                        if c.elapsed().unwrap().as_secs() <= 1 {
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
            if c.elapsed().unwrap().as_secs() >= 2 {
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

#[monster_derive]
pub struct Nolok {}

impl Nolok {
    pub fn new() -> Self {
        Self {
            name: MonsterName::Nolok,
            room: Room::None,

            next_room: Room::None,
            ai_level: DEFAULT_AI_LEVEL,
            active: NOLOK_START,
            entered_from_left: false,
            entered_from_right: false,
            progress_to_hallway: 1,

            last_scared_at: SystemTime::now(),
            timer_until_office: SystemTime::now(),
            move_timer: 0,
            time_in_room: SystemTime::now(),

            move_after_timer: true,
        }
    }
}

impl Monster for Nolok {
    monster_function_macro!();

    fn try_move(&mut self) {
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
                }
            }
            Room::Room3 => {
                self.set_entered_from_left(true);
                self.set_room(Room::Office);
                self.set_last_scared_at(SystemTime::now());
            }
            Room::Room5 => {
                self.set_entered_from_right(true);
                self.set_room(Room::Office);
                self.set_last_scared_at(SystemTime::now());
            }
            _ => {}
        }
    }
    fn room_after_office(&mut self) -> Room {
        Room::None
    }
}

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
            active: GOLDEN_TUX_START,
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

    fn get_texture<'a>(&'a self, textures: &'a Textures) -> Option<&'a Texture2D> {
        if self.active {
            Some(&textures.golden_tux)
        } else {
            None
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
    one_am_checked: bool,
    two_am_checked: bool,
    three_am_checked: bool,
    four_am_checked: bool,
    five_am_checked: bool,
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
        }
    }

    pub fn hours(&mut self, time: Duration) -> u64 {
        time.as_secs() / 200
    }
    pub fn step(&mut self, time: Duration) -> bool {
        let hours = self.hours(time);
        self.penny.step();
        self.beastie.step();
        self.tux.step();
        self.gogopher.step();

        // every few seconds, generate a random number between 1 and 20, for each enemy. if the animatronic's current ai level is greater/equal to the number, the animatronic moves.
        if self.since_last_move.elapsed().unwrap().as_secs() >= 5 {
            self.since_last_move = SystemTime::now();
            if self.penny.active {
                if !self.penny.entered_from_left() {
                    self.penny.try_move();
                }
            }
            if self.beastie.active {
                if !self.beastie.entered_from_right() {
                    if self.beastie.last_scared_at().elapsed().unwrap().as_secs() >= 30 {
                        self.beastie.begin_move_timer();
                    } else {
                        self.beastie.try_move();
                    }
                }
            }

            if self.tux.active {
                self.tux.try_move();
            }

            if self.nolok.active {
                self.nolok.try_move();
            }
        } else {
            self.moved = true;
        }

        // 1 AM
        if hours == 1 && !self.one_am_checked {
            self.wilber.activate();
            self.one_am_checked = true;
            self.ai_level_increase();
        }
        // 2 AM
        if hours == 2 && !self.two_am_checked {
            self.gogopher.activate();
            self.two_am_checked = true;
            self.ai_level_increase();
        }
        // 3 AM
        if hours == 3 && !self.three_am_checked {
            self.tux.activate();
            self.three_am_checked = true;
            self.ai_level_increase();
            self.tux.can_move = true;
        }
        if (hours == 4 && !self.four_am_checked) || (hours == 5 && !self.five_am_checked) {
            self.tux.can_move = true;
        }
        if hours == 5 && !self.five_am_checked {
            self.tux.ai_level = 10;
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
        self.penny.ai_level += 3;
        self.beastie.ai_level += 3;
        self.wilber.ai_level += 3;
        self.gogopher.ai_level += 3;
        //self.tux.ai_level += 3;       // Tux's AI level does not increase naturally, it bumps at 5AM
        //self.nolok.ai_level += 3;     // Nolok is cut.
        self.golden_tux.ai_level += 3;
    }
}

/*

Penny: Penny is the first monster to start moving. She starts in Cam 2, then progresses to Cam 3, then the office. After she leaves cam 3, the player has a brief period to shut the door before she peeks in and starts Tainting their kernel.

Beastie: Beastie also starts at 12 AM, shortly after Penny. He is nearly identical, merely going to the right side instead.

Wilber: Around 1 AM, a hangman poster will fade into the player’s office. From now on, they’ll have to frequently check Cam 6 to check on Wilber. Wilber has a Rage meter that continuously fills whenever he is not being viewed and drains whenever Cam 6 is being watched. Whenever the meter fills, the hangman poster will progress 1 phase. If the hangman game is completed, Wilber will kill the player.

GoGopher: The Go Gopher becomes a threat at 2 AM. He’ll occasionally appear in Cam 4. The player must heat up the duct or else he’ll pop out and fill the Tainted bar by 50%.

Tux: The big bad himself, Tux, awakens at 3 AM. He starts in Tux’s Domain (Cam 1) and will be seen programming something. Eventually, he’ll leave his chair and get closer and closer to the camera while getting angrier. Then, he’ll leave the room and either slide down Cam 3 or 5. The player must shut the corresponding door or else Tux will slide into their room and instantly kill them, regardless of how Tainted they were.

Nolok: Starting at 4 AM, the player might start seeing Nolok’s glowing pupils in either of their doorways. They have a few seconds to shut the corresponding door before he peeks in and spikes their Tainted bar faster than any other character.

Golden Tux: Golden Tux is an easter egg character that has a chance of appearing in the player’s office when they drop their monitor around 5 AM. If he appears in their room, the player must quickly lift the camera to avoid an instant Game Over.

*/
