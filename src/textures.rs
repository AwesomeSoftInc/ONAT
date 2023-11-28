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

    pub cam1: Texture2D,
    pub cam2: Texture2D,
    pub cam3: Texture2D,
    pub cam4: Texture2D,
    pub cam5: Texture2D,
    pub cam6: Texture2D,

    pub penny_stock_texture: Texture2D,
    pub beastie_stock_texture: Texture2D,
    pub wilber_stock_texture: Texture2D,
    pub gogopher_stock_texture: Texture2D,
    pub tux_stock_texture: Texture2D,
    pub nolok_stock_texture: Texture2D,
    pub golden_tux_texture: Texture2D,
}

impl Textures {
    pub fn new(rl: &mut RaylibHandle, thread: &RaylibThread) -> Result<Self, Box<dyn Error>> {
        let office = rl.load_texture(&thread, "./assets/office2.png")?;
        let laptop = rl.load_texture(&thread, "./assets/laptop.png")?;
        let camera = rl.load_texture(&thread, "./assets/camera.png")?;

        let gimp1 = rl.load_texture(&thread, "./assets/gimp1.png")?;
        let gimp2 = rl.load_texture(&thread, "./assets/gimp2.png")?;
        let gimp3 = rl.load_texture(&thread, "./assets/gimp3.png")?;
        let gimp4 = rl.load_texture(&thread, "./assets/gimp4.png")?;
        let gimp5 = rl.load_texture(&thread, "./assets/gimp5.png")?;

        let cam1 = rl.load_texture(&thread, "./assets/cam1.png")?;
        let cam2 = rl.load_texture(&thread, "./assets/cam2.png")?;
        let cam3 = rl.load_texture(&thread, "./assets/cam3.png")?;
        let cam4 = rl.load_texture(&thread, "./assets/cam4.png")?;
        let cam5 = rl.load_texture(&thread, "./assets/cam5.png")?;
        let cam6 = rl.load_texture(&thread, "./assets/cam6.png")?;

        let penny_stock_texture = rl.load_texture(&thread, "./assets/stock_penny.png")?;
        let beastie_stock_texture = rl.load_texture(&thread, "./assets/stock_beastie.png")?;
        let wilber_stock_texture = rl.load_texture(&thread, "./assets/stock_wilber.png")?;
        let gogopher_stock_texture = rl.load_texture(&thread, "./assets/stock_gogopher.png")?;
        let tux_stock_texture = rl.load_texture(&thread, "./assets/stock_tux.png")?;
        let nolok_stock_texture = rl.load_texture(&thread, "./assets/stock_nolok.png")?;
        let golden_tux_texture = rl.load_texture(&thread, "./assets/stock_golden_tux.png")?;

        office.set_texture_filter(&thread, TextureFilter::TEXTURE_FILTER_BILINEAR);
        laptop.set_texture_filter(&thread, TextureFilter::TEXTURE_FILTER_BILINEAR);
        camera.set_texture_filter(&thread, TextureFilter::TEXTURE_FILTER_BILINEAR);
        gimp1.set_texture_filter(&thread, TextureFilter::TEXTURE_FILTER_BILINEAR);
        gimp2.set_texture_filter(&thread, TextureFilter::TEXTURE_FILTER_BILINEAR);
        gimp3.set_texture_filter(&thread, TextureFilter::TEXTURE_FILTER_BILINEAR);
        gimp4.set_texture_filter(&thread, TextureFilter::TEXTURE_FILTER_BILINEAR);
        gimp5.set_texture_filter(&thread, TextureFilter::TEXTURE_FILTER_BILINEAR);
        cam1.set_texture_filter(&thread, TextureFilter::TEXTURE_FILTER_BILINEAR);
        cam2.set_texture_filter(&thread, TextureFilter::TEXTURE_FILTER_BILINEAR);
        cam3.set_texture_filter(&thread, TextureFilter::TEXTURE_FILTER_BILINEAR);
        cam4.set_texture_filter(&thread, TextureFilter::TEXTURE_FILTER_BILINEAR);
        cam5.set_texture_filter(&thread, TextureFilter::TEXTURE_FILTER_BILINEAR);
        cam6.set_texture_filter(&thread, TextureFilter::TEXTURE_FILTER_BILINEAR);

        penny_stock_texture.set_texture_filter(&thread, TextureFilter::TEXTURE_FILTER_BILINEAR);
        beastie_stock_texture.set_texture_filter(&thread, TextureFilter::TEXTURE_FILTER_BILINEAR);
        wilber_stock_texture.set_texture_filter(&thread, TextureFilter::TEXTURE_FILTER_BILINEAR);
        gogopher_stock_texture.set_texture_filter(&thread, TextureFilter::TEXTURE_FILTER_BILINEAR);
        tux_stock_texture.set_texture_filter(&thread, TextureFilter::TEXTURE_FILTER_BILINEAR);
        nolok_stock_texture.set_texture_filter(&thread, TextureFilter::TEXTURE_FILTER_BILINEAR);
        golden_tux_texture.set_texture_filter(&thread, TextureFilter::TEXTURE_FILTER_BILINEAR);

        Ok(Self {
            office,
            laptop,
            camera,
            gimp1,
            gimp2,
            gimp3,
            gimp4,
            gimp5,
            cam1,
            cam2,
            cam3,
            cam4,
            cam5,
            cam6,
            penny_stock_texture,
            beastie_stock_texture,
            wilber_stock_texture,
            gogopher_stock_texture,
            tux_stock_texture,
            nolok_stock_texture,
            golden_tux_texture,
        })
    }
}
