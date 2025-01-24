mod camera;
mod credits;
mod debug;
mod game_over;
mod general_ui;
mod office;
mod settings;
mod title_screen;
mod you_win;

use ::core::f32;
use std::alloc::System;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use rand::{thread_rng, Rng};
use raylib::prelude::*;
use sdl2::mixer::AUDIO_F32;

pub const CAMERA_TIME: f32 = 0.1;
pub const DOOR_ANIM_SPEED: f32 = 100.0;

use crate::audio::audio_init;
use crate::config::{config, config_mut};
use crate::{
    audio::Audio,
    enums::Room,
    monster::{Gang, Monster, MonsterName, MONSTER_TIME_OFFICE_WAIT_THING},
    textures::Textures,
};

#[derive(PartialEq, Debug)]
pub enum Screen {
    TitleScreen,
    Credits,
    Office,
    CameraRebooting,
    Camera,
    GameOver,
    YouWin,
    Settings,
}

impl Screen {
    pub fn is_passive(&self) -> bool {
        return *self == Screen::TitleScreen
            || *self == Screen::YouWin
            || *self == Screen::GameOver
            || *self == Screen::Credits
            || *self == Screen::Settings;
    }
}

pub struct State<'a> {
    pub audio: &'a mut Audio,
    pub screen: Screen,
    pub last_screen: Screen,
    pub bg_offset_x: f32,
    pub laptop_offset_y: f64,
    pub sel_camera: Room,
    pub timer: SystemTime,

    pub ingame_time: SystemTime,
    pub gang: Gang,
    pub tainted: f32,

    pub camera_timer: f32,
    pub camera_booting: bool,
    pub camera_booting_timer: f32,

    pub gameover_time: SystemTime,
    pub win_time: SystemTime,

    pub camera_last_changed: SystemTime,

    pub can_open_left_door: bool,
    pub can_open_right_door: bool,

    pub left_door_anim_timer: f32,
    pub right_door_anim_timer: f32,

    pub left_door_shut: bool,
    pub right_door_shut: bool,

    pub left_door_bypass_cooldown: bool,
    pub right_door_bypass_cooldown: bool,
    pub left_door_last_shut: SystemTime,
    pub right_door_last_shut: SystemTime,

    pub skinman_chance: u32,
    pub skinman_appeared: bool,

    pub going_to_office: bool,
    pub going_to_camera: bool,
    pub going_to_office_from_title: bool,
    pub title_clicked: SystemTime,
    pub title_fade_skip: bool,

    pub jumpscare_counter: usize,
    pub getting_jumpscared: bool,
    pub jumpscarer: MonsterName,
    pub has_won: bool,

    pub textures: &'a mut Textures,
    pub scroll_amount: f32,
    pub var_name: f64,
    pub framebuffer: RenderTexture2D,

    pub wallpaper_shader: Shader,
    pub wallpaper_framebuffer: RenderTexture2D,

    pub you_win_framebuffer: RenderTexture2D,

    pub laptop_shader: Shader,

    pub tux_texture_hold: bool,
    pub tux_texture_hold_frames: i32,
    pub open_left_door_back_up: bool,
    pub open_right_door_back_up: bool,

    pub camera: Camera2D,

    pub font: Font,

    pub reset_and_goto_title: bool,

    pub plush_clickable: Rectangle,
    pub door_buttons: Vec<Rectangle>,

    pub mouse_pointer: bool,

    pub pan_left: u8,
    pub pan_right: u8,

    pub wilbur_snd_played: bool,
    pub tux_snd_played: bool,
    pub gopher_snd_played: bool,

    pub camera_changing: bool,

    pub force_win: bool,

    pub bonzi_timer: SystemTime,
    pub bonzi_play_timer: SystemTime,
    pub bonzi_swoosh_played: bool,
    pub bonzi_thud_played: bool,
    pub bonzi_played: bool,
    pub bonzi_wait_for: f32,
    pub bonzi_idx: usize,
    pub bonzi_line: String,

    pub gogopher_stinger_played: bool,
}

