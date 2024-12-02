use raylib::prelude::*;

use crate::{get_height, get_margin, get_width, monster::MonsterName, texture_rect};

use super::{Screen, State};

impl<'a> State<'a> {
    pub fn gameover_draw(
        &mut self,
        d: &mut RaylibDrawHandle,
        _mx: i32,
        _my: i32,
        tex: Texture2D,
    ) -> Result<(), Box<dyn std::error::Error>> {
        d.clear_background(Color::BLACK);
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

        let nolok_text = format!("TIP: Awakening Nolok from the depths of unused content hell is not advised. The game will crash in {} seconds.",15 - gameover_time.as_secs());
        let text = match self.jumpscarer {
            MonsterName::Penny => {
                "TIP: When Penny leaves CAM 3, close the door immediately to avoid being tainted."
            }
            MonsterName::Beastie => {
                "TIP: When Beastie leaves CAM 5, close the door immediately to avoid being tainted."
            }
            MonsterName::GoGopher => "TIP: Heat up the air duct to reset the gopher's progress.",
            MonsterName::Wilber => "TIP: Check Wilbur extremely frequently to prevent his attack.",
            MonsterName::Nolok => nolok_text.as_str(),
            MonsterName::GoldenTux => "",
            _ => "TIP: When Tux leaves his domain, he will immediately rush one of the hallways.",
        };
        let y = get_height() as f32 / 2.0;
        d.draw_texture_pro(
            &self.textures.damnyoudied,
            texture_rect!(self.textures.damnyoudied),
            Rectangle::new(get_margin(), 0.0, get_width() as f32, get_height() as f32),
            Vector2::new(0.0, 0.0),
            0.0,
            Color::WHITE,
        );
        d.draw_text_ex(
            &self.default_font,
            text,
            Vector2::new(get_margin() + 48.0, y),
            50.0,
            3.0,
            Color::RED,
        );
        d.draw_texture_pro(
            &tex,
            texture_rect!(tex),
            Rectangle::new(get_margin(), 0.0, get_width() as f32, get_height() as f32),
            Vector2::new(0.0, 0.0),
            0.0,
            Color::new(255, 255, 255, alpha as u8),
        );

        if gameover_time.as_secs() >= 15 {
            if self.jumpscarer == MonsterName::Nolok {
                panic!("Segmentation fault");
            }
            self.screen = Screen::TitleScreen;
            self.going_to_office_from_title = false;
            self.audio.brownian_halt();
        }
        Ok(())
    }
}
