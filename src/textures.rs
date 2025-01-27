#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use parking_lot::{Mutex, MutexGuard};
use proc::asset_fill;
use raylib::prelude::*;
use std::error::Error;
asset_fill!();

impl Textures {
    pub fn set_texture_filter(
        &self,
        rl: &mut RaylibHandle,
        thread: &RaylibThread,
        filter: TextureFilter,
    ) {
        self.bsd.set_texture_filter(rl, thread, filter);
        self.golden_tux.set_texture_filter(rl, thread, filter);
        self.gopher.set_texture_filter(rl, thread, filter);
        self.misc.set_texture_filter(rl, thread, filter);
        self.office.set_texture_filter(rl, thread, filter);
        self.penny.set_texture_filter(rl, thread, filter);
        self.rooms.set_texture_filter(rl, thread, filter);
        self.tux.set_texture_filter(rl, thread, filter);
        self.wilbur.set_texture_filter(rl, thread, filter);
    }
}
