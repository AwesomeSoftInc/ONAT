use raylib::prelude::*;

use crate::{audio::audio_init, config::config};

use super::{Screen, State};

impl<'a> State<'a> {
    pub fn win_draw(
        &mut self,
        d: &mut RaylibDrawHandle,
        thread: &RaylibThread,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let num = self.time()?;

        if !self.audio.bells.is_playing() && !config().night_2() {
            self.audio.bells.play()?;
        }

        let font_size = config().width() / 7;
        let x = (config().width() / 2)
            - (d.measure_text(format!("{}:00AM", num).as_str(), font_size) / 2);
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

        if self.win_time.elapsed()?.as_secs() >= {
            if config().night_2() {
                14
            } else {
                20
            }
        } {
            self.screen = Screen::Credits;
            self.going_to_office_from_title = false;
        }
        let mut d = d.begin_texture_mode(&thread, &mut self.you_win_framebuffer);
        d.clear_background(Color::BLACK);

        d.draw_text_ex(
            &self.font,
            format!("{}", num - 1).as_str(),
            Vector2::new(x as f32, y_),
            font_size as f32,
            6.0,
            Color::WHITE,
        );
        let text = format!("{}", num);
        let text_len = d.measure_text(text.as_str(), font_size);
        d.draw_text_ex(
            &self.font,
            text.as_str(),
            Vector2::new(x as f32, y_ + (font_size as f32 * 1.0)),
            font_size as f32,
            6.0,
            Color::WHITE,
        );

        d.draw_text_ex(
            &self.font,
            " :00AM",
            Vector2::new(x as f32 + text_len as f32, y as f32),
            font_size as f32,
            6.0,
            Color::WHITE,
        );
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

        Ok(())
    }
}
