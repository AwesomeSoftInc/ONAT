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
        Rectangle::new(304.0, 247.0, 210.0, 160.0),
        Rectangle::new(160.0, 180.0, 140.0, 100.0),
        Rectangle::new(140.0, 480.0, 120.0, 85.0),
        Rectangle::new(334.0, 466.0, 120.0, 85.0),
        Rectangle::new(543.0, 450.0, 120.0, 85.0),
        Rectangle::new(139.0, 616.0, 160.0, 130.0),
        Rectangle::new(520.0, 630.0, 160.0, 130.0),
        Rectangle::new(12.0, 636.0, 116.0, 121.0),
        Rectangle::new(721.0, 631.0, 89.0, 116.0),
        Rectangle::new(23.0, 184.0, 137.0, 72.0),
    ];

    let mut sel_camera = Room::None;
    let mut timer = SystemTime::now();

    let mut ingame_time = UNIX_EPOCH;
    let mut gang = Gang::new();

    let mut tainted = 0.0;

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
        gang.step(cur_time);
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
                    Room::Room3A => d.clear_background(Color::YELLOW),
                    Room::Room3B => d.clear_background(Color::GREEN),
                    Room::Room3C => d.clear_background(Color::BLUE),
                    Room::Room4A => d.clear_background(Color::DARKBLUE),
                    Room::Room4B => d.clear_background(Color::VIOLET),
                    Room::Room5A => d.clear_background(Color::BROWN),
                    Room::Room5B => d.clear_background(Color::DARKBROWN),
                    Room::Room6 => d.clear_background(Color::DARKGREEN),
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
            format!("{}:00PM", num).as_str(),
            WIDTH - 128,
            0,
            32,
            Color::BLACK,
        );

        let inroom = gang.in_room(&sel_camera);
        let mut y = 5;
        for mons in inroom {
            d.draw_text(&mons.name(), 5, y, 32, Color::BLACK);
            y += 48;
        }

        let inoffice = gang.in_room(&Room::Office);
        for mons in inoffice {
            d.draw_text(&mons.name(), 5, y, 32, Color::BLACK);
            y += 48;
            match mons {
                _ => {
                    tainted += 0.02;
                }
            }
        }

        if tainted >= 100.0 {
            screen = Screen::GameOver;
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
