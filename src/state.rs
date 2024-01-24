use std::{
    alloc::System,
    time::{SystemTime, UNIX_EPOCH},
};

use rand::{rngs::ThreadRng, thread_rng};
use raylib::prelude::*;

use crate::{
    audio::Audio,
    enums::{Room, Screen},
    get_height, get_margin, get_ratio, get_width,
    monster::{Gang, Monster, MonsterName},
};

pub struct State {
    pub screen: Screen,
    pub bg_offset_x: f32,
    pub laptop_offset_y: f64,
    pub camera_clickables: Vec<Rectangle>,
    pub plush_clickable: Rectangle,
    pub door_buttons: Vec<Rectangle>,
    pub duct_button: Rectangle,
    pub sel_camera: Room,
    pub timer: SystemTime,

    pub ingame_time: SystemTime,
    pub gang: Gang,
    pub tainted: f32,
    pub tainted_cache: f32,

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

    pub duct_heat_timer: f64,

    pub rand: ThreadRng,
    pub skinman_chance: u32,
    pub skinman_appeared: bool,

    pub going_to_office: bool,
    pub going_to_camera: bool,
    pub going_to_office_from_title: bool,
    pub title_clicked: SystemTime,
    pub going_to_youwin: bool,

    pub wilber_snd_played: bool,
    pub tux_snd_played: bool,
    pub gopher_snd_played: bool,

    pub jumpscare_counter: usize,
    pub getting_jumpscared: bool,
    pub jumpscarer: MonsterName,
    pub has_won: bool,
}

impl State {
    pub fn new() -> Self {
        let screen = Screen::TitleScreen;
        let bg_offset_x = get_width() as f32 / 2.0;
        let laptop_offset_y = get_height() as f64;

        let modifier = get_ratio().floor() * 0.1;
        let camera_clickables = vec![
            Rectangle::new(
                get_margin() + get_width() as f32 * (0.685 + modifier),
                get_height() as f32 * 0.44,
                get_width() as f32 * 0.05,
                get_height() as f32 * 0.04,
            ), // Room1
            Rectangle::new(
                get_margin() + get_width() as f32 * (0.685 + modifier),
                get_height() as f32 * 0.65,
                get_width() as f32 * 0.05,
                get_height() as f32 * 0.04,
            ), // Room2
            Rectangle::new(
                get_margin() + get_width() as f32 * (0.55 + modifier),
                get_height() as f32 * 0.865,
                get_width() as f32 * 0.05,
                get_height() as f32 * 0.04,
            ), // Room3
            Rectangle::new(
                get_margin() + get_width() as f32 * (0.82 + modifier),
                get_height() as f32 * 0.865,
                get_width() as f32 * 0.05,
                get_height() as f32 * 0.04,
            ), // Room4
            Rectangle::new(
                get_margin() + get_width() as f32 * (0.685 + modifier),
                get_height() as f32 * 0.79,
                get_width() as f32 * 0.05,
                get_height() as f32 * 0.04,
            ), // Room5
            Rectangle::new(
                get_margin() + get_width() as f32 * (0.55 + modifier),
                get_height() as f32 * 0.44,
                get_width() as f32 * 0.05,
                get_height() as f32 * 0.04,
            ), // Room6
        ];

        let plush_clickable = Rectangle::new(
            ((get_width() / 3) as f32 * 1.6),
            (get_height() / 4) as f32 + (get_height() / 2) as f32,
            200.0,
            200.0,
        );
        let door_buttons = vec![
            Rectangle::new(
                get_margin() + get_width() as f32 * 0.36,
                get_height() as f32 * 0.42,
                get_width() as f32 * 0.10,
                get_width() as f32 * 0.10,
            ),
            Rectangle::new(
                get_margin() + get_width() as f32 * 1.13,
                get_height() as f32 * 0.42,
                get_width() as f32 * 0.10,
                get_width() as f32 * 0.10,
            ),
        ];

        let duct_button = Rectangle::new(
            get_margin() + get_width() as f32 * 0.01,
            get_height() as f32 * 0.80,
            get_width() as f32 * 0.20,
            get_height() as f32 * 0.10,
        );

        let sel_camera = Room::Room1;
        let timer = SystemTime::now();

        let camera_last_changed = SystemTime::now();

        let ingame_time = UNIX_EPOCH;
        let gang = Gang::new();

        let tainted = 0.0;
        let tainted_cache = 0.0;

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

        let duct_heat_timer = 0.0;

        let rand = thread_rng();
        let skinman_chance = 1000;
        let skinman_appeared = false;

        let state = Self {
            screen,
            bg_offset_x,
            laptop_offset_y,
            camera_clickables,
            plush_clickable,
            door_buttons,
            duct_button,
            sel_camera,
            timer,
            ingame_time,
            gang,
            tainted,
            tainted_cache,
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
            duct_heat_timer,
            left_door_anim_timer: -(get_height() as f32 * 0.09),
            right_door_anim_timer: -(get_height() as f32 * 0.09),
            rand,
            skinman_chance,
            skinman_appeared,
            going_to_camera: false,
            going_to_office: false,
            going_to_office_from_title: false,
            going_to_youwin: false,
            title_clicked: SystemTime::now(),
            jumpscare_counter: 0,
            getting_jumpscared: false,
            jumpscarer: MonsterName::None,
            wilber_snd_played: false,
            tux_snd_played: false,
            gopher_snd_played: false,
            has_won: false,
        };
        state
    }
}
