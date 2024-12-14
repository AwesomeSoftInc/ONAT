use std::sync::atomic::AtomicBool;

use crate::config::{config, config_mut};

use super::{Screen, State};
use ::imgui::Condition;
use parking_lot::Mutex;
use raylib::prelude::*;

impl State<'_> {
    pub fn settings_draw(
        &mut self,
        d: &mut RaylibDrawHandle,
        thread: &RaylibThread,
        mx: i32,
        my: i32,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut back = AtomicBool::new(false);
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

                    if ui.button("<- Back") {
                        back.store(true, std::sync::atomic::Ordering::Relaxed);
                    }
                });
        });

        if *back.get_mut() {
            self.screen = Screen::TitleScreen;
        }
        Ok(())
    }
}
