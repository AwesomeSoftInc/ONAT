use monster::{GoGopher, Monster, MonsterName};
use raylib::{
    ffi::{GetMonitorHeight, GetMonitorWidth},
    prelude::*,
};

use num_traits::FromPrimitive;
use state::State;
use std::{
    error::Error,
    time::{Duration, SystemTime, UNIX_EPOCH},
};

use enums::{Room, Screen};
use once_cell::sync::Lazy;
use textures::Textures;

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
        let (mut rl, thread) = raylib::init().fullscreen().title("ONAT").build();
        let monitor_width = get_monitor_width(get_current_monitor_index());
        let monitor_height = get_monitor_height(get_current_monitor_index());

        let default_ratio = monitor_width as f32 / monitor_height as f32;
        let desired_ratio = 4.0 / 3.0;
        let ratio = 1.0 + (default_ratio - desired_ratio);

        let margin = monitor_width as f32 - ((monitor_width as f32) / ratio);

        drop(rl);

        Self {
            width: monitor_width,
            height: monitor_height,
            ratio: ratio,
            margin: margin,
        }
    }
}

pub static SCREEN: Lazy<ScreenInfo> = Lazy::new(|| ScreenInfo::new());

pub fn get_width() -> i32 {
    ((SCREEN.width as f32) / SCREEN.ratio) as i32
}

pub fn get_height() -> i32 {
    SCREEN.height
}

pub fn get_margin() -> f32 {
    SCREEN.margin / 2.0
}

