use std::sync::atomic::{AtomicBool, Ordering};
use std::time::SystemTime;

use super::{Screen, State};
use crate::config::config;
use crate::{enums::Room, monster::Monster, state::CAMERA_TIME, texture_rect};

use ::imgui::{Condition, StyleColor};
use raylib::prelude::*;

const TEXT_WIDTH: i32 = ("Laptop Rebooting".len() as i32) * 24;

impl<'a> State<'a> {
    pub fn camera_draw(
        &mut self,
        d: &mut RaylibDrawHandle,
        thread: &RaylibThread,
        mx: i32,
        my: i32,
        tex: Texture2D,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut d = d.begin_texture_mode(&thread, &mut self.framebuffer);
        d.clear_background(Color::BLACK);

        #[cfg(not(feature = "no_camera_timer"))]
        if self.camera_timer >= 0.0 {
            self.camera_timer -= CAMERA_TIME;
        } else {
            self.camera_booting = true;
            self.sel_camera = Room::Room1;
            self.screen = Screen::Office;
        }
        if self.going_to_office {
            if self.laptop_offset_y < config().height() as f64 {
                self.laptop_offset_y += self.var_name;
            } else {
                self.screen = Screen::Office;
                self.going_to_office = false;
            }
        }

        if self.camera_booting {
            self.screen = Screen::CameraRebooting;
            return Ok(());
        }

        {
            let textures = &self.textures.misc;

            let texture = &*match self.sel_camera {
                Room::Room1 => textures.cam1(),
                Room::Room2 => textures.cam2(),
                Room::Room3 => {
                    if !self.skinman_appeared {
                        if self.skinman_chance <= 1 {
                            if self.camera_last_changed.elapsed()?.as_millis() <= 250 {
                                textures.cam3_happyskinman()
                            } else {
                                self.skinman_appeared = true;
                                textures.cam3()
                            }
                        } else {
                            textures.cam3()
                        }
                    } else {
                        textures.cam3()
                    }
                }
                Room::Room4 => textures.cam4(),
                Room::Room5 => textures.cam5(),
                Room::Room6 => textures.cam6(),
                _ => {
                    panic!("tried to draw unsupported room {:?}", self.sel_camera)
                }
            };

            if self.sel_camera == Room::Room4 {
                let red = self.gang.gogopher.duct_heat_timer as u8;
                d.draw_texture_pro(
                    texture,
                    texture_rect!(texture),
                    Rectangle::new(
                        config().margin() + 0.0,
                        0.0,
                        config().width() as f32,
                        config().height() as f32,
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
                        config().margin() + 0.0,
                        0.0,
                        config().width() as f32,
                        config().height() as f32,
                    ),
                    Vector2::new(0.0, 0.0),
                    0.0,
                    Color::WHITE,
                );
            }
            if self.sel_camera == Room::Room6 {
                self.gang.wilber.rage_decrement();
            } else {
                self.gang.wilber.rage_increment(&mut self.audio);
            }
        }

        let inroom = self.gang.in_room(self.sel_camera.clone());
        for mons in inroom {
            mons.draw(self.textures, &mut d, config().margin(), 0.0, 1.0, 1.0);
            if mons.move_timer() >= 1 || mons.time_in_room().elapsed()?.as_millis() <= 50 {
                self.audio.play_noise()?;
                d.draw_texture_pro(
                    &tex,
                    texture_rect!(tex),
                    Rectangle::new(
                        config().margin() + 0.0,
                        0.0,
                        config().width() as f32,
                        config().height() as f32,
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
                config().margin() + 0.0,
                0.0,
                config().width() as f32,
                config().height() as f32,
            ),
            Vector2::new(0.0, 0.0),
            0.0,
            Color::new(255, 255, 255, 48),
        );

        if self.laptop_offset_y > 0.0 {
            let laptop = &*self.textures.misc.laptop();
            d.draw_texture_pro(
                &laptop,
                texture_rect!(laptop),
                Rectangle::new(
                    config().margin() + 0.0,
                    self.laptop_offset_y as f32,
                    config().width() as f32,
                    config().height() as f32,
                ),
                Vector2::new(0.0, 0.0),
                0.0,
                Color::WHITE,
            );
        }
        if self.sel_camera == Room::Room4 {
            d.draw_rectangle(
                self.duct_button.x as i32 + 1,
                self.duct_button.y as i32,
                self.duct_button.width as i32,
                self.duct_button.height as i32,
                Color::GRAY,
            );
            d.draw_rectangle_lines_ex(self.duct_button, 5.0, Color::BLACK);
            d.draw_text_ex(
                &self.font,
                "HEAT UP",
                Vector2::new(self.duct_button.x + 32.0, self.duct_button.y + 32.0),
                48.0,
                6.0,
                Color::BLACK,
            );
            if d.is_mouse_button_released(MouseButton::MOUSE_BUTTON_LEFT)
                && (mx as f32 >= (self.duct_button.x)
                    && mx as f32 <= (self.duct_button.x) + self.duct_button.width
                    && my as f32 >= self.duct_button.y
                    && my as f32 <= self.duct_button.y + self.duct_button.height)
            {
                self.gang.gogopher.duct_heat_timer = 250;
                self.gang.gogopher.duct_timer = 0;
            }
        }
        if self.sel_camera == Room::Room6 && self.gang.wilber.active() {
            let battery_bar_height = config().height() as f32 / 13.5;
            let battery_bar_y = config().height() as f32 - (config().height() as f32 / 5.0);
            let rage = self.gang.wilber.rage();
            let gimp_width = (165.0 * (rage / 100.0)) as i32 - 4;

            d.draw_rectangle_gradient_h(
                config().margin() as i32 + 20,
                battery_bar_y as i32 + 2,
                gimp_width,
                (config().height() as f32 / 15.0) as i32,
                Color::BLACK,
                Color::new(255, 23, 62, 255),
            );
            let rage_bar = &*self.textures.misc.rage_bar();
            d.draw_texture_pro(
                &rage_bar,
                texture_rect!(rage_bar),
                Rectangle::new(
                    config().margin() + 14.0,
                    battery_bar_y,
                    config().width() as f32 / 7.5,
                    battery_bar_height,
                ),
                Vector2::new(0.0, 0.0),
                0.0,
                Color::WHITE,
            );
        }
        let millis = self.camera_last_changed.elapsed()?.as_millis();

        if millis <= 50 {
            //self.audio.play_noise()?;
            d.draw_texture_pro(
                &tex,
                texture_rect!(tex),
                Rectangle::new(
                    config().margin() + 0.0,
                    0.0,
                    config().width() as f32,
                    config().height() as f32,
                ),
                Vector2::new(0.0, 0.0),
                0.0,
                Color::WHITE,
            );
        }

        if millis > 50 && millis <= 60 {
            self.audio.noise_halt();
        }
        Ok(())
    }

    pub fn camera_rebooting_draw(
        &mut self,
        d: &mut RaylibDrawHandle,
        thread: &RaylibThread,
        mx: i32,
        my: i32,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut d = d.begin_texture_mode(&thread, &mut self.framebuffer);
        d.clear_background(Color::BLACK);

        if self.going_to_office {
            if self.laptop_offset_y < config().height() as f64 {
                self.laptop_offset_y += self.var_name;
            } else {
                self.screen = Screen::Office;
                self.going_to_office = false;
            }
            return Ok(());
        }
        #[cfg(not(feature = "no_camera_timer"))]
        if self.camera_timer <= 100.0 {
            self.camera_timer += CAMERA_TIME;
            let x = ((config().width() as i32 / 2) as f32) - (TEXT_WIDTH / 2) as f32;
            let y = config().height() / 2;

            d.draw_text_ex(
                &self.font,
                "Laptop Rebooting",
                Vector2::new(x + (TEXT_WIDTH / 8) as f32, y as f32 - 16.0),
                32.0,
                6.0,
                Color::WHITE,
            );
        } else {
            self.camera_booting = false;
            self.screen = Screen::Camera;
        }
        Ok(())
    }

    pub fn camera_ui_draw(
        &mut self,
        d: &mut RaylibDrawHandle,
        thread: &RaylibThread,
        mx: i32,
        my: i32,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut goto_cam1 = AtomicBool::new(false);
        let mut goto_cam2 = AtomicBool::new(false);
        let mut goto_cam3 = AtomicBool::new(false);
        let mut goto_cam4 = AtomicBool::new(false);
        let mut goto_cam5 = AtomicBool::new(false);
        let mut goto_cam6 = AtomicBool::new(false);

        d.start_imgui(|ui| {
            ui.window("Rooms")
                .title_bar(false)
                .position([20.0, 20.0], Condition::Always)
                .size([0.0, 0.0], Condition::Always)
                .resizable(false)
                .movable(false)
                .build(|| {
                    let room_buttons = vec![
                        ("Tux", &goto_cam1),
                        ("Penny+Beastie", &goto_cam2),
                        ("Left Hallway", &goto_cam3),
                        ("Right Hallway", &goto_cam5),
                        ("Gopher Vent", &goto_cam4),
                        ("Wilbur's Room", &goto_cam6),
                    ];
                    ui.set_window_font_scale(3.0);
                    let styles = vec![
                        ui.push_style_color(StyleColor::Button, [0.25, 0.25, 0.25, 1.0]),
                        ui.push_style_color(StyleColor::ButtonHovered, [0.15, 0.15, 0.15, 1.0]),
                        ui.push_style_color(StyleColor::ButtonActive, [0.05, 0.05, 0.05, 1.0]),
                        ui.push_style_color(StyleColor::Separator, [0.0, 0.0, 0.0, 0.0]),
                    ];

                    for (title, value) in room_buttons {
                        if ui.button_with_size(title, [config().width() as f32 / 4.0, 100.0]) {
                            value.store(true, Ordering::Relaxed);
                            ui.separator();
                        };
                    }

                    for style in styles {
                        style.pop();
                    }
                });
        });

        if *goto_cam1.get_mut() {
            self.sel_camera = Room::Room1;
        }
        if *goto_cam2.get_mut() {
            self.sel_camera = Room::Room2;
        }
        if *goto_cam3.get_mut() {
            self.sel_camera = Room::Room3;
        }
        if *goto_cam4.get_mut() {
            self.sel_camera = Room::Room4;
        }
        if *goto_cam5.get_mut() {
            self.sel_camera = Room::Room5;
        }
        if *goto_cam6.get_mut() {
            self.sel_camera = Room::Room6;
        }

        Ok(())
    }
}
