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
        draw_list.add_text(
            [config().real_margin() + 50.0, 50.0],
            ImColor32::WHITE,
            self.bonzi_text(),
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

    pub fn bonzi_text(&self) -> &str {
        let time = self.bonzi_timer.elapsed().unwrap().as_secs_f32() - 10.0;
        if time <= 0.0 {
            ""
        } else if time <= 2.0 {
            "Well! Hello there!"
        } else if time <= 5.0 {
            "I don't believe we've been properly introduced."
        } else if time <= 6.0 {
            "I am Bonzi."
        } else if time <= 8.0 {
            "Tux wants you to play a little game!"
        } else if time <= 12.0 {
            "For the crime of using Windows,\nwe have locked you in this office."
        } else if time <= 16.0 {
            "Tux's followers will be coming\nshortly to deal with you."
        } else if time <= 20.0 {
            "When they come in, they will corrupt\nyour PC until they are able to attack."
        } else if time <= 22.0 {
            "You have doors you can shut on them,"
        } else if time <= 26.0 {
            "but they will open on their own\nand be jammed for a bit"
        } else if time <= 28.0 {
            "...that is, unless you are wise\nwith shutting them,"
        } else if time <= 30.0 {
            "as if they run into it"
        } else if time <= 33.0 {
            "they will unjam it before\nwalking back to their post."
        } else if time <= 35.0 {
            "You have 6 hours to fend them off."
        } else if time <= 37.0 {
            "A word of advice?"
        } else if time <= 39.0 {
            "Keep track of them on the\ncameras to shut the doors"
        } else if time <= 43.0 {
            "before they can start\ncorrupting anything."
        } else {
            "Have fun!"
        }
    }
}
