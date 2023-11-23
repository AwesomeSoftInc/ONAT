use proc::{monster_derive, monster_function_macro};
use std::time::Duration;

use rand::{thread_rng, Rng};

use crate::enums::Room;

pub const PENNY_START: bool = true;
pub const BEASTIE_START: bool = true;
pub const WILBER_START: bool = false;
pub const GO_GOPHER_START: bool = false;
pub const TUX_START: bool = false;
pub const NOLOK_START: bool = false;
pub const GOLDEN_TUX_START: bool = false;

pub const DEFAULT_AI_LEVEL: u8 = 5;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MonsterName {
    Penny,
    Beastie,
    Wilber,
    GoGopher,
    Tux,
    Nolok,
    GoldenTux,
}

pub trait Monster {
    fn id(&self) -> MonsterName;
    fn room(&self) -> &Room;
    fn ai_level(&self) -> u8;
    fn set_room(&mut self, room: Room);
    fn active(&self) -> bool;
    fn activate(&mut self);
    fn entered_from_left(&self) -> bool;
    fn entered_from_right(&self) -> bool;
    fn set_entered_from_left(&mut self, res: bool);
    fn set_entered_from_right(&mut self, res: bool);

    fn name(&self) -> String {
        return format!("{:?}", self.id());
    }
    fn taint_percent(&self) -> f32 {
        0.02
    }

    fn try_move(&mut self) {
        let chance = thread_rng().gen_range(0..20);
        // if any of them are in the hallways, have them move in.
        if self.room() == &Room::Room3 {
            self.set_entered_from_left(true);
            self.set_room(Room::Office);
        } else if self.room() == &Room::Room5 {
            self.set_entered_from_right(true);
            self.set_room(Room::Office);
        } else {
            if chance <= self.ai_level() {
                self.go_prev_or_next(chance);
            }
        }
    }

    fn go_prev_or_next(&mut self, chance: u8) {
        let b = thread_rng().gen_range(0..2);
        if b == 0 {
            self.prev();
        } else {
            self.next();
        }
    }

    fn prev(&mut self) {
        match self.room().prev() {
            crate::enums::RoomOption::Room(a) => {
                self.set_room(a);
            }
            crate::enums::RoomOption::Multiple(a) => {
                let rnd = thread_rng().gen_range(0..a.len());
                self.set_room(a.get(rnd).unwrap().clone());
            }
            crate::enums::RoomOption::None => {
                self.next();
            }
        }
    }

    fn next(&mut self) {
        match self.room().next() {
            crate::enums::RoomOption::Room(a) => {
                self.set_room(a);
            }
            crate::enums::RoomOption::Multiple(a) => {
                let rnd = thread_rng().gen_range(0..a.len());
                self.set_room(a.get(rnd).unwrap().clone());
            }
            crate::enums::RoomOption::None => {}
        }
    }

    fn room_after_office(&self) -> Room {
        Room::random()
    }

    fn special_debug_info(&self) -> String {
        String::new()
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
            ai_level: DEFAULT_AI_LEVEL,
            active: PENNY_START,
            entered_from_left: false,
            entered_from_right: false,
            door_shut: false,
        }
    }
}

impl Monster for Penny {
    monster_function_macro!();
    fn next(&mut self) {
        self.set_room(match self.room() {
            Room::Room1 => Room::Room2,
            Room::Room2 => {
                if !self.door_shut {
                    Room::Room3
                } else {
                    Room::Room1
                }
            }
            Room::Room3 => Room::Office,
            _ => {
                panic!()
            }
        });
    }
    fn room_after_office(&self) -> Room {
        Room::Room2
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
            ai_level: DEFAULT_AI_LEVEL,
            active: BEASTIE_START,
            entered_from_left: false,
            entered_from_right: false,
            door_shut: false,
        }
    }
}

impl Monster for Beastie {
    monster_function_macro!();
    fn next(&mut self) {
        self.set_room(match self.room() {
            Room::Room1 => Room::Room2,
            Room::Room2 => {
                if !self.door_shut {
                    Room::Room5
                } else {
                    Room::Room1
                }
            }
            Room::Room3 => Room::Office,
            _ => {
                panic!()
            }
        });
    }
    fn room_after_office(&self) -> Room {
        Room::Room1
    }
}

#[monster_derive]
pub struct Wilber {
    rage: f32,
    pub stage: u8,
}

impl Wilber {
    pub fn new() -> Self {
        Self {
            name: MonsterName::Wilber,
            room: Room::Room6,
            ai_level: DEFAULT_AI_LEVEL,
            active: WILBER_START,
            entered_from_left: false,
            entered_from_right: false,
            rage: 0.0,
            stage: 0,
        }
    }
    pub fn rage(&self) -> f32 {
        self.rage
    }
    pub fn rage_increment(&mut self) {
        if self.rage < 100.0 {
            self.rage += 0.001;
        } else {
            self.stage += 1;
            self.rage = 0.0;
        }
    }
    pub fn rage_decrement(&mut self) {
        if self.rage > 0.0 {
            self.rage -= 0.002;
        }
    }
}

