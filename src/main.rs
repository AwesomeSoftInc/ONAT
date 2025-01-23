use audio::audio_load_status;
use monster::Monster;
use parking_lot::Mutex;
use raylib::prelude::*;

use state::{Screen, State};
use std::{
    alloc::System,
    error::Error,
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

use config::{config, config_mut};

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

static mut AUDIO: Mutex<Option<Audio>> = Mutex::new(None);
fn init_audio() {
    let mut aud = Audio::new().unwrap();
    aud.set_volume(config().volume() as i32);
    unsafe { *AUDIO.lock() = Some(aud) };
}

#[error_window::main]
fn main() -> Result<(), Box<dyn Error>> {
    unsafe {
        let f = handler as *const fn(libc::c_int);
        libc::signal(libc::SIGSEGV, f as libc::size_t);
    }
    config(); // initializes the CONFIG variable.

    let (mut rl, thread) = raylib::init()
        .resizable()
        .log_level(TraceLogLevel::LOG_WARNING)
        .title("ONAT")
        .build();

    if config().fullscreen() {
        rl.toggle_fullscreen();
    }

    rl.set_window_icon(&Image::load_image_from_mem(
        ".png",
        &include_bytes!("../assets/misc/icon.png").to_vec(),
    )?);
    println!("loading audio...");

    std::thread::spawn(|| {
        init_audio();
    });

    let mut time = SystemTime::now();

    loop {
        // Every 250 milliseconds check if the audio is still being initialized.
        // If it's not,display a status message. Otherwise, break.
        if time.elapsed()?.as_millis() >= 250 {
            if unsafe { AUDIO.lock().is_none() } {
                let status = audio_load_status();
                let mut d = rl.begin_drawing(&thread);
                d.clear_background(Color::BLACK);
                d.draw_text(
                    status.as_str(),
                    config().real_margin() as i32,
                    0,
                    32,
                    Color::WHITE,
                );
            } else {
                break;
            }
            time = SystemTime::now();
        }
    }

    println!("loading textures...");
    let textures = Box::leak(Box::new(Textures::new()?));

    let mut audio = unsafe { AUDIO.lock() };
    let mut state = State::new(&mut rl, &thread, audio.as_mut().unwrap(), textures)?;

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
                    state.title_fade_skip = true;
                }
            }

            if config().night_2() {
                state.gang.penny.activate();
                state.gang.beastie.activate();
                state.gang.gogopher.activate();
                state.gang.tux.activate();
                state.gang.wilber.activate();

                state.gang.penny.ai_level = 10;
                state.gang.beastie.ai_level = 10;
                state.gang.gogopher.ai_level = 10;
                state.gang.tux.ai_level = 10;
                state.gang.wilber.ai_level = 10;

                state.gang.wilber.time_since_appeared = Some(SystemTime::now());
            }
        }

        if rl.is_key_pressed(KeyboardKey::KEY_F11) && !fullscreened {
            config_mut().toggle_fullscreen(&mut rl);
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

        state.mouse_pointer = false;

        let (mx, my) = state.mouse_position(&mut rl)?;

        if state.timer.elapsed()?.as_millis() >= 1000 / 60 {
            state.timer = SystemTime::now();

            // Due to- I don't even know what this bug is but we have to force the window size to
            // whatever the screen size is.
            if rl.is_window_fullscreen() {
                rl.set_window_size(config().real_width_raw(), config().real_height());
            }

            state.ingame_time += Duration::from_millis(36);

            state.audio_step()?;

            if !state.screen.is_passive() {
                state.step(&mut rl, &thread, mx, my)?;
                state.audio_play_step()?;
            }
        }

        let mut d = rl.begin_drawing(&thread);
        state.draw_step(&mut d, &thread, mx, my)?;
        state.input_step(&mut d, &thread, mx, my)?;
        if state.mouse_pointer {
            d.set_mouse_cursor(MouseCursor::MOUSE_CURSOR_POINTING_HAND);
        } else {
            d.set_mouse_cursor(MouseCursor::MOUSE_CURSOR_DEFAULT);
        }
    }

    Ok(())
}
