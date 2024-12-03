use once_cell::sync::Lazy;
use raylib::prelude::*;

use state::State;
use std::{
    error::Error,
    time::{Duration, SystemTime},
};
use textures::Textures;

use crate::audio::Audio;

mod audio;
mod enums;
mod macros;
mod monster;
mod state;
mod textures;

pub struct ScreenInfo {
    width: i32,
    height: i32,
    ratio: f32,
    margin: f32,
}

impl ScreenInfo {
    pub fn new() -> Self {
        let (rl, _) = raylib::init().title("ONAT").build();
        let monitor_width = get_monitor_width(get_current_monitor_index());
        let monitor_height = get_monitor_height(get_current_monitor_index());

        let default_ratio = monitor_width as f32 / monitor_height as f32;
        let desired_ratio = 4.0 / 3.0;
        let ratio = 1.0 + (default_ratio - desired_ratio);

        let mut margin = monitor_width as f32 - ((monitor_width as f32) / ratio);
        if margin < 0.0 {
            margin = 0.0;
        }

        drop(rl);

        Self {
            width: monitor_width,
            height: monitor_height,
            ratio: ratio,
            margin: margin,
        }
    }
}

pub static mut SCREEN: Lazy<ScreenInfo> = Lazy::new(|| ScreenInfo::new());

pub fn get_width() -> i32 {
    unsafe { ((SCREEN.width as f32) / get_ratio()) as i32 }
}

pub fn get_width_unaltered() -> i32 {
    unsafe { (SCREEN.width as f32) as i32 }
}
pub fn get_height() -> i32 {
    unsafe { SCREEN.height }
}

pub fn get_margin() -> f32 {
    unsafe { SCREEN.margin / 2.0 }
}

pub fn get_ratio() -> f32 {
    unsafe { SCREEN.ratio }
}

#[error_window::main]
fn main() -> Result<(), Box<dyn Error>> {
    get_width(); // initializes the SCREEN variable.

    let (mut rl, thread) = raylib::init()
        .fullscreen()
        .log_level(TraceLogLevel::LOG_ERROR)
        .title("ONAT")
        .build();

    rl.set_window_icon(&Image::load_image_from_mem(
        ".png",
        &include_bytes!("../assets/misc/icon.png").to_vec(),
    )?);
    let audio = Box::leak(Box::new(Audio::new()?));
    let mut textures = Textures::new()?;

    let mut state = State::new(&mut rl, &thread, audio, &mut textures)?;

    while !rl.window_should_close() {
        if rl.is_key_released(KeyboardKey::KEY_F11) {
            rl.toggle_fullscreen();
        }

        if rl.is_key_released(KeyboardKey::KEY_ESCAPE) {
            break;
        }

        if state.timer.elapsed()?.as_millis() >= 1000 / 60 {
            state.timer = SystemTime::now();

            // Due to- I don't even know what this bug is but we have to force the window size to
            // whatever the screen size is.
            if rl.is_window_fullscreen() {
                rl.set_window_size(get_width_unaltered(), get_height());
            }

            state.ingame_time += Duration::from_millis(36);

            if state.going_to_office_from_title {
                rl.set_mouse_position(Vector2::new(
                    get_width_unaltered() as f32 / 2.0,
                    get_height() as f32 / 2.0,
                ));
                rl.hide_cursor();
            } else {
                rl.show_cursor();
            }

            state.step(&mut rl, &thread)?;
        }
    }

    Ok(())
}
