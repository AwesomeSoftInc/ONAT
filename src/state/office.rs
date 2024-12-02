use std::time::{Duration, SystemTime};

use super::{Screen, State};
use crate::{
    enums::Room,
    get_height, get_margin, get_ratio, get_width,
    monster::{Monster, MonsterName, MONSTER_TIME_OFFICE_WAIT_THING},
    state::{CAMERA_TIME, DOOR_ANIM_SPEED},
    texture_rect,
};
use raylib::prelude::*;

impl<'a> State<'a> {
    pub fn office_draw(
        &mut self,
        d: &mut RaylibDrawHandle,
        thread: &RaylibThread,
        mx: i32,
        my: i32,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut d = d.begin_texture_mode(&thread, &mut self.framebuffer);
        d.clear_background(Color::BLACK);

        let cx = (get_margin() - self.bg_offset_x) as i32 + ((get_width() / 3) as f32 * 1.6) as i32;
        let cy = (get_height() / 4) + (get_height() / 2);
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
                                            get_margin() + -self.bg_offset_x,
                                            0.0,
                                            get_width() as f32 * 1.6,
                                            get_height() as f32,
                                        ),
                                        Vector2::new(0.0, 0.0),
                                        0.0,
                                        Color::WHITE,
                                    );
                                };
                            }

        a!(self.textures.office_corners);
        d.draw_texture_pro(
            &self.textures.door_left,
            texture_rect!(self.textures.door_left),
            Rectangle::new(
                get_margin() + -self.bg_offset_x,
                self.left_door_anim_timer,
                get_width() as f32 * 1.6,
                get_height() as f32,
            ),
            Vector2::new(0.0, 0.0),
            0.0,
            Color::WHITE,
        );
        d.draw_texture_pro(
            &self.textures.door_right,
            texture_rect!(self.textures.door_right),
            Rectangle::new(
                get_margin() + -self.bg_offset_x,
                self.right_door_anim_timer,
                get_width() as f32 * 1.6,
                get_height() as f32,
            ),
            Vector2::new(0.0, 0.0),
            0.0,
            Color::WHITE,
        );
        let var_name = (1.0 + get_ratio()) as i32;

        d.draw_texture_pro(
            &self.textures.wallpaper,
            texture_rect!(self.textures.wallpaper),
            Rectangle::new(
                ((get_width() as f32 + get_margin() as f32) - get_width() as f32 / 3.5)
                    - self.bg_offset_x,
                get_height() as f32 / 1.65,
                get_width() as f32 / 3.5,
                get_height() as f32 / 3.5,
            ),
            Vector2::new(0.0, 0.0),
            0.0,
            Color::WHITE,
        );
        d.draw_rectangle(
            (((get_width() as f32 / 1.233) + get_margin()) - self.bg_offset_x) as i32 - 50,
            (get_height() as f32 / 1.20) as i32,
            200,
            32,
            Color::new(0, 128, 0, 255),
        );
        d.draw_rectangle(
            (((get_width() as f32 / 1.233) + get_margin()) - self.bg_offset_x) as i32
                - (50 - var_name),
            ((get_height() as f32 / 1.20) as i32) + var_name,
            (self.tainted as i32 - 4) * (get_ratio().ceil()) as i32,
            32 - (var_name * 2),
            Color::GREEN,
        );

        d.draw_texture_pro(
            &self.textures.tainted_logo,
            texture_rect!(self.textures.tainted_logo),
            Rectangle::new(
                ((get_width() as f32 / 1.233) + get_margin()) - self.bg_offset_x,
                get_height() as f32 / 1.25,
                (get_width() as f32 + get_margin()) / 16.0,
                get_height() as f32 / 46.0,
            ),
            Vector2::new(0.0, 0.0),
            0.0,
            Color::WHITE,
        );

        a!(self.textures.office_part1);

        if self.gang.wilber.active() {
            let texture = match self.gang.wilber.stage {
                0 => &self.textures.wilberPoster.poster,
                1 => &self.textures.wilberPoster.posterprogress1,
                2 => &self.textures.wilberPoster.posterprogress2,
                _ => &self.textures.wilberPoster.posterprogress3,
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
                &texture,
                texture_rect!(texture),
                Rectangle::new(
                    get_margin() + -self.bg_offset_x,
                    0.0,
                    get_width() as f32 * 1.6,
                    get_height() as f32,
                ),
                Vector2::new(0.0, 0.0),
                0.0,
                Color::new(255, 255, 255, time),
            );
        }
        if !self.getting_jumpscared {
            for mons in self.gang.in_room(Room::Office) {
                mons.draw(
                    &self.textures,
                    &mut d,
                    get_margin() - self.bg_offset_x,
                    0.0,
                    1.6,
                    1.0,
                );
            }
        }

        a!(self.textures.office_part2);
        a!(self.textures.button1);
        a!(self.textures.button2);
        if !self.can_open_left_door {
            a!(self.textures.door_light_left_on);
        } else {
            a!(self.textures.door_light_left_off);
        }

        if !self.can_open_right_door {
            a!(self.textures.door_light_right_on);
        } else {
            a!(self.textures.door_light_right_off);
        }

        let mut i = 0;
        let mut hovering = false;
        for button in &self.door_buttons {
            if mx as f32 >= (button.x - self.bg_offset_x)
                && mx as f32 <= (button.x - self.bg_offset_x) + button.width
                && my as f32 >= button.y
                && my as f32 <= button.y + button.height
            {
                hovering = true;
                d.set_mouse_cursor(MouseCursor::MOUSE_CURSOR_POINTING_HAND);
                if d.is_mouse_button_released(MouseButton::MOUSE_BUTTON_LEFT) {
                    if i == 0 && !self.left_door_shut {
                        if self.can_open_left_door {
                            self.left_door_shut = true;
                            self.can_open_left_door = false;
                            self.left_door_last_shut = SystemTime::now();
                            if self.gang.tux.room() == Room::Room3 {
                                self.gang.tux.set_room(Room::Room1);
                                self.gang.tux.can_move = false;
                                self.gang.tux.set_entered_from_left(false);
                                self.gang.tux.goto_room_after_office();
                                self.open_left_door_back_up = true;
                                self.gang.tux.checked_camera = None;
                                self.gang.tux.moved_to_hallway_at = SystemTime::now();
                            }
                            self.audio.play_door_left()?;
                        } else {
                            self.audio.play_jammed()?;
                        }
                    } else if i == 1 && !self.right_door_shut {
                        if self.can_open_right_door {
                            self.right_door_shut = true;
                            self.can_open_right_door = false;
                            self.right_door_last_shut = SystemTime::now();
                            if self.gang.tux.room() == Room::Room5 {
                                self.gang.tux.set_room(Room::Room1);
                                self.gang.tux.can_move = false;
                                self.gang.tux.set_entered_from_right(false);
                                self.gang.tux.goto_room_after_office();
                                self.open_right_door_back_up = true;
                                self.gang.tux.checked_camera = None;
                                self.gang.tux.moved_to_hallway_at = SystemTime::now();
                            }
                            self.audio.play_door_right()?;
                        } else {
                            self.audio.play_jammed()?;
                        }
                    }
                }
            }

            i += 1;
        }

        if !hovering {
            d.set_mouse_cursor(MouseCursor::MOUSE_CURSOR_DEFAULT);
        }

        // LEFT DOOR
        if self.left_door_shut {
            if self.left_door_anim_timer <= 0.0 {
                self.left_door_anim_timer += DOOR_ANIM_SPEED;
            }
        } else {
            if self.left_door_anim_timer >= -(get_height() as f32) {
                self.left_door_anim_timer -= DOOR_ANIM_SPEED;
            }
        }

        // RIGHT DOOR
        if self.right_door_shut {
            if self.right_door_anim_timer <= 0.0 {
                self.right_door_anim_timer += DOOR_ANIM_SPEED;
            }
        } else {
            if self.right_door_anim_timer >= -(get_height() as f32) {
                self.right_door_anim_timer -= DOOR_ANIM_SPEED;
            }
        }
        self.gang.wilber.rage_increment(&mut self.audio);

        if self.laptop_offset_y < get_height() as f64 {
            d.draw_texture_pro(
                &self.textures.laptop,
                texture_rect!(self.textures.laptop),
                Rectangle::new(
                    get_margin() + 0.0,
                    self.laptop_offset_y as f32,
                    get_width() as f32,
                    get_height() as f32,
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
                                get_margin() as i32,
                                0,
                                get_width(),
                                get_height(),
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
            match self.jumpscarer {
                MonsterName::Penny
                | MonsterName::Tux
                | MonsterName::GoGopher
                | MonsterName::GoldenTux => {
                    let (width, height, x, y, mons, framerate) = match self.jumpscarer {
                        MonsterName::Penny => {
                            let x_offset = {
                                if self.gameover_time.elapsed()?.as_millis() <= 150 {
                                    8.5 * (self.gameover_time.elapsed()?.as_millis() as f32)
                                } else {
                                    150.0 * 8.5
                                }
                            };
                            (
                                (get_width() as f32),
                                get_height() as f32 / 1.5,
                                -get_width() as f32 + x_offset + get_margin(),
                                get_height() as f32 - (get_height() as f32 / 1.5),
                                &self.penny,
                                30,
                            )
                        }
                        MonsterName::Tux => (
                            get_width() as f32 + (get_margin() + get_margin()),
                            get_height() as f32,
                            0.0,
                            0.0,
                            &self.tux,
                            18,
                        ),
                        MonsterName::GoldenTux => (
                            get_width() as f32 + (get_margin() + get_margin()),
                            get_height() as f32,
                            0.0,
                            0.0,
                            &self.golden_tux,
                            18,
                        ),
                        MonsterName::GoGopher => {
                            let height = get_height() as f32 / 1.3;
                            let y_offset =
                                (height as f32 * (self.jumpscare_counter as f32 / 15.0)) / 750.0;
                            (
                                get_width() as f32 + (get_width() as f32 * y_offset),
                                height + (height * y_offset),
                                get_margin() - (y_offset * 750.0),
                                (-height) + (height / 1.5),
                                &self.gopher,
                                15,
                            )
                        }
                        _ => todo!(),
                    };
                    if let Some(tex) = mons.get(self.jumpscare_counter / (60 / framerate)) {
                        d.draw_texture_pro(
                            &tex,
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
                        get_width() as f32,
                        get_height() as f32,
                        get_margin(),
                        get_height() as f32 - (self.jumpscare_counter * 115) as f32,
                        8,
                    );
                    if y >= 0.0 {
                        let tex = self.wilber.first().unwrap();
                        d.draw_texture_pro(
                            &tex,
                            texture_rect!(tex),
                            Rectangle::new(x, y, width, height),
                            Vector2::new(0.0, 0.0),
                            0.0,
                            Color::WHITE,
                        );
                        self.jumpscare_counter += 1;
                    } else {
                        y = 0.0;
                        if let Some(tex) = self
                            .wilber
                            .get((self.jumpscare_counter - 5) / (60 / framerate))
                        {
                            d.draw_texture_pro(
                                &tex,
                                texture_rect!(tex),
                                Rectangle::new(x, y, width, height),
                                Vector2::new(0.0, 0.0),
                                0.0,
                                Color::WHITE,
                            );
                            self.jumpscare_counter += 1;
                        } else {
                            if self.gameover_time.elapsed()?.as_millis() <= 800 {
                                let tex = self.wilber.last().unwrap();
                                d.draw_texture_pro(
                                    &tex,
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
                    let width = self.textures.beastie.slide.width;
                    let height = self.textures.beastie.slide.height;
                    let cutoff = self.gameover_time.elapsed()?.as_millis() <= 500;
                    let x_offset = {
                        let o = self.gameover_time.elapsed()?.as_millis() as f32 * 2.0;
                        let w = get_width() as f32 + get_margin();
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
                    let x = (get_width() as f32) - x_offset;
                    let y = get_height() as f32 - height as f32;
                    if cutoff {
                        d.draw_texture_pro(
                            &self.textures.beastie.slide,
                            texture_rect!(self.textures.beastie.slide),
                            Rectangle::new(x, y, width as f32, height as f32),
                            Vector2::new(0.0, 0.0),
                            0.0,
                            Color::WHITE,
                        );
                    } else {
                        if let Some(tex) = self.beastie.get(self.jumpscare_counter / (60 / 24)) {
                            d.draw_texture_pro(
                                &tex,
                                texture_rect!(tex),
                                Rectangle::new(
                                    x - get_margin(),
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
                _ => {}
            }
        }
        Ok(())
    }
}
