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
mod config;

use config::config;
pub struct ScreenInfo {
    width: i32,
    height: i32,
    ratio: f32,
    margin: f32,
}

#[error_window::main]
fn main() -> Result<(), Box<dyn Error>> {
    config(); // initializes the CONFIG variable.

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
                rl.set_window_size(config().real_width_raw(), config().real_height());
            }

            state.ingame_time += Duration::from_millis(36);

            if state.going_to_office_from_title {
                rl.set_mouse_position(Vector2::new(
                    config().real_width_raw() as f32 / 2.0,
                    config().real_height() as f32 / 2.0,
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
