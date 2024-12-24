use std::sync::atomic::AtomicBool;

use crate::config::{config, config_mut};

use super::{Screen, State};
use ::imgui::Condition;
use parking_lot::Mutex;
use raylib::prelude::*;

impl State<'_> {
    pub fn settings_draw(
        &mut self,
        mut d: &mut RaylibDrawHandle,
        thread: &RaylibThread,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut back = AtomicBool::new(false);

        let texturefilter = Mutex::new(None);

        let filters = [
            ("Bilinear", TextureFilter::TEXTURE_FILTER_BILINEAR),
            ("Trilinear", TextureFilter::TEXTURE_FILTER_TRILINEAR),
            (
                "Anisotrpoic 4x",
                TextureFilter::TEXTURE_FILTER_ANISOTROPIC_4X,
            ),
            (
                "Anisotrpoic 8x",
                TextureFilter::TEXTURE_FILTER_ANISOTROPIC_8X,
            ),
            (
                "Anisotrpoic 16x",
                TextureFilter::TEXTURE_FILTER_ANISOTROPIC_16X,
            ),
            ("Point", TextureFilter::TEXTURE_FILTER_POINT),
        ];
        d.start_imgui(|ui| {
            ui.window("Settings")
                .position(
                    [
                        config().real_margin() + 10.0,
                        (config().real_height() as f32 / 2.0),
                    ],
                    Condition::Always,
                )
                .size([0.0, 0.0], Condition::Always)
                .movable(false)
                .resizable(false)
                .title_bar(false)
                .build(|| {
                    ui.set_window_font_scale(config().ui_scale());

                    let mut scale = config().ui_scale();
                    ui.slider("UI Scale", 0.1, 4.0, &mut scale);
                    config_mut().set_ui_scale(scale);

                    ui.menu("Texture Filter", || {
                        for filter in filters {
                            if ui
                                .menu_item_config(filter.0)
                                .selected(self.cur_texture_filter == filter.1)
                                .build()
                            {
                                *texturefilter.lock() = Some(filter.1)
                            };
                        }
                    });

                    if ui.button("<- Back") {
                        back.store(true, std::sync::atomic::Ordering::Relaxed);
                    }
                });
        });

        if let Some(a) = *texturefilter.lock() {
            self.textures.set_texture_filter(&mut d, &thread, a);
            self.cur_texture_filter = a;
        }
        if *back.get_mut() {
            self.screen = Screen::TitleScreen;
        }
        Ok(())
    }
}
