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
    fn title_x() -> f32 {
        config().width() as f32 / 16.0
    }
    fn title_y() -> f32 {
        config().height() as f32 / 16.0
    }
    fn title_w() -> f32 {
        config().ui_scale() as f32 * 256.0
    }
    pub fn title_screen_draw(
        &mut self,
        d: &mut RaylibDrawHandle,
        thread: &RaylibThread,
        mx: i32,
        my: i32,
        tex: Texture2D,
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
            "A Moderately\nUncomfortable\nNight\nwith Tux",
            Vector2::new(Self::title_x() as f32, Self::title_y() as f32),
            64.0,
            6.0,
            Color::new(255, 255, 255, alpha),
        );

        d.draw_texture_pro(
            &tex,
            texture_rect!(tex),
            Rectangle::new(
                0.0,
                0.0,
                config().width_raw() as f32,
                config().height() as f32,
            ),
            Vector2::new(0.0, 0.0),
            0.0,
            Color::new(255, 255, 255, alpha / 8),
        );

        Ok(())
    }

    pub fn title_screen_menu(
        &mut self,
        d: &mut RaylibDrawHandle,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut goto_title = AtomicBool::new(false);
        let mut goto_credits = AtomicBool::new(false);

        let alpha = self.title_alpha() / 255.0;

        d.start_imgui(|ui| {
            ui.window("Menu")
                .position(
                    [
                        config().real_margin() + (Self::title_x() / 2.0),
                        (config().real_height() as f32 / 2.0),
                    ],
                    Condition::Always,
                )
                .size([Self::title_w(), 0.0], Condition::Always)
                .movable(false)
                .resizable(false)
                .title_bar(false)
                .bg_alpha(alpha)
                .build(|| {
                    ui.set_window_font_scale(config().ui_scale());

                    let styles = style_push!(ui);
                    let alpha_style = ui.push_style_color(StyleColor::Text, [1.0, 1.0, 1.0, alpha]);
                    let bsize = ui.push_style_var(StyleVar::FrameBorderSize(0.0));

                    if ui.button_with_size("Start Game", [Self::title_w() - 15.0, 100.0]) {
                        goto_title.store(true, Ordering::Relaxed);
                    };
                    ui.separator();
                    ui.button_with_size("Options", [Self::title_w() - 15.0, 100.0]);
                    ui.separator();
                    if ui.button_with_size("Credits", [Self::title_w() - 15.0, 100.0]) {
                        goto_credits.store(true, Ordering::Relaxed);
                    };

                    style_pop!(styles);
                    bsize.pop();
                    alpha_style.pop();
                });
        });

        if *goto_title.get_mut() {
            self.reset_and_goto_title = true;
            return Ok(());
        }

        if *goto_credits.get_mut() {
            self.screen = Screen::Credits;
        }

        if self.going_to_office_from_title && self.title_clicked.elapsed()?.as_secs() >= 5 {
            self.audio.halt();
        }
        if self.going_to_office_from_title && self.title_clicked.elapsed()?.as_secs() >= 6 {
            // state = State::new();
            self.screen = Screen::Office;
            self.win_time = SystemTime::now();
            self.going_to_office_from_title = false;
            self.audio.play_brownian_noise()?;
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
