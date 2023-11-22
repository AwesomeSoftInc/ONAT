use std::error::Error;

use raylib::prelude::*;

pub struct Textures {
    pub office: Texture2D,
    pub laptop: Texture2D,
    pub camera: Texture2D,
}

impl Textures {
    pub fn new(rl: &mut RaylibHandle, thread: &RaylibThread) -> Result<Self, Box<dyn Error>> {
        let office = rl.load_texture(&thread, "./assets/office.png")?;
        let laptop = rl.load_texture(&thread, "./assets/laptop.png")?;
        let camera = rl.load_texture(&thread, "./assets/camera.png")?;
        Ok(Self {
            office,
            laptop,
            camera,
        })
    }
}
