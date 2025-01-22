use std::{
    fs::OpenOptions,
    io::{Read, Write},
    sync::OnceLock,
};

use raylib::prelude::*;

use serde::{Deserialize, Serialize};

use crate::textures::Textures;

#[derive(Clone, Copy)]
pub struct Dimensions {
    width: f32,
    height: f32,
    ratio: f32,
    margin: f32,
}

#[repr(u32)]
#[derive(Serialize, Deserialize, Clone, Copy)]
pub enum WritableTextureFilter {
    Point = TextureFilter::TEXTURE_FILTER_POINT as u32,
    Bilinear = TextureFilter::TEXTURE_FILTER_BILINEAR as u32,
    Trilinear = TextureFilter::TEXTURE_FILTER_TRILINEAR as u32,
    Anisotrpoic4x = TextureFilter::TEXTURE_FILTER_ANISOTROPIC_4X as u32,
    Anisotrpoic8x = TextureFilter::TEXTURE_FILTER_ANISOTROPIC_8X as u32,
    Anisotrpoic16x = TextureFilter::TEXTURE_FILTER_ANISOTROPIC_16X as u32,
}

#[derive(Serialize, Deserialize)]
pub struct Writable {
    ui_scale: f32,
    fullscreen: bool,
    night_2_unlocked: bool,
    on_tutorial: bool,
    texture_filter: WritableTextureFilter,
    volume: i32,
}

impl Default for Writable {
    fn default() -> Self {
        Self {
            ui_scale: 2.0,
            fullscreen: true,
            night_2_unlocked: false,
            on_tutorial: true,
            texture_filter: WritableTextureFilter::Bilinear,
            volume: 100,
        }
    }
}

pub struct Config {
    writable: Writable,

    // The framebuffer the user can set for the game.
    emulated_dimensions: Dimensions,

    // The real dimensions of the user's screen.
    real_dimensions: Dimensions,

    // Night 2
    night_2: bool,

    dimensions_fn: fn(&Self) -> Dimensions,
}

impl Config {
    pub fn new() -> Self {
        let cfg_dir = dirs::config_dir()
            .expect("A standard config directory could not be found on your OS.")
            .join(".onat");

        std::fs::create_dir_all(cfg_dir.clone()).unwrap();
        let cfg_path = cfg_dir.join("config.toml");
        let mut cfg_file = OpenOptions::new()
            .create(true)
            .write(true)
            .read(true)
            .append(false)
            .open(cfg_path)
            .unwrap();

        let mut buf = String::new();
        cfg_file.read_to_string(&mut buf).unwrap();

        let writable: Writable = toml::from_str(&mut buf).unwrap_or_else(|err| {
            println!("[ERROR] {}", err.to_string());
            Writable::default()
        });
        Self::write_static(&Writable::default());

        // Get the user's real screen dimensions.
        let (rl, _) = raylib::init().title("ONAT Screen Size checker").build();
        let real_dimensions = calculate_dimensions();

        drop(rl);

        let emulated_dimensions = Dimensions {
            width: 1440.0,
            height: 1080.0,
            margin: 0.0,
            ratio: 1.0,
        };

        Self {
            writable,
            emulated_dimensions,
            real_dimensions,
            dimensions_fn: fullscreen_dimensions_fn,
            night_2: false,
        }
    }

    pub fn write_static(writable: &Writable) {
        let cfg_dir = dirs::config_dir()
            .expect("A standard config directory could not be found on your OS.")
            .join(".onat");

        std::fs::create_dir_all(cfg_dir.clone()).unwrap();
        let cfg_path = cfg_dir.join("config.toml");

        let mut file = OpenOptions::new()
            .create(true)
            .write(true)
            .read(true)
            .append(false)
            .open(cfg_path)
            .unwrap();

        file.set_len(0).unwrap();

        file.write(toml::to_string(writable).unwrap().as_bytes())
            .unwrap();
    }

    pub fn write(&self) {
        Self::write_static(&self.writable);
    }

    pub fn ui_scale(&self) -> f32 {
        self.writable.ui_scale
    }

    pub fn fullscreen(&self) -> bool {
        self.writable.fullscreen
    }
    pub fn night_2_unlocked(&self) -> bool {
        self.writable.night_2_unlocked
    }

    pub fn on_tutorial(&self) -> bool {
        self.writable.on_tutorial
    }

    pub fn set_ui_scale(&mut self, val: f32) {
        self.writable.ui_scale = val;
        self.write();
    }

