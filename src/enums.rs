use num_derive::{FromPrimitive, ToPrimitive};
use num_traits::FromPrimitive;

use rand::{thread_rng, Rng};

pub enum Screen {
    Office,
    Camera,
    GameOver,
}

extern crate num_derive;
#[derive(FromPrimitive, ToPrimitive, PartialEq, Clone, Debug)]
pub enum Room {
    Room1,
    Room2,
    Room3,
    Room5,
    Room4,
    Room6,

    None,
    Office,
}

pub enum RoomOption {
    Room(Room),
    Multiple(Vec<Room>),
    None,
}

impl Room {
    pub fn random() -> Self {
        let ran = thread_rng().gen_range(0..3);
        match Room::from_u64(ran as u64) {
            Some(a) => a,
            None => Room::None, // should never happen
        }
    }

    pub fn prev(&self) -> RoomOption {
        match self {
            Room::Room1 => RoomOption::None,
            Room::Room2 => RoomOption::Room(Room::Room1),
            Room::Room3 => RoomOption::Multiple(vec![Room::Room1, Room::Room2]),
            Room::Room5 => RoomOption::Multiple(vec![Room::Room1, Room::Room2]),
            Room::Room4 => RoomOption::None,
            Room::Room6 => RoomOption::None,
            Room::None => RoomOption::None,
            Room::Office => RoomOption::Room(Room::Office),
        }
    }

    pub fn next(&self) -> RoomOption {
        match self {
            Room::Room1 => RoomOption::Multiple(vec![Room::Room3, Room::Room5]),
            Room::Room2 => RoomOption::Multiple(vec![Room::Room3, Room::Room5]),
            Room::Room3 => RoomOption::Room(Room::Office),
            Room::Room5 => RoomOption::Room(Room::Office),
            Room::Room4 => RoomOption::None,
            Room::Room6 => RoomOption::None,
            Room::None => RoomOption::None,
            Room::Office => RoomOption::Room(Room::Office),
        }
    }
}
