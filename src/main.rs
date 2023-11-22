use monster::Gang;
use raylib::prelude::*;

use num_traits::FromPrimitive;
use std::{
    error::Error,
    time::{Duration, SystemTime, UNIX_EPOCH},
};

use enums::{Room, Screen};
use textures::Textures;

mod enums;
mod macros;
mod monster;
mod textures;

pub const WIDTH: i32 = 1200;
pub const HEIGHT: i32 = 900;

fn main() -> Result<(), Box<dyn Error>> {
    let (mut rl, thread) = raylib::init().size(WIDTH, HEIGHT).title("ONAT").build();

    let textures = Textures::new(&mut rl, &thread)?;

    let mut screen = Screen::Office;
    let mut bg_offset_x = 400.0;
    let mut laptop_offset_y = 0.0;

    let camera_clickables = vec![
        Rectangle::new(380.0, 121.0, 240.0, 128.0),
        Rectangle::new(362.0, 293.0, 336.0, 208.0),
        Rectangle::new(114.0, 724.0, 190.0, 150.0),
        Rectangle::new(441.0, 539.0, 161.0, 123.0),
        Rectangle::new(722.0, 674.0, 190.0, 150.0),
        Rectangle::new(35.0, 101.0, 150.0, 128.0),
    ];

    let door_buttons = vec![
        Rectangle::new(430.0, 330.0, 128.0, 128.0),
        Rectangle::new(1360.0, 330.0, 128.0, 128.0),
    ];

    let mut sel_camera = Room::None;
    let mut timer = SystemTime::now();

    let mut ingame_time = UNIX_EPOCH;
    let mut gang = Gang::new();

    let mut tainted = 0.0;

    let mut left_door_shut = false;
    let mut right_door_shut = false;

    while !rl.window_should_close() {
        if timer.elapsed()?.as_millis() <= 1 / 30 {
            continue;
        }
        timer = SystemTime::now();

        ingame_time += Duration::from_millis(36);

        let mut d = rl.begin_drawing(&thread);

        d.clear_background(Color::WHITE);

        let laptop_height = HEIGHT as f32 - (textures.laptop.height as f32 * 0.1) - laptop_offset_y;

        let mx = d.get_mouse_x();
        let my = d.get_mouse_y();

        let cur_time = ingame_time.duration_since(UNIX_EPOCH)?;
        gang.step(cur_time, left_door_shut, right_door_shut);
        let num = {
            let ct = cur_time.as_secs() / 3600;
            if ct == 0 {
                12
            } else {
                ct
            }
        };

        match screen {
            Screen::Office => {
                d.draw_texture_pro(
                    &textures.office,
                    texture_rect!(textures.office),
                    Rectangle::new(-bg_offset_x, 0.0, WIDTH as f32 * 1.6, HEIGHT as f32),
                    Vector2::new(0.0, 0.0),
                    0.0,
                    Color::WHITE,
                );
                d.draw_texture_pro(
                    &textures.laptop,
                    texture_rect!(textures.laptop),
                    texture_rect!(
                        textures.laptop * 2,
                        (WIDTH / 2 - textures.laptop.width),
                        laptop_height
                    ),
                    Vector2::new(0.0, 0.0),
                    0.0,
                    Color::WHITE,
                );

                // 379 330 32 32
                let mut i = 0;
                for button in &door_buttons {
                    d.draw_rectangle_lines(
                        (button.x - bg_offset_x) as i32,
                        button.y as i32,
                        button.width as i32,
                        button.height as i32,
                        Color::RED,
                    );
                    if d.is_mouse_button_released(MouseButton::MOUSE_LEFT_BUTTON)
                        && (mx as f32 >= (button.x - bg_offset_x)
                            && mx as f32 <= (button.x - bg_offset_x) + button.width
                            && my as f32 >= button.y
                            && my as f32 <= button.y + button.height)
                    {
                        if i == 0 {
                            left_door_shut = !left_door_shut;
                        } else {
                            right_door_shut = !right_door_shut;
                        }
                    }
                    i += 1;
                }
                // LEFT DOOR
                if left_door_shut {
                    d.draw_rectangle(92 - bg_offset_x as i32, 42, 340, 940, Color::RED);
                }

                // RIGHT DOOR
                if right_door_shut {
                    d.draw_rectangle(1522 - bg_offset_x as i32, 42, 340, 940, Color::RED);
                }

                if mx <= (WIDTH / 4) {
                    if bg_offset_x > 0.0 {
                        bg_offset_x -= 1.1;
                    }
                }
                if mx >= WIDTH - (WIDTH / 4) {
                    if bg_offset_x < 600.0 {
                        bg_offset_x += 1.1;
                    }
                }
                if laptop_offset_y > 0.0 {
                    laptop_offset_y -= 1.0;
                }
                if my >= HEIGHT - (HEIGHT / 16)
                    && d.is_mouse_button_released(MouseButton::MOUSE_LEFT_BUTTON)
                {
                    screen = Screen::Camera;
                }
            }
            Screen::Camera => {
                match sel_camera {
                    Room::None => {
                        d.clear_background(Color::WHITE);
                    }
                    Room::Room1 => d.clear_background(Color::RED),
                    Room::Room2 => d.clear_background(Color::ORANGE),
                    Room::Room3 => d.clear_background(Color::YELLOW),
                    Room::Room4 => d.clear_background(Color::GREEN),
                    Room::Room5 => d.clear_background(Color::BLUE),
                    Room::Room6 => d.clear_background(Color::DARKBLUE),
                    Room::Office => panic!("tried to draw office"),
                };

                d.draw_texture_pro(
                    &textures.camera,
                    texture_rect!(textures.camera),
                    texture_rect!(textures.camera),
                    Vector2::new(0.0, 0.0),
                    0.0,
                    Color::WHITE,
                );
                d.draw_texture_pro(
                    &textures.laptop,
                    texture_rect!(textures.laptop),
                    texture_rect!(
                        textures.laptop * 2,
                        (WIDTH / 2 - textures.laptop.width),
                        laptop_height
                    ),
                    Vector2::new(0.0, 0.0),
                    0.0,
                    Color::WHITE,
                );

                if laptop_offset_y < 50.0 {
                    laptop_offset_y += 1.0;
                }
                if my >= HEIGHT - (HEIGHT / 8)
                    && d.is_mouse_button_released(MouseButton::MOUSE_LEFT_BUTTON)
                {
                    screen = Screen::Office;
                }
                for i in 0..camera_clickables.len() {
                    let clickable = &camera_clickables.get(i).unwrap();
                    d.draw_rectangle_lines(
                        clickable.x as i32,
                        clickable.y as i32,
                        clickable.width as i32,
                        clickable.height as i32,
                        Color::RED,
                    );

                    let cam = Room::from_u64(i as u64).unwrap();

                    let inroom = gang.in_room(&cam);
                    let mut y = 0;
                    for mons in inroom {
                        d.draw_text(
                            format!("{} - {}", &mons.name(), &mons.ai_level()).as_str(),
                            5 + clickable.x as i32,
                            5 + clickable.y as i32 + y,
                            16,
                            Color::BLACK,
                        );
                        y += 16;
                    }

                    if d.is_mouse_button_released(MouseButton::MOUSE_LEFT_BUTTON)
                        && (mx as f32 >= clickable.x
                            && mx as f32 <= clickable.x + clickable.width
                            && my as f32 >= clickable.y
                            && my as f32 <= clickable.y + clickable.height)
                    {
                        sel_camera = Room::from_u64(i as u64).unwrap();
                        // :3
                    }
                }
            }
            Screen::GameOver => {
                d.clear_background(Color::RED);
            }
        }

        d.draw_text(
            format!("{}:00AM", num).as_str(),
            WIDTH - 128,
            0,
            32,
            Color::BLACK,
        );

        if !left_door_shut && !right_door_shut {
            let inoffice = gang.in_room(&Room::Office);
            let mut y = 0;
            for mons in inoffice {
                d.draw_text(&mons.name(), 5, y, 32, Color::BLACK);
                y += 48;
                match mons {
                    _ => {
                        tainted += mons.taint_percent();
                    }
                }
            }
            if tainted >= 100.0 {
                screen = Screen::GameOver;
            }
        }

        d.draw_text(
            format!("Tainted: {:.0}", tainted).as_str(),
            5,
            HEIGHT - 32,
            32,
            Color::BLACK,
        )
    }

    Ok(())
}
