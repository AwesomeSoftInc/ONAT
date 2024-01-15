use std::{error::Error, io::Read};

fn decompress(data: Vec<u8>) -> Vec<u8> {
    let mut d = GzDecoder::new(data.as_slice());
    let mut buf = Vec::new();
    d.read_to_end(&mut buf).unwrap();
    buf
}

use flate2::bufread::GzDecoder;
use proc::asset_fill;
use raylib::prelude::*;
asset_fill!();
