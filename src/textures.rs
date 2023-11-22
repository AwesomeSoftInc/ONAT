use std::error::Error;

use raylib::prelude::*;

pub struct Textures {
    pub office: Texture2D,
    pub laptop: Texture2D,
    pub camera: Texture2D,

    pub gimp1: Texture2D,
    pub gimp2: Texture2D,
    pub gimp3: Texture2D,
    pub gimp4: Texture2D,
    pub gimp5: Texture2D,
}

impl Textures {
    pub fn new(rl: &mut RaylibHandle, thread: &RaylibThread) -> Result<Self, Box<dyn Error>> {
        let office = rl.load_texture(&thread, "./assets/office.png")?;
        let laptop = rl.load_texture(&thread, "./assets/laptop.png")?;
        let camera = rl.load_texture(&thread, "./assets/camera.png")?;

        let gimp1 = rl.load_texture(&thread, "./assets/gimp1.png")?;
        let gimp2 = rl.load_texture(&thread, "./assets/gimp2.png")?;
        let gimp3 = rl.load_texture(&thread, "./assets/gimp3.png")?;
        let gimp4 = rl.load_texture(&thread, "./assets/gimp4.png")?;
        let gimp5 = rl.load_texture(&thread, "./assets/gimp5.png")?;

        Ok(Self {
            office,
            laptop,
            camera,
            gimp1,
            gimp2,
            gimp3,
            gimp4,
            gimp5,
        })
    }
}