impl<'a> State<'a> {
    /**
    Instantiate the global state.
    */
    pub fn new(
        rl: &mut RaylibHandle,
        thread: &RaylibThread,
        audio: &'a mut Audio,
        textures: &'a mut Textures,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let screen = Screen::TitleScreen;
        let bg_offset_x = config().width() as f32 / 2.0;
        let laptop_offset_y = config().height() as f64;

        let sel_camera = Room::Room1;
        let timer = SystemTime::now();

        let camera_last_changed = SystemTime::now();

        let ingame_time = UNIX_EPOCH;
        let gang = Gang::new();

        let tainted = 0.0;

        let camera_timer = 100.0;
        let camera_booting = false;
        let camera_booting_timer = 0.0;

        let gameover_time = SystemTime::now();
        let win_time = SystemTime::now();

        let can_open_left_door = true;
        let can_open_right_door = true;

        let left_door_shut = false;
        let right_door_shut = false;

        let left_door_last_shut: SystemTime = UNIX_EPOCH;
        let right_door_last_shut: SystemTime = UNIX_EPOCH;

        let skinman_chance = 1000;
        let skinman_appeared = false;

        let scroll_amount = config().width().clone() as f32 * 0.01;

        let var_name = config().height() as f64 / 4.0;

        let framebuffer =
            rl.load_render_texture(&thread, config().width() as u32, config().height() as u32)?;
        let wallpaper_framebuffer = rl.load_render_texture(&thread, 640, 480)?;
        let you_win_framebuffer =
            rl.load_render_texture(&thread, config().width() as u32, config().height() as u32)?;
        let wallpaper_shader =
            rl.load_shader_from_memory(&thread, None, Some(include_str!("../shader/crt.fs")));

        let laptop_shader =
            rl.load_shader_from_memory(&thread, None, Some(include_str!("../shader/laptop.fs")));

        let tux_texture_hold = false;
        let tux_texture_hold_frames = 0;

        let open_left_door_back_up = false;
        let open_right_door_back_up = false;

        let camera = Camera2D {
            offset: Vector2::zero(),
            target: Vector2::new(1.0, 1.0),
            rotation: 0.0,
            zoom: 1.0,
        };

        let plush_clickable = Rectangle::new(747.0, 782.0, 250.0, 250.0);

        let door_buttons = vec![
            Rectangle::new(547.0, 502.0, 150.0, 150.0),
            Rectangle::new(1597.0, 502.0, 150.0, 150.0),
        ];

        let font = unsafe { Font::from_raw(rl.get_font_default().to_raw()) };
        let state = Self {
            audio,

            screen,
            last_screen: Screen::Office,
            bg_offset_x,
            laptop_offset_y,

            sel_camera,
            timer,
            ingame_time,
            gang,
            tainted,
            camera_timer,
            camera_booting,
            camera_booting_timer,
            gameover_time,
            win_time,
            camera_last_changed,
            can_open_left_door,
            can_open_right_door,
            left_door_shut,
            right_door_shut,
            left_door_bypass_cooldown: false,
            right_door_bypass_cooldown: false,
            left_door_last_shut,
            right_door_last_shut,
            left_door_anim_timer: -(config().height() as f32 * 0.09),
            right_door_anim_timer: -(config().height() as f32 * 0.09),
            skinman_chance,
            skinman_appeared,
            going_to_camera: false,
            going_to_office: false,
            going_to_office_from_title: false,
            title_clicked: SystemTime::now(),
            title_fade_skip: false,
            jumpscare_counter: 0,
            getting_jumpscared: false,
            jumpscarer: MonsterName::None,
            has_won: false,
            textures,
            scroll_amount,
            var_name,
            framebuffer,
            wallpaper_shader,
            laptop_shader,
            wallpaper_framebuffer,
            tux_texture_hold,
            tux_texture_hold_frames,
            open_left_door_back_up,
            open_right_door_back_up,
            camera: camera,
            font: font,
            reset_and_goto_title: false,
            plush_clickable,
            door_buttons,
            mouse_pointer: false,
            pan_left: 0,
            pan_right: 0,
            tux_snd_played: false,
            gopher_snd_played: false,
            wilbur_snd_played: false,
            camera_changing: false,
            force_win: false,
            you_win_framebuffer,
            bonzi_timer: SystemTime::now(),
            bonzi_play_timer: SystemTime::now(),
            bonzi_swoosh_played: false,
            bonzi_thud_played: false,
            bonzi_played: false,
            bonzi_wait_for: 1.0,
            bonzi_idx: 0,
            bonzi_line: String::new(),
            gogopher_stinger_played: false,
        };
        Ok(state)
    }