impl Monster for Wilber {
    monster_function_macro!();
    fn special_debug_info(&self) -> String {
        format!(" - {} - {}", self.rage, self.stage)
    }
}

#[monster_derive]
pub struct GoGopher {
    pub duct_timer: u16,
    pub duct_heat_timer: u16,
}

impl GoGopher {
    pub fn new() -> Self {
        Self {
            name: MonsterName::GoGopher,
            room: Room::None,
            ai_level: DEFAULT_AI_LEVEL,
            active: GO_GOPHER_START,
            entered_from_left: false,
            entered_from_right: false,
            duct_timer: 0,
            duct_heat_timer: 0,
        }
    }
}

impl Monster for GoGopher {
    monster_function_macro!();

    fn try_move(&mut self) {
        if self.duct_heat_timer == 0 {
            match self.room {
                Room::None => {
                    let coin_flip = thread_rng().gen_range(0..10000);
                    if coin_flip <= 1 {
                        self.set_room(Room::Room4)
                    }
                }
                Room::Room4 => {
                    self.duct_timer += 1;
                    if self.duct_timer >= 2500 {
                        self.set_room(Room::Office);
                    }
                }
                Room::Office => {}
                _ => {}
            }
        } else {
            if self.duct_timer > 0 {
                self.duct_timer -= 1;
            }
            self.set_room(Room::None);
        }
    }

    fn special_debug_info(&self) -> String {
        format!("- {} - {}", self.duct_timer, self.duct_heat_timer)
    }
}

#[monster_derive]
pub struct Tux {}

impl Tux {
    pub fn new() -> Self {
        Self {
            name: MonsterName::Tux,
            room: Room::Room1,
            ai_level: DEFAULT_AI_LEVEL,
            active: TUX_START,
            entered_from_left: false,
            entered_from_right: false,
        }
    }
}

impl Monster for Tux {
    monster_function_macro!();

    fn go_prev_or_next(&mut self, _chance: u8) {
        self.next();
    }
    fn room_after_office(&self) -> Room {
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
            ai_level: DEFAULT_AI_LEVEL,
            active: NOLOK_START,
            entered_from_left: false,
            entered_from_right: false,
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
            }
            Room::Room5 => {
                self.set_entered_from_right(true);
                self.set_room(Room::Office);
            }
            _ => {}
        }
    }
    fn room_after_office(&self) -> Room {
        Room::None
    }
    fn taint_percent(&self) -> f32 {
        0.1
    }
}

#[monster_derive]
pub struct GoldenTux {}

impl GoldenTux {
    pub fn new() -> Self {
        Self {
            name: MonsterName::GoldenTux,
            room: Room::Office,
            ai_level: DEFAULT_AI_LEVEL,
            active: GOLDEN_TUX_START,
            entered_from_left: false,
            entered_from_right: false,
        }
    }
}

impl Monster for GoldenTux {
    monster_function_macro!();

    fn taint_percent(&self) -> f32 {
        0.0
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

    moved: bool,
    one_am_checked: bool,
    two_am_checked: bool,
    three_am_checked: bool,
    four_am_checked: bool,
}

fn round(num: u64, mul: u64) -> u64 {
    if num <= 60 {
        num
    } else {
        let rnd = (((num + mul - 1) / mul) * mul) - 60;
        num - rnd
    }
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
            moved: true,
            one_am_checked: false,
            two_am_checked: false,
            three_am_checked: false,
            four_am_checked: false,
        }
    }

    pub fn step(&mut self, time: Duration) {
        let hours = time.as_secs() / 3600;
        let minutes = time.as_secs() / 60;
        let seconds = round(time.as_secs(), 60);

        // every few seconds (one in game minute), generate a random number between 1 and 20, for each enemy. if the animatronic's current ai level is greater/equal to the number, the animatronic moves.
        if minutes & 1 == 1 {
            if self.moved {
                println!("NOW: {:#02}h {:#02}m {:#02}s", hours, minutes, seconds);
                self.moved = false;

                if self.penny.active {
                    self.penny.try_move();
                }
                if self.tux.active {
                    self.beastie.try_move();
                }

                // wilber doesn't move

                if self.tux.active {
                    self.tux.try_move();
                }

                if self.nolok.active {
                    self.nolok.try_move();
                }
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
        }
        // 4 AM
        if hours == 4 && !self.four_am_checked {
            self.nolok.activate();
            self.four_am_checked = true;
            self.ai_level_increase();
        }

        // gogopher gets special permission to try and move every tick
        if self.gogopher.active {
            self.gogopher.try_move();
        }
    }
    pub fn in_room(&mut self, room: &Room) -> Vec<&mut dyn Monster> {
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
        self.penny.ai_level += 5;
        self.beastie.ai_level += 5;
        self.wilber.ai_level += 5;
        self.gogopher.ai_level += 5;
        self.tux.ai_level += 5;
        self.nolok.ai_level += 5;
        self.golden_tux.ai_level += 5;
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
