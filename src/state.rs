use std::{
    alloc::System,
    time::{SystemTime, UNIX_EPOCH},
};

use raylib::prelude::*;

use crate::{
    enums::{Room, Screen},
    get_height, get_margin, get_width,
    monster::Gang,
};

pub struct State {
    pub screen: Screen,
    pub bg_offset_x: f32,
    pub laptop_offset_y: f64,
    pub camera_clickables: Vec<Rectangle>,
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

    pub can_open_left_door: bool,
    pub can_open_right_door: bool,

    pub left_door_anim_timer: f32,
    pub right_door_anim_timer: f32,

    pub left_door_shut: bool,
    pub right_door_shut: bool,

    pub left_door_last_shut: SystemTime,
    pub right_door_last_shut: SystemTime,

    pub duct_heat_timer: f64,
}

impl State {
    pub fn new() -> Self {
        let screen = Screen::TitleScreen;
        let bg_offset_x = get_width() as f32 / 2.0;
        let laptop_offset_y = 0.0;

        let camera_clickables = vec![
            Rectangle::new(
                get_margin() + get_width() as f32 * 0.40, // 60
                get_height() as f32 * 0.12,               // 20
                get_width() as f32 * 0.20,
                get_height() as f32 * 0.15,
            ), // Room1
            Rectangle::new(
                get_margin() + get_width() as f32 * 0.40,
                get_height() as f32 * 0.30,
                get_width() as f32 * 0.30,
                get_height() as f32 * 0.20,
            ), // Room2
            Rectangle::new(
                get_margin() + get_width() as f32 * 0.10,
                get_height() as f32 * 0.70,
                get_width() as f32 * 0.20,
                get_height() as f32 * 0.15,
            ), // Room3
            Rectangle::new(
                get_margin() + get_width() as f32 * 0.73,
                get_height() as f32 * 0.69,
                get_width() as f32 * 0.20,
                get_height() as f32 * 0.15,
            ), // Room5
            Rectangle::new(
                get_margin() + get_width() as f32 * 0.45,
                get_height() as f32 * 0.55,
                get_width() as f32 * 0.15,
                get_height() as f32 * 0.10,
            ), // Room4
            Rectangle::new(
                get_margin() + get_width() as f32 * 0.05,
                get_height() as f32 * 0.08,
                get_width() as f32 * 0.15,
                get_height() as f32 * 0.15,
            ), // Room6
        ];

        let door_buttons = vec![
            Rectangle::new(
                get_margin() + get_width() as f32 * 0.35,
                get_height() as f32 * 0.35,
                get_width() as f32 * 0.10,
                get_width() as f32 * 0.10,
            ),
            Rectangle::new(
                get_margin() + get_width() as f32 * 1.15,
                get_height() as f32 * 0.35,
                get_width() as f32 * 0.10,
                get_width() as f32 * 0.10,
            ),
        ];

        let duct_button = Rectangle::new(
            get_margin() + get_width() as f32 * 0.15,
            get_height() as f32 * 0.40,
            get_width() as f32 * 0.10,
            get_width() as f32 * 0.10,
        );

        let sel_camera = Room::Room1;
        let timer = SystemTime::now();

        let ingame_time = UNIX_EPOCH;
        let gang = Gang::new();

        let tainted = 0.0;
        let tainted_cache = 0.0;

        let camera_timer = 100.0;
        let camera_booting = false;
        let camera_booting_timer = 0.0;

        let gameover_time = SystemTime::now();

        let can_open_left_door = true;
        let can_open_right_door = true;

        let left_door_shut = false;
        let right_door_shut = false;

        let left_door_last_shut: SystemTime = SystemTime::now();
        let right_door_last_shut: SystemTime = SystemTime::now();

        let duct_heat_timer = 0.0;

        Self {
            screen,
            bg_offset_x,
            laptop_offset_y,
            camera_clickables,
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
            can_open_left_door,
            can_open_right_door,
            left_door_shut,
            right_door_shut,
            left_door_last_shut,
            right_door_last_shut,
            duct_heat_timer,
            left_door_anim_timer: -(get_height() as f32 * 0.09),
            right_door_anim_timer: -(get_height() as f32 * 0.09),
        }
    }
}