    /**
    Update the game's "timer state"; monster positions, the in game timer, etc.

    It is IMPERITIVE that anything that should be in `State::step` is in the right function, as the `State::draw_step` is called at an uncapped rate as opposed to 60 times a second. Vice versa applies.
    */
    pub fn step(
        &mut self,
        rl: &mut RaylibHandle,
        thread: &RaylibThread,
        mx: i32,
        my: i32,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let bonzi_secs = self.bonzi_timer.elapsed().unwrap().as_secs_f32() - 5.0;
        let on_tutorial = config().on_tutorial();

        if on_tutorial {
            if bonzi_secs >= 4.0 {
                if !self.bonzi_swoosh_played {
                    self.audio.swoosh.play()?;
                    self.bonzi_swoosh_played = true;
                }
            }
            if bonzi_secs >= 4.5 {
                if !self.bonzi_thud_played {
                    self.audio.thud.play()?;
                    self.bonzi_thud_played = true;
                }
            }
            if bonzi_secs >= 5.0 {
                let elapsed = self.bonzi_play_timer.elapsed()?.as_secs_f32();
                if let Some(snd) = self.audio.tts.get_mut(self.bonzi_idx) {
                    if elapsed >= self.bonzi_wait_for {
                        snd.2.play()?;
                        self.bonzi_wait_for = snd.1 as f32 / 176400.0;
                        self.bonzi_play_timer = SystemTime::now();
                        self.bonzi_idx += 1;
                        self.bonzi_line = snd.0.clone();
                    }
                }
                if self.bonzi_idx == self.audio.tts.len()
                    && self.bonzi_play_timer.elapsed()?.as_secs() >= 3
                {
                    // Consider us finished.
                    config_mut().set_on_tutorial(false);
                    self.ingame_time = UNIX_EPOCH;
                }
            }
        }

        if self.camera_booting {
            self.camera_booting_timer += 0.01;
            if self.camera_booting_timer >= 250.0 {
                self.camera_booting = false;
                self.camera_booting_timer = 0.0;
            }
        }

        let sc = (self.scroll_amount + (mx - config().width_raw() / 2) as f32) / 24.0;
        if mx <= (config().width_raw() / 2) {
            if self.bg_offset_x > 0.0 {
                self.bg_offset_x += sc;
            }
        }
        if mx >= config().width_raw() - (config().width_raw() / 2) {
            if self.bg_offset_x < (config().width() as f32) / 1.75 {
                self.bg_offset_x += sc;
            }
        }

        let cur_time = self.ingame_time.duration_since(UNIX_EPOCH)?;

        match self.screen {
            Screen::Office => self.office_step()?,
            Screen::Camera => self.camera_step(),
            Screen::CameraRebooting => self.camera_rebooting_step(),
            _ => {}
        }

        if self.left_door_last_shut.elapsed()?.as_secs() >= 5 {
            if !self.left_door_bypass_cooldown {
                self.can_open_left_door = false;
                self.left_door_bypass_cooldown = false;
                self.left_door_shut = false;
            } else {
                self.audio.door.halt();
                self.audio
                    .door_thud
                    .play_reserved(0, self.pan_left, self.pan_right)?;
                self.left_door_bypass_cooldown = false;

                self.left_door_last_shut = SystemTime::now() - Duration::from_secs(10);
            }
        }
        if self.left_door_last_shut.elapsed()?.as_secs() >= 10 {
            self.left_door_shut = false;
            self.can_open_left_door = true;
        }

        if self.right_door_last_shut.elapsed()?.as_secs() >= 5 {
            if !self.right_door_bypass_cooldown {
                self.can_open_right_door = false;
                self.right_door_bypass_cooldown = false;
                self.right_door_shut = false;
            } else {
                self.audio.door.halt();

                self.audio
                    .door_thud
                    .play_reserved(1, self.pan_left, self.pan_right)?;
                self.right_door_bypass_cooldown = false;
                self.right_door_last_shut = SystemTime::now() - Duration::from_secs(10);
            }
        }
        if self.right_door_last_shut.elapsed()?.as_secs() >= 10 {
            self.right_door_shut = false;
            self.can_open_right_door = true;
        }

        if self.open_left_door_back_up {
            self.left_door_last_shut = SystemTime::now() - Duration::from_secs(4);

            self.left_door_bypass_cooldown = true;
            self.open_left_door_back_up = false;
        }
        if self.open_right_door_back_up {
            self.right_door_last_shut = SystemTime::now() - Duration::from_secs(4);
            self.right_door_bypass_cooldown = true;
            self.open_right_door_back_up = false;
        }
        if self.gang.wilber.stage >= 3 && self.gang.wilber.rage() >= 0.2 {
            if self.jumpscarer == MonsterName::None {
                self.going_to_office = true;
                self.jumpscarer = MonsterName::Wilber;
                self.gameover_time = SystemTime::now();
                self.getting_jumpscared = true;
            }
        }

        if self.gang.gogopher.duct_heat_timer > 0 {
            self.gang.gogopher.duct_heat_timer -= 1;
        }

        if on_tutorial {
            return Ok(());
        }

        let is_over = self.gang.step(cur_time, &mut self.audio) || self.force_win;

        if is_over && !self.has_won {
            self.audio.brownian_noise.halt();
            self.audio.open_source_closed_casket.halt();
            self.has_won = true;
            self.last_screen = Screen::YouWin;
            self.screen = Screen::YouWin;
            config_mut().unlock_night_2();
            self.win_time = SystemTime::now();
            return Ok(());
        }

        if self.screen.is_passive() {
            return Ok(());
        }

        let inoffice = self.gang.in_room(Room::Office);

        for mons in inoffice {
            if mons.active() {
                let duration: &Duration = &mons.timer_until_office().elapsed()?;

                let is_tux = mons.id() == MonsterName::Tux || mons.id() == MonsterName::GoldenTux;
                let is_gopher = mons.id() == MonsterName::GoGopher;

                if is_gopher
                    || is_tux
                    || duration.as_millis() >= (MONSTER_TIME_OFFICE_WAIT_THING as u128 * 1000) - 500
                {
                    let var_name = MONSTER_TIME_OFFICE_WAIT_THING as u128 * 1000000000;

                    let mut do_flickering = true;

                    if is_tux {
                        do_flickering = false;
                    }
                    if mons.entered_from_left() {
                        if !self.left_door_shut {
                            self.tainted += mons.taint_percent();
                        } else {
                            if duration.as_nanos() <= var_name {
                                self.open_left_door_back_up = true;
                            }
                            //mons.set_entered_from_left(false);
                            mons.goto_room_after_office();
                            do_flickering = false;
                        }
                    }
                    if mons.entered_from_right() {
                        if !self.right_door_shut {
                            self.tainted += mons.taint_percent();
                        } else {
                            if duration.as_nanos() <= var_name {
                                self.open_right_door_back_up = true;
                            }
                            //mons.set_entered_from_right(false);
                            mons.goto_room_after_office();
                            do_flickering = false;
                        }
                    }
                    // go gopher just does it regardless.
                    if mons.id() == MonsterName::GoGopher {
                        self.tainted += mons.taint_percent();
                        do_flickering = true;
                    }

                    if do_flickering {
                        if !mons.stinger_played() {
                            if !self.audio.stinger.is_playing() {
                                self.audio.stinger.play()?;
                            }
                            mons.set_stinger_played(true);
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

        Ok(())
    }

    /**
    Draw things from the state onto the window.

    It is IMPERITIVE that anything that should be in `State::step` is in the right function, as the `State::draw_step` is called at an uncapped rate as opposed to 60 times a second. Vice versa applies.
     */
    pub fn draw_step(
        &mut self,
        rl: &mut RaylibDrawHandle,
        thread: &RaylibThread,
        mx: i32,
        my: i32,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut d = rl.begin_mode2D(self.camera);
        d.clear_background(Color::BLACK);

        match self.screen {
            Screen::TitleScreen => self.title_screen_draw(&mut d, &thread)?,
            Screen::Credits => self.credits_draw(&mut d, &thread, mx, my)?,
            Screen::GameOver => self.gameover_draw(&mut d, &thread)?,
            Screen::YouWin => self.win_draw(&mut d, &thread)?,
            Screen::Office => {
                self.office_draw(&mut d, &thread)?;
            }
            Screen::CameraRebooting => self.camera_rebooting_draw(&mut d, &thread)?,
            Screen::Camera => self.camera_draw(&mut d, &thread)?,
            Screen::Settings => {}
        }

        let rot = {
            if self.screen == Screen::Office
                && (self.jumpscarer == MonsterName::Tux
                    || self.jumpscarer == MonsterName::GoldenTux)
            {
                let r = thread_rng().gen_range(-5..5);
                r as f32
            } else {
                0.0
            }
        };

        let mut corrected_width = (self.framebuffer.width() as f32
            * (d.get_render_width() as f32 / self.framebuffer.width() as f32))
            .ceil();
        let mut corrected_height = (self.framebuffer.height() as f32
            * (d.get_render_height() as f32 / self.framebuffer.height() as f32))
            .ceil();

        if corrected_height <= corrected_width {
            corrected_width = corrected_height / 0.75;
        } else {
            corrected_height = corrected_width * 0.75;
        }

        self.laptop_shader.set_shader_value(
            self.laptop_shader.get_shader_location("rand"),
            UNIX_EPOCH.elapsed()?.as_nanos() as i32,
        );
        self.laptop_shader.set_shader_value(
            self.laptop_shader.get_shader_location("camera_changing"),
            self.camera_changing as i32,
        );

        self.laptop_shader.set_shader_value(
            self.laptop_shader.get_shader_location("width"),
            d.get_render_width(),
        );
        self.laptop_shader.set_shader_value(
            self.laptop_shader.get_shader_location("height"),
            d.get_render_height(),
        );
        if self.screen != Screen::Office {
            let mut d = d.begin_shader_mode(&mut self.laptop_shader);

            d.draw_texture_pro(
                &self.framebuffer,
                Rectangle::new(
                    config().width() as f32,
                    0.0,
                    -config().width() as f32,
                    config().height() as f32,
                ),
                Rectangle::new(
                    (d.get_render_width() as f32 / 2.0) + rot,
                    (d.get_render_height() as f32 / 2.0) + rot,
                    corrected_width,
                    corrected_height,
                ),
                Vector2::new(corrected_width / 2.0, corrected_height / 2.0),
                180.0 + rot,
                Color::WHITE,
            );
        } else {
            d.draw_texture_pro(
                &self.framebuffer,
                Rectangle::new(
                    config().width() as f32,
                    0.0,
                    -config().width() as f32,
                    config().height() as f32,
                ),
                Rectangle::new(
                    (d.get_render_width() as f32 / 2.0) + rot,
                    (d.get_render_height() as f32 / 2.0) + rot,
                    corrected_width,
                    corrected_height,
                ),
                Vector2::new(corrected_width / 2.0, corrected_height / 2.0),
                180.0 + rot,
                Color::WHITE,
            );
        }

        let win_alpha = if self.screen == Screen::YouWin {
            if config().night_2() {
                self.win_time.elapsed()?.as_secs_f32() * 0.07142857142
            } else {
                self.win_time.elapsed()?.as_secs_f32() * 0.5
            }
        } else {
            0.0
        };

        d.draw_texture_pro(
            &self.you_win_framebuffer,
            Rectangle::new(
                config().width() as f32,
                0.0,
                -config().width() as f32,
                config().height() as f32,
            ),
            Rectangle::new(
                (d.get_render_width() as f32 / 2.0) + rot,
                (d.get_render_height() as f32 / 2.0) + rot,
                corrected_width,
                corrected_height,
            ),
            Vector2::new(corrected_width / 2.0, corrected_height / 2.0),
            180.0 + rot,
            Color::WHITE.alpha(win_alpha),
        );

        d.draw_fps(10, 10);
        Ok(())
    }

    /**
    Process the clickable areas and the imgui menu.
    */
    pub fn input_step(
        &mut self,
        mut d: &mut RaylibDrawHandle,
        thread: &RaylibThread,
        mx: i32,
        my: i32,
    ) -> Result<(), Box<dyn std::error::Error>> {
        if d.is_key_down(KeyboardKey::KEY_LEFT_ALT) {
            self.debug_draw(&mut d)?;
        } else {
            match self.screen {
                Screen::TitleScreen => {
                    self.title_screen_menu(&mut d)?;
                }
                Screen::Camera => {
                    self.arrow_click(d)?;
                    self.camera_ui_draw(&mut d, &thread)?;
                }
                Screen::Credits | Screen::GameOver => {}
                Screen::YouWin => {}
                Screen::CameraRebooting => {
                    self.arrow_click(d)?;
                    self.camera_ui_draw(&mut d, &thread)?;
                }
                Screen::Office => {
                    self.office_clickable(&mut d, mx, my)?;
                    self.office_ui_draw(&mut d, &thread)?;
                }
                Screen::Settings => self.settings_draw(&mut d, &thread)?,
            }
        }
        Ok(())
    }

    /**
     Sets up audio based on the bg_offset_x, state, etc.
    */
    pub fn audio_step(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        if self.screen.is_passive() || config().on_tutorial() {
            return Ok(());
        }
        let panner = self.bg_offset_x / 3.0;
        let mut left = 191.0 - panner;
        if left <= 64.0 {
            left = 64.0;
        }
        if left >= 191.0 {
            left = 191.0;
        }
        let mut right = panner;
        if right <= 64.0 {
            right = 64.0;
        }
        if right >= 191.0 {
            right = 191.0;
        }
        self.pan_left = left as u8;
        self.pan_right = right as u8;

        self.audio.halt_not_playing();

        self.audio.play_ambience()?;

        Ok(())
    }

    /**
    Plays audio based on relevant factors.
    */
    pub fn audio_play_step(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        if self.screen == Screen::TitleScreen || self.screen == Screen::GameOver {
            return Ok(());
        }

        if self.gang.wilber.active() && !config().night_2() {
            if !self.audio.wilber_appear.is_playing() && !self.wilbur_snd_played {
                self.audio.wilber_appear.play()?;
                self.wilbur_snd_played = true;
            }
        }
        if self.gang.tux.active() && !config().night_2() {
            if !self.audio.tux_appears.is_playing() && !self.tux_snd_played {
                self.audio.tux_appears.play()?;
                self.tux_snd_played = true;
            }
        }
        if self.gang.gogopher.active() && !config().night_2() {
            if !self.audio.gopher.is_playing() && !self.gopher_snd_played {
                self.audio.gopher.play()?;
                self.gopher_snd_played = true;
            }
        }
        for mons in self.gang.in_room(Room::Office) {
            if mons.active() {
                let duration: &Duration = &mons.timer_until_office().elapsed()?;

                let is_tux = mons.id() == MonsterName::Tux || mons.id() == MonsterName::GoldenTux;
                if !is_tux
                    && duration.as_millis() >= (MONSTER_TIME_OFFICE_WAIT_THING as u128 * 1000) - 500
                {
                    let note: usize = (self.tainted * 0.36) as usize;
                    self.audio.play_tainted(note).unwrap();
                }
            }
        }
        Ok(())
    }

    /**
    Get the game's current hour.
    */
    pub fn time(&self) -> Result<i64, Box<dyn std::error::Error>> {
        let cur_time = self.ingame_time.duration_since(UNIX_EPOCH)?;
        let ct = self.gang.hours(cur_time);
        if ct <= 0 {
            Ok(12 + ct)
        } else {
            Ok(ct)
        }
    }

    /**
    Get the mouse position, scaled according to what the framebuffer size is.
    */
    pub fn mouse_position(
        &self,
        d: &mut RaylibHandle,
    ) -> Result<(i32, i32), Box<dyn std::error::Error>> {
        let width_mul = d.get_screen_width() as f32 / self.framebuffer.width() as f32;
        let height_mul = d.get_screen_height() as f32 / self.framebuffer.height() as f32;

        let mx = ({
            if d.get_touch_x() != 0 {
                d.get_touch_x()
            } else {
                d.get_mouse_x()
            }
        } as f32
            / width_mul) as i32;

        let my = ({
            if d.get_touch_y() != 0 {
                d.get_touch_y()
            } else {
                d.get_mouse_y()
            }
        } as f32
            / height_mul) as i32;

        Ok((mx, my))
    }

    pub fn arrow_click(&mut self, rl: &mut RaylibHandle) -> Result<(), Box<dyn std::error::Error>> {
        let my = rl.get_mouse_y();
        let cur_time = self.ingame_time.duration_since(UNIX_EPOCH)?;

        if my >= config().real_height() - (Self::bat_height() * 2.5) as i32
            && !self.getting_jumpscared
        {
            if rl.is_mouse_button_released(MouseButton::MOUSE_BUTTON_LEFT) {
                self.audio.camera_flip.play()?;
                match self.screen {
                    Screen::Office => {
                        self.gang.golden_tux.deactivate();
                        self.going_to_camera = true;
                    }
                    Screen::CameraRebooting | Screen::Camera => {
                        if self.gang.hours(cur_time) >= 5 {
                            if thread_rng().gen_range(1..100) == 1 {
                                self.gang.golden_tux.activate();
                                self.gang.golden_tux.appeared = SystemTime::now();
                            }
                        }
                        self.going_to_office = true;
                    }
                    _ => (),
                }
            }
        }
        Ok(())
    }
}
