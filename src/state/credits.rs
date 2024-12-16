use raylib::prelude::*;

use crate::config::config;

use super::{Screen, State};

impl<'a> State<'a> {
    pub fn credits_draw(
        &mut self,
        d: &mut RaylibDrawHandle,
        thread: &RaylibThread,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut d = d.begin_texture_mode(&thread, &mut self.framebuffer);

        self.audio.play_title(self.has_won)?;

        d.clear_background(Color::BLACK);

        d.draw_text_ex(
            &self.font,
            "Programming\nDirector/Art/Play Testing\nMusic\nArt/Animator\nWisdom
            ",
            Vector2::new(0.0 + 48.0, 48.0),
            30.0,
            6.0,
            Color::WHITE,
        );
        d.draw_text_ex(
            &self.font,
            "Gavin \"ioi_xd\" Parker\nBigTuxFan223*\nNichael Brimbleton\nGiovanna \"mochi\" Poggi\nThe Eye
            ",
            Vector2::new(config().width_raw() as f32 / 2.0, 48.0),
            30.0,
            6.0,
            Color::WHITE,
        );

        d.draw_text_ex(
            &self.font,
            "*Uses Windows",
            Vector2::new(0.0 + 5.0, config().height() as f32 - 48.0),
            32.0,
            6.0,
            Color::new(255, 255, 255, 255),
        );
        let cx = config().width_raw() - (config().width_raw() / 4);
        let cy = config().height() - 48;
        d.draw_text_ex(
            &self.font,
            "Back to Title Screen",
            Vector2::new(cx as f32, cy as f32),
            32.0,
            6.0,
            Color::WHITE,
        );
        // if d.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT) {
        //     if mx >= cx && my >= cy {
        //         self.screen = Screen::TitleScreen;
        //     }
        // }
        Ok(())
    }
}
