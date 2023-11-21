use std::time::{Duration, SystemTime};

use rand::rngs;

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

#[derive(Clone)]
pub struct Monster {
    name: MonsterName,
    room: Room,
}

impl Monster {
    fn new(name: MonsterName, room: Room) -> Self {
        Self { name, room }
    }
    fn penny() -> Self {
        Self::new(MonsterName::Penny, Room::Room2)
    }
    fn beastie() -> Self {
        Self::new(MonsterName::Beastie, Room::random())
    }
    fn wilber() -> Self {
        Self::new(MonsterName::Wilber, Room::Room6)
    }
    fn gogopher() -> Self {
        Self::new(MonsterName::GoGopher, Room::random_three())
    }
    fn tux() -> Self {
        Self::new(MonsterName::Tux, Room::Room1)
    }
    fn nolok() -> Self {
        Self::new(MonsterName::Nolok, Room::None)
    }
    fn golden_tux() -> Self {
        Self::new(MonsterName::GoldenTux, Room::Office) // there is no room
    }
    fn null() -> Self {
        Self::new(MonsterName::Null, Room::None)
    }

    pub fn name(&self) -> String {
        return format!("{:?}", self.name);
    }

    fn set_room(&mut self, room: Room) {
        self.room = room;
    }
}

pub struct Gang {
    penny: Monster,
    beastie: Monster,
    wilber: Monster,
    gogopher: Monster,
    tux: Monster,
    nolok: Monster,
    golden_tux: Monster,
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
            penny: Monster::penny(),
            beastie: Monster::beastie(),
            wilber: Monster::null(),
            gogopher: Monster::null(),
            tux: Monster::null(),
            nolok: Monster::null(),
            golden_tux: Monster::null(),
        }
    }
    pub fn step(&mut self, time: Duration) {
        let hours = time.as_secs() / 3600;
        let minutes = time.as_secs() / 60;
        let seconds = round(time.as_secs(), 60);

        if minutes == 1 {
            self.penny.set_room(Room::Office);
        }
        println!("{:#02}h {:#02}m {:#02}s", hours, minutes, seconds);
    }
    fn push_if_in_room(&self, mon: &Monster, room: &Room, vec: &mut Vec<Monster>) {
        if mon.room == room.clone() {
            vec.push(mon.clone());
        }
    }
    pub fn in_room(&mut self, room: &Room) -> Vec<Monster> {
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
