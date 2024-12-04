use raylib::prelude::*;

use crate::{state::CAMERA_TIME};
use crate::config::config;

use super::{Screen, State};

const TEXT_WIDTH: i32 = ("Laptop Rebooting".len() as i32) * 24;

impl<'a> State<'a> {
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
                &self.default_font,
                "Laptop Rebooting",
                Vector2::new(x + (TEXT_WIDTH / 8) as f32, y as f32 - 16.0),
                32.0,
                3.0,
                Color::WHITE,
            );
        } else {
            self.camera_booting = false;
            self.screen = Screen::Camera;
        }
        Ok(())
    }
}
