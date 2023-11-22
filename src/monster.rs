use num_traits::{FromPrimitive, ToPrimitive};
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

#[derive(Clone, Debug)]
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
    fn name(&self) -> String;
    fn room(&self) -> &Room;
    fn ai_level(&self) -> u8;
    fn set_room(&mut self, room: Room);
    fn active(&self) -> bool;
    fn activate(&mut self);

    fn taint_percent(&self) -> f32 {
        0.02
    }

    fn move_by(&mut self, move_by: i64) {
        let mut room = self.room().to_u64().unwrap() as i64;
        room += move_by;
        self.set_room(Room::from_u64(room as u64).unwrap());
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
            if chance >= self.ai_level() {
                let b = thread_rng().gen_range(0..1);
                if b == 0 {
                    self.prev();
                } else {
                    self.next();
                }
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

    fn entered_from_left(&self) -> bool;
    fn entered_from_right(&self) -> bool;
    fn set_entered_from_left(&mut self, res: bool);
    fn set_entered_from_right(&mut self, res: bool);
}

#[monster_derive]
pub struct Penny {}

impl Penny {
    pub fn new() -> Self {
        Self {
            name: MonsterName::Penny,
            room: Room::Room2,
            ai_level: thread_rng().gen_range(0..20),
            active: PENNY_START,
            entered_from_left: false,
            entered_from_right: false,
        }
    }
}

impl Monster for Penny {
    monster_function_macro!();
    fn next(&mut self) {
        match self.room() {
            Room::Room1 => self.set_room(Room::Room2),
            Room::Room2 => self.set_room(Room::Room3),
            Room::Room3 => self.set_room(Room::Office),
            _ => {}
        }
    }
}

#[monster_derive]
pub struct Beastie {}

impl Beastie {
    pub fn new() -> Self {
        Self {
            name: MonsterName::Beastie,
            room: Room::Room2,
            ai_level: thread_rng().gen_range(0..20),
            active: BEASTIE_START,
            entered_from_left: false,
            entered_from_right: false,
        }
    }
}

impl Monster for Beastie {
    monster_function_macro!();
    fn next(&mut self) {
        match self.room() {
            Room::Room1 => self.set_room(Room::Room2),
            Room::Room2 => self.set_room(Room::Room5),
            Room::Room3 => self.set_room(Room::Office),
            _ => {}
        }
    }
}

#[monster_derive]
pub struct Wilber {}

impl Wilber {
    pub fn new() -> Self {
        Self {
            name: MonsterName::Wilber,
            room: Room::Room6,
            ai_level: thread_rng().gen_range(0..20),
            active: WILBER_START,
            entered_from_left: false,
            entered_from_right: false,
        }
    }
}

impl Monster for Wilber {
    monster_function_macro!();
}

#[monster_derive]
pub struct GoGopher {}

impl GoGopher {
    pub fn new() -> Self {
        Self {
            name: MonsterName::GoGopher,
            room: Room::Room4,
            ai_level: thread_rng().gen_range(0..20),
            active: GO_GOPHER_START,
            entered_from_left: false,
            entered_from_right: false,
        }
    }
}

impl Monster for GoGopher {
    monster_function_macro!();
}

#[monster_derive]
pub struct Tux {}

impl Tux {
    pub fn new() -> Self {
        Self {
            name: MonsterName::Tux,
            room: Room::Room1,
            ai_level: thread_rng().gen_range(0..20),
            active: TUX_START,
            entered_from_left: false,
            entered_from_right: false,
        }
    }
}

impl Monster for Tux {
    monster_function_macro!();
}

#[monster_derive]
pub struct Nolok {}

impl Nolok {
    pub fn new() -> Self {
        Self {
            name: MonsterName::Nolok,
            room: Room::None,
            ai_level: thread_rng().gen_range(0..20),
            active: NOLOK_START,
            entered_from_left: false,
            entered_from_right: false,
        }
    }
}

impl Monster for Nolok {
    monster_function_macro!();
}

#[monster_derive]
pub struct GoldenTux {}

impl GoldenTux {
    pub fn new() -> Self {
        Self {
            name: MonsterName::GoldenTux,
            room: Room::Office,
            ai_level: thread_rng().gen_range(0..20),
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
    penny: Penny,
    beastie: Beastie,
    wilber: Wilber,
    gogopher: GoGopher,
    tux: Tux,
    nolok: Nolok,
    golden_tux: GoldenTux,

    moved: bool,
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

                self.penny.try_move();
                self.beastie.try_move();
            }
        } else {
            self.moved = true;
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
