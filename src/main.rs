use std::error::Error;

use num_derive::FromPrimitive;
use num_traits::FromPrimitive;
use raylib::prelude::*;
use textures::Textures;

mod macros;
mod textures;

pub const WIDTH: i32 = 1200;
pub const HEIGHT: i32 = 900;

enum Screen {
    Office,
    Camera,
}

extern crate num_derive;
#[derive(FromPrimitive)]
enum SelectedCamera {
    Cam1,
    Cam2,
    Cam3A,
    Cam3B,
    Cam3C,
    Cam4A,
    Cam4B,
    Cam5A,
    Cam5B,
    None,
}

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
    ];

    let mut sel_camera = SelectedCamera::None;

    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);

        d.clear_background(Color::WHITE);

        let laptop_height = HEIGHT as f32 - (textures.laptop.height as f32 * 0.1) - laptop_offset_y;

        let mx = d.get_mouse_x();
        let my = d.get_mouse_y();

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
                let color = match sel_camera {
                    SelectedCamera::None => Color::WHITE,
                    SelectedCamera::Cam1 => Color::RED,
                    SelectedCamera::Cam2 => Color::ORANGE,
                    SelectedCamera::Cam3A => Color::YELLOW,
                    SelectedCamera::Cam3B => Color::GREEN,
                    SelectedCamera::Cam3C => Color::BLUE,
                    SelectedCamera::Cam4A => Color::DARKBLUE,
                    SelectedCamera::Cam4B => Color::VIOLET,
                    SelectedCamera::Cam5A => Color::BROWN,
                    SelectedCamera::Cam5B => Color::DARKBROWN,
                };
                d.clear_background(color);
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
                        sel_camera = SelectedCamera::from_u64(i as u64).unwrap();
                        // :3
                    }
                }
            }
        }
    }

    Ok(())
}
