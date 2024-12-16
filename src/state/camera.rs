use std::sync::atomic::{AtomicBool, Ordering};

use super::{Screen, State};
use crate::config::config;
use crate::{enums::Room, monster::Monster, state::CAMERA_TIME, texture_rect};
use crate::{style_pop, style_push};

use ::imgui::{Condition, StyleColor};
use parking_lot::Mutex;
use raylib::prelude::*;

const TEXT_WIDTH: i32 = ("Laptop Rebooting".len() as i32) * 24;

impl<'a> State<'a> {
    pub fn camera_step(&mut self) {
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
    }

    pub fn camera_draw(
        &mut self,
        d: &mut RaylibDrawHandle,
        thread: &RaylibThread,
        tex: Texture2D,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut d = d.begin_texture_mode(&thread, &mut self.framebuffer);
        d.clear_background(Color::BLACK);

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
                        0.0 + 0.0,
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
                        0.0 + 0.0,
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
            mons.draw(self.textures, &mut d, 0.0, 0.0, 1.0, 1.0);
            if mons.move_timer() >= 1 || mons.time_in_room().elapsed()?.as_millis() <= 50 {
                self.audio.play_noise()?;
                d.draw_texture_pro(
                    &tex,
                    texture_rect!(tex),
                    Rectangle::new(
                        0.0 + 0.0,
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
                0.0 + 0.0,
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
                    0.0 + 0.0,
                    self.laptop_offset_y as f32,
                    config().width() as f32,
                    config().height() as f32,
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
                    0.0 + 0.0,
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

    pub fn camera_rebooting_step(&mut self) {
        if self.going_to_office {
            if self.laptop_offset_y < config().height() as f64 {
                self.laptop_offset_y += self.var_name;
            } else {
                self.screen = Screen::Office;
                self.going_to_office = false;
            }
        }
    }

    pub fn camera_rebooting_draw(
        &mut self,
        d: &mut RaylibDrawHandle,
        thread: &RaylibThread,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut d = d.begin_texture_mode(&thread, &mut self.framebuffer);
        d.clear_background(Color::BLACK);

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
    ) -> Result<(), Box<dyn std::error::Error>> {
        let btn_width = config().ui_scale() * 100.0;
        let mut goto_cam1 = AtomicBool::new(false);
        let mut goto_cam2 = AtomicBool::new(false);
        let mut goto_cam3 = AtomicBool::new(false);
        let mut goto_cam4 = AtomicBool::new(false);
        let mut goto_cam5 = AtomicBool::new(false);
        let mut goto_cam6 = AtomicBool::new(false);
        let mut duct_heatup = AtomicBool::new(false);

        let s = Mutex::new(&self);

        d.start_imgui(|ui| {
            let se = s.lock();

            ui.window("Rooms")
                .position(
                    [config().real_margin() + 20.0, 20.0],
                    Condition::FirstUseEver,
                )
                .size([0.0, 0.0], Condition::Always)
                .build(|| {
                    let room_buttons = vec![
                        ("Tux", &goto_cam1),
                        ("Penny+Beastie", &goto_cam2),
                        ("Left Hallway", &goto_cam3),
                        ("Right Hallway", &goto_cam5),
                        ("Gopher Vent", &goto_cam4),
                        ("Wilbur's Room", &goto_cam6),
                    ];
                    ui.set_window_font_scale(config().ui_scale());
                    let styles = style_push!(ui);

                    for (title, value) in room_buttons {
                        if ui.button(title) {
                            value.store(true, Ordering::Relaxed);
                            ui.separator();
                        };
                    }
                    style_pop!(styles);
                });

            // We need to draw the ui here as well because we're another imgui element and can't have two frames at once.
            ui.window("ui")
                .resizable(false)
                .movable(false)
                .title_bar(false)
                .bg_alpha(0.0)
                .position([config().real_margin(), 0.0], ::imgui::Condition::Always)
                .size(
                    [config().real_width() as f32, config().real_height() as f32],
                    ::imgui::Condition::Always,
                )
                .build(|| {
                    ui.set_window_font_scale(config().ui_scale());

                    se.draw_battery(ui.get_window_draw_list()).unwrap();
                    se.draw_arrow(ui.get_window_draw_list()).unwrap();
                    se.draw_time(ui.get_window_draw_list()).unwrap();
                    if se.sel_camera == Room::Room6 && se.gang.wilber.active() {
                        se.draw_rage(ui.get_window_draw_list()).unwrap();
                    }
                    if se.sel_camera == Room::Room4 && se.gang.gogopher.active() {
                        let bat_height = Self::bat_height();
                        ui.set_cursor_pos([
                            Self::bat_start(),
                            config().real_height() as f32 - (bat_height * 3.0),
                        ]);
                        let styles = style_push!(ui);
                        if ui.button_with_size(
                            "HEAT UP",
                            [Self::bat_width() as f32, bat_height as f32 / 2.0],
                        ) {
                            duct_heatup.store(true, Ordering::Relaxed);
                        }
                        style_pop!(styles);
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
        if *duct_heatup.get_mut() {
            self.gang.gogopher.duct_heat_timer = 250;
            self.gang.gogopher.duct_timer = 0;
        }
        Ok(())
    }
}
