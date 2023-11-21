use num_derive::FromPrimitive;
use num_traits::FromPrimitive;

use rand::{thread_rng, Rng, RngCore};

pub enum Screen {
    Office,
    Camera,
    GameOver,
}

extern crate num_derive;
#[derive(FromPrimitive, PartialEq, Clone)]
pub enum Room {
    Room1,
    Room2,
    Room3A,
    Room3B,
    Room3C,
    Room4A,
    Room4B,
    Room5A,
    Room5B,
    Room6,
    None,
    Office,
}

impl Room {
    pub fn random() -> Self {
        let ran = thread_rng().gen_range(0..8);
        match Room::from_u64(ran as u64) {
            Some(a) => a,
            None => Room::None, // should never happen
        }
    }
    pub fn random_three() -> Self {
        let ran = thread_rng().gen_range(0..2);
        match ran {
            0 => Room::Room3A,
            1 => Room::Room3B,
            2 => Room::Room3C,
            _ => Room::None, // should never happen
        }
    }
}
