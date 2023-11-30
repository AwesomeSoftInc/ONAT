use monster::{Monster, MonsterName};
use raylib::prelude::*;

use num_traits::{FromPrimitive, Pow};
use state::State;
use std::{
    error::Error,
    time::{Duration, SystemTime, UNIX_EPOCH},
};

use enums::{Room, Screen};
use textures::Textures;

mod enums;
mod macros;
mod monster;
mod state;
mod textures;

pub const WIDTH: i32 = 600;
pub const HEIGHT: i32 = 450;
pub const SCROLL_AMOUNT: f32 = WIDTH as f32 * 0.0025;

fn main() -> Result<(), Box<dyn Error>> {
    let (mut rl, thread) = raylib::init()
        .size(WIDTH, HEIGHT)
        .title("ONAT")
        .resizable()
        .fullscreen()
        .build();

    let textures = Textures::new(&mut rl, &thread)?;

    let mut state = State::new();

    let mut framebuffer = rl.load_render_texture(&thread, WIDTH as u32, HEIGHT as u32)?;
    while !rl.window_should_close() {
        if rl.is_key_released(KeyboardKey::KEY_F11) {
            rl.toggle_fullscreen();
        }

        let mx = rl.get_mouse_x();
        let my = rl.get_mouse_y();
        let mouse_down = rl.is_mouse_button_released(MouseButton::MOUSE_LEFT_BUTTON)
            || rl.is_mouse_button_released(MouseButton::MOUSE_MIDDLE_BUTTON)
            || rl.is_mouse_button_released(MouseButton::MOUSE_RIGHT_BUTTON);

        state.step(rl.get_screen_width() as f32, rl.get_screen_height() as f32);

        if state.timer.elapsed()?.as_millis() <= 1 / 30 {
            continue;
        }

        let width = rl.get_screen_width();
        let height = rl.get_screen_height();

        let wait = width as f32 / height as f32;
        let what = 4.0 / 3.0;
        let ratio = {
            if wait == what {
                1.0
            } else {
                0.9 + (wait - what)
            }
        };
        state.timer = SystemTime::now();

        state.ingame_time += Duration::from_millis(36);
        let mut img = None;
        let mut tex = None;

        if state.screen == Screen::Office {
            img = Some(Image::gen_image_white_noise(256, 172, 0.5));
            tex = Some(rl.load_texture_from_image(&thread, &(img.unwrap()))?);
        }

        {
            let mut d_ = rl.begin_drawing(&thread);
            let mut d = d_.begin_texture_mode(&thread, &mut framebuffer);
            d.clear_background(Color::BLACK);
            d.set_mouse_cursor(MouseCursor::MOUSE_CURSOR_ARROW);

            match state.screen {
                Screen::TitleScreen => {
                    d.clear_background(Color::BLACK);
                    d.draw_text("One Night at Tux", 5, 5, 32, Color::WHITE);
                    d.draw_text("Click anywhere to start", 5, 48, 32, Color::WHITE);
                    if d.is_mouse_button_pressed(MouseButton::MOUSE_LEFT_BUTTON) {
                        state = State::new();
                        state.screen = Screen::Office;
                    }
                }
                Screen::YouWin => {
                    d.clear_background(Color::GREEN);
                }
                Screen::Office => {
                    if state.laptop_offset_y < HEIGHT as f64 {
                        state.laptop_offset_y += 3.0;
                    }
                    if state.laptop_offset_y != HEIGHT as f64 {
                        d.clear_background(Color::BLACK);
                        d.draw_texture_pro(
                            &textures.laptop,
                            texture_rect!(textures.laptop),
                            Rectangle::new(
                                0.0,
                                state.laptop_offset_y as f32,
                                WIDTH as f32,
                                HEIGHT as f32,
                            ),
                            Vector2::new(0.0, 0.0),
                            0.0,
                            Color::WHITE,
                        );
                        continue;
                    }
                    let t = tex.as_ref().unwrap();
                    d.draw_texture_pro(
                        t,
                        texture_rect!(t),
                        Rectangle::new(
                            WIDTH as f32 / 1.5 - state.bg_offset_x,
                            HEIGHT as f32 / 1.75,
                            t.width as f32,
                            t.height as f32,
                        ),
                        Vector2::new(0.0, 0.0),
                        0.0,
                        Color::new(0, 50, 0, 255),
                    );
                    d.draw_rectangle(
                        (WIDTH as f32 / 1.30 - state.bg_offset_x) as i32,
                        (HEIGHT as f32 / 1.35) as i32,
                        100,
                        32,
                        Color::new(0, 128, 0, 255),
                    );
                    d.draw_rectangle(
                        ((WIDTH as f32 / 1.30 - state.bg_offset_x) as i32) + 2,
                        ((HEIGHT as f32 / 1.35) as i32) + 2,
                        state.tainted as i32 - 4,
                        28,
                        Color::GREEN,
                    );
                    d.draw_texture_pro(
                        &textures.tainted_logo,
                        texture_rect!(textures.tainted_logo),
                        Rectangle::new(
                            WIDTH as f32 / 1.28 - state.bg_offset_x,
                            HEIGHT as f32 / 1.45,
                            textures.tainted_logo.width as f32,
                            textures.tainted_logo.height as f32,
                        ),
                        Vector2::new(0.0, 0.0),
                        0.0,
                        Color::WHITE,
                    );

                    d.draw_texture_pro(
                        &textures.office,
                        texture_rect!(textures.office),
                        Rectangle::new(-state.bg_offset_x, 0.0, WIDTH as f32 * 1.6, HEIGHT as f32),
                        Vector2::new(0.0, 0.0),
                        0.0,
                        Color::WHITE,
                    );

                    d.draw_texture_pro(
                        &textures.door_left,
                        texture_rect!(textures.door_left),
                        Rectangle::new(
                            -state.bg_offset_x,
                            state.left_door_anim_timer,
                            WIDTH as f32 * 1.6,
                            HEIGHT as f32,
                        ),
                        Vector2::new(0.0, 0.0),
                        0.0,
                        Color::WHITE,
                    );

                    d.draw_texture_pro(
                        &textures.door_right,
                        texture_rect!(textures.door_right),
                        Rectangle::new(
                            -state.bg_offset_x,
                            state.right_door_anim_timer,
                            WIDTH as f32 * 1.6,
                            HEIGHT as f32,
                        ),
                        Vector2::new(0.0, 0.0),
                        0.0,
                        Color::WHITE,
                    );
                    d.draw_texture_pro(
                        &textures.office,
                        texture_rect!(textures.office_corners),
                        Rectangle::new(-state.bg_offset_x, 0.0, WIDTH as f32 * 1.6, HEIGHT as f32),
                        Vector2::new(0.0, 0.0),
                        0.0,
                        Color::WHITE,
                    );
                    if state.gang.wilber.active() {
                        let texture = match state.gang.wilber.stage {
                            0 => &textures.gimp1,
                            1 => &textures.gimp2,
                            2 => &textures.gimp3,
                            3 => &textures.gimp4,
                            _ => &textures.gimp5,
                        };
                        d.draw_texture_pro(
                            &texture,
                            texture_rect!(texture),
                            Rectangle::new(
                                (WIDTH as f32) / 2.0 - state.bg_offset_x,
                                HEIGHT as f32 / 2.0,
                                86.0,
                                84.0,
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
                        if mouse_down
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
                        if state.left_door_anim_timer >= -(HEIGHT as f32) {
                            state.left_door_anim_timer -= 5.0;
                        }
                    }

                    // RIGHT DOOR
                    if state.right_door_shut {
                        if state.right_door_anim_timer <= 0.0 {
                            state.right_door_anim_timer += 5.0;
                        }
                    } else {
                        if state.right_door_anim_timer >= -(HEIGHT as f32) {
                            state.right_door_anim_timer -= 5.0;
                        }
                    }
                    state.gang.wilber.rage_increment();
                    if state.camera_timer <= 100.0 {
                        state.camera_timer += 0.04;
                    }
                }
                Screen::CameraRebooting => {
                    if state.camera_timer <= 100.0 {
                        state.camera_timer += 0.02;
                        // TODO: Rebooting animation
                        d.draw_text("Laptop Charging...", 0, HEIGHT / 2, 32, Color::WHITE);
                        if state.camera_timer >= 100.0 {
                            state.screen = Screen::Camera;
                        }
                    } else {
                        // TODO: Rebooting animation
                        d.draw_text(
                            format!("Laptop Rebooting: {:.0}", state.camera_booting_timer).as_str(),
                            0,
                            HEIGHT / 2,
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
                                0.0,
                                state.laptop_offset_y as f32,
                                WIDTH as f32,
                                HEIGHT as f32,
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
                            Rectangle::new(0.0, 0.0, WIDTH as f32, HEIGHT as f32),
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
                        if mouse_down
                            && (mx as f32 >= (state.duct_button.x - state.bg_offset_x)
                                && mx as f32
                                    <= (state.duct_button.x - state.bg_offset_x)
                                        + state.duct_button.width
                                && my as f32 >= state.duct_button.y
                                && my as f32 <= state.duct_button.y + state.duct_button.height)
                        {
                            state.duct_heat_timer = 2500.0;
                        }
                    } else {
                        d.draw_texture_pro(
                            texture,
                            texture_rect!(texture),
                            Rectangle::new(0.0, 0.0, WIDTH as f32, HEIGHT as f32),
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
                        if mons.active() {
                            let tex = match mons.id() {
                                MonsterName::Penny => &textures.penny_stock_texture,
                                MonsterName::Beastie => &textures.beastie_stock_texture,
                                MonsterName::Wilber => &textures.wilber_stock_texture,
                                MonsterName::GoGopher => &textures.gogopher_stock_texture,
                                MonsterName::Tux => &textures.tux_stock_texture,
                                MonsterName::Nolok => &textures.nolok_stock_texture,
                                MonsterName::GoldenTux => &textures.golden_tux_texture,
                            };
                            let (x, y) = match &state.sel_camera {
                                Room::Room1 => match mons.id() {
                                    MonsterName::Penny => (120.0, 200.0),
                                    MonsterName::Beastie => (250.0, 200.0),
                                    MonsterName::Wilber => (0.0, 0.0),
                                    MonsterName::GoGopher => (0.0, 0.0),
                                    MonsterName::Tux => (0.0, 0.0),
                                    MonsterName::Nolok => (0.0, 0.0),
                                    MonsterName::GoldenTux => (0.0, 0.0),
                                },
                                Room::Room2 => match mons.id() {
                                    MonsterName::Penny | MonsterName::Beastie => {
                                        let (x_, y_) = match mons.id() {
                                            MonsterName::Penny => {
                                                ((WIDTH / 2) as f32 - 100.0, 100.0)
                                            }
                                            MonsterName::Beastie => ((WIDTH / 2) as f32, 100.0),
                                            _ => (0.0, 0.0),
                                        };

                                        (
                                            x_ + (mons.progress_to_hallway() as f32 * 50.0),
                                            y_ + (mons.progress_to_hallway() as f32 * 50.0).abs(),
                                        )
                                    }
                                    MonsterName::Wilber => (0.0, 0.0),
                                    MonsterName::GoGopher => (0.0, 0.0),
                                    MonsterName::Tux => (0.0, 0.0),
                                    MonsterName::Nolok => (0.0, 0.0),
                                    MonsterName::GoldenTux => (0.0, 0.0),
                                },
                                Room::Room3 | Room::Room5 => {
                                    ((WIDTH / 2) as f32, (HEIGHT / 2) as f32)
                                }
                                Room::Room4 => (0.0, 0.0),
                                Room::Room6 => (0.0, 0.0),
                                Room::None => (0.0, 0.0),
                                Room::Office => (0.0, 0.0),
                            };

                            d.draw_texture_pro(
                                &tex,
                                texture_rect!(tex),
                                Rectangle::new(x, y, 100.0, 100.0),
                                Vector2::new(0.0, 0.0),
                                0.0,
                                Color::WHITE,
                            );

                            d.draw_text(
                                mons.special_debug_info().as_str(),
                                0,
                                HEIGHT / 2,
                                32,
                                Color::WHITE,
                            );
                        }
                    }

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

                        if (mx as f32 >= clickable.x
                            && mx as f32 <= clickable.x + clickable.width
                            && my as f32 >= clickable.y
                            && my as f32 <= clickable.y + clickable.height)
                        {
                            d.set_mouse_cursor(MouseCursor::MOUSE_CURSOR_POINTING_HAND);
                            if mouse_down {
                                state.sel_camera = Room::from_u64(i as u64).unwrap();
                            }
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
                    if state.gameover_time.elapsed()?.as_secs() >= 5 {
                        state.screen = Screen::TitleScreen;
                    }
                }
            }

            if Screen::TitleScreen != state.screen && Screen::GameOver != state.screen {
                d.draw_texture_pro(
                    &textures.arrow,
                    texture_rect!(textures.arrow),
                    Rectangle::new(
                        0.0,
                        HEIGHT as f32 - (HEIGHT as f32 / 16.0),
                        WIDTH as f32,
                        HEIGHT as f32 / 16.0,
                    ),
                    Vector2::new(0.0, 0.0),
                    0.0,
                    Color::new(255, 255, 255, 128),
                );

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

                d.draw_text(
                    format!("{}:00AM", num).as_str(),
                    WIDTH - 128,
                    0,
                    32,
                    Color::BLACK,
                );

                // Bars

                d.draw_rectangle(5, HEIGHT - 42, 100, 32, Color::BLACK);
                d.draw_rectangle_gradient_h(
                    7,
                    HEIGHT - 40,
                    (100.0 * (state.camera_timer / 100.0)) as i32 - 4,
                    28,
                    Color::WHITE,
                    Color::WHITE,
                );
                let width = 100.0 * (state.camera_timer / 100.0);
                d.draw_texture_pro(
                    &textures.battery_text,
                    Rectangle::new(
                        textures.battery_text.width() as f32,
                        0.0,
                        -width + 7.0,
                        textures.battery_text.height() as f32,
                    ),
                    Rectangle::new(width - 100.0, HEIGHT as f32 - 40.0, width - 7.0, 28.0),
                    Vector2::new(
                        textures.battery_text.width() as f32,
                        textures.battery_text.height() as f32,
                    ),
                    180.0,
                    Color::WHITE,
                );
            };
        }

        let mut d = rl.begin_drawing(&thread);

        let prop_width = width as f32 / ratio;
        let margin = width as f32 - prop_width;
        d.clear_background(Color::BLACK);
        d.draw_texture_pro(
            &framebuffer,
            Rectangle::new(
                framebuffer.width() as f32,
                -framebuffer.height() as f32,
                -framebuffer.width() as f32,
                framebuffer.height() as f32,
            ),
            Rectangle::new(0.0, 0.0, prop_width, height as f32),
            Vector2::new(prop_width + (margin / 2.0), height as f32),
            180.0,
            Color::WHITE,
        );
        if let Screen::TitleScreen = state.screen {
            continue;
        }
        if let Screen::GameOver = state.screen {
            continue;
        }

        if mx <= (width / 4) {
            if state.bg_offset_x > 0.0 {
                state.bg_offset_x -= SCROLL_AMOUNT;
            }
        }
        if mx >= width - (width / 4) {
            if state.bg_offset_x < (width as f32) / 2.0 {
                state.bg_offset_x += SCROLL_AMOUNT;
            }
        }

        if my >= height - (height / 16) && mouse_down {
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
                        WIDTH - 128 - 5
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
                // special cases
                match mons.id() {
                    // GoGopher fills the tainted meter by 50% and then leaves. Once he is in the office,
                    // he won't leave until he's finished.
                    MonsterName::GoGopher => {
                        if state.tainted_cache == 0.0 {
                            state.tainted_cache = state.tainted;
                        }
                        if state.tainted <= state.tainted_cache + 50.0 {
                            state.tainted += mons.taint_percent();
                        } else {
                            mons.set_room(Room::None);
                        }
                    }
                    _ => {}
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
            state.gameover_time = SystemTime::now();
            state.screen = Screen::GameOver;
        }
    }

    Ok(())
}
