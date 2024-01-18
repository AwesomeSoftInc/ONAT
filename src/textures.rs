use std::{error::Error, io::Read};

#[cfg(not(debug_assertions))]
fn decompress(data: Vec<u8>) -> Vec<u8> {
    let mut d = GzDecoder::new(data.as_slice());
    let mut buf = Vec::new();
    d.read_to_end(&mut buf).unwrap();
    buf
}

#[cfg(debug_assertions)]
#[inline(always)]
fn decompress(data: Vec<u8>) -> Vec<u8> {
    data
}

use proc::asset_fill;
use raylib::prelude::*;
asset_fill!();
