use std::time::{SystemTime, UNIX_EPOCH};

use crate::texture_rect;

use super::{Screen, State};
use rand::Rng;
use raylib::prelude::*;
use ::imgui::Condition;
use crate::config::config;

impl<'a> State<'a> {
    pub fn title_screen_draw(
        &mut self,
        d: &mut RaylibDrawHandle,
        thread: &RaylibThread,
        mx: i32,
        my: i32,
        tex: Texture2D,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut d = d.begin_texture_mode(&thread, &mut self.framebuffer);

        self.audio.play_title(self.has_won)?;
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

        let alpha = {
            if self.going_to_office_from_title {
                255.0 - (self.title_clicked.elapsed()?.as_millis() as f32 / (5000.0 / 255.0))
            } else {
                255.0
            }
        } as u8;

        d.draw_texture_pro(
            tux_texture_title,
            texture_rect!(tux_texture_title),
            Rectangle::new(config().margin(), 0.0, config().width() as f32, config().height() as f32),
            Vector2::new(0.0, 0.0),
            0.0,
            Color::new(255, 255, 255, alpha),
        );

        d.start_imgui(|ui| {
            ui.window("A Moderately Uncomfortable Night with Tux").position([5.0,5.0], Condition::FirstUseEver).size([config().width() as f32 / 4.0, config().height() as f32 / 4.0], Condition::Always).resizable(false).build(|| {
                ui.menu_item("Start game");
                ui.menu_item("Options");
                ui.menu_item("Credits");
            });
        });

        d.draw_text(
            "A Moderately\nUncomfortable\nNight\nwith Tux",
            5,
            5,
            64,
            Color::new(255, 255, 255, alpha),
        );
        d.draw_text(
            "Click anywhere to start",
            5,
            config().height() - 48,
            32,
            Color::new(255, 255, 255, alpha),
        );

        let cx = config().width_raw() - (config().width_raw() / 8);
        let cy = config().height() - 48;
        d.draw_text("Credits", cx, cy, 32, Color::new(255, 255, 255, alpha));
        if d.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT)
            && !self.going_to_office_from_title
        {
            if mx >= cx && my >= cy {
                self.screen = Screen::Credits;
            } else {
                self.going_to_office_from_title = true;
                if !d.is_key_down(KeyboardKey::KEY_LEFT_SHIFT) {
                    self.title_clicked = SystemTime::now();
                } else {
                    self.title_clicked = UNIX_EPOCH;
                }
            }
        }
        d.draw_texture_pro(
            &tex,
            texture_rect!(tex),
            Rectangle::new(0.0, 0.0, config().width_raw() as f32, config().height() as f32),
            Vector2::new(0.0, 0.0),
            0.0,
            Color::new(255, 255, 255, alpha / 8),
        );
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
}
