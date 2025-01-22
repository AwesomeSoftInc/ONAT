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
        let mut fullscreen = AtomicBool::new(config().fullscreen());
        let mut changed_fullscreen = AtomicBool::new(false);

        let texturefilter = Mutex::new(None);
        let volumechange = Mutex::new(None);

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
                    if ui.slider("UI Scale", 0.1, 4.0, &mut scale) {
                        config_mut().set_ui_scale(scale);
                    };

                    let mut volume = config().volume() as f32;
                    if ui.slider("Volume", 0.0, 128.0, &mut volume) {
                        config_mut().set_volume(volume as i32);
                        *volumechange.lock() = Some(volume);
                    };

                    let mut _fullscreen = config().fullscreen();
                    if ui.checkbox("Fullscreen", &mut _fullscreen) {
                        changed_fullscreen.store(true, std::sync::atomic::Ordering::Relaxed);
                    }
                    fullscreen.store(_fullscreen, std::sync::atomic::Ordering::Relaxed);

                    ui.menu("Texture Filter", || {
                        for filter in filters {
                            if ui
                                .menu_item_config(filter.0)
                                .selected(config().texture_filter() == filter.1)
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
            config_mut().set_texture_filter(&mut self.textures, &mut d, &thread, a);
        }
        if let Some(a) = *volumechange.lock() {
            self.audio.set_volume(a as i32);
        }
        if *back.get_mut() {
            self.screen = Screen::TitleScreen;
        }
        if *changed_fullscreen.get_mut() {
            config_mut().set_fullscreen(&mut d, *fullscreen.get_mut());
        }
        Ok(())
    }
}
