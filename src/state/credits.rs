use raylib::prelude::*;

use crate::config::config;

use super::{Screen, State};

impl<'a> State<'a> {
    pub fn credits_draw(
        &mut self,
        d: &mut RaylibDrawHandle,
        mx: i32,
        my: i32,
    ) -> Result<(), Box<dyn std::error::Error>> {
        self.audio.play_title(self.has_won)?;

        d.clear_background(Color::BLACK);
        d.draw_text_ex(
            &self.default_font,
            "Programming\nDirector/Art/Play Testing\nMusic\nArt/Animator\nWisdom
            ",
            Vector2::new(config().margin() + 48.0, 48.0),
            30.0,
            6.0,
            Color::WHITE,
        );
        d.draw_text_ex(
            &self.default_font,
            "Gavin \"ioi_xd\" Parker\nBigTuxFan223*\nNichael Brimbleton\nGiovanna \"mochi\" Poggi\nThe Eye
            ",
            Vector2::new(config().width_raw() as f32 / 2.0, 48.0),
            30.0,
            6.0,
            Color::WHITE,
        );

        d.draw_text(
            "*Uses Windows",
            config().margin() as i32 + 5,
            config().height() - 48,
            32,
            Color::new(255, 255, 255, 255),
        );
        let cx = config().width_raw() - (config().width_raw() / 4);
        let cy = config().height() - 48;
        d.draw_text("Back to Title Screen", cx, cy, 32, Color::WHITE);
        if d.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT) {
            if mx >= cx && my >= cy {
                self.screen = Screen::TitleScreen;
            }
        }
        Ok(())
    }
}
