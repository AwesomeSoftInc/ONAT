use std::alloc::System;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::SystemTime;

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
            self.screen = Screen::CameraRebooting;
        }
        if self.going_to_office {
            if self.laptop_offset_y < config().height() as f64 {
                self.laptop_offset_y += self.var_name;
            } else {
                self.screen = Screen::Office;
                self.going_to_office = false;
            }
        }

        if self.sel_camera == Room::Room6 {
            self.gang.wilber.rage_decrement();
        } else {
            self.gang.wilber.rage_increment(&mut self.audio);
        }
    }

    pub fn camera_draw(
        &mut self,
        d: &mut RaylibDrawHandle,
        thread: &RaylibThread,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut d = d.begin_texture_mode(&thread, &mut self.framebuffer);
        d.clear_background(Color::BLACK);

        if self.camera_booting {
            self.screen = Screen::CameraRebooting;
            return Ok(());
        }

        {
            let textures = &self.textures;

            let texture = &*match self.sel_camera {
                Room::Room1 => textures.rooms.tuxslair(),
                Room::Room2 => textures.rooms.mainhall(),
                Room::Room3 => {
                    if !self.skinman_appeared {
                        if self.skinman_chance <= 1 {
                            if self.camera_last_changed.elapsed()?.as_millis() <= 250 {
                                textures.rooms.bigtuxwatching_happyskinman()
                            } else {
                                self.skinman_appeared = true;
                                textures.rooms.bigtuxwatching()
                            }
                        } else {
                            textures.rooms.bigtuxwatching()
                        }
                    } else {
                        textures.rooms.bigtuxwatching()
                    }
                }
                Room::Room4 => textures.rooms.airduct(),
                Room::Room5 => textures.rooms.otherhall(),
                Room::Room6 => textures.rooms.drawingroom(),
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
        }

        let inroom = self.gang.in_room(self.sel_camera.clone());
        for mons in inroom {
            mons.draw(self.textures, &mut d, 0.0, 0.0, 1.0, 1.0);
            if mons.time_in_room().elapsed()?.as_millis() <= 50 {
                self.camera_last_changed = SystemTime::now();
            }
        }

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

        self.camera_changing = millis <= 50 && self.screen != Screen::GameOver;
        if self.camera_changing {
            if !self.audio.noise.is_playing() {
                self.audio.noise.play()?;
            }
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
        if self.camera_timer <= 100.0 {
            self.camera_timer += CAMERA_TIME;
        } else {
            self.camera_booting = false;
            self.screen = Screen::Camera;
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

            let mut builder = ui
                .window("Rooms")
                .position(
                    [
                        config().real_width_raw() as f32 - config().real_margin() - 400.0,
                        config().real_height() as f32 - 700.0,
                    ],
                    Condition::Always,
                )
                .movable(false)
                .resizable(false)
                .title_bar(false);

            if se.sel_camera != Room::Room4 {
                builder = builder.focused(true);
            }

            // We need to draw the ui here as well because we're another imgui element and can't have two frames at once.
            ui.window("ui")
                .resizable(false)
                .movable(false)
                .title_bar(false)
                .bg_alpha(0.0)
                .focused(false)
                .position([0.0, 0.0], ::imgui::Condition::Always)
                .size(
                    [
                        config().real_width_raw() as f32 + config().real_margin(),
                        config().real_height() as f32,
                    ],
                    ::imgui::Condition::Always,
                )
                .build(|| {
                    ui.set_window_font_scale(config().ui_scale());
                    let styles = style_push!(ui);

                    se.draw_battery(ui.get_window_draw_list()).unwrap();
                    se.draw_arrow(ui.get_window_draw_list()).unwrap();

                    ui.set_window_font_scale(config().ui_scale() * 2.0);
                    let time = format!("{}:00AM", se.time().unwrap());
                    let font_off = ui.calc_text_size(time.clone())[0];
                    se.draw_time(&time, font_off, ui.get_window_draw_list())
                        .unwrap();
                    if config().on_tutorial() {
                        se.draw_bonzi_text(ui.get_window_draw_list()).unwrap();
                    }

                    ui.set_window_font_scale(config().ui_scale());

                    if se.sel_camera == Room::Room6 && se.gang.wilber.active() {
                        se.draw_rage(ui.get_window_draw_list()).unwrap();
                    }
                    let off_x = (config().real_width_raw() as f32 - config().real_margin() as f32)
                        - (200.0 * config().ui_scale());
                    let off_y = (config().real_height() as f32) - (350.0 * config().ui_scale());
                    let room_buttons = vec![
                        ("CAM 6", Some(&goto_cam6), [off_x + 25.0, off_y + 25.0]),
                        ("CAM 1", Some(&goto_cam1), [off_x + 150.0, off_y + 150.0]),
                        ("CAM 2", Some(&goto_cam2), [off_x + 150.0, off_y + 250.0]),
                        ("CAM 3", Some(&goto_cam3), [off_x + 50.0, off_y + 500.0]),
                        ("CAM 4", Some(&goto_cam4), [off_x + 150.0, off_y + 425.0]),
                        ("CAM 5", Some(&goto_cam5), [off_x + 250.0, off_y + 500.0]),
                        ("OFFICE", None, [off_x + 150.0, off_y + 500.0]),
                    ];
                    let styles = (
                        vec![
                            ui.push_style_color(::imgui::StyleColor::FrameBg, [0.0, 0.0, 0.0, 0.0]),
                            ui.push_style_color(
                                ::imgui::StyleColor::WindowBg,
                                [0.0, 0.0, 0.0, 0.0],
                            ),
                            ui.push_style_color(
                                ::imgui::StyleColor::Separator,
                                [0.0, 0.0, 0.0, 0.0],
                            ),
                            ui.push_style_color(
                                ::imgui::StyleColor::Button,
                                [0.25, 0.25, 0.25, 1.0],
                            ),
                            ui.push_style_color(
                                ::imgui::StyleColor::ButtonHovered,
                                [0.50, 0.50, 0.50, 1.0],
                            ),
                            ui.push_style_color(
                                ::imgui::StyleColor::ButtonActive,
                                [0.75, 0.75, 0.75, 1.0],
                            ),
                        ],
                        vec![ui.push_style_var(::imgui::StyleVar::FramePadding([2.0, 2.0]))],
                    );

                    for (title, value, position) in room_buttons {
                        ui.set_cursor_pos(position);
                        if ui.button_with_size(title, [100.0, 75.0]) {
                            if let Some(value) = value {
                                value.store(true, Ordering::Relaxed);
                            }
                            ui.separator();
                        };
                    }

                    if se.sel_camera == Room::Room4 && se.gang.gogopher.active() {
                        let bat_height = Self::bat_height();
                        ui.set_cursor_pos([
                            Self::bat_start(),
                            (config().real_height() as f32 - bat_height)
                                - (50.0 * config().ui_scale())
                                - bat_height,
                        ]);
                        if ui.button_with_size(
                            "HEAT UP",
                            [Self::bat_width() as f32, bat_height as f32],
                        ) {
                            duct_heatup.store(true, Ordering::Relaxed);
                        }
                    }
                    style_pop!(styles);
                });
        });

        if *goto_cam1.get_mut() && self.sel_camera != Room::Room1 {
            self.sel_camera = Room::Room1;
            self.camera_last_changed = SystemTime::now();
        }
        if *goto_cam2.get_mut() && self.sel_camera != Room::Room2 {
            self.sel_camera = Room::Room2;
            self.camera_last_changed = SystemTime::now();
        }
        if *goto_cam3.get_mut() && self.sel_camera != Room::Room3 {
            self.sel_camera = Room::Room3;
            self.camera_last_changed = SystemTime::now();
        }
        if *goto_cam4.get_mut() && self.sel_camera != Room::Room4 {
            self.sel_camera = Room::Room4;
            self.camera_last_changed = SystemTime::now();
        }
        if *goto_cam5.get_mut() && self.sel_camera != Room::Room5 {
            self.sel_camera = Room::Room5;
            self.camera_last_changed = SystemTime::now();
        }
        if *goto_cam6.get_mut() && self.sel_camera != Room::Room6 {
            self.sel_camera = Room::Room6;
            self.camera_last_changed = SystemTime::now();
        }
        if *duct_heatup.get_mut() {
            self.gang.gogopher.duct_heat_timer = 250;
            self.gang.gogopher.duct_timer = 0;
        }
        Ok(())
    }
}
