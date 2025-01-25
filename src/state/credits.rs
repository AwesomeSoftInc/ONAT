use std::sync::atomic::AtomicBool;

use ::imgui::{Condition, ImColor32};
use raylib::prelude::*;

use crate::{config::config, style_pop, style_push, DEBUG};

use super::{Screen, State};

impl<'a> State<'a> {
    pub fn credits_draw(
        &mut self,
        d: &mut RaylibDrawHandle,
        thread: &RaylibThread,
    ) -> Result<(), Box<dyn std::error::Error>> {
        self.audio.play_title(self.has_won)?;

        let credits = vec![
            ("Programming", "Gavin \"ioi_xd\" Parker"),
            ("Director/Art", "BigTuxFan223*"),
            ("Music", "Nichael Brimbleton"),
            ("Art/Animator", "Giovanna \"mochi\" Poggi"),
            ("Wisdom", "The Eye"),
        ];
        let mut to_title = AtomicBool::new(false);
        d.start_imgui(|ui| {
            ui.window("Credits")
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
                .focused(!DEBUG)
                .build(|| {
                    ui.set_window_font_scale(config().ui_scale());

                    if let Some(table) = ui.begin_table("Credits", 2) {
                        ui.table_next_row();
                        for credit in &credits {
                            ui.table_next_column();
                            ui.text(credit.0);
                            ui.table_next_column();
                            ui.text(credit.1);
                        }
                        table.end();
                    };

                    if ui.button("Back To Title") {
                        to_title.store(true, std::sync::atomic::Ordering::Relaxed);
                    }

                    let styles = style_push!(ui);
                    for _ in 0..15 {
                        ui.separator();
                    }
                    style_pop!(styles);

                    ui.text("*Uses Windows");
                });
        });
        if *to_title.get_mut() {
            self.screen = Screen::TitleScreen;
        }

        Ok(())
    }
}
