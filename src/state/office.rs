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
use crate::{style_pop, style_push};

use parking_lot::{Mutex, MutexGuard};
use raylib::prelude::*;

macro_rules! office_texture_draw {
    ($self:ident, $d:ident, $($val:tt).*) => {
        $d.draw_texture_pro(
            &$($val).*,
            texture_rect!($($val).*),
            Rectangle::new(
                -$self.bg_offset_x,
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

impl<'a> State<'a> {
    pub fn office_step(&mut self) -> Result<(), Box<dyn std::error::Error>> {
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

        Ok(())
    }
    pub fn office_draw(
        &mut self,
        d: &mut RaylibDrawHandle,
        thread: &RaylibThread,
    ) -> Result<(), Box<dyn std::error::Error>> {
        self.wallpaper_draw(d, &thread);

        let mut d = d.begin_texture_mode(&thread, &mut self.framebuffer);
        d.clear_background(Color::BLACK);

        {
            {
                let mut d = d.begin_shader_mode(&mut self.wallpaper_shader);
                let tex = self.wallpaper_framebuffer.texture;
                d.draw_texture_pro(
                    &self.wallpaper_framebuffer,
                    Rectangle::new(tex.width as f32, 0.0, -tex.width as f32, tex.height as f32),
                    Rectangle::new(
                        1010.0 - self.bg_offset_x,
                        730.0,
                        config().width() as f32 / 4.0,
                        config().height() as f32 / 4.0,
                    ),
                    Vector2::new(320.0, 240.0),
                    180.0,
                    Color::WHITE,
                );
            }
            let office_corners = &*self.textures.office.corners();
            office_texture_draw!(self, d, office_corners);
            let door_left = &*self.textures.office.door_left();
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
            let door_right = &*self.textures.office.door_right();
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

            let office_part1 = &*self.textures.office.office_part1();
            office_texture_draw!(self, d, office_part1);

            if self.gang.wilber.active() {
                let texture = &*match self.gang.wilber.stage {
                    0 => self.textures.office.poster1(),
                    1 => self.textures.office.poster2(),
                    2 => self.textures.office.poster3(),
                    _ => self.textures.office.postergameover(),
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

        let office_part2 = &*self.textures.office.office_part2();
        let button1 = &*self.textures.office.button_left();
        let button2 = &*self.textures.office.button_right();
        office_texture_draw!(self, d, office_part2);
        office_texture_draw!(self, d, button1);
        office_texture_draw!(self, d, button2);

        let door_light_left_on = &*self.textures.office.door_light_left_on();
        let door_light_left_off = &*self.textures.office.door_light_left_off();
        if !self.can_open_left_door {
            office_texture_draw!(self, d, door_light_left_on);
        } else {
            office_texture_draw!(self, d, door_light_left_off);
        }

        let door_light_right_on = &*self.textures.office.door_light_right_on();
        let door_light_right_off = &*self.textures.office.door_light_right_off();
        if !self.can_open_right_door {
            office_texture_draw!(self, d, door_light_right_on);
        } else {
            office_texture_draw!(self, d, door_light_right_off);
        }

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

                let is_tux = mons.id() == MonsterName::Tux || mons.id() == MonsterName::GoldenTux;
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
                    self.audio.tux_jumpscare.play()?;
                }
                _ => {
                    self.audio.regular_jumpscare.play()?;
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
                        self.audio.brownian_noise.halt();

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
                    let width = self.textures.bsd.slide().width;
                    let height = self.textures.bsd.slide().height;
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
                        let slide = &*self.textures.bsd.slide();
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
    ) -> Result<(), Box<dyn std::error::Error>> {
        let s = Mutex::new(self);

        d.start_imgui(|ui| {
            ui.window("ui")
                .resizable(false)
                .movable(false)
                .title_bar(false)
                .bg_alpha(0.0)
                .position([0.0, 0.0], ::imgui::Condition::Always)
                .size(
                    [
                        config().real_width_raw() as f32 + config().real_margin(),
                        config().real_height() as f32,
                    ],
                    ::imgui::Condition::Always,
                )
                .build(|| {
                    let se = s.lock();

                    let styles = style_push!(ui);
                    ui.set_window_font_scale(config().ui_scale());

                    se.draw_battery(ui.get_window_draw_list()).unwrap();
                    se.draw_arrow(ui.get_window_draw_list()).unwrap();

                    ui.set_window_font_scale(config().ui_scale() * 2.0);

                    let time = format!("{}:00AM", se.time().unwrap());
                    let font_off = ui.calc_text_size(time.clone())[0];
                    se.draw_time(&time, font_off, ui.get_window_draw_list())
                        .unwrap();

                    style_pop!(styles);
                });
        });
        Ok(())
    }

    pub fn office_clickable(
        &mut self,
        d: &mut RaylibHandle,
        mx: i32,
        my: i32,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let button = self.plush_clickable;
        if mx as f32 >= (button.x - self.bg_offset_x)
            && mx as f32 <= (button.x - self.bg_offset_x) + button.width
            && my as f32 >= button.y
            && my as f32 <= button.y + button.height
        {
            self.mouse_pointer = true;
            if d.is_mouse_button_released(MouseButton::MOUSE_BUTTON_LEFT) {
                self.audio.plush.play()?;
            }
        }

        let mut i = 0;

        for button in &self.door_buttons {
            if mx as f32 >= (button.x - self.bg_offset_x)
                && mx as f32 <= (button.x - self.bg_offset_x) + button.width
                && my as f32 >= button.y
                && my as f32 <= button.y + button.height
            {
                self.mouse_pointer = true;

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
                            self.audio.door.play_panned(self.pan_left, self.pan_right)?;
                        } else {
                            self.audio.jammed.play()?;
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
                            self.audio.door.play_panned(self.pan_left, self.pan_right)?;
                        } else {
                            self.audio.jammed.play()?;
                        }
                    }
                }
            }

            i += 1;
        }

        self.arrow_click(d)?;

        Ok(())
    }

    pub fn wallpaper_draw(&mut self, d: &mut RaylibDrawHandle, thread: &RaylibThread) {
        let mut d = d.begin_texture_mode(&thread, &mut self.wallpaper_framebuffer);

        let wallpaper = &*self.textures.office.wallpaper();

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
            textures.penny.jumpscare1(),
            textures.penny.jumpscare2(),
            textures.penny.jumpscare3(),
            textures.penny.jumpscare4(),
            textures.penny.jumpscare5(),
            textures.penny.jumpscare6(),
            textures.penny.jumpscare7(),
            textures.penny.jumpscare8(),
            textures.penny.jumpscare9(),
            textures.penny.jumpscare10(),
            textures.penny.jumpscare11(),
            textures.penny.jumpscare12(),
            textures.penny.jumpscare13(),
            textures.penny.jumpscare14(),
            textures.penny.jumpscare15(),
            textures.penny.jumpscare16(),
            textures.penny.jumpscare17(),
            textures.penny.jumpscare18(),
            textures.penny.jumpscare19(),
        ],
        MonsterName::Beastie => vec![
            textures.bsd.jumpscare1(),
            textures.bsd.jumpscare2(),
            textures.bsd.jumpscare3(),
            textures.bsd.jumpscare4(),
            textures.bsd.jumpscare5(),
            textures.bsd.jumpscare6(),
            textures.bsd.jumpscare7(),
            textures.bsd.jumpscare8(),
            textures.bsd.jumpscare9(),
            textures.bsd.jumpscare10(),
            textures.bsd.jumpscare11(),
            textures.bsd.jumpscare12(),
            textures.bsd.jumpscare13(),
            textures.bsd.jumpscare14(),
        ],
        MonsterName::Wilber => vec![
            textures.wilbur.jumpscare1(),
            textures.wilbur.jumpscare2(),
            textures.wilbur.jumpscare3(),
            textures.wilbur.jumpscare4(),
        ],
        MonsterName::GoGopher => vec![
            textures.gopher.jumpscare1(),
            textures.gopher.jumpscare2(),
            textures.gopher.jumpscare3(),
            textures.gopher.jumpscare4(),
            textures.gopher.jumpscare5(),
            textures.gopher.jumpscare6(),
            textures.gopher.jumpscare7(),
            textures.gopher.jumpscare8(),
            textures.gopher.jumpscare9(),
            textures.gopher.jumpscare10(),
        ],
        MonsterName::Tux => vec![
            textures.tux.jumpscare1(),
            textures.tux.jumpscare2(),
            textures.tux.jumpscare3(),
            textures.tux.jumpscare4(),
            textures.tux.jumpscare5(),
            textures.tux.jumpscare6(),
            textures.tux.jumpscare7(),
            textures.tux.jumpscare8(),
            textures.tux.jumpscare9(),
            textures.tux.jumpscare10(),
            textures.tux.jumpscare11(),
            textures.tux.jumpscare12(),
        ],
        MonsterName::Nolok => vec![],
        MonsterName::GoldenTux => vec![
            textures.golden_tux.jumpscare1(),
            textures.golden_tux.jumpscare2(),
            textures.golden_tux.jumpscare3(),
            textures.golden_tux.jumpscare4(),
            textures.golden_tux.jumpscare5(),
            textures.golden_tux.jumpscare6(),
            textures.golden_tux.jumpscare7(),
            textures.golden_tux.jumpscare8(),
            textures.golden_tux.jumpscare9(),
            textures.golden_tux.jumpscare10(),
            textures.golden_tux.jumpscare11(),
            textures.golden_tux.jumpscare12(),
        ],
        MonsterName::None => vec![],
    }
}
