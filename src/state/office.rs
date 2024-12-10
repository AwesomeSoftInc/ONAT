use std::time::{Duration, SystemTime};

use super::{Screen, State};
use crate::config::config;
use crate::{
    enums::Room,
    monster::{Monster, MonsterName, MONSTER_TIME_OFFICE_WAIT_THING},
    state::{CAMERA_TIME, DOOR_ANIM_SPEED},
    texture_rect,
    textures::Textures,
};

use num::Float;
use num_traits::float::FloatCore;
use parking_lot::{Mutex, MutexGuard};
use raylib::prelude::*;

impl<'a> State<'a> {
    pub fn office_draw(
        &mut self,
        d: &mut RaylibDrawHandle,
        thread: &RaylibThread,
        mx: i32,
        my: i32,
    ) -> Result<(), Box<dyn std::error::Error>> {
        self.wallpaper_draw(d, &thread);

        let mut d = d.begin_texture_mode(&thread, &mut self.framebuffer);
        d.clear_background(Color::BLACK);

        let cx = (0.0 - self.bg_offset_x) as i32 + ((config().width() / 3) as f32 * 1.6) as i32;
        let cy = (config().height() / 4) + (config().height() / 2);
        if mx >= cx && mx <= cx + 200 && my >= cy && my <= cy + 200 {
            d.set_mouse_cursor(MouseCursor::MOUSE_CURSOR_POINTING_HAND);
            if d.is_mouse_button_released(MouseButton::MOUSE_BUTTON_LEFT) {
                self.audio.play_plush()?;
            }
        }
        #[cfg(not(feature = "no_camera_timer"))]
        if self.camera_timer <= 100.0 {
            self.camera_timer += CAMERA_TIME;
        }
        if self.going_to_camera {
            if self.laptop_offset_y > 0.0 {
                self.laptop_offset_y -= self.var_name as f64;
            } else {
                self.screen = Screen::Camera;
                self.going_to_camera = false;
            }
        }

        if self.gang.golden_tux.active() {
            if self.gang.golden_tux.appeared.elapsed()?.as_secs() >= 5 {
                if self.jumpscarer == MonsterName::None {
                    self.gang.golden_tux.deactivate();
                    self.jumpscarer = MonsterName::GoldenTux;
                    self.getting_jumpscared = true;
                }
            }
        }
        macro_rules! a {
                                ($($val:tt).*) => {
                                    d.draw_texture_pro(
                                        &$($val).*,
                                        texture_rect!($($val).*),
                                        Rectangle::new(
                                            -self.bg_offset_x,
                                            0.0,
                                            config().width() as f32 * 1.6,
                                            config().height() as f32,
                                        ),
                                        Vector2::new(0.0, 0.0),
                                        0.0,
                                        Color::WHITE,
                                    );
                                };
                            }
        {
            {
                let mut d = d.begin_shader_mode(&mut self.wallpaper_shader);
                let tex = self.wallpaper_framebuffer.texture;
                d.draw_texture_pro(
                    &self.wallpaper_framebuffer,
                    Rectangle::new(tex.width as f32, 0.0, -tex.width as f32, tex.height as f32),
                    Rectangle::new(
                        790.0 - self.bg_offset_x,
                        720.0,
                        config().width() as f32 / 3.5,
                        config().height() as f32 / 3.5,
                    ),
                    Vector2::new(640.0, 240.0),
                    180.0,
                    Color::WHITE,
                );
            }
            let office_corners = &*self.textures.misc.office_corners();
            a!(office_corners);
            let door_left = &*self.textures.misc.door_left();
            d.draw_texture_pro(
                &door_left,
                texture_rect!(door_left),
                Rectangle::new(
                    0.0 + -self.bg_offset_x,
                    self.left_door_anim_timer,
                    config().width() as f32 * 1.6,
                    config().height() as f32,
                ),
                Vector2::new(0.0, 0.0),
                0.0,
                Color::WHITE,
            );
            let door_right = &*self.textures.misc.door_right();
            d.draw_texture_pro(
                &door_right,
                texture_rect!(door_right),
                Rectangle::new(
                    0.0 + -self.bg_offset_x,
                    self.right_door_anim_timer,
                    config().width() as f32 * 1.6,
                    config().height() as f32,
                ),
                Vector2::new(0.0, 0.0),
                0.0,
                Color::WHITE,
            );

            let office_part1 = &*self.textures.misc.office_part1();
            a!(office_part1);

            if self.gang.wilber.active() {
                let texture = &*match self.gang.wilber.stage {
                    0 => self.textures.wilberPoster.poster(),
                    1 => self.textures.wilberPoster.posterprogress1(),
                    2 => self.textures.wilberPoster.posterprogress2(),
                    _ => self.textures.wilberPoster.posterprogress3(),
                };
                let time = match self.gang.wilber.time_since_appeared {
                    Some(a) => {
                        let b = a.elapsed()?.as_millis() / 2;
                        if b >= 255 {
                            255
                        } else {
                            b as u8
                        }
                    }
                    None => 0,
                };
                d.draw_texture_pro(
                    texture,
                    texture_rect!(texture),
                    Rectangle::new(
                        0.0 + -self.bg_offset_x,
                        0.0,
                        config().width() as f32 * 1.6,
                        config().height() as f32,
                    ),
                    Vector2::new(0.0, 0.0),
                    0.0,
                    Color::new(255, 255, 255, time),
                );
            }
        }
        if !self.getting_jumpscared {
            for mons in self.gang.in_room(Room::Office) {
                mons.draw(self.textures, &mut d, 0.0 - self.bg_offset_x, 0.0, 1.6, 1.0);
            }
        }

        let office_part2 = &*self.textures.misc.office_part2();
        let button1 = &*self.textures.misc.button1();
        let button2 = &*self.textures.misc.button2();
        a!(office_part2);
        a!(button1);
        a!(button2);

        let door_light_left_on = &*self.textures.misc.door_light_left_on();
        let door_light_left_off = &*self.textures.misc.door_light_left_off();
        if !self.can_open_left_door {
            a!(door_light_left_on);
        } else {
            a!(door_light_left_off);
        }

        let door_light_right_on = &*self.textures.misc.door_light_right_on();
        let door_light_right_off = &*self.textures.misc.door_light_right_off();
        if !self.can_open_right_door {
            a!(door_light_right_on);
        } else {
            a!(door_light_right_off);
        }

        let mut i = 0;
        let mut hovering = false;
        // for button in &self.door_buttons {
        //     if mx as f32 >= (button.x - self.bg_offset_x)
        //         && mx as f32 <= (button.x - self.bg_offset_x) + button.width
        //         && my as f32 >= button.y
        //         && my as f32 <= button.y + button.height
        //     {
        //         hovering = true;
        //         d.set_mouse_cursor(MouseCursor::MOUSE_CURSOR_POINTING_HAND);
        //         if d.is_mouse_button_released(MouseButton::MOUSE_BUTTON_LEFT) {
        //             if i == 0 && !self.left_door_shut {
        //                 if self.can_open_left_door {
        //                     self.left_door_shut = true;
        //                     self.can_open_left_door = false;
        //                     self.left_door_last_shut = SystemTime::now();
        //                     if self.gang.tux.room() == Room::Room3 {
        //                         self.gang.tux.set_room(Room::Room1);
        //                         self.gang.tux.can_move = false;
        //                         self.gang.tux.set_entered_from_left(false);
        //                         self.gang.tux.goto_room_after_office();
        //                         self.open_left_door_back_up = true;
        //                         self.gang.tux.checked_camera = None;
        //                         self.gang.tux.moved_to_hallway_at = SystemTime::now();
        //                     }
        //                     self.audio.play_door_left()?;
        //                 } else {
        //                     self.audio.play_jammed()?;
        //                 }
        //             } else if i == 1 && !self.right_door_shut {
        //                 if self.can_open_right_door {
        //                     self.right_door_shut = true;
        //                     self.can_open_right_door = false;
        //                     self.right_door_last_shut = SystemTime::now();
        //                     if self.gang.tux.room() == Room::Room5 {
        //                         self.gang.tux.set_room(Room::Room1);
        //                         self.gang.tux.can_move = false;
        //                         self.gang.tux.set_entered_from_right(false);
        //                         self.gang.tux.goto_room_after_office();
        //                         self.open_right_door_back_up = true;
        //                         self.gang.tux.checked_camera = None;
        //                         self.gang.tux.moved_to_hallway_at = SystemTime::now();
        //                     }
        //                     self.audio.play_door_right()?;
        //                 } else {
        //                     self.audio.play_jammed()?;
        //                 }
        //             }
        //         }
        //     }

        //     i += 1;
        // }

        if !hovering {
            d.set_mouse_cursor(MouseCursor::MOUSE_CURSOR_DEFAULT);
        }

        // LEFT DOOR
        if self.left_door_shut {
            if self.left_door_anim_timer <= 0.0 {
                self.left_door_anim_timer += DOOR_ANIM_SPEED;
            }
        } else {
            if self.left_door_anim_timer >= -(config().height() as f32) {
                self.left_door_anim_timer -= DOOR_ANIM_SPEED;
            }
        }

        // RIGHT DOOR
        if self.right_door_shut {
            if self.right_door_anim_timer <= 0.0 {
                self.right_door_anim_timer += DOOR_ANIM_SPEED;
            }
        } else {
            if self.right_door_anim_timer >= -(config().height() as f32) {
                self.right_door_anim_timer -= DOOR_ANIM_SPEED;
            }
        }
        self.gang.wilber.rage_increment(&mut self.audio);

        if self.laptop_offset_y < config().height() as f64 {
            let laptop = &*self.textures.misc.laptop();
            d.draw_texture_pro(
                &laptop,
                texture_rect!(laptop),
                Rectangle::new(
                    0.0 + 0.0,
                    self.laptop_offset_y as f32,
                    config().width() as f32,
                    config().height() as f32,
                ),
                Vector2::new(0.0, 0.0),
                0.0,
                Color::WHITE,
            );
        }
        let inoffice = self.gang.in_room(Room::Office);

        for mons in inoffice {
            if mons.active() {
                let duration: &Duration = &mons.timer_until_office().elapsed()?;
                let mut door_open_check = false;

                let is_tux = (mons.id() == MonsterName::Tux || mons.id() == MonsterName::GoldenTux);
                if !is_tux
                    && duration.as_millis() >= (MONSTER_TIME_OFFICE_WAIT_THING as u128 * 1000) - 500
                {
                    if duration.as_nanos() <= MONSTER_TIME_OFFICE_WAIT_THING as u128 * 1000000000 {
                        if duration.as_nanos() & 256 == 256 && mons.id() != MonsterName::Tux {
                            d.draw_rectangle(
                                0.0 as i32,
                                0,
                                config().width(),
                                config().height(),
                                Color::BLACK,
                            );
                        }
                    }
                }
            }

            if mons.entered_from_left()
                || mons.entered_from_right()
                || mons.id() == MonsterName::GoGopher
            {
                if self.tainted >= 100.0 {
                    if self.jumpscarer == MonsterName::None {
                        self.going_to_office = true;
                        self.jumpscarer = mons.id();
                        self.gameover_time = SystemTime::now();
                        self.getting_jumpscared = true;
                    }
                }
            }
        }
        if self.getting_jumpscared {
            // sound
            match self.jumpscarer {
                MonsterName::Tux | MonsterName::GoldenTux => {
                    self.audio.play_tux_jumpscare()?;
                }
                _ => {
                    self.audio.play_regular_jumpscare()?;
                }
            }

            // animation
            self.bg_offset_x = 450.0;
            let mons = get_jumpscare(self.jumpscarer.clone(), self.textures);

            match self.jumpscarer {
                MonsterName::Penny
                | MonsterName::Tux
                | MonsterName::GoGopher
                | MonsterName::GoldenTux => {
                    let (width, height, x, y, framerate) = match self.jumpscarer {
                        MonsterName::Penny => {
                            let x_offset = {
                                if self.gameover_time.elapsed()?.as_millis() <= 150 {
                                    8.5 * (self.gameover_time.elapsed()?.as_millis() as f32)
                                } else {
                                    150.0 * 8.5
                                }
                            };
                            (
                                (config().width() as f32),
                                config().height() as f32 / 1.5,
                                -config().width() as f32 + x_offset + 0.0,
                                config().height() as f32 - (config().height() as f32 / 1.5),
                                30,
                            )
                        }
                        MonsterName::Tux => (
                            config().width() as f32 + (0.0 + 0.0),
                            config().height() as f32,
                            0.0,
                            0.0,
                            18,
                        ),
                        MonsterName::GoldenTux => (
                            config().width() as f32 + (0.0 + 0.0),
                            config().height() as f32,
                            0.0,
                            0.0,
                            18,
                        ),
                        MonsterName::GoGopher => {
                            let height = config().height() as f32 / 1.3;
                            let y_offset =
                                (height as f32 * (self.jumpscare_counter as f32 / 15.0)) / 750.0;
                            (
                                config().width() as f32 + (config().width() as f32 * y_offset),
                                height + (height * y_offset),
                                0.0 - (y_offset * 750.0),
                                (-height) + (height / 1.5),
                                15,
                            )
                        }
                        _ => todo!(),
                    };
                    if let Some(tex) = mons.get(self.jumpscare_counter / (60 / framerate)) {
                        let tex = &*&**&*tex;
                        d.draw_texture_pro(
                            tex,
                            texture_rect!(tex),
                            Rectangle::new(x, y, width, height),
                            Vector2::new(0.0, 0.0),
                            0.0,
                            Color::WHITE,
                        );
                    } else {
                        self.audio.brownian_halt();

                        if self.jumpscarer != MonsterName::GoldenTux {
                            self.screen = Screen::GameOver;
                        } else {
                            self.screen = Screen::TitleScreen;
                        }

                        self.gameover_time = SystemTime::now();
                    }
                    self.jumpscare_counter += 1;
                }
                MonsterName::Wilber => {
                    let (width, height, x, mut y, framerate) = (
                        config().width() as f32,
                        config().height() as f32,
                        0.0,
                        config().height() as f32 - (self.jumpscare_counter * 115) as f32,
                        8,
                    );
                    if y >= 0.0 {
                        let tex = &**&*mons.first().unwrap();
                        d.draw_texture_pro(
                            tex,
                            texture_rect!(tex),
                            Rectangle::new(x, y, width, height),
                            Vector2::new(0.0, 0.0),
                            0.0,
                            Color::WHITE,
                        );
                        self.jumpscare_counter += 1;
                    } else {
                        y = 0.0;
                        if let Some(tex) = mons.get((self.jumpscare_counter - 5) / (60 / framerate))
                        {
                            let tex = &**&*tex;
                            d.draw_texture_pro(
                                tex,
                                texture_rect!(tex),
                                Rectangle::new(x, y, width, height),
                                Vector2::new(0.0, 0.0),
                                0.0,
                                Color::WHITE,
                            );
                            self.jumpscare_counter += 1;
                        } else {
                            if self.gameover_time.elapsed()?.as_millis() <= 800 {
                                let tex = &**&*mons.last().unwrap();
                                d.draw_texture_pro(
                                    tex,
                                    texture_rect!(tex),
                                    Rectangle::new(x, y, width, height),
                                    Vector2::new(0.0, 0.0),
                                    0.0,
                                    Color::WHITE,
                                );
                            } else {
                                self.screen = Screen::GameOver;

                                self.gameover_time = SystemTime::now();
                            }
                        }
                    }
                }
                MonsterName::Beastie => {
                    let width = self.textures.beastie.slide().width;
                    let height = self.textures.beastie.slide().height;
                    let cutoff = self.gameover_time.elapsed()?.as_millis() <= 500;
                    let x_offset = {
                        let o = self.gameover_time.elapsed()?.as_millis() as f32 * 2.0;
                        let w = config().width() as f32 + 0.0;
                        if o <= w / 4.0 {
                            o
                        } else {
                            if cutoff {
                                w / 4.0
                            } else {
                                w / 2.0
                            }
                        }
                    };
                    let x = (config().width() as f32) - x_offset;
                    let y = config().height() as f32 - height as f32;
                    if cutoff {
                        let slide = &*self.textures.beastie.slide();
                        d.draw_texture_pro(
                            &slide,
                            texture_rect!(slide),
                            Rectangle::new(x, y, width as f32, height as f32),
                            Vector2::new(0.0, 0.0),
                            0.0,
                            Color::WHITE,
                        );
                    } else {
                        if let Some(tex) = mons.get(self.jumpscare_counter / (60 / 24)) {
                            let tex = &**&*tex;
                            d.draw_texture_pro(
                                tex,
                                texture_rect!(tex),
                                Rectangle::new(
                                    x - 0.0,
                                    y,
                                    tex.width as f32 * 2.5,
                                    tex.height as f32 * 2.5,
                                ),
                                Vector2::new(0.0, 0.0),
                                0.0,
                                Color::WHITE,
                            );
                        } else {
                            self.screen = Screen::GameOver;
                            self.gameover_time = SystemTime::now();
                        }
                        self.jumpscare_counter += 1;
                    }
                }
                _ => {
                    self.screen = Screen::GameOver;
                    self.gameover_time = SystemTime::now();
                }
            }
        }
        Ok(())
    }

