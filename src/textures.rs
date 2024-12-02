#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use parking_lot::{Mutex, MutexGuard};
use proc::asset_fill;
use raylib::prelude::*;
use std::error::Error;
asset_fill!();
