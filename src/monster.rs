use num_traits::{FromPrimitive, ToPrimitive};
use proc::{monster_derive, monster_function_macro};
use std::time::{Duration, SystemTime};

use rand::{rngs, thread_rng, Rng};

use crate::enums::Room;

#[derive(Clone, Debug)]
pub enum MonsterName {
    Penny,
    Beastie,
    Wilber,
    GoGopher,
    Tux,
    Nolok,
    GoldenTux,
    Null,
}

pub trait Monster {
    fn name(&self) -> String;
    fn room(&self) -> &Room;
    fn ai_level(&self) -> u8;
    fn set_room(&mut self, room: Room);
    fn active(&self) -> bool;

    fn taint_percent(&self) -> f32 {
        0.02
    }

    fn move_by(&mut self, move_by: i64) {
        let mut room = self.room().to_u64().unwrap() as i64;
        room += move_by;
        self.set_room(Room::from_u64(room as u64).unwrap());
    }

    fn try_move(&mut self, left_door_shut: bool, right_door_shut: bool) {
        let chance = thread_rng().gen_range(0..20);
        if chance >= self.ai_level() {
            // if any of them are in the hallways, have them move in.
            if self.room() == &Room::Room3 || self.room() == &Room::Room6 {
                self.set_room(Room::Office);
            } else {
                let b = thread_rng().gen_range(0..1);
                if b == 0 {
                    self.prev(left_door_shut, right_door_shut);
                } else {
                    self.next(left_door_shut, right_door_shut);
                }
            }
        }
    }

    fn next(&mut self, left_door_shut: bool, right_door_shut: bool) {
        println!("next");
        match self.room().next(left_door_shut, right_door_shut) {
            crate::enums::RoomOption::Room(a) => self.set_room(a),
            crate::enums::RoomOption::Multiple(a) => {
                let rnd = thread_rng().gen_range(0..a.len());
                self.set_room(a.get(rnd).unwrap().clone());
            }
            crate::enums::RoomOption::None => {}
        }
    }
    fn prev(&mut self, left_door_shut: bool, right_door_shut: bool) {
        println!("prev");
        match self.room().prev(left_door_shut, right_door_shut) {
            crate::enums::RoomOption::Room(a) => self.set_room(a),
            crate::enums::RoomOption::Multiple(a) => {
                let rnd = thread_rng().gen_range(0..a.len());
                self.set_room(a.get(rnd).unwrap().clone());
            }
            crate::enums::RoomOption::None => {
                self.next(left_door_shut, right_door_shut);
            }
        }
    }
}

#[monster_derive]
pub struct Penny {}

impl Penny {
    pub fn new() -> Self {
        Self {
            name: MonsterName::Penny,
            room: Room::Room2,
            ai_level: thread_rng().gen_range(0..20),
            active: false,
        }
    }
}

impl Monster for Penny {
    monster_function_macro!();
}

#[monster_derive]
pub struct Beastie {}

impl Beastie {
    pub fn new() -> Self {
        Self {
            name: MonsterName::Beastie,
            room: Room::Room2,
            ai_level: thread_rng().gen_range(0..20),
            active: false,
        }
    }
}

impl Monster for Beastie {
    monster_function_macro!();
}

#[monster_derive]
pub struct Wilber {}

impl Wilber {
    pub fn new() -> Self {
        Self {
            name: MonsterName::Wilber,
            room: Room::Room6,
            ai_level: thread_rng().gen_range(0..20),
            active: false,
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
            active: false,
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
            active: false,
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
            active: false,
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
            active: false,
        }
    }
}

impl Monster for GoldenTux {
    monster_function_macro!();

    fn taint_percent(&self) -> f32 {
        0.0
    }
}

#[monster_derive]
pub struct NullMonster {}

impl NullMonster {
    pub fn new() -> Self {
        Self {
            name: MonsterName::Null,
            room: Room::None,
            ai_level: 0,
            active: false,
        }
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

    // every few seconds, generate a random number between 1 and 20, for each enemy. if the animatronic's current ai level is greater/equal to the number, the animatronic moves.

    pub fn step(&mut self, time: Duration, left_door_shut: bool, right_door_shut: bool) {
        let hours = time.as_secs() / 3600;
        let minutes = time.as_secs() / 60;
        let seconds = round(time.as_secs(), 60);

        if minutes & 1 == 1 {
            if self.moved {
                println!("NOW: {:#02}h {:#02}m {:#02}s", hours, minutes, seconds);
                self.moved = false;

                self.penny.try_move(left_door_shut, right_door_shut);
                self.beastie.try_move(left_door_shut, right_door_shut);
            }
        } else {
            self.moved = true;
        }
    }
    fn push_if_in_room<'a, A>(&self, mon: &'a A, room: &Room, vec: &mut Vec<&'a dyn Monster>)
    where
        A: Monster,
    {
        if mon.room() == &room.clone() {
            vec.push(mon);
        }
    }
    pub fn in_room(&mut self, room: &Room) -> Vec<&dyn Monster> {
        let mut res = vec![];
        self.push_if_in_room(&self.penny, &room, &mut res);
        self.push_if_in_room(&self.beastie, &room, &mut res);
        self.push_if_in_room(&self.wilber, &room, &mut res);
        self.push_if_in_room(&self.gogopher, &room, &mut res);
        self.push_if_in_room(&self.tux, &room, &mut res);
        self.push_if_in_room(&self.nolok, &room, &mut res);
        self.push_if_in_room(&self.golden_tux, &room, &mut res);

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