    pub fn office_ui_draw(
        &mut self,
        d: &mut RaylibDrawHandle,
        thread: &RaylibThread,
        mx: i32,
        my: i32,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let ui_scale = config().ui_scale() - 2.0;

        let ok = (config().real_width() as f32 / 1024.0).ceil();
        let mut fuck = ui_scale as i32;
        if (fuck & 1) == 0 {
            fuck -= 1;
        }
        let what = ui_scale + (0.90 * fuck as f32);
        let offx = &self.bg_offset_x * (what);
        let s = Mutex::new(self);

        d.start_imgui(|ui| {
            ui.window("ui")
                .resizable(false)
                .movable(false)
                .title_bar(false)
                .bg_alpha(0.0)
                .position([config().real_margin(), 0.0], ::imgui::Condition::Always)
                .size(
                    [config().real_width() as f32, config().real_height() as f32],
                    ::imgui::Condition::Always,
                )
                .build(|| {
                    let se = s.lock();

                    ui.set_window_font_scale(config().ui_scale());

                    se.draw_battery(ui.get_window_draw_list()).unwrap();
                    se.draw_arrow(ui.get_window_draw_list()).unwrap();
                    se.draw_time(ui.get_window_draw_list()).unwrap();
                });
            let office_part1 = s.lock().textures.misc.office_part1().clone();

            ui.window("door_buttons")
                .position([-(offx / ui_scale), 0.0], ::imgui::Condition::Always)
                .size(
                    [
                        office_part1.width as f32 * what,
                        office_part1.height as f32 * what,
                    ],
                    ::imgui::Condition::Always,
                )
                .bg_alpha(0.0)
                .title_bar(false)
                .build(|| {
                    let mut se = s.lock();

                    ui.set_window_font_scale(ui_scale);

                    ui.set_cursor_pos([400.0 * ok, 325.0 * ok]);
                    if ui.button_with_size("LEFT", [150.0, 150.0]) {
                        if !se.left_door_shut {
                            if se.can_open_left_door {
                                se.left_door_shut = true;
                                se.can_open_left_door = false;
                                se.left_door_last_shut = SystemTime::now();
                                if se.gang.tux.room() == Room::Room3 {
                                    se.gang.tux.set_room(Room::Room1);
                                    se.gang.tux.can_move = false;
                                    se.gang.tux.set_entered_from_left(false);
                                    se.gang.tux.goto_room_after_office();
                                    se.open_left_door_back_up = true;
                                    se.gang.tux.checked_camera = None;
                                    se.gang.tux.moved_to_hallway_at = SystemTime::now();
                                }
                                se.audio.play_door_left().unwrap();
                            } else {
                                se.audio.play_jammed().unwrap();
                            }
                        }
                    }
                    ui.set_cursor_pos([1175.0 * ok, 325.0 * ok]);
                    if ui.button_with_size("RIGHT", [150.0, 150.0]) {
                        if !se.right_door_shut {
                            if se.can_open_right_door {
                                se.right_door_shut = true;
                                se.can_open_right_door = false;
                                se.right_door_last_shut = SystemTime::now();
                                if se.gang.tux.room() == Room::Room5 {
                                    se.gang.tux.set_room(Room::Room1);
                                    se.gang.tux.can_move = false;
                                    se.gang.tux.set_entered_from_right(false);
                                    se.gang.tux.goto_room_after_office();
                                    se.open_right_door_back_up = true;
                                    se.gang.tux.checked_camera = None;
                                    se.gang.tux.moved_to_hallway_at = SystemTime::now();
                                }
                                se.audio.play_door_right().unwrap();
                            } else {
                                se.audio.play_jammed().unwrap();
                            }
                        }
                    }
                });
        });
        Ok(())
    }
    pub fn wallpaper_draw(&mut self, d: &mut RaylibDrawHandle, thread: &RaylibThread) {
        let mut d = d.begin_texture_mode(&thread, &mut self.wallpaper_framebuffer);

        let wallpaper = &*self.textures.misc.wallpaper();

        let center = Vector2::new((wallpaper.width / 2) as f32, (wallpaper.height / 2) as f32);

        d.draw_texture(&wallpaper, 0, 0, Color::WHITE);

        d.draw_rectangle(
            246,
            250,
            ((self.tainted / 100.0) * 152.0) as i32,
            50,
            Color::GREEN,
        );
    }
}

