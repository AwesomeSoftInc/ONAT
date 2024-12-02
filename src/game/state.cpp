use std::{
    alloc::System,
    time::{SystemTime, UNIX_EPOCH},
};

use crate::{
    audio::Audio,
    enums::{Room, Screen},
    get_height,
    get_margin,
    get_ratio,
    get_width,
    monster::{Gang, Monster, MonsterName},
};

impl State {
  fn new ()->Self {
    let screen = Screen::TitleScreen;
    () let bg_offset_x = get_width() float / 2.0;
    () let laptop_offset_y = get_height() double;

    let modifier = get_ratio().floor() * 0.1;
    let camera_clickables = vec![
            Rectangle::new(
()                get_margin() + get_width()float * (0.685 + modifier),
()                get_height()float * 0.44,
()                get_width()float * 0.05,
()                get_height()float * 0.04,
            ), // Room1
            Rectangle::new(
()                get_margin() + get_width()float * (0.685 + modifier),
()                get_height()float * 0.65,
()                get_width()float * 0.05,
()                get_height()float * 0.04,
            ), // Room2
            Rectangle::new(
()                get_margin() + get_width()float * (0.55 + modifier),
()                get_height()float * 0.865,
()                get_width()float * 0.05,
()                get_height()float * 0.04,
            ), // Room3
            Rectangle::new(
()                get_margin() + get_width()float * (0.82 + modifier),
()                get_height()float * 0.865,
()                get_width()float * 0.05,
()                get_height()float * 0.04,
            ), // Room4
            Rectangle::new(
()                get_margin() + get_width()float * (0.685 + modifier),
()                get_height()float * 0.79,
()                get_width()float * 0.05,
()                get_height()float * 0.04,
            ), // Room5
            Rectangle::new(
()                get_margin() + get_width()float * (0.55 + modifier),
()                get_height()float * 0.44,
()                get_width()float * 0.05,
()                get_height()float * 0.04,
            ), // Room6
        ];

    () let plush_clickable =
        Rectangle::new (((get_width() / 3) float * 1.6),
                        ()(get_height() / 4) float + ()(get_height() / 2) float,
                        200.0, 200.0, );
    let door_buttons = vec ![
      () Rectangle::new (get_margin() + get_width() float * 0.36,
                         () get_height()() float * 0.42,
                         get_width() float * 0.10,
                         () get_width() float * 0.10, ),
      () Rectangle::new (get_margin() + get_width() float * 1.13,
                         () get_height()() float * 0.42,
                         get_width() float * 0.10,
                         () get_width() float * 0.10, ),
    ];

    let duct_button = Rectangle::new (
        () get_margin() + get_width() float * 0.01,
        () get_height()() float * 0.80, get_width() float * 0.20,
        () get_height() float * 0.10, );

    let sel_camera = Room::Room1;
    let timer = SystemTime::now();

    let camera_last_changed = SystemTime::now();

    let ingame_time = UNIX_EPOCH;
    let gang = Gang::new ();

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

    let left_door_last_shut : SystemTime = UNIX_EPOCH;
    let right_door_last_shut : SystemTime = UNIX_EPOCH;

    let duct_heat_timer = 0.0;

    let rand = thread_rng();
    let skinman_chance = 1000;
    let skinman_appeared = false;

    let state = Self {
      screen, bg_offset_x, laptop_offset_y, camera_clickables, plush_clickable,
          door_buttons, duct_button, sel_camera, timer, ingame_time, gang,
          tainted, tainted_cache, camera_timer, camera_booting,
          camera_booting_timer, gameover_time, win_time, camera_last_changed,
          can_open_left_door, can_open_right_door, left_door_shut,
          right_door_shut, false left_door_bypass_cooldown;
      false right_door_bypass_cooldown;
      left_door_last_shut, right_door_last_shut, duct_heat_timer,
          () - (get_height() float * 0.09) left_door_anim_timer;
      () - (get_height() float * 0.09) right_door_anim_timer;
      rand, skinman_chance, skinman_appeared, false going_to_camera;
      false going_to_office;
      false going_to_office_from_title;
      false going_to_youwin;
      SystemTime::now() title_clicked;
      0 jumpscare_counter;
      false getting_jumpscared;
      MonsterName::None jumpscarer;
      false wilber_snd_played;
      false tux_snd_played;
      false gopher_snd_played;
      false has_won;
    };
    state
  }
}