fn main() -> Result<(), Box<dyn Error>> {
    get_width();

    let (mut rl, thread) = raylib::init()
        .size(SCREEN.width, get_height())
        .fullscreen()
        .title("ONAT")
        .build();

    let textures = Textures::new(&mut rl, &thread)?;

    let mut state = State::new();

    let scroll_amount = get_width().clone() as f32 * 0.0025;

    while !rl.window_should_close() {
        if state.timer.elapsed()?.as_millis() <= 1 / 30 {
            continue;
        }
        state.timer = SystemTime::now();

        state.ingame_time += Duration::from_millis(36);

        let img = Image::gen_image_white_noise(320, 240, 0.1);
        let tex = rl.load_texture_from_image(&thread, &img)?;

        let mut d = rl.begin_drawing(&thread);

        d.clear_background(Color::BLACK);

        let mx = d.get_mouse_x();
        let my = d.get_mouse_y();

        state.tainted += d.get_mouse_wheel_move();
        match state.screen {
            Screen::TitleScreen => {
                d.clear_background(Color::BLACK);
                d.draw_text(
                    "One Night at Tux",
                    get_margin() as i32 + 5,
                    5,
                    32,
                    Color::WHITE,
                );
                d.draw_text(
                    "Click anywhere to start",
                    get_margin() as i32 + 5,
                    48,
                    32,
                    Color::WHITE,
                );
                if d.is_mouse_button_pressed(MouseButton::MOUSE_LEFT_BUTTON) {
                    state = State::new();
                    state.screen = Screen::Office;
                }
            }
            Screen::YouWin => {
                d.clear_background(Color::GREEN);
            }
            Screen::Office => {
                if state.laptop_offset_y < get_height() as f64 {
                    state.laptop_offset_y += 3.0;
                }
                if state.laptop_offset_y != get_height() as f64 {
                    d.clear_background(Color::BLACK);
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
                    continue;
                }
                d.draw_rectangle(
                    get_margin() as i32 + (get_width() as f32 / 1.32 - state.bg_offset_x) as i32,
                    (get_height() as f32 / 1.20) as i32,
                    (100.0 * (SCREEN.ratio + 1.0)) as i32,
                    32,
                    Color::new(0, 128, 0, 255),
                );
                d.draw_rectangle(
                    get_margin() as i32
                        + ((get_width() as f32 / 1.32 - state.bg_offset_x) as i32)
                        + SCREEN.ratio.ceil() as i32,
                    ((get_height() as f32 / 1.20) as i32) + SCREEN.ratio.ceil() as i32,
                    ((state.tainted as i32 - 4) as f32 * (SCREEN.ratio + 1.0)) as i32,
                    28,
                    Color::GREEN,
                );
                d.draw_texture_pro(
                    &textures.tainted_logo,
                    texture_rect!(textures.tainted_logo),
                    Rectangle::new(
                        get_margin() + get_width() as f32 / 1.25 - state.bg_offset_x,
                        get_height() as f32 / 1.25,
                        textures.tainted_logo.width as f32 * SCREEN.ratio,
                        textures.tainted_logo.height as f32 * SCREEN.ratio,
                    ),
                    Vector2::new(0.0, 0.0),
                    0.0,
                    Color::WHITE,
                );

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
                    &textures.office,
                    texture_rect!(textures.office),
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
                d.draw_texture_pro(
                    &textures.office,
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
                if state.gang.wilber.active() {
                    let texture = match state.gang.wilber.stage {
                        0 => &textures.wilber_poster.poster,
                        1 => &textures.wilber_poster.posterprogress1,
                        2 => &textures.wilber_poster.posterprogress2,
                        _ => &textures.wilber_poster.posterprogress3,
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
                        Color::WHITE,
                    );
                }

                let mut i = 0;
                for button in &state.door_buttons {
                    d.draw_rectangle(
                        (button.x - state.bg_offset_x) as i32,
                        button.y as i32,
                        button.width as i32,
                        button.height as i32,
                        Color::RED,
                    );
                    if d.is_mouse_button_released(MouseButton::MOUSE_LEFT_BUTTON)
                        && (mx as f32 >= (button.x - state.bg_offset_x)
                            && mx as f32 <= (button.x - state.bg_offset_x) + button.width
                            && my as f32 >= button.y
                            && my as f32 <= button.y + button.height)
                    {
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
                    i += 1;
                }

                // LEFT DOOR
                if state.left_door_shut {
                    if state.left_door_anim_timer <= 0.0 {
                        state.left_door_anim_timer += 5.0;
                    }
                } else {
                    if state.left_door_anim_timer >= -(get_height() as f32) {
                        state.left_door_anim_timer -= 5.0;
                    }
                }

                // RIGHT DOOR
                if state.right_door_shut {
                    if state.right_door_anim_timer <= 0.0 {
                        state.right_door_anim_timer += 5.0;
                    }
                } else {
                    if state.right_door_anim_timer >= -(get_height() as f32) {
                        state.right_door_anim_timer -= 5.0;
                    }
                }
                state.gang.wilber.rage_increment();
                if state.camera_timer <= 100.0 {
                    //state.camera_timer += 0.02;
                }

                for mons in state.gang.in_room(&Room::Office) {
                    d.draw_text(
                        mons.special_debug_info().as_str(),
                        get_margin() as i32,
                        64,
                        32,
                        Color::RED,
                    );
                    mons.draw(
                        &textures,
                        &mut d,
                        &thread,
                        (get_width() / 2) as f32 - state.bg_offset_x,
                        0.0,
                    );
                }
            }
            Screen::CameraRebooting => {
                if state.camera_timer <= 100.0 {
                    //state.camera_timer += 0.02;
                    // TODO: Rebooting animation
                    d.draw_text(
                        "Laptop Charging...",
                        get_margin() as i32 + (get_width() / 2) - (7 * 32),
                        get_height() / 2,
                        32,
                        Color::WHITE,
                    );
                    if state.camera_timer >= 100.0 {
                        state.screen = Screen::Camera;
                    }
                } else {
                    // TODO: Rebooting animation
                    d.draw_text(
                        format!("Laptop Rebooting: {:.0}", state.camera_booting_timer).as_str(),
                        get_margin() as i32 + (get_width() / 2) - (7 * 32),
                        get_height() / 2,
                        32,
                        Color::WHITE,
                    );
                }
            }
            Screen::Camera => {
                if state.laptop_offset_y > 0.0 {
                    state.laptop_offset_y -= 3.0;
                }
                if state.laptop_offset_y != 0.0 {
                    d.clear_background(Color::BLACK);
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
                    continue;
                }
                if state.camera_booting {
                    state.screen = Screen::CameraRebooting;
                    continue;
                }

                let texture = match state.sel_camera {
                    Room::Room1 => &textures.cam1,
                    Room::Room2 => &textures.cam2,
                    Room::Room3 => &textures.cam3,
                    Room::Room4 => &textures.cam4,
                    Room::Room5 => &textures.cam5,
                    Room::Room6 => &textures.cam6,
                    _ => panic!("tried to draw unsupported room {:?}", state.sel_camera),
                };

                if state.sel_camera == Room::Room4 {
                    let red = state.duct_heat_timer as u8;
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
                    d.draw_rectangle(
                        state.duct_button.x as i32,
                        state.duct_button.y as i32,
                        state.duct_button.width as i32,
                        state.duct_button.height as i32,
                        Color::BLACK,
                    );
                    if d.is_mouse_button_released(MouseButton::MOUSE_LEFT_BUTTON)
                        && (mx as f32 >= (state.duct_button.x)
                            && mx as f32 <= (state.duct_button.x) + state.duct_button.width
                            && my as f32 >= state.duct_button.y
                            && my as f32 <= state.duct_button.y + state.duct_button.height)
                    {
                        if state.gang.gogopher.room() == &Room::Office {
                            state.duct_heat_timer = 3000.0;
                        } else {
                            state.duct_heat_timer = 2500.0;
                        }
                    }
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

                let inroom = state.gang.in_room(&state.sel_camera);
                for mons in inroom {
                    d.draw_text(
                        mons.special_debug_info().as_str(),
                        get_margin() as i32,
                        64,
                        32,
                        Color::RED,
                    );
                    mons.draw(&textures, &mut d, &thread, get_margin(), 0.0);
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

                for i in 0..state.camera_clickables.len() {
                    let clickable = state.camera_clickables.get(i).unwrap();

                    d.draw_rectangle(
                        clickable.x as i32,
                        clickable.y as i32,
                        clickable.width as i32,
                        clickable.height as i32,
                        Color::WHITE.fade(0.25),
                    );
                    d.draw_rectangle_lines_ex(clickable, 4, Color::WHITE);
                    d.draw_text(
                        format!("CAM {}", i + 1).as_str(),
                        clickable.x as i32 + 20,
                        clickable.y as i32 + 20,
                        20,
                        Color::WHITE,
                    );

                    if d.is_mouse_button_released(MouseButton::MOUSE_LEFT_BUTTON)
                        && (mx as f32 >= clickable.x
                            && mx as f32 <= clickable.x + clickable.width
                            && my as f32 >= clickable.y
                            && my as f32 <= clickable.y + clickable.height)
                    {
                        state.sel_camera = Room::from_u64(i as u64).unwrap();
                    }
                }
                if state.camera_timer >= 0.0 {
                    //state.camera_timer -= 0.02;
                } else {
                    state.camera_booting = true;
                    state.sel_camera = Room::Room1;
                    state.screen = Screen::Office;
                }
            }
            Screen::GameOver => {
                d.clear_background(Color::RED);
                if state.tainted >= 200.0 {
                    state.screen = Screen::TitleScreen;
                }
            }
        }

        if let Screen::TitleScreen = state.screen {
            continue;
        }
        if let Screen::GameOver = state.screen {
            continue;
        }

        let cur_time = state.ingame_time.duration_since(UNIX_EPOCH)?;
        let is_over = state.gang.step(cur_time);
        if is_over {
            state.screen = Screen::YouWin;
        }
        let num = {
            let ct = cur_time.as_secs() / 3600;
            if ct == 0 {
                12
            } else {
                ct
            }
        };

        if mx <= (get_width() / 4) {
            if state.bg_offset_x > 0.0 {
                state.bg_offset_x -= scroll_amount;
            }
        }
        if mx >= get_width() - (get_width() / 4) {
            if state.bg_offset_x < (get_width() as f32) / 2.0 {
                state.bg_offset_x += scroll_amount;
            }
        }

        d.draw_texture_pro(
            &textures.arrow,
            texture_rect!(textures.arrow),
            Rectangle::new(
                get_margin() + 0.0,
                get_height() as f32 - (get_height() as f32 / 16.0),
                get_width() as f32,
                get_height() as f32 / 16.0,
            ),
            Vector2::new(0.0, 0.0),
            0.0,
            Color::new(255, 255, 255, 128),
        );

        if my >= get_height() - (get_height() / 16)
            && d.is_mouse_button_released(MouseButton::MOUSE_LEFT_BUTTON)
        {
            state.screen = match state.screen {
                Screen::Office => Screen::Camera,
                Screen::CameraRebooting => Screen::Office,
                Screen::Camera => Screen::Office,
                _ => state.screen,
            };
        }

        if state.camera_booting {
            state.camera_booting_timer += 0.02;
            if state.camera_booting_timer >= 250.0 {
                state.camera_booting = false;
                state.camera_booting_timer = 0.0;
            }
        }
        d.draw_text(
            format!("{}:00AM", num).as_str(),
            get_margin() as i32 + get_width() - (128.0 * SCREEN.ratio) as i32,
            0,
            (32.0 * SCREEN.ratio) as i32,
            Color::BLACK,
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

        let inoffice = state.gang.in_room(&Room::Office);
        let mut y = 48;
        for mons in inoffice {
            if mons.active() {
                let x = {
                    if mons.entered_from_right() {
                        get_width() - 128 - 5
                    } else {
                        5
                    }
                };
                d.draw_text(&mons.name(), x, y, 32, Color::BLACK);
                y += 48;
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
            }
        }
        if state.duct_heat_timer > 0.0 {
            state.duct_heat_timer -= 1.0;
        }
        state.gang.gogopher.duct_heat_timer = state.duct_heat_timer as u16;

        if state.tainted >= 100.0
            || (state.gang.wilber.stage == 4 && state.gang.wilber.rage() >= 0.2)
        {
            state.screen = Screen::GameOver;
        }

        // Bars

        d.draw_rectangle(
            get_margin() as i32 + 5,
            get_height() - 42,
            100,
            32,
            Color::BLACK,
        );
        d.draw_rectangle_gradient_h(
            get_margin() as i32 + 7,
            get_height() - 40,
            (100.0 * (state.camera_timer / 100.0)) as i32 - 4,
            28,
            Color::WHITE,
            Color::WHITE,
        );
        let width = 100.0 * (state.camera_timer / 100.0);
        d.draw_texture_pro(
            &textures.battery_text,
            Rectangle::new(0.0, 0.0, width, textures.battery_text.height as f32),
            Rectangle::new(
                get_margin() + 7.0,
                get_height() as f32 - 40.0,
                width - 7.0,
                18.0,
            ),
            Vector2::new(0.0, 0.0),
            0.0,
            Color::WHITE,
        );

        d.draw_rectangle(0, 0, get_margin() as i32, get_height() as i32, Color::BLACK);
        d.draw_rectangle(
            get_width() + get_margin() as i32 + 1,
            0,
            get_margin() as i32,
            get_height() as i32,
            Color::BLACK,
        );
    }

    Ok(())
}