/// (Wilber, Tux, Penny, Beastie, Gopher)
pub fn get_jumpscare(id: MonsterName, textures: &Textures) -> Vec<MutexGuard<Texture2D>> {
    match id {
        MonsterName::Penny => vec![
            textures.penny_jumpscare.frame1(),
            textures.penny_jumpscare.frame2(),
            textures.penny_jumpscare.frame3(),
            textures.penny_jumpscare.frame4(),
            textures.penny_jumpscare.frame5(),
            textures.penny_jumpscare.frame6(),
            textures.penny_jumpscare.frame7(),
            textures.penny_jumpscare.frame8(),
            textures.penny_jumpscare.frame9(),
            textures.penny_jumpscare.frame10(),
            textures.penny_jumpscare.frame11(),
            textures.penny_jumpscare.frame12(),
            textures.penny_jumpscare.frame13(),
            textures.penny_jumpscare.frame14(),
            textures.penny_jumpscare.frame15(),
            textures.penny_jumpscare.frame16(),
            textures.penny_jumpscare.frame17(),
            textures.penny_jumpscare.frame18(),
            textures.penny_jumpscare.frame19(),
        ],
        MonsterName::Beastie => vec![
            textures.beastie_jumpscare.frame1(),
            textures.beastie_jumpscare.frame2(),
            textures.beastie_jumpscare.frame3(),
            textures.beastie_jumpscare.frame4(),
            textures.beastie_jumpscare.frame5(),
            textures.beastie_jumpscare.frame6(),
            textures.beastie_jumpscare.frame7(),
            textures.beastie_jumpscare.frame8(),
            textures.beastie_jumpscare.frame9(),
            textures.beastie_jumpscare.frame10(),
            textures.beastie_jumpscare.frame11(),
            textures.beastie_jumpscare.frame12(),
            textures.beastie_jumpscare.frame13(),
            textures.beastie_jumpscare.frame14(),
        ],
        MonsterName::Wilber => vec![
            textures.wilber_jumpscare.frame1(),
            textures.wilber_jumpscare.frame2(),
            textures.wilber_jumpscare.frame3(),
            textures.wilber_jumpscare.frame4(),
        ],
        MonsterName::GoGopher => vec![
            textures.gopher_jumpscare.frame1(),
            textures.gopher_jumpscare.frame2(),
            textures.gopher_jumpscare.frame3(),
            textures.gopher_jumpscare.frame4(),
            textures.gopher_jumpscare.frame5(),
            textures.gopher_jumpscare.frame6(),
            textures.gopher_jumpscare.frame7(),
            textures.gopher_jumpscare.frame8(),
            textures.gopher_jumpscare.frame9(),
            textures.gopher_jumpscare.frame10(),
        ],
        MonsterName::Tux => vec![
            textures.tux_jumpscare_direct.frame1(),
            textures.tux_jumpscare_direct.frame2(),
            textures.tux_jumpscare_direct.frame3(),
            textures.tux_jumpscare_direct.frame4(),
            textures.tux_jumpscare_direct.frame5(),
            textures.tux_jumpscare_direct.frame6(),
            textures.tux_jumpscare_direct.frame7(),
            textures.tux_jumpscare_direct.frame8(),
            textures.tux_jumpscare_direct.frame9(),
            textures.tux_jumpscare_direct.frame10(),
            textures.tux_jumpscare_direct.frame11(),
            textures.tux_jumpscare_direct.frame12(),
        ],
        MonsterName::Nolok => vec![],
        MonsterName::GoldenTux => vec![
            textures.golden_tux_jumpscare_direct.frame1(),
            textures.golden_tux_jumpscare_direct.frame2(),
            textures.golden_tux_jumpscare_direct.frame3(),
            textures.golden_tux_jumpscare_direct.frame4(),
            textures.golden_tux_jumpscare_direct.frame5(),
            textures.golden_tux_jumpscare_direct.frame6(),
            textures.golden_tux_jumpscare_direct.frame7(),
            textures.golden_tux_jumpscare_direct.frame8(),
            textures.golden_tux_jumpscare_direct.frame9(),
            textures.golden_tux_jumpscare_direct.frame10(),
            textures.golden_tux_jumpscare_direct.frame11(),
            textures.golden_tux_jumpscare_direct.frame12(),
        ],
        MonsterName::None => vec![],
    }
}
