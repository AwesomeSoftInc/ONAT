use raylib::prelude::*;

use crate::config::config;

use super::{Screen, State};

impl<'a> State<'a> {
    pub fn win_draw(
        &mut self,
        d: &mut RaylibDrawHandle,
        mx: i32,
        my: i32,
    ) -> Result<(), Box<dyn std::error::Error>> {
        self.audio.play_bells()?;
        d.clear_background(Color::BLACK);
        let fb_a = {
            if self.screen == Screen::YouWin {
                255.0 - (self.win_time.elapsed()?.as_secs_f32() * 128.0)
            } else {
                255.0
            }
        } as u8;

        let font_size = config().width() / 7;
        let x = config().width() / 2;
        let y = (config().height() / 2) - (font_size / 2);
        let y_ = {
            if self.win_time.elapsed()?.as_secs() < 1 {
                y as f32
            } else {
                let new = y as f32 - ((self.win_time.elapsed()?.as_millis() - 1000) as f32 / 25.0);
                if new <= (y - font_size) as f32 {
                    y as f32 - font_size as f32
                } else {
                    new
                }
            }
        };

        let num = self.time()?;

        d.draw_text_ex(
            &self.default_font,
            format!("{}", num - 1).as_str(),
            Vector2::new(x as f32 - (8.0 * 5.0), y_),
            font_size as f32,
            3.0,
            Color::WHITE,
        );
        d.draw_text_ex(
            &self.default_font,
            format!("{}", num).as_str(),
            Vector2::new(x as f32 - (8.0 * 5.0), y_ + (font_size as f32 * 1.0)),
            font_size as f32,
            3.0,
            Color::WHITE,
        );

        d.draw_text(" :00AM", x, y, font_size, Color::WHITE);
        d.draw_rectangle(
            0,
            (y - font_size) + 16,
            config().width_raw(),
            font_size,
            Color::BLACK,
        );
        d.draw_rectangle(
            0,
            (y + font_size) - 32,
            config().width_raw(),
            font_size,
            Color::BLACK,
        );
        d.draw_texture_pro(
            &self.framebuffer,
            Rectangle::new(
                self.framebuffer.width() as f32,
                0.0,
                -self.framebuffer.width() as f32,
                self.framebuffer.height() as f32,
            ),
            Rectangle::new(
                self.framebuffer.width() as f32 / 2.0,
                self.framebuffer.height() as f32 / 2.0,
                self.framebuffer.width() as f32,
                self.framebuffer.height() as f32,
            ),
            Vector2::new(
                self.framebuffer.width() as f32 / 2.0,
                self.framebuffer.height() as f32 / 2.0,
            ),
            180.0,
            Color::new(255, 255, 255, fb_a),
        );
        d.draw_rectangle(0, 0, config().margin() as i32, config().height() as i32, Color::BLACK);
        d.draw_rectangle(
            config().width() + config().margin() as i32 + 1,
            0,
            config().margin() as i32,
            config().height() as i32,
            Color::BLACK,
        );
        if self.win_time.elapsed()?.as_secs() >= 20 {
            self.screen = Screen::Credits;
            self.going_to_office_from_title = false;
        }
        Ok(())
    }
}
