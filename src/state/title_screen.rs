use std::{
    sync::atomic::{AtomicBool, AtomicUsize},
    time::{SystemTime, UNIX_EPOCH},
};

use crate::{config::config, style_pop, style_push, texture_rect};

use super::{Screen, State};
use ::imgui::StyleColor;
use ::imgui::{Condition, StyleVar};
use rand::Rng;
use raylib::prelude::*;
use std::sync::atomic::Ordering;

impl<'a> State<'a> {
    pub fn title_x() -> f32 {
        config().width() as f32 / 16.0
    }
    pub fn title_y() -> f32 {
        config().height() as f32 / 16.0
    }
    pub fn title_w() -> f32 {
        config().ui_scale() as f32 * 256.0
    }
    pub fn title_screen_draw(
        &mut self,
        d: &mut RaylibDrawHandle,
        thread: &RaylibThread,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let alpha = self.title_alpha() as u8;

        self.audio.play_title(self.has_won)?;
        let mut d = d.begin_texture_mode(&thread, &mut self.framebuffer);

        d.clear_background(Color::BLACK);

        let tux_texture_title = &*if !self.tux_texture_hold {
            let gen_range = rand::thread_rng().gen_range(0..1000);
            match gen_range {
                0 | 1 | 2 | 3 => {
                    self.tux_texture_hold = true;
                    match gen_range {
                        0 => self.textures.misc.title2(),
                        1 => self.textures.misc.title3(),
                        2 => self.textures.misc.title4(),
                        3 => self.textures.misc.title5(),
                        _ => self.textures.misc.title1(),
                    }
                }
                _ => self.textures.misc.title1(),
            }
        } else {
            if self.tux_texture_hold_frames < 3 {
                self.tux_texture_hold_frames += 1;
            } else {
                self.tux_texture_hold_frames = 0;
                self.tux_texture_hold = false;
            }
            self.textures.misc.title1()
        };

        d.draw_texture_pro(
            tux_texture_title,
            texture_rect!(tux_texture_title),
            Rectangle::new(0.0, 0.0, config().width() as f32, config().height() as f32),
            Vector2::new(0.0, 0.0),
            0.0,
            Color::new(255, 255, 255, alpha),
        );

        d.draw_text_ex(
            &self.font,
            "A Moderately Uncomfortable\nNight with Tux",
            Vector2::new(Self::title_x() as f32, Self::title_y() as f32),
            64.0,
            6.0,
            Color::new(255, 255, 255, alpha),
        );

        Ok(())
    }

    pub fn title_screen_menu(
        &mut self,
        d: &mut RaylibDrawHandle,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut goto_title = AtomicBool::new(false);
        let mut goto_settings = AtomicBool::new(false);

        let mut goto_credits = AtomicBool::new(false);

        let alpha = self.title_alpha() / 255.0;

        d.start_imgui(|ui| {
            ui.window("Menu")
                .position(
                    [
                        config().real_margin() + 20.0,
                        (config().real_height() as f32 / 2.0),
                    ],
                    Condition::Always,
                )
                .size([0.0, 0.0], Condition::Always)
                .movable(false)
                .resizable(false)
                .title_bar(false)
                .bg_alpha(alpha)
                .build(|| {
                    ui.set_window_font_scale(config().ui_scale());

                    let styles = style_push!(ui);
                    let alpha_style = ui.push_style_color(StyleColor::Text, [1.0, 1.0, 1.0, alpha]);

                    if ui.button("Start Game") {
                        goto_title.store(true, Ordering::Relaxed);
                    };
                    ui.separator();
                    if ui.button("Options") {
                        goto_settings.store(true, Ordering::Relaxed);
                    };
                    ui.separator();
                    if ui.button("Credits") {
                        goto_credits.store(true, Ordering::Relaxed);
                    };

                    style_pop!(styles);
                    alpha_style.pop();
                });
        });

        if *goto_title.get_mut() {
            self.reset_and_goto_title = true;
            return Ok(());
        }

        if *goto_settings.get_mut() {
            self.screen = Screen::Settings;
            return Ok(());
        }

        if *goto_credits.get_mut() {
            self.screen = Screen::Credits;
            return Ok(());
        }

        if self.going_to_office_from_title && self.title_clicked.elapsed()?.as_secs() >= 5 {
            self.audio.halt_title(self.has_won);
        }
        if self.going_to_office_from_title && self.title_clicked.elapsed()?.as_secs() >= 6 {
            // state = State::new();
            self.screen = Screen::Office;
            self.win_time = SystemTime::now();
            self.going_to_office_from_title = false;
            self.audio.brownian_noise.play_loop()?;
        }

        if self.going_to_office_from_title {
            d.set_mouse_position(Vector2::new(
                config().real_width_raw() as f32 / 2.0,
                config().real_height() as f32 / 2.0,
            ));
            d.hide_cursor();
        } else {
            d.show_cursor();
        }

        Ok(())
    }

    pub fn title_alpha(&self) -> f32 {
        if self.going_to_office_from_title {
            255.0 - (self.title_clicked.elapsed().unwrap().as_millis() as f32 / (5000.0 / 255.0))
        } else {
            255.0
        }
    }
}
