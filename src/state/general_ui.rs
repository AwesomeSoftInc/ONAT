use ::imgui::{DrawListMut, ImColor32};
use parking_lot::Mutex;
use raylib::prelude::*;

use crate::config;

use super::State;

impl<'a> State<'a> {
    pub fn general_ui_draw(
        &mut self,
        d: &mut RaylibDrawHandle,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let s = Mutex::new(self);
        d.start_imgui(|ui| {
            let se = s.lock();
            ui.window("ui")
                .resizable(false)
                .movable(false)
                .title_bar(false)
                .bg_alpha(0.0)
                .position([config().margin(), 0.0], ::imgui::Condition::Always)
                .size(
                    [config().real_width() as f32, config().real_height() as f32],
                    ::imgui::Condition::Always,
                )
                .build(|| {
                    ui.set_window_font_scale(4.0);

                    se.draw_battery(ui.get_window_draw_list()).unwrap();
                    se.draw_arrow(ui.get_window_draw_list()).unwrap();
                    se.draw_time(ui.get_window_draw_list()).unwrap();
                });
        });

        Ok(())
    }

    pub fn draw_battery(
        &self,
        draw_list: DrawListMut<'_>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        draw_list.add_text(
            [50.0, config().real_height() as f32 - 215.0],
            ImColor32::WHITE,
            "BATTERY",
        );

        let bat_start = 50.0;
        let bat_end = 300.0;
        let bat_width = (bat_end - bat_start) as i32;
        let bat_bar_width = (self.camera_timer * (bat_width as f32 / 100.0)) as i32;

        let bar_height = 100.0;
        for i in 0..bat_bar_width {
            let x = bat_start + i as f32;
            let mut off_y = (10.0 - i as f32).clamp(0.0, bar_height);
            if i >= bat_width - 10 {
                off_y += (i - (bat_width - 10)) as f32;
            }

            draw_list
                .add_line(
                    [x, config().real_height() as f32 - 150.0 + off_y],
                    [x, config().real_height() as f32 - 50.0 - off_y],
                    ImColor32::from_rgb(255 - i as u8, i as u8, 0),
                )
                .build();
        }

        draw_list
            .add_rect(
                [bat_start, config().real_height() as f32 - 150.0],
                [bat_end, config().real_height() as f32 - 50.0],
                ImColor32::WHITE,
            )
            .thickness(10.0)
            .rounding(25.0)
            .build();

        Ok(())
    }

    pub fn draw_arrow(&self, draw_list: DrawListMut<'_>) -> Result<(), Box<dyn std::error::Error>> {
        let center = config().real_width() as f32 / 2.0;
        let width = config().real_width() as f32 / 4.0;
        let bottom = config().real_height() as f32;
        draw_list
            .add_rect(
                [center - width, bottom - 100.0],
                [center + width, bottom],
                ImColor32::from_rgba(255, 255, 255, 128),
            )
            .rounding(25.0)
            .round_bot_left(false)
            .round_bot_right(false)
            .filled(true)
            .build();

        draw_list
            .add_polyline(
                vec![
                    [center - 50.0, bottom - 25.0],
                    [center, bottom - 75.0],
                    [center + 50.0, bottom - 25.0],
                ],
                ImColor32::from_rgba(0, 0, 0, 128),
            )
            .thickness(25.0)
            .build();

        Ok(())
    }

    pub fn draw_time(&self, draw_list: DrawListMut<'_>) -> Result<(), Box<dyn std::error::Error>> {
        let time = self.time()?;

        let time = format!("{}:00AM", time);
        let font_size = 64.0 * config().ratio();
        draw_list.add_text(
            [
                (config().real_width() as f32)
                    - self.font.measure_text(&time, font_size, 3.0).x
                    - 50.0,
                50.0,
            ],
            ImColor32::WHITE,
            time,
        );

        Ok(())
    }
}
