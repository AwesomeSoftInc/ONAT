use std::sync::{LazyLock, OnceLock};

use raylib::{
    ffi::{self, GetRenderHeight, GetRenderWidth},
    window::{get_current_monitor_index, get_monitor_height, get_monitor_width},
    RaylibHandle,
};

#[derive(Clone, Copy)]
pub struct Dimensions {
    width: f32,
    height: f32,
    ratio: f32,
    margin: f32,
}

pub struct Config {
    // The framebuffer the user can set for the game.
    emulated_dimensions: Dimensions,

    // The real dimensions of the user's screen.
    real_dimensions: Dimensions,

    // The UI scale.
    ui_scale: f32,

    // Fullscreen?
    fullscreen: bool,
    dimensions_fn: fn(&Self) -> Dimensions,
}

impl Config {
    pub fn new() -> Self {
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
            emulated_dimensions,
            real_dimensions,
            ui_scale: 2.0,
            fullscreen: true,
            dimensions_fn: fullscreen_dimensions_fn,
        }
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

    pub fn ui_scale(&self) -> f32 {
        self.ui_scale
    }

    pub fn set_ui_scale(&mut self, val: f32) {
        self.ui_scale = val;
    }

    pub fn toggle_fullscreen(&mut self, rl: &mut RaylibHandle) {
        rl.toggle_fullscreen();
        self.fullscreen = !self.fullscreen;
        if self.fullscreen {
            self.dimensions_fn = fullscreen_dimensions_fn;
        } else {
            self.dimensions_fn = windowed_dimensions_fn;
        }
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
        let monitor_width = GetRenderWidth();
        let monitor_height = GetRenderHeight();
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
