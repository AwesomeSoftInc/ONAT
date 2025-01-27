use ::imgui::{DrawListMut, ImColor32};

use crate::config;

use super::State;

impl<'a> State<'a> {
    pub fn draw_battery(
        &self,
        draw_list: DrawListMut<'_>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        // Battery size
        let bat_width = Self::bat_width();
        let bat_height = Self::bat_height();
        let bat_start = Self::bat_start();
        let bat_end = bat_start + bat_width as f32;
        let bat_y = config().real_height() as f32 - bat_height;

        // Inner bar size
        let bar_width = (self.camera_timer * (bat_width as f32 / 100.0)) as i32;
        let bar_height = 100.0;

        draw_list.add_text(
            [bat_start, bat_y - (20.0 * config().ui_scale()) - bat_height],
            ImColor32::WHITE,
            "BATTERY",
        );

        let roundness = config().ui_scale() * 2.50;

        for i in 0..bar_width {
            let x = bat_start + i as f32;
            let mut off_y = (roundness - i as f32).clamp(0.0, bar_height);
            if i >= bat_width - 10 {
                off_y += (i - (bat_width - 10)) as f32;
            }

            draw_list
                .add_line(
                    [x, (bat_y + off_y) - bat_height],
                    [x, (bat_y - off_y)],
                    ImColor32::from_rgb(255 - i as u8, i as u8, 0),
                )
                .build();
        }

        draw_list
            .add_rect(
                [bat_start, bat_y - bat_height],
                [bat_end, bat_y],
                ImColor32::WHITE,
            )
            .thickness(config().ui_scale() * 2.0)
            .rounding(roundness)
            .build();

        Ok(())
    }

    pub fn draw_arrow(&self, draw_list: DrawListMut<'_>) -> Result<(), Box<dyn std::error::Error>> {
        let center = config().real_width_raw() as f32 / 2.0;
        let width = config().real_width() as f32 / 4.0;
        let bottom = config().real_height() as f32;

        let height = Self::bat_height() + 50.0;

        draw_list
            .add_rect(
                [center - width, bottom - height],
                [center + width, bottom],
                ImColor32::from_rgba(255, 255, 255, 128),
            )
            .rounding(config().ui_scale() * 4.50)
            .round_bot_left(false)
            .round_bot_right(false)
            .filled(true)
            .build();

        draw_list
            .add_polyline(
                vec![
                    [center - 50.0, bottom - (height * 0.25)],
                    [center, bottom - (height * 0.60)],
                    [center + 50.0, bottom - (height * 0.25)],
                ],
                ImColor32::from_rgba(0, 0, 0, 128),
            )
            .thickness(25.0)
            .build();

        Ok(())
    }

    pub fn draw_time(
        &self,
        time: &str,
        font_off: f32,
        draw_list: DrawListMut<'_>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        draw_list.add_text(
            [
                config().real_width_raw() as f32 - config().real_margin() - font_off - 50.0,
                50.0,
            ],
            ImColor32::WHITE,
            time,
        );

        Ok(())
    }

    pub fn draw_bonzi_text(
        &self,
        draw_list: DrawListMut<'_>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let space_every = (config().ui_scale() as i32) * 3;
        let mut i = 1;
        let lines = self
            .bonzi_line
            .split(" ")
            .map(|f| {
                let ret = if i % space_every == 0 {
                    f.to_string() + "\n"
                } else {
                    f.to_string() + " "
                };

                i += 1;
                ret
            })
            .collect::<Vec<_>>()
            .join("");
        draw_list.add_text(
            [config().real_margin() + 50.0, 50.0],
            ImColor32::WHITE,
            lines.as_str(),
        );

        Ok(())
    }

    pub fn draw_rage(&self, draw_list: DrawListMut<'_>) -> Result<(), Box<dyn std::error::Error>> {
        // Battery size
        let bat_width = Self::bat_width();
        let bat_height = Self::bat_height();
        let bat_start = Self::bat_start();
        let bat_end = bat_start + bat_width as f32;
        let bat_y = config().real_height() as f32 - bat_height - (75.0 * config().ui_scale());

        // Inner bar size
        let bar_width = (self.gang.wilber.rage() * (bat_width as f32 / 100.0)) as i32;
        let bar_height = 100.0;

        draw_list.add_text(
            [bat_start, bat_y - (20.0 * config().ui_scale()) - bat_height],
            ImColor32::WHITE,
            "RAGE",
        );

        for i in 0..bar_width {
            let x = bat_start + i as f32;
            let mut off_y = (10.0 - i as f32).clamp(0.0, bar_height);
            if i >= bat_width - 10 {
                off_y += (i - (bat_width - 10)) as f32;
            }

            draw_list
                .add_line(
                    [x, (bat_y + off_y) - bat_height],
                    [x, (bat_y - off_y)],
                    ImColor32::from_rgb(255 - i as u8, 0, 0),
                )
                .build();
        }

        draw_list
            .add_rect(
                [bat_start, bat_y - bat_height],
                [bat_end, bat_y],
                ImColor32::WHITE,
            )
            .thickness(config().ui_scale() * 2.0)
            .rounding(config().ui_scale() * 2.50)
            .build();

        Ok(())
    }

    pub fn bat_width() -> i32 {
        128
    }

    pub fn bat_height() -> f32 {
        50.0
    }

    pub fn bat_start() -> f32 {
        config().real_margin() + 50.0
    }
}