    pub fn unlock_night_2(&mut self) {
        self.writable.night_2_unlocked = true;
        self.write();
    }
    pub fn set_on_tutorial(&mut self, val: bool) {
        self.writable.on_tutorial = val;
        self.write();
    }

    pub fn toggle_fullscreen(&mut self, rl: &mut RaylibHandle) {
        rl.toggle_fullscreen();
        self.writable.fullscreen = !self.writable.fullscreen;
        if self.writable.fullscreen {
            self.dimensions_fn = fullscreen_dimensions_fn;
        } else {
            self.dimensions_fn = windowed_dimensions_fn;
        }

        self.write();
    }
    pub fn set_fullscreen(&mut self, rl: &mut RaylibHandle, val: bool) {
        self.writable.fullscreen = val;
        if self.writable.fullscreen {
            self.dimensions_fn = fullscreen_dimensions_fn;
            if !rl.is_window_fullscreen() {
                rl.toggle_fullscreen();
            }
        } else {
            self.dimensions_fn = windowed_dimensions_fn;
            if rl.is_window_fullscreen() {
                rl.toggle_fullscreen();
            }
        }

        self.write();
    }

    pub fn night_2(&self) -> bool {
        self.night_2
    }
    pub fn set_night_2(&mut self, val: bool) {
        self.night_2 = val;
    }

    pub fn width(&self) -> i32 {
        (self.emulated_dimensions.width / self.emulated_dimensions.ratio) as i32
    }
    pub fn width_raw(&self) -> i32 {
        self.emulated_dimensions.width as i32
    }
    pub fn height(&self) -> i32 {
        self.emulated_dimensions.height as i32
    }

    pub fn real_width(&self) -> i32 {
        ((self.dimensions_fn)(self).width / (self.dimensions_fn)(self).ratio) as i32
    }
    pub fn real_width_raw(&self) -> i32 {
        (self.dimensions_fn)(self).width as i32
    }
    pub fn real_height(&self) -> i32 {
        (self.dimensions_fn)(self).height as i32
    }
    pub fn real_margin(&self) -> f32 {
        (self.dimensions_fn)(self).margin
    }

    pub fn texture_filter(&self) -> TextureFilter {
        unsafe { std::mem::transmute(self.writable.texture_filter) }
    }
    pub fn set_texture_filter(
        &mut self,
        textures: &mut Textures,
        rl: &mut RaylibHandle,
        thread: &RaylibThread,
        val: TextureFilter,
    ) {
        textures.set_texture_filter(rl, thread, val);
        self.writable.texture_filter = unsafe { std::mem::transmute(val) }
    }

    pub fn volume(&self) -> i32 {
        self.writable.volume
    }
    pub fn set_volume(&mut self, val: i32) {
        self.writable.volume = val;
        self.write();
    }
}

static mut CONFIG: OnceLock<Config> = OnceLock::new();

pub fn config<'a>() -> &'a Config {
    unsafe { CONFIG.get_or_init(|| Config::new()) }
}
pub fn config_mut<'a>() -> &'a mut Config {
    unsafe { CONFIG.get_mut().unwrap() }
}

fn calculate_dimensions() -> Dimensions {
    let monitor_width = get_monitor_width(get_current_monitor_index());
    let monitor_height = get_monitor_height(get_current_monitor_index());

    let default_ratio = monitor_width as f32 / monitor_height as f32;
    let desired_ratio = 4.0 / 3.0;
    let ratio = 1.0 + (default_ratio - desired_ratio);

    let mut margin = monitor_width as f32 - ((monitor_width as f32) / ratio);
    if margin < 0.0 {
        margin = 0.0;
    }
    margin /= 2.0;
    Dimensions {
        width: monitor_width as f32,
        height: monitor_height as f32,
        margin: margin,
        ratio: ratio,
    }
}

fn fullscreen_dimensions_fn(se: &Config) -> Dimensions {
    se.real_dimensions
}

fn windowed_dimensions_fn(_se: &Config) -> Dimensions {
    unsafe {
        let monitor_width = ffi::GetRenderWidth();
        let monitor_height = ffi::GetRenderHeight();
        let default_ratio = monitor_width as f32 / monitor_height as f32;
        let desired_ratio = 4.0 / 3.0;
        let ratio = 1.0 + (default_ratio - desired_ratio);

        let mut margin = monitor_width as f32 - ((monitor_width as f32) / ratio);
        if margin < 0.0 {
            margin = 0.0;
        }
        margin /= 2.0;

        Dimensions {
            width: monitor_width as f32,
            height: monitor_height as f32,
            ratio: ratio,
            margin: margin,
        }
    }
}
