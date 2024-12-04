use std::sync::{LazyLock, OnceLock};

use raylib::window::{get_current_monitor_index, get_monitor_height, get_monitor_width};

pub struct Dimensions {
    width: f32,
    height: f32,
    ratio: f32,
    margin: f32,
}

pub struct Config {    
    // The framebuffer the user can set for the game. Mutable
    emulated_dimensions: Dimensions,

    // The real dimensions of the user's screen. Immutable.
    real_dimensions: Dimensions,
}

impl Config {
    pub fn new() -> Self {
        // Get the user's real screen dimensions.
        let (rl, _) = raylib::init().title("ONAT Screen Size checker").build();
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

        drop(rl);


        let emulated_dimensions = Dimensions {width: 1440.0, height: 1080.0, margin: 0.0, ratio: 4.0 / 3.0};
        let real_dimensions = Dimensions { width: monitor_width as f32, height: monitor_height as f32, margin: margin, ratio: ratio };

        Self {
            emulated_dimensions,
            real_dimensions,
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
    pub fn margin(&self) -> f32 {
        self.emulated_dimensions.margin
    }
    pub fn ratio(&self) -> f32 {
        self.emulated_dimensions.ratio
    }

    pub fn real_width(&self) -> i32 {
        (self.real_dimensions.width / self.real_dimensions.ratio) as i32
    }
    pub fn real_width_raw(&self) -> i32 {
        self.real_dimensions.width as i32
    }
    pub fn real_height(&self) -> i32 {
        self.real_dimensions.height as i32
    }
    pub fn real_margin(&self) -> f32 {
        self.real_dimensions.margin
    }
    pub fn real_ratio(&self) -> f32 {
        self.real_dimensions.ratio
    }
}


static mut CONFIG: OnceLock<Config> = OnceLock::new();

pub fn config<'a>() -> &'a Config {
    unsafe { CONFIG.get_or_init(|| Config::new()) }
}