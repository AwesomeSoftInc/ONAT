#![feature(vec_into_raw_parts)]

use monster::{GoGopher, Monster, MonsterName};
use rand::{thread_rng, Rng};
use raylib::{
    ffi::{GetMonitorHeight, GetMonitorWidth, MeasureText},
    prelude::*,
};

use num_traits::{float::FloatCore, real::Real, Float, FromPrimitive};
use state::State;
use std::{
    error::Error,
    ffi::CString,
    time::{Duration, SystemTime, UNIX_EPOCH},
};

use enums::{Room, Screen};
use once_cell::sync::Lazy;
use textures::Textures;

use crate::{
    jumpscares::load_jumpscares,
    monster::{Tux, MONSTER_TIME_OFFICE_WAIT_THING},
};

mod enums;
mod jumpscares;
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
        let (rl, _) = raylib::init().fullscreen().title("ONAT").build();
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

    pub fn update(&mut self, rl: &mut RaylibHandle) {
        let monitor_width = rl.get_screen_width();
        let monitor_height = rl.get_screen_height();
        let default_ratio = monitor_width as f32 / monitor_height as f32;
        let desired_ratio = 4.0 / 3.0;
        let ratio = 1.0 + (default_ratio - desired_ratio);

        let mut margin = monitor_width as f32 - ((monitor_width as f32) / ratio);
        if margin < 0.0 {
            margin = 0.0;
        }

        self.width = monitor_width;
        self.height = monitor_height;
        self.ratio = ratio;
        self.margin = margin;
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

fn main() -> Result<(), Box<dyn Error>> {
    set_trace_log(TraceLogLevel::LOG_ERROR);

    get_width();

    let (mut rl, thread) = raylib::init()
        .size(unsafe { SCREEN.width }, get_height())
        .fullscreen()
        .resizable()
        .title("ONAT")
        .build();

    let textures = Textures::new(&mut rl, &thread)?;

    let mut state = State::new();

    let default_font = rl.get_font_default();
    let scroll_amount = get_width().clone() as f32 * 0.01;

    const CAMERA_TIME: f32 = 0.1;
    const DOOR_ANIM_SPEED: f32 = 100.0;
    let var_name = get_height() as f64 / 4.0;

    let (wilber, tux, penny, beastie, gopher, golden_tux) = load_jumpscares(&textures);

    let mut framebuffer =
        rl.load_render_texture(&thread, get_width_unaltered() as u32, get_height() as u32)?;
    state.gameover_time = SystemTime::now();
    while !rl.window_should_close() {
        if state.timer.elapsed()?.as_millis() >= 1000 / 60 {
            state.timer = SystemTime::now();

            if rl.is_key_released(KeyboardKey::KEY_F11) {
                rl.toggle_fullscreen();
            }

            // Due to a fatal bug with KDE(/X11?), we can't make the window non-resizable and fullscreen. So we force it to be whatever it was originally.
            rl.set_window_size(get_width_unaltered(), get_height());

            state.ingame_time += Duration::from_millis(36);

            let (img, tex) = match state.screen {
                Screen::Camera | Screen::GameOver => {
                    let img = Image::gen_image_white_noise(320, 240, 0.1);
                    let tex = rl.load_texture_from_image(&thread, &img)?;
                    (img, tex)
                }
                _ => {
                    let img = Image::gen_image_white_noise(1, 1, 0.0);
                    let tex = rl.load_texture_from_image(&thread, &img)?;
                    (img, tex)
                }
            };

            let mut d_ = rl.begin_drawing(&thread);
            match state.screen {
                // for some fucken reason we can't draw some of these on a texture? idfk
                Screen::TitleScreen => {
                    d_.clear_background(Color::BLACK);
                    d_.draw_text(
                        "One Night at Tux",
                        get_margin() as i32 + 5,
                        5,
                        32,
                        Color::WHITE,
                    );
                    d_.draw_text(
                        "Click anywhere to start",
                        get_margin() as i32 + 5,
                        48,
                        32,
                        Color::WHITE,
                    );
                    if d_.is_mouse_button_pressed(MouseButton::MOUSE_LEFT_BUTTON) {
                        state = State::new();
                        state.screen = Screen::Office;
                    }
                }
                Screen::GameOver => {
                    d_.draw_texture_pro(
                        &tex,
                        texture_rect!(tex),
                        Rectangle::new(get_margin(), 0.0, get_width() as f32, get_height() as f32),
                        Vector2::new(0.0, 0.0),
                        0.0,
                        Color::WHITE,
                    );
                    if state.gameover_time.elapsed()?.as_secs() >= 5 {
                        state.screen = Screen::TitleScreen;
                    }
                }

                _ => {
                    {
                        d_.clear_background(Color::BLACK);
                        let mut d: RaylibTextureMode<'_, RaylibDrawHandle<'_>> =
                            d_.begin_texture_mode(&thread, &mut framebuffer);
                        d.clear_background(Color::BLACK);

                        let mx = d.get_mouse_x();
                        let my = d.get_mouse_y();

                        match state.screen {
                            Screen::YouWin => {
                                d.clear_background(Color::GREEN);
                            }
                            Screen::Office => {
                                #[cfg(not(feature = "no_camera_timer"))]
                                if state.camera_timer <= 100.0 {
                                    state.camera_timer += CAMERA_TIME;
                                }
                                if state.going_to_camera {
                                    if state.laptop_offset_y > 0.0 {
                                        state.laptop_offset_y -= var_name as f64;
                                    } else {
                                        state.screen = Screen::Camera;
                                        state.going_to_camera = false;
                                    }
                                }

                                if state.gang.golden_tux.active() {
                                    if state.gang.golden_tux.appeared.elapsed()?.as_secs() >= 10 {
                                        if state.jumpscarer == MonsterName::None {
                                            state.gang.golden_tux.deactivate();
                                            state.jumpscarer = MonsterName::GoldenTux;
                                            state.getting_jumpscared = true;
                                        }
                                    }
                                }

                                d.draw_texture_pro(
                                    &textures.office_corners,
                                    texture_rect!(textures.office_corners),
                                    Rectangle::new(
                                        get_margin() + -state.bg_offset_x,
                                        0.0,
                                        get_width() as f32 * 1.6,
                                        get_height() as f32,
                                    ),
                                    Vector2::new(0.0, 0.0),
                                    0.0,
                                    Color::WHITE,
                                );
                                d.draw_texture_pro(
                                    &textures.door_left,
                                    texture_rect!(textures.door_left),
                                    Rectangle::new(
                                        get_margin() + -state.bg_offset_x,
                                        state.left_door_anim_timer,
                                        get_width() as f32 * 1.6,
                                        get_height() as f32,
                                    ),
                                    Vector2::new(0.0, 0.0),
                                    0.0,
                                    Color::WHITE,
                                );

                                d.draw_texture_pro(
                                    &textures.door_right,
                                    texture_rect!(textures.door_right),
                                    Rectangle::new(
                                        get_margin() + -state.bg_offset_x,
                                        state.right_door_anim_timer,
                                        get_width() as f32 * 1.6,
                                        get_height() as f32,
                                    ),
                                    Vector2::new(0.0, 0.0),
                                    0.0,
                                    Color::WHITE,
                                );
                                let var_name = (1.0 + get_ratio()) as i32;

                                d.draw_texture_pro(
                                    &textures.wallpaper,
                                    texture_rect!(textures.wallpaper),
                                    Rectangle::new(
                                        ((get_width() as f32 + get_margin() as f32)
                                            - get_width() as f32 / 3.5)
                                            - state.bg_offset_x,
                                        get_height() as f32 / 1.65,
                                        get_width() as f32 / 3.5,
                                        get_height() as f32 / 3.5,
                                    ),
                                    Vector2::new(0.0, 0.0),
                                    0.0,
                                    Color::WHITE,
                                );
                                d.draw_rectangle(
                                    (((get_width() as f32 / 1.233) + get_margin())
                                        - state.bg_offset_x)
                                        as i32
                                        - 50,
                                    (get_height() as f32 / 1.20) as i32,
                                    200,
                                    32,
                                    Color::new(0, 128, 0, 255),
                                );
                                d.draw_rectangle(
                                    (((get_width() as f32 / 1.233) + get_margin())
                                        - state.bg_offset_x)
                                        as i32
                                        - (50 - var_name),
                                    ((get_height() as f32 / 1.20) as i32) + var_name,
                                    (state.tainted as i32 - 4) * (get_ratio().ceil()) as i32,
                                    32 - (var_name * 2),
                                    Color::GREEN,
                                );

                                d.draw_texture_pro(
                                    &textures.tainted_logo,
                                    texture_rect!(textures.tainted_logo),
                                    Rectangle::new(
                                        ((get_width() as f32 / 1.233) + get_margin())
                                            - state.bg_offset_x,
                                        get_height() as f32 / 1.25,
                                        (get_width() as f32 + get_margin()) / 16.0,
                                        get_height() as f32 / 46.0,
                                    ),
                                    Vector2::new(0.0, 0.0),
                                    0.0,
                                    Color::WHITE,
                                );

                                d.draw_texture_pro(
                                    &textures.office_part1,
                                    texture_rect!(textures.office_part1),
                                    Rectangle::new(
                                        get_margin() + -state.bg_offset_x,
                                        0.0,
                                        get_width() as f32 * 1.6,
                                        get_height() as f32,
                                    ),
                                    Vector2::new(0.0, 0.0),
                                    0.0,
                                    Color::WHITE,
                                );

                                for mons in state.gang.in_room(Room::Office) {
                                    mons.draw(
                                        &textures,
                                        &mut d,
                                        ((get_width() as f32 + get_margin()) as i32 / 4) as f32
                                            - state.bg_offset_x,
                                        0.0,
                                        1.6,
                                        1.0,
                                    );
                                }

                                d.draw_texture_pro(
                                    &textures.office_part2,
                                    texture_rect!(textures.office_part2),
                                    Rectangle::new(
                                        get_margin() + -state.bg_offset_x,
                                        0.0,
                                        get_width() as f32 * 1.6,
                                        get_height() as f32,
                                    ),
                                    Vector2::new(0.0, 0.0),
                                    0.0,
                                    Color::WHITE,
                                );

                                d.draw_texture_pro(
                                    &textures.button1,
                                    texture_rect!(textures.button1),
                                    Rectangle::new(
                                        get_margin() + -state.bg_offset_x,
                                        0.0,
                                        get_width() as f32 * 1.6,
                                        get_height() as f32,
                                    ),
                                    Vector2::new(0.0, 0.0),
                                    0.0,
                                    Color::WHITE,
                                );
                                d.draw_texture_pro(
                                    &textures.button2,
                                    texture_rect!(textures.button2),
                                    Rectangle::new(
                                        get_margin() + -state.bg_offset_x,
                                        0.0,
                                        get_width() as f32 * 1.6,
                                        get_height() as f32,
                                    ),
                                    Vector2::new(0.0, 0.0),
                                    0.0,
                                    Color::WHITE,
                                );

                                if state.gang.wilber.active() {
                                    let texture = match state.gang.wilber.stage {
                                        0 => &textures.wilberPoster.poster,
                                        1 => &textures.wilberPoster.posterprogress1,
                                        2 => &textures.wilberPoster.posterprogress2,
                                        _ => &textures.wilberPoster.posterprogress3,
                                    };
                                    let time = match state.gang.wilber.time_since_appeared {
                                        Some(a) => {
                                            let b = a.elapsed()?.as_millis() / 2;
                                            if b >= 255 {
                                                255
                                            } else {
                                                b as u8
                                            }
                                        }
                                        None => {
                                            state.gang.wilber.time_since_appeared =
                                                Some(SystemTime::now());
                                            0
                                        }
                                    };
                                    d.draw_texture_pro(
                                        &texture,
                                        texture_rect!(texture),
                                        Rectangle::new(
                                            get_margin() + -state.bg_offset_x,
                                            0.0,
                                            get_width() as f32 * 1.6,
                                            get_height() as f32,
                                        ),
                                        Vector2::new(0.0, 0.0),
                                        0.0,
                                        Color::new(255, 255, 255, time),
                                    );
                                }

                                let mut i = 0;
                                let mut hovering = false;
                                for button in &state.door_buttons {
                                    if mx as f32 >= (button.x - state.bg_offset_x)
                                        && mx as f32
                                            <= (button.x - state.bg_offset_x) + button.width
                                        && my as f32 >= button.y
                                        && my as f32 <= button.y + button.height
                                    {
                                        hovering = true;
                                        d.set_mouse_cursor(MouseCursor::MOUSE_CURSOR_POINTING_HAND);
                                        if d.is_mouse_button_released(
                                            MouseButton::MOUSE_LEFT_BUTTON,
                                        ) {
                                            if i == 0 && state.can_open_left_door {
                                                state.left_door_shut = true;
                                                state.left_door_last_shut = SystemTime::now();
                                                state.can_open_left_door = false;
                                            } else if i == 1 && state.can_open_right_door {
                                                state.right_door_shut = true;
                                                state.right_door_last_shut = SystemTime::now();
                                                state.can_open_right_door = false;
                                            }
                                        }
                                    }

                                    i += 1;
                                }

                                if !hovering {
                                    d.set_mouse_cursor(MouseCursor::MOUSE_CURSOR_DEFAULT);
                                }

                                // LEFT DOOR
                                if state.left_door_shut {
                                    if state.left_door_anim_timer <= 0.0 {
                                        state.left_door_anim_timer += DOOR_ANIM_SPEED;
                                    }
                                } else {
                                    if state.left_door_anim_timer >= -(get_height() as f32) {
                                        state.left_door_anim_timer -= DOOR_ANIM_SPEED;
                                    }
                                }

                                // RIGHT DOOR
                                if state.right_door_shut {
                                    if state.right_door_anim_timer <= 0.0 {
                                        state.right_door_anim_timer += DOOR_ANIM_SPEED;
                                    }
                                } else {
                                    if state.right_door_anim_timer >= -(get_height() as f32) {
                                        state.right_door_anim_timer -= DOOR_ANIM_SPEED;
                                    }
                                }
                                state.gang.wilber.rage_increment();

                                if state.laptop_offset_y < get_height() as f64 {
                                    d.draw_texture_pro(
                                        &textures.laptop,
                                        texture_rect!(textures.laptop),
                                        Rectangle::new(
                                            get_margin() + 0.0,
                                            state.laptop_offset_y as f32,
                                            get_width() as f32,
                                            get_height() as f32,
                                        ),
                                        Vector2::new(0.0, 0.0),
                                        0.0,
                                        Color::WHITE,
                                    );
                                }

                                if state.getting_jumpscared {
                                    state.bg_offset_x = 450.0;
                                    match state.jumpscarer {
                                        // static.
                                        MonsterName::Penny
                                        | MonsterName::Tux
                                        | MonsterName::GoGopher
                                        | MonsterName::GoldenTux => {
                                            let (width, height, x, y, mons, framerate) = match state
                                                .jumpscarer
                                            {
                                                MonsterName::Penny => {
                                                    let x_offset = {
                                                        if state
                                                            .gameover_time
                                                            .elapsed()?
                                                            .as_millis()
                                                            <= 150
                                                        {
                                                            8.5 * (state
                                                                .gameover_time
                                                                .elapsed()?
                                                                .as_millis()
                                                                as f32)
                                                        } else {
                                                            150.0 * 8.5
                                                        }
                                                    };
                                                    (
                                                        (get_width() as f32),
                                                        get_height() as f32 / 1.5,
                                                        -get_width() as f32
                                                            + x_offset
                                                            + get_margin(),
                                                        get_height() as f32
                                                            - (get_height() as f32 / 1.5),
                                                        &penny,
                                                        30,
                                                    )
                                                }
                                                MonsterName::Tux => (
                                                    get_width() as f32
                                                        + (get_margin() + get_margin()),
                                                    get_height() as f32,
                                                    0.0,
                                                    0.0,
                                                    &tux,
                                                    18,
                                                ),
                                                MonsterName::GoldenTux => (
                                                    get_width() as f32
                                                        + (get_margin() + get_margin()),
                                                    get_height() as f32,
                                                    0.0,
                                                    0.0,
                                                    &golden_tux,
                                                    18,
                                                ),
                                                MonsterName::GoGopher => {
                                                    let height = get_height() as f32 / 1.3;
                                                    let y_offset = (height as f32
                                                        * (state.jumpscare_counter as f32 / 15.0))
                                                        / 750.0;
                                                    (
                                                        get_width() as f32
                                                            + (get_width() as f32 * y_offset),
                                                        height + (height * y_offset),
                                                        get_margin() - (y_offset * 750.0),
                                                        (-height) + (height / 1.5),
                                                        &gopher,
                                                        15,
                                                    )
                                                }
                                                _ => todo!(),
                                            };
                                            if let Some(tex) =
                                                mons.get(state.jumpscare_counter / (60 / framerate))
                                            {
                                                d.draw_texture_pro(
                                                    &tex,
                                                    texture_rect!(tex),
                                                    Rectangle::new(x, y, width, height),
                                                    Vector2::new(0.0, 0.0),
                                                    0.0,
                                                    Color::WHITE,
                                                );
                                            } else {
                                                state.screen = Screen::GameOver;

                                                state.gameover_time = SystemTime::now();
                                            }
                                            state.jumpscare_counter += 1;
                                        }
                                        MonsterName::Wilber => {
                                            let (width, height, x, mut y, framerate) = (
                                                get_width() as f32,
                                                get_height() as f32,
                                                get_margin(),
                                                get_height() as f32
                                                    - (state.jumpscare_counter * 115) as f32,
                                                8,
                                            );
                                            if y >= 0.0 {
                                                let tex = wilber.first().unwrap();
                                                d.draw_texture_pro(
                                                    &tex,
                                                    texture_rect!(tex),
                                                    Rectangle::new(x, y, width, height),
                                                    Vector2::new(0.0, 0.0),
                                                    0.0,
                                                    Color::WHITE,
                                                );
                                                state.jumpscare_counter += 1;
                                            } else {
                                                y = 0.0;
                                                if let Some(tex) = wilber.get(
                                                    (state.jumpscare_counter - 5)
                                                        / (60 / framerate),
                                                ) {
                                                    d.draw_texture_pro(
                                                        &tex,
                                                        texture_rect!(tex),
                                                        Rectangle::new(x, y, width, height),
                                                        Vector2::new(0.0, 0.0),
                                                        0.0,
                                                        Color::WHITE,
                                                    );
                                                    state.jumpscare_counter += 1;
                                                } else {
                                                    if state.gameover_time.elapsed()?.as_millis()
                                                        <= 800
                                                    {
                                                        let tex = wilber.last().unwrap();
                                                        d.draw_texture_pro(
                                                            &tex,
                                                            texture_rect!(tex),
                                                            Rectangle::new(x, y, width, height),
                                                            Vector2::new(0.0, 0.0),
                                                            0.0,
                                                            Color::WHITE,
                                                        );
                                                    } else {
                                                        state.screen = Screen::GameOver;

                                                        state.gameover_time = SystemTime::now();
                                                    }
                                                }
                                            }
                                        }
                                        MonsterName::Beastie => {
                                            let width = textures.beastie.slide.width;
                                            let height = textures.beastie.slide.height;
                                            let cutoff =
                                                state.gameover_time.elapsed()?.as_millis() <= 500;
                                            let x_offset = {
                                                let o = state.gameover_time.elapsed()?.as_millis()
                                                    as f32
                                                    * 2.0;
                                                let w = get_width() as f32 + get_margin();
                                                if o <= w / 4.0 {
                                                    o
                                                } else {
                                                    if cutoff {
                                                        w / 4.0
                                                    } else {
                                                        w / 2.0
                                                    }
                                                }
                                            };
                                            let x = (get_width() as f32) - x_offset;
                                            let y = get_height() as f32 - height as f32;
                                            if cutoff {
                                                d.draw_texture_pro(
                                                    &textures.beastie.slide,
                                                    texture_rect!(textures.beastie.slide),
                                                    Rectangle::new(
                                                        x,
                                                        y,
                                                        width as f32,
                                                        height as f32,
                                                    ),
                                                    Vector2::new(0.0, 0.0),
                                                    0.0,
                                                    Color::WHITE,
                                                );
                                            } else {
                                                if let Some(tex) =
                                                    beastie.get(state.jumpscare_counter / (60 / 24))
                                                {
                                                    d.draw_texture_pro(
                                                        &tex,
                                                        texture_rect!(tex),
                                                        Rectangle::new(
                                                            x - get_margin(),
                                                            y,
                                                            tex.width as f32 * 2.5,
                                                            tex.height as f32 * 2.5,
                                                        ),
                                                        Vector2::new(0.0, 0.0),
                                                        0.0,
                                                        Color::WHITE,
                                                    );
                                                } else {
                                                    state.screen = Screen::GameOver;
                                                    state.gameover_time = SystemTime::now();
                                                }
                                                state.jumpscare_counter += 1;
                                            }
                                        }
                                        _ => {}
                                    }
                                }
                            }
                            Screen::CameraRebooting => {
                                if state.going_to_office {
                                    if state.laptop_offset_y < get_height() as f64 {
                                        state.laptop_offset_y += var_name;
                                    } else {
                                        state.screen = Screen::Office;
                                        state.going_to_office = false;
                                    }
                                    continue;
                                }
                                #[cfg(not(feature = "no_camera_timer"))]
                                if state.camera_timer <= 100.0 {
                                    state.camera_timer += CAMERA_TIME;
                                    const width: i32 = ("Laptop Rebooting".len() as i32) * 24;
                                    let x = ((get_width() as i32 / 2) as f32) - (width / 2) as f32;
                                    let y = get_height() / 2;

                                    d.draw_text_rec(
                                        &default_font,
                                        "Laptop Rebooting",
                                        Rectangle::new(
                                            x + (width / 8) as f32,
                                            y as f32 - 16.0,
                                            width as f32,
                                            48.0,
                                        ),
                                        32.0,
                                        3.0,
                                        true,
                                        Color::WHITE,
                                    );
                                } else {
                                    state.camera_booting = false;
                                    state.screen = Screen::Camera;
                                }
                            }
                            Screen::Camera => {
                                #[cfg(not(feature = "no_camera_timer"))]
                                if state.camera_timer >= 0.0 {
                                    state.camera_timer -= CAMERA_TIME;
                                } else {
                                    state.camera_booting = true;
                                    state.sel_camera = Room::Room1;
                                    state.screen = Screen::Office;
                                }
                                if state.going_to_office {
                                    if state.laptop_offset_y < get_height() as f64 {
                                        state.laptop_offset_y += var_name;
                                    } else {
                                        state.screen = Screen::Office;
                                        state.going_to_office = false;
                                    }
                                }

                                if state.camera_booting {
                                    state.screen = Screen::CameraRebooting;
                                    continue;
                                }

                                let texture = match state.sel_camera {
                                    Room::Room1 => &textures.cam1,
                                    Room::Room2 => &textures.cam2,
                                    Room::Room3 => {
                                        if !state.skinman_appeared {
                                            if state.skinman_chance <= 1 {
                                                if state.camera_last_changed.elapsed()?.as_millis()
                                                    <= 250
                                                {
                                                    &textures.cam3_happyskinman
                                                } else {
                                                    state.skinman_appeared = true;
                                                    &textures.cam3
                                                }
                                            } else {
                                                &textures.cam3
                                            }
                                        } else {
                                            &textures.cam3
                                        }
                                    }
                                    Room::Room4 => &textures.cam4,
                                    Room::Room5 => &textures.cam5,
                                    Room::Room6 => &textures.cam6,
                                    _ => {
                                        panic!(
                                            "tried to draw unsupported room {:?}",
                                            state.sel_camera
                                        )
                                    }
                                };

                                if state.sel_camera == Room::Room4 {
                                    let red = state.gang.gogopher.duct_heat_timer as u8;
                                    d.draw_texture_pro(
                                        texture,
                                        texture_rect!(texture),
                                        Rectangle::new(
                                            get_margin() + 0.0,
                                            0.0,
                                            get_width() as f32,
                                            get_height() as f32,
                                        ),
                                        Vector2::new(0.0, 0.0),
                                        0.0,
                                        Color::new(255, 255 - red, 255 - red, 255),
                                    );
                                } else {
                                    d.draw_texture_pro(
                                        texture,
                                        texture_rect!(texture),
                                        Rectangle::new(
                                            get_margin() + 0.0,
                                            0.0,
                                            get_width() as f32,
                                            get_height() as f32,
                                        ),
                                        Vector2::new(0.0, 0.0),
                                        0.0,
                                        Color::WHITE,
                                    );
                                }
                                if state.sel_camera == Room::Room6 {
                                    state.gang.wilber.rage_decrement();
                                } else {
                                    state.gang.wilber.rage_increment();
                                }

                                let inroom = state.gang.in_room(state.sel_camera.clone());
                                for mons in inroom {
                                    mons.draw(&textures, &mut d, get_margin(), 0.0, 1.0, 1.0);
                                    if mons.move_timer() >= 1
                                        || mons.time_in_room().elapsed()?.as_millis() <= 50
                                    {
                                        d.draw_texture_pro(
                                            &tex,
                                            texture_rect!(tex),
                                            Rectangle::new(
                                                get_margin() + 0.0,
                                                0.0,
                                                get_width() as f32,
                                                get_height() as f32,
                                            ),
                                            Vector2::new(0.0, 0.0),
                                            0.0,
                                            Color::WHITE,
                                        );
                                        break;
                                    }
                                }

                                d.draw_texture_pro(
                                    &tex,
                                    texture_rect!(tex),
                                    Rectangle::new(
                                        get_margin() + 0.0,
                                        0.0,
                                        get_width() as f32,
                                        get_height() as f32,
                                    ),
                                    Vector2::new(0.0, 0.0),
                                    0.0,
                                    Color::new(255, 255, 255, 48),
                                );
                                d.draw_texture_pro(
                                    &textures.camera,
                                    texture_rect!(textures.camera),
                                    Rectangle::new(
                                        ((get_width() as f32 / 2.0) * (get_ratio().ceil() * 1.075))
                                            - get_margin(),
                                        get_height() as f32 * 0.42,
                                        get_width() as f32
                                            / (2.82
                                                + ((get_ratio().floor() * 1.075) / 10.0).round()),
                                        get_height() as f32 / 1.97,
                                    ),
                                    Vector2::new(0.0, 0.0),
                                    0.0,
                                    Color::WHITE,
                                );

                                for i in 0..state.camera_clickables.len() {
                                    let clickable = state.camera_clickables.get_mut(i).unwrap();
                                    d.draw_rectangle_rec(*clickable, Color::LIGHTGRAY);
                                    d.draw_rectangle_lines_ex(*clickable, 2, Color::GRAY);

                                    let text = format!("{}", i + 1);

                                    for x in 0..2 {
                                        d.draw_text_rec(
                                            d.get_font_default(),
                                            "CAM",
                                            Rectangle::new(
                                                clickable.x + 10.0 + x as f32,
                                                clickable.y + ((clickable.height / 2.0) - 20.0),
                                                clickable.width - 3.0,
                                                clickable.height + 3.0,
                                            ),
                                            20.0 * d.get_window_scale_dpi().x,
                                            3.0,
                                            true,
                                            Color::new(50, 50, 50, 255),
                                        );

                                        let rust_cstring = CString::new(text.clone()).unwrap();
                                        // Extract null-terminated raw data
                                        let byteslice = rust_cstring.as_bytes_with_nul();

                                        let r = byteslice[0] as i8;
                                        let font_size = 20.0 * d.get_window_scale_dpi().x;
                                        d.draw_text_rec(
                                            d.get_font_default(),
                                            &text.as_str(),
                                            Rectangle::new(
                                                clickable.x + 10.0 + x as f32, /*+ (unsafe {
                                                                                   MeasureText(
                                                                                       rust_cstring.as_ptr(),
                                                                                       font_size as i32,
                                                                                   )
                                                                               } / 2)
                                                                                   as f32*/
                                                clickable.y + (clickable.height / 2.0),
                                                clickable.width - 3.0,
                                                clickable.height + 3.0,
                                            ),
                                            font_size,
                                            3.0,
                                            true,
                                            Color::new(50, 50, 50, 255),
                                        );
                                    }

                                    if d.is_mouse_button_released(MouseButton::MOUSE_LEFT_BUTTON)
                                        && (mx as f32 >= clickable.x
                                            && mx as f32 <= clickable.x + clickable.width
                                            && my as f32 >= clickable.y
                                            && my as f32 <= clickable.y + clickable.height)
                                    {
                                        let sel_camera = Room::from_u64(i as u64).unwrap();
                                        if state.sel_camera != sel_camera {
                                            state.skinman_chance = state.rand.gen_range(0..1000);
                                            state.camera_last_changed = SystemTime::now();
                                            state.sel_camera = sel_camera
                                        }
                                    }
                                }

                                d.draw_text(
                                    "OFFICE",
                                    (get_margin()
                                        + get_width() as f32 * (0.68 + get_ratio().floor() * 0.1))
                                        as i32,
                                    (get_height() as f32 * 0.87) as i32,
                                    20,
                                    Color::WHITE,
                                );

                                if state.laptop_offset_y > 0.0 {
                                    d.draw_texture_pro(
                                        &textures.laptop,
                                        texture_rect!(textures.laptop),
                                        Rectangle::new(
                                            get_margin() + 0.0,
                                            state.laptop_offset_y as f32,
                                            get_width() as f32,
                                            get_height() as f32,
                                        ),
                                        Vector2::new(0.0, 0.0),
                                        0.0,
                                        Color::WHITE,
                                    );
                                }
                                if state.sel_camera == Room::Room4 && state.gang.gogopher.active() {
                                    d.draw_rectangle(
                                        state.duct_button.x as i32 + 1,
                                        state.duct_button.y as i32,
                                        state.duct_button.width as i32,
                                        state.duct_button.height as i32,
                                        Color::GRAY,
                                    );
                                    d.draw_rectangle_lines_ex(state.duct_button, 5, Color::BLACK);
                                    d.draw_text_rec(
                                        &default_font,
                                        "HEAT UP",
                                        Rectangle::new(
                                            state.duct_button.x + 32.0,
                                            state.duct_button.y + 32.0,
                                            state.duct_button.width - 32.0,
                                            state.duct_button.height - 32.0,
                                        ),
                                        48.0,
                                        3.0,
                                        true,
                                        Color::BLACK,
                                    );
                                    if d.is_mouse_button_released(MouseButton::MOUSE_LEFT_BUTTON)
                                        && (mx as f32 >= (state.duct_button.x)
                                            && mx as f32
                                                <= (state.duct_button.x) + state.duct_button.width
                                            && my as f32 >= state.duct_button.y
                                            && my as f32
                                                <= state.duct_button.y + state.duct_button.height)
                                    {
                                        state.gang.gogopher.duct_heat_timer = 250;
                                        state.gang.gogopher.duct_timer = 0;
                                    }
                                }
                                if state.sel_camera == Room::Room6 && state.gang.wilber.active() {
                                    let battery_bar_height = get_height() as f32 / 13.5;
                                    let battery_bar_y =
                                        get_height() as f32 - (get_height() as f32 / 5.0);
                                    let gimp_width =
                                        (165.0 * (state.gang.wilber.rage() / 100.0)) as i32 - 4;

                                    d.draw_rectangle(
                                        get_margin() as i32 + 20,
                                        battery_bar_y as i32 + (get_height() as f32 / 48.0) as i32
                                            - 1,
                                        gimp_width,
                                        (get_height() as f32 / 20.0) as i32,
                                        Color::new(50 * (state.gang.wilber.stage + 1), 0, 0, 255),
                                    );
                                    d.draw_texture_pro(
                                        &textures.battery,
                                        texture_rect!(textures.battery),
                                        Rectangle::new(
                                            get_margin() + 14.0,
                                            battery_bar_y,
                                            get_width() as f32 / 7.5,
                                            battery_bar_height,
                                        ),
                                        Vector2::new(0.0, 0.0),
                                        0.0,
                                        Color::WHITE,
                                    );
                                }
                            }
                            _ => {}
                        }

                        if let Screen::TitleScreen = state.screen {
                            continue;
                        }
                        if let Screen::GameOver = state.screen {
                            continue;
                        }
                        if let Screen::YouWin = state.screen {
                            continue;
                        }

                        let cur_time = state.ingame_time.duration_since(UNIX_EPOCH)?;
                        let is_over = state.gang.step(cur_time);
                        if is_over {
                            state.screen = Screen::YouWin;
                            continue;
                        }
                        let num = {
                            let ct = cur_time.as_secs() / 200;
                            if ct == 0 {
                                12
                            } else {
                                ct
                            }
                        };

                        let sc = (scroll_amount + (mx - get_width() / 2) as f32) / 24.0;
                        if mx <= (get_width() / 2) {
                            if state.bg_offset_x > 0.0 {
                                state.bg_offset_x += sc;
                            }
                        }
                        if mx >= get_width() - (get_width() / 2) {
                            if state.bg_offset_x < (get_width() as f32) / 2.0 {
                                state.bg_offset_x += sc;
                            }
                        }

                        d.draw_texture_pro(
                            &textures.arrow,
                            texture_rect!(textures.arrow),
                            Rectangle::new(
                                (get_width() as f32 / 4.0) + get_margin(),
                                get_height() as f32 - (get_height() as f32 / 16.0),
                                get_width() as f32 / 2.0,
                                get_height() as f32 / 16.0,
                            ),
                            Vector2::new(0.0, 0.0),
                            0.0,
                            Color::new(255, 255, 255, 128),
                        );

                        if my >= get_height() - (get_height() / 16)
                            && d.is_mouse_button_released(MouseButton::MOUSE_LEFT_BUTTON)
                            && !state.getting_jumpscared
                        {
                            match state.screen {
                                Screen::Office => {
                                    state.gang.golden_tux.deactivate();
                                    state.going_to_camera = true
                                }
                                Screen::CameraRebooting | Screen::Camera => {
                                    if state.gang.hours(cur_time) >= 5 {
                                        if thread_rng().gen_range(1..100) == 1 {
                                            state.gang.golden_tux.activate();
                                            state.gang.golden_tux.appeared = SystemTime::now();
                                        }
                                    }
                                    state.going_to_office = true
                                }
                                _ => (),
                            }
                        }

                        if state.camera_booting {
                            state.camera_booting_timer += 0.01;
                            if state.camera_booting_timer >= 250.0 {
                                state.camera_booting = false;
                                state.camera_booting_timer = 0.0;
                            }
                        }
                        let time = format!("{}:00AM", num);
                        d.draw_text(
                            time.as_str(),
                            get_margin() as i32 + get_width() - (time.len() as f32 * 50.0) as i32,
                            0,
                            (64.0 * get_ratio()) as i32,
                            Color::WHITE,
                        );

                        if state.left_door_last_shut.elapsed()?.as_secs() >= 5 {
                            state.left_door_shut = false;
                        }
                        if state.left_door_last_shut.elapsed()?.as_secs() >= 10 {
                            state.can_open_left_door = true;
                        }

                        if state.right_door_last_shut.elapsed()?.as_secs() >= 5 {
                            state.right_door_shut = false;
                        }
                        if state.right_door_last_shut.elapsed()?.as_secs() >= 10 {
                            state.can_open_right_door = true;
                        }

                        let inoffice = state.gang.in_room(Room::Office);
                        for mons in inoffice {
                            if mons.active() {
                                let duration: &Duration = &mons.timer_until_office().elapsed()?;
                                if mons.id() == MonsterName::Tux
                                    || duration.as_millis()
                                        >= (MONSTER_TIME_OFFICE_WAIT_THING as u128 * 1000) - 500
                                {
                                    if mons.entered_from_left() {
                                        if !state.left_door_shut {
                                            state.tainted += mons.taint_percent();
                                        } else {
                                            mons.set_entered_from_left(false);
                                            mons.goto_room_after_office();
                                        }
                                    }
                                    if mons.entered_from_right() {
                                        if !state.right_door_shut {
                                            state.tainted += mons.taint_percent();
                                        } else {
                                            mons.set_entered_from_right(false);
                                            mons.goto_room_after_office();
                                        }
                                    }
                                    // go gopher just does it regardless.
                                    if mons.id() == MonsterName::GoGopher {
                                        state.tainted += mons.taint_percent();
                                    }
                                    if mons.entered_from_left()
                                        || mons.entered_from_right()
                                        || mons.id() == MonsterName::GoGopher
                                    {
                                        if state.tainted >= 100.0 {
                                            if state.jumpscarer == MonsterName::None {
                                                state.going_to_office = true;
                                                state.jumpscarer = mons.id();
                                                state.gameover_time = SystemTime::now();
                                                state.getting_jumpscared = true;
                                            }
                                        }
                                    }

                                    if duration.as_nanos()
                                        <= MONSTER_TIME_OFFICE_WAIT_THING as u128 * 1000000000
                                    {
                                        if duration.as_nanos() & 256 == 256
                                            && mons.id() != MonsterName::Tux
                                        {
                                            d.draw_rectangle(
                                                get_margin() as i32,
                                                0,
                                                get_width(),
                                                get_height(),
                                                Color::BLACK,
                                            );
                                        }
                                    }
                                };
                            }
                        }
                        if state.gang.wilber.stage == 4 && state.gang.wilber.rage() >= 0.2 {
                            if state.jumpscarer == MonsterName::None {
                                state.jumpscarer = MonsterName::Wilber;
                                state.gameover_time = SystemTime::now();
                                state.getting_jumpscared = true;
                            }
                        }

                        if state.gang.gogopher.duct_heat_timer > 0 {
                            state.gang.gogopher.duct_heat_timer -= 1;
                        }

                        // Bars
                        let battery_bar_y = get_height() as f32
                            - (get_height() as f32 / 13.5)
                            - (get_height() as f32 / 64.0);
                        let battery_bar_height = get_height() as f32 / 13.5;
                        let width = (165.0 * (state.camera_timer / 100.0)) as i32 - 4;
                        let color_width = (200.0 * (state.camera_timer / 100.0)) as u8;

                        d.draw_rectangle_gradient_h(
                            get_margin() as i32 + 20,
                            battery_bar_y as i32 + (get_height() as f32 / 48.0) as i32,
                            width,
                            (get_height() as f32 / 20.0) as i32,
                            Color::RED,
                            Color::new(255 - color_width as u8, color_width as u8, 0, 255),
                        );
                        d.draw_texture_pro(
                            &textures.battery,
                            texture_rect!(textures.battery),
                            Rectangle::new(
                                get_margin() + 14.0,
                                battery_bar_y,
                                get_width() as f32 / 7.5,
                                battery_bar_height,
                            ),
                            Vector2::new(0.0, 0.0),
                            0.0,
                            Color::WHITE,
                        );
                    }
                    let rot = {
                        if state.jumpscarer == MonsterName::Tux {
                            let r = thread_rng().gen_range(-5..5);
                            r as f32
                        } else {
                            0.0
                        }
                    };
                    d_.draw_texture_pro(
                        &framebuffer,
                        Rectangle::new(
                            framebuffer.width() as f32,
                            0.0,
                            -framebuffer.width() as f32,
                            framebuffer.height() as f32,
                        ),
                        Rectangle::new(
                            (framebuffer.width() as f32 / 2.0) + rot,
                            (framebuffer.height() as f32 / 2.0) + rot,
                            framebuffer.width() as f32,
                            framebuffer.height() as f32,
                        ),
                        Vector2::new(
                            framebuffer.width() as f32 / 2.0,
                            framebuffer.height() as f32 / 2.0,
                        ),
                        180.0 + rot,
                        Color::WHITE,
                    );
                    d_.draw_rectangle(0, 0, get_margin() as i32, get_height() as i32, Color::BLACK);
                    d_.draw_rectangle(
                        get_width() + get_margin() as i32 + 1,
                        0,
                        get_margin() as i32,
                        get_height() as i32,
                        Color::BLACK,
                    );
                }
            }
        }
    }

    Ok(())
}
