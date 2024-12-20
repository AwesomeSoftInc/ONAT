use raylib::prelude::*;

use crate::config::config;
use crate::{monster::MonsterName, texture_rect};

use super::{Screen, State};

impl<'a> State<'a> {
    pub fn gameover_draw(
        &mut self,
        d: &mut RaylibDrawHandle,
        thread: &RaylibThread,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut d = d.begin_texture_mode(&thread, &mut self.framebuffer);

        let gameover_time = self.gameover_time.elapsed()?;
        let alpha = {
            if gameover_time.as_secs() < 1 {
                255
            } else {
                if gameover_time.as_secs() <= 5 {
                    255 - ((gameover_time.as_millis() as i32 - 1000) / 20)
                } else {
                    0
                }
            }
        };

        let nolok_text = format!("TIP: Awakening Nolok\nfrom the\ndepths of\nunused content\nhell is not\nadvised. The\ngame will\ncrash in\n{} seconds.",15 - gameover_time.as_secs());
        let text = match self.jumpscarer {
            MonsterName::Penny => {
                "TIP: When Penny\nleaves CAM 3,\nclose the door\nimmediately to\navoid being\ntainted."
            }
            MonsterName::Beastie => {
                "TIP: When Beastie\nleaves CAM 5,\nclose the\ndoor immediately\nto avoid\nbeing tainted."
            }
            MonsterName::GoGopher => "TIP: Heat up\nthe air\nduct to\nreset the\ngopher's progress.",
            MonsterName::Wilber => "TIP: Check Wilbur\nextremely frequently\nto prevent\nhis attack.",
            MonsterName::Nolok => nolok_text.as_str(),
            MonsterName::GoldenTux => "",
            _ => "TIP: When Tux\nleaves his\ndomain, he\nwill immediately\nrush one\nof the\nhallways.",
        };
        let y = config().height() as f32 / 2.0;
        let damnyoudied = &*self.textures.misc.damnyoudied();
        d.draw_texture_pro(
            damnyoudied,
            texture_rect!(damnyoudied),
            Rectangle::new(0.0, 0.0, config().width() as f32, config().height() as f32),
            Vector2::new(0.0, 0.0),
            0.0,
            Color::WHITE,
        );
        d.draw_text_ex(
            &self.font,
            text,
            Vector2::new(0.0 + 48.0, y),
            50.0,
            6.0,
            Color::RED,
        );

        if gameover_time.as_secs() >= 15 {
            if self.jumpscarer == MonsterName::Nolok {
                #[allow(deref_nullptr)]
                let go_go_gadget_segfault: i32 = unsafe { *std::ptr::null_mut() };
                println!("{}", go_go_gadget_segfault);
            }
            self.screen = Screen::TitleScreen;
            self.going_to_office_from_title = false;
            self.audio.brownian_noise.halt();
        }
        Ok(())
    }
}
