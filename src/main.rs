use once_cell::sync::Lazy;
use raylib::prelude::*;

use state::State;
use std::{
    error::Error,
    os::raw::c_void,
    process::exit,
    time::{Duration, SystemTime, UNIX_EPOCH},
};
use textures::Textures;

use crate::audio::Audio;

mod audio;
mod config;
mod enums;
mod macros;
mod monster;
mod state;
mod textures;

use config::config;
pub struct ScreenInfo {
    width: i32,
    height: i32,
    ratio: f32,
    margin: f32,
}

unsafe extern "C" fn handler(signum: libc::c_int) {
    let bt = std::backtrace::Backtrace::force_capture();
    dialog::Message::new(format!(
        "Signal {} received. Backtrace:\n{}",
        signum,
        bt.to_string()
    ))
    .show()
    .unwrap();
    exit(0);
}

#[error_window::main]
fn main() -> Result<(), Box<dyn Error>> {
    unsafe {
        let f = handler as *const fn(libc::c_int);
        libc::signal(libc::SIGSEGV, f as libc::size_t);
    }
    config(); // initializes the CONFIG variable.

    let (mut rl, thread) = raylib::init()
        .fullscreen()
        .resizable()
        .log_level(TraceLogLevel::LOG_ERROR)
        .title("ONAT")
        .build();

    rl.set_window_icon(&Image::load_image_from_mem(
        ".png",
        &include_bytes!("../assets/misc/icon.png").to_vec(),
    )?);
    let audio = Box::leak(Box::new(Audio::new()?));
    let textures = Box::leak(Box::new(Textures::new()?));

    let mut state = State::new(&mut rl, &thread, audio, textures)?;

    let mut fullscreened = false;
    let mut remembered_x = rl.get_window_position().x;
    let mut remembered_y = rl.get_window_position().y;
    let mut remembered_width = rl.get_screen_width();
    let mut remembered_height = rl.get_screen_height();

    while !rl.window_should_close() {
        if state.reset_and_goto_title {
            state = State::new(&mut rl, &thread, state.audio, state.textures)?;

            if !state.going_to_office_from_title {
                state.going_to_office_from_title = true;
                if !rl.is_key_down(KeyboardKey::KEY_LEFT_SHIFT) {
                    state.title_clicked = SystemTime::now();
                } else {
                    state.title_clicked = UNIX_EPOCH;
                }
            }
        }

        if rl.is_key_pressed(KeyboardKey::KEY_F11) && !fullscreened {
            rl.toggle_fullscreen();
            fullscreened = true;
            if !rl.is_window_fullscreen() {
                rl.set_window_position(remembered_x as i32, remembered_y as i32);
                rl.set_window_size(remembered_width, remembered_height);
            } else {
                remembered_x = rl.get_window_position().x;
                remembered_y = rl.get_window_position().y;
                remembered_width = rl.get_screen_width();
                remembered_height = rl.get_screen_height();
            }
        }
        if rl.is_key_released(KeyboardKey::KEY_F11) {
            fullscreened = false;
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
