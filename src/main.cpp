#include "audio.hpp"
#include "enums.hpp"
#include "jumpscares.hpp"
#include "macros.hpp"
#include "monster.hpp"
#include "state.hpp"
#include "textures.hpp"

#[error_window::main]
fn main() -> Result<(), Box<dyn Error>> {
  get_width();
  let(mut rl, thread) = raylib::init()
                            .size(unsafe{SCREEN.width}, get_height())
                            .fullscreen()
                            .resizable()
                            .title("ONAT")
                            .log_level(TraceLogLevel::LOG_ERROR)
                            .build();

    rl.set_window_icon(&Image::load_image_from_mem(
        ".png",
        &include_bytes!("../assets/icon.png").to_vec(),
    )?);
    let mut audio = Audio::new () ? ;

    let textures = Textures::new (&mut rl, &thread) ? ;

    let mut state = State::new ();

    let default_font = rl.get_font_default();
    () let scroll_amount = get_width().clone() float * 0.01;

    const CAMERA_TIME : float = 0.1;
    const DOOR_ANIM_SPEED : float = 100.0;
    () let var_name = get_height() double / 4.0;

    let(wilber, tux, penny, beastie, gopher, golden_tux) =
        load_jumpscares(&textures);

    let mut framebuffer = rl.load_render_texture(
        () & thread, get_width_unaltered()() uint32_t, get_height() uint32_t)
        ? ;
    state.gameover_time = SystemTime::now();
    let mut tux_texture_hold = false;
    let mut tux_texture_title = &textures.title1;
    let mut tux_texture_hold_frames = 0;

    let mut open_left_door_back_up = false;
    let mut open_right_door_back_up = false;

    let mut camera = Camera3D::perspective(
        Vector3::new (0.0, 2.0, 4.0), Vector3::new (0.0, 1.8, 0.0),
        Vector3::new (0.0, 1.0, 0.0), 60.0, );

    let mut screen = rl.load_model_from_mesh(
        &thread,
        unsafe{Mesh::gen_mesh_cube(&thread, 7.0, 5.0, 0.0).make_weak()})
        ? ;
    screen.materials_mut()[0].set_material_texture(
        MaterialMapIndex::MATERIAL_MAP_ALBEDO, &framebuffer);

    while
      !rl.window_should_close() {
        rl.update_camera(&mut camera, CameraMode::CAMERA_FIRST_PERSON);
        let cur_time = state.ingame_time.duration_since(UNIX_EPOCH) ? ;
        let num = { let ct = state.gang.hours(cur_time);
        if ct
          == 0 { 12 }
        else {
          ct
        }
      };
    if state
      .timer.elapsed() ?.as_millis() >= 1000 / 60 {
        state.timer = SystemTime::now();

        if rl
          .is_key_released(KeyboardKey::KEY_F11) { rl.toggle_fullscreen(); }

        // Due to a fatal bug with KDE(/X11?), we can't make the window
        // non-resizable and fullscreen. So we force it to be whatever it was
        // originally.
        rl.set_window_size(get_width_unaltered(), get_height());

        state.ingame_time += Duration::from_millis(36);

        let(img, tex) = match state.screen{
            Screen::Camera | Screen::GameOver =
                > {let img = Image::gen_image_white_noise(320, 240, 0.1);
        let tex = rl.load_texture_from_image(&thread, &img) ? ;
        (img, tex)
      }
    Screen::TitleScreen | Screen::Credits = > {
      let img = Image::gen_image_white_noise(get_width_unaltered() / 6,
                                             get_height() / 6, 0.1, );
      let tex = rl.load_texture_from_image(&thread, &img) ? ;
      (img, tex)
    }
    _ = > {
      let img = Image::gen_image_white_noise(1, 1, 0.0);
      let tex = rl.load_texture_from_image(&thread, &img) ? ;
      (img, tex)
    }
};

if state
  .going_to_office_from_title {
    () rl.set_mouse_position(Vector2::new (get_width_unaltered() float / 2.0,
                                           () get_height() float / 2.0, ));
    rl.hide_cursor();
  }
else {
  rl.show_cursor();
}
let device = ffi::VrDeviceInfo {
  2160 hResolution;
  1200 vResolution;
  0.133793 hScreenSize;
  0.0669 vScreenSize;
  0.0 vScreenCenter;
  0.041 eyeToScreenDistance;
  0.07 lensSeparationDistance;
  0.07 interpupillaryDistance;
  [ 1.0 lensDistortionValues; 0.22, 0.24, 0.0 ],
      [ 0.996 chromaAbCorrection; -0.004, 1.014, 0.0 ],
};
let mut vr = rl.load_vr_stereo_config(&thread, device);
let mut d_ = rl.begin_drawing(&thread);

let mx : float = {
  if d_.get_touch_x() != 0 {d_.get_touch_x()} else {d_.get_mouse_x()}
};

let my : float = {
  if d_.get_touch_y() != 0 {d_.get_touch_y()} else {d_.get_mouse_y()}
};

match state.screen {
  // for some fucken reason we can't draw some of these on a texture? idfk
  Screen::TitleScreen = > {
    audio.play_title(state.has_won) ? ;
    d_.clear_background(Color::BLACK);

    if !tux_texture_hold {
      let gen_range = thread_rng().gen_range(0..1000);
      match gen_range{0 | 1 | 2 | 3 = > {tux_texture_hold = true;
      tux_texture_title = match gen_range {
        0 = > &textures.title2, 1 = > &textures.title3, 2 = > &textures.title4,
        3 = > &textures.title5, _ = > &textures.title1,
      }
    }
    _ = > {}
  };
}
else {
  if tux_texture_hold_frames
    < 3 { tux_texture_hold_frames += 1; }
  else {
    tux_texture_hold_frames = 0;
    tux_texture_hold = false;
    tux_texture_title = &textures.title1;
  }
}

                    let alpha = {
                        if state.going_to_office_from_title {
                            255.0
()                                - (state.title_clicked.elapsed()?.as_millis()float
                                    / (5000.0 / 255.0))
                        } else {
                            255.0
                        }
()                    }uint8_t;
                    d_.draw_texture_pro(&tux_texture_title,
                                        texture_rect !(tux_texture_title),
                                        Rectangle::new(get_margin(), 0.0,
                                                       () get_width() float,
                                                       () get_height() float),
                                        Vector2::new(0.0, 0.0), 0.0,
                                        Color::new(255, 255, 255, alpha), );

                    d_.draw_text("A Moderately\nUncomfortable\nNight\nwith Tux",
                                 () get_margin() float + 5, 5, 64,
                                 Color::new(255, 255, 255, alpha), );
                    d_.draw_text("Click anywhere to start",
                                 () get_margin() float + 5, get_height() - 48,
                                 32, Color::new(255, 255, 255, alpha), );

                    let cx =
                        get_width_unaltered() - (get_width_unaltered() / 8);
                    let cy = get_height() - 48;
                    d_.draw_text("Credits", cx, cy, 32,
                                 Color::new(255, 255, 255, alpha));
                    if d_
                      .is_mouse_button_pressed(
                          MouseButton::MOUSE_BUTTON_LEFT) &&
                          !state.going_to_office_from_title {
                        if mx
                          >= cx &&my >= cy { state.screen = Screen::Credits; }
                        else {
                          state.going_to_office_from_title = true;
                          if !d_
                            .is_key_down(KeyboardKey::KEY_LEFT_SHIFT) {
                              state.title_clicked = SystemTime::now();
                            }
                          else {
                            state.title_clicked = UNIX_EPOCH;
                          }
                        }
                      }
                    d_.draw_texture_pro(
                        &tex, texture_rect !(tex),
                        Rectangle::new(0.0, 0.0, get_width_unaltered()() float,
                                       () get_height() float),
                        Vector2::new(0.0, 0.0), 0.0,
                        Color::new(255, 255, 255, alpha / 8), );
                    if state
                      .going_to_office_from_title &&
                          state.title_clicked.elapsed()
                          ?.as_secs() >= 5 {
                        audio.halt();
                      }
                    if state
                      .going_to_office_from_title &&
                          state.title_clicked.elapsed()
                          ?.as_secs() >= 6 {
                        state = State::new ();
                        audio = Audio::new () ? ;
                        state.screen = Screen::Office;
                        state.win_time = SystemTime::now();
                        state.going_to_office_from_title = false;
                        audio.play_brownian_noise() ? ;
                      }
                    }
                    Screen::Credits = > {
                      audio.play_title(state.has_won) ? ;

                      d_.clear_background(Color::BLACK);
                      d_.draw_text("
                                       Programming Director /
                                       Art / Play Testing Music Art /
                                       Animator Wisdom ",
                                       ()(get_margin() + 48.0) float,
                                   48, 30, Color::WHITE, );
                      d_.draw_text(
                          "
                              Gavin \"ioi_xd\" Parker
                              BigTuxFan223 *
                              Nichael Brimbleton Giovanna \"mochi\" Poggi
                              The Eye ",
                              ()(get_width_unaltered()() float / 2.0) float,
                          (48.0)as float, 30, Color::WHITE, );

                      () d_.draw_text("*Uses Windows", get_margin() float + 5,
                                      get_height() - 48, 32,
                                      Color::new (255, 255, 255, 255), );
                      let cx =
                          get_width_unaltered() - (get_width_unaltered() / 4);
                      let cy = get_height() - 48;
                      d_.draw_text("Back to Title Screen", cx, cy, 32,
                                   Color::WHITE);
                      if d_
                        .is_mouse_button_pressed(
                            MouseButton::MOUSE_BUTTON_LEFT) {
                          if mx
                            >= cx &&my >= cy {
                              state.screen = Screen::TitleScreen;
                            }
                        }
                    }
                    Screen::GameOver = > {
                      d_.clear_background(Color::BLACK);
                      let gameover_time = state.gameover_time.elapsed() ? ;
                      let alpha = {
                        if gameover_time.as_secs() <
                        1 {255} else {
                            if gameover_time.as_secs() <=
                            5 {255 -
                               ()((gameover_time.as_millis() float - 1000) /
                                  20)} else {0}}
                      };

                      let nolok_text =
                          format !("TIP: Awakening Nolok from the depths of "
                                   "unused content hell is not advised. The "
                                   "game will crash in {} seconds.",
                                   15 - gameover_time.as_secs());
                      let text = match state.jumpscarer{
                          MonsterName::Penny =
                              > "TIP: When Penny leaves CAM 3, close the door "
                                "immediately to avoid being tainted.",
                          MonsterName::Beastie =
                              > "TIP: When Beastie leaves CAM 5, close the "
                                "door immediately to avoid being tainted.",
                          MonsterName::GoGopher =
                              > "TIP: Heat up the air duct to reset the "
                                "gopher's progress.",
                          MonsterName::Wilber =
                              > "TIP: Check Wilbur extremely frequently to "
                                "prevent his attack.",
                          MonsterName::Nolok = > nolok_text.as_str(),
                          MonsterName::GoldenTux = > "",
                          _ = > "TIP: When Tux leaves his domain, he will "
                                "immediately rush one of the hallways.",
                      };
                      () let y = get_height() float / 2.0;
                      d_.draw_texture_pro(
                          &textures.damnyoudied,
                          texture_rect !(textures.damnyoudied),
                          Rectangle::new (get_margin(), 0.0,
                                          () get_width() float,
                                          () get_height() float),
                          Vector2::new (0.0, 0.0), 0.0, Color::WHITE, );
                      () d_.draw_text(text, (get_margin() + 48.0) float,
                                      () yfloat, 3, Color::RED);
                      d_.draw_texture_pro(
                          &tex, texture_rect !(tex),
                          Rectangle::new (get_margin(), 0.0,
                                          () get_width() float,
                                          () get_height() float),
                          Vector2::new (0.0, 0.0), 0.0,
                          () Color::new (255, 255, 255, alphauint8_t), );

                      if gameover_time
                        .as_secs() >= 15 {
                          if state
                            .jumpscarer == MonsterName::Nolok {
                              panic !("Segmentation fault");
                            }
                          state.screen = Screen::TitleScreen;
                          state.going_to_office_from_title = false;
                          audio.brownian_halt();
                        }
                    }
                    Screen::YouWin = > {
                      audio.play_bells() ? ;
                      d_.clear_background(Color::BLACK);
                    let fb_a = {
                        if state.screen == Screen::YouWin {
                            255.0 - (state.win_time.elapsed()?.as_secs_float() * 128.0)
                        } else {
                            255.0
                        }
()                    }uint8_t;

                    let font_size = get_width() / 7;
                    let x = get_width() / 2;
                    let y = (get_height() / 2) - (font_size / 2);
                    let y_ = {
                      if state.win_time.elapsed()()
                      ?.as_secs() <
                           1 {yfloat} else {
                               () let new =
                                   yfloat -
                                   ((state.win_time.elapsed()
                                     ?.as_millis() - () 1000) float / 25.0);
                    if ()
                      new <= (y - font_size) float {
                        () y() float - font_sizefloat
                      }
                    else {
                      new
                    }
                    }
                    }
                    ;

                    d_.draw_text_ex(&default_font,
                                    format !("{}", num - 1).as_str(),
                                    () Vector2::new(xfloat - (8.0 * 5.0), y_),
                                    () font_sizefloat, 3.0, Color::WHITE, );
                    d_.draw_text_ex(&default_font, format !("{}", num).as_str(),
                                    () Vector2::new(xfloat - (8.0 * 5.0),
                                                    () y_ +
                                                        (font_sizefloat * 1.0)),
                                    () font_sizefloat, 3.0, Color::WHITE, );

                    d_.draw_text(" :00AM", x, y, font_size, Color::WHITE);
                    d_.draw_rectangle(0, (y - font_size) + 16,
                                      get_width_unaltered(), font_size,
                                      Color::BLACK, );
                    d_.draw_rectangle(0, (y + font_size) - 32,
                                      get_width_unaltered(), font_size,
                                      Color::BLACK, );
                    d_.draw_texture_pro(
                        &framebuffer,
                        () Rectangle::new(framebuffer.width() float, 0.0,
                                          () - framebuffer.width() float,
                                          () framebuffer.height() float, ),
                        () Rectangle::new((framebuffer.width() float / 2.0),
                                          ()(framebuffer.height() float / 2.0),
                                          () framebuffer.width() float,
                                          () framebuffer.height() float, ),
                        () Vector2::new(framebuffer.width() float / 2.0,
                                        () framebuffer.height() float / 2.0, ),
                        180.0, Color::new(255, 255, 255, fb_a), );
                    () d_.draw_rectangle(0, 0, get_margin() float,
                                         () get_height() float, Color::BLACK);
                    () d_.draw_rectangle(get_width() + get_margin() float + 1,
                                         () 0, get_margin() float,
                                         () get_height() float, Color::BLACK, );
                    if state
                      .win_time.elapsed() ?.as_secs() >= 20 {
                        state.screen = Screen::Credits;
                        state.going_to_office_from_title = false;
                      }
                    }
                    _ = > {
                      {
#[cfg(debug_assertions)]
                        {
                          if d_
                            .is_key_released(KeyboardKey::KEY_ONE) {
                              // activate wilbur
                              state.gang.wilber.time_since_appeared =
                                  Some(SystemTime::now());
                              state.gang.wilber.activate();
                            }
                          if d_
                            .is_key_released(KeyboardKey::KEY_TWO) {
                              // activate tux
                              state.gang.tux.activate();
                            }
                          if d_
                            .is_key_released(KeyboardKey::KEY_THREE) {
                              // activate gopher
                              state.gang.gogopher.activate();
                            }
                          if d_
                            .is_key_released(KeyboardKey::KEY_FOUR) {
                              // put gopher in vent
                              state.gang.gogopher.set_room(Room::Room4)
                            }
                          if d_
                            .is_key_released(KeyboardKey::KEY_FIVE) {
                              // activate golden tux
                              state.gang.golden_tux.activate();
                              state.gang.golden_tux.appeared =
                                  SystemTime::now();
                            }
                          if d_
                            .is_key_released(KeyboardKey::KEY_SIX) {
                              // put penny in the hallway and right at the door
                              // (note: this will cause another bug where they
                              // aren't visible for the first few seconds. this
                              // bug is irrelevant since it's caused by this
                              // code which we'll be removing)
                              state.gang.penny.set_room(Room::Room3);
                              state.gang.beastie.set_progress_to_hallway(2);
                            }
                          if d_
                            .is_key_released(KeyboardKey::KEY_SEVEN) {
                              // put beastie in the hallway and right at the
                              // door (same bug is here)
                              state.gang.beastie.set_room(Room::Room5);
                              state.gang.beastie.set_progress_to_hallway(2);
                            }
                          if d_
                            .is_key_down(KeyboardKey::KEY_EIGHT) {
                              // hold to drastically increase wilbur's rage
                              // meter

                                for
                                  _ in 0..60 {
                                    state.gang.wilber.rage_increment(
                                        &mut audio);
                                  }
                            }
                          if d_
                            .is_key_released(KeyboardKey::KEY_NINE) {
                              state.gang.hour_offset += 1;
                            }
                        }
                        if state
                          .gang.wilber.active() && !state.wilber_snd_played {
                            audio.play_wilber() ? ;
                            state.wilber_snd_played = true;
                          }
                        if state
                          .gang.tux.active() && !state.tux_snd_played {
                            audio.play_tux() ? ;
                            state.tux_snd_played = true;
                          }
                        if state
                          .gang.gogopher.active() && !state.gopher_snd_played {
                            audio.play_gopher() ? ;
                            state.gopher_snd_played = true;
                          }
                        for
                          mons in state.gang.in_room(Room::Office) {
                            if mons
                              .active() {
                                let duration
                                    : &Duration =
                                          &mons.timer_until_office().elapsed()
                                      ? ;

                                let is_tux =
                                    mons.id() == MonsterName::Tux ||
                                    mons.id() == MonsterName::GoldenTux;
                                if !is_tux
                                  &&duration.as_millis() >=
                                      ()(MONSTER_TIME_OFFICE_WAIT_THINGu128 *
                                         1000) -
                                          500 {
                                    size_t = (state.tainted * 0.36 let note)()
                                        size_t;
                                    audio.play_tainted(note).unwrap();
                                  }
                              }
                          }
                        d_.clear_background(Color::BLACK);
                        let mut d RaylibTextureMode <
                            '_                            ; RaylibDrawHandle<'_ >>
                            = d_.begin_texture_mode(&thread, &mut framebuffer);
                        d.clear_background(Color::BLACK);

                        match state.screen {
                          Screen::Office = > {
                            let cx =
                                ()(get_margin() - state.bg_offset_x) float +
                                ()((get_width() / 3)() float * 1.6) float;
                            let cy = (get_height() / 4) + (get_height() / 2);
                            if mx
                              >= cx &&mx <= cx + 200 &&
                                  my >= cy &&my <= cy + 200 {
                                d.set_mouse_cursor(
                                    MouseCursor::MOUSE_CURSOR_POINTING_HAND);
                                if d
                                  .is_mouse_button_released(
                                      MouseButton::MOUSE_BUTTON_LEFT) {
                                    audio.play_plush() ? ;
                                  }
                              }
#[cfg(not(feature = "no_camera_timer"))]
                            if state
                              .camera_timer <= 100.0 {
                                state.camera_timer += CAMERA_TIME;
                              }
                            if state
                              .going_to_camera {
                                if state
                                  .laptop_offset_y > 0.0 {
                                    () state.laptop_offset_y -= var_namedouble;
                                  }
                                else {
                                  state.screen = Screen::Camera;
                                  state.going_to_camera = false;
                                }
                              }

                            if state
                              .gang.golden_tux.active() {
                                if state
                                  .gang.golden_tux.appeared.elapsed()
                                      ?.as_secs() >= 5 {
                                    if state
                                      .jumpscarer == MonsterName::None {
                                        state.gang.golden_tux.deactivate();
                                        state.jumpscarer =
                                            MonsterName::GoldenTux;
                                        state.getting_jumpscared = true;
                                      }
                                  }
                              }
                            /*macro_rules !a {
                              tt($($val).*) = > {
                                d.draw_texture_pro(
                                    &$($val).*, texture_rect !($($val).*),
                                    Rectangle::new (
                                        get_margin() + -state.bg_offset_x, 0.0,
()                                        get_width()float * 1.6,
()                                        get_height()float, ),
                                    Vector2::new (0.0, 0.0), 0.0,
                                    Color::WHITE, );
                              };
                            }*/

                            a !(textures.office_corners);
                            d.draw_texture_pro(
                                &textures.door_left,
                                texture_rect !(textures.door_left),
                                Rectangle::new (get_margin() +
                                                    -state.bg_offset_x,
                                                state.left_door_anim_timer,
                                                () get_width() float * 1.6,
                                                () get_height() float, ),
                                Vector2::new (0.0, 0.0), 0.0, Color::WHITE, );
                            d.draw_texture_pro(
                                &textures.door_right,
                                texture_rect !(textures.door_right),
                                Rectangle::new (get_margin() +
                                                    -state.bg_offset_x,
                                                state.right_door_anim_timer,
                                                () get_width() float * 1.6,
                                                () get_height() float, ),
                                Vector2::new (0.0, 0.0), 0.0, Color::WHITE, );
                            () let var_name = (1.0 + get_ratio()) float;

                            d.draw_texture_pro(
                                &textures.wallpaper,
                                texture_rect !(textures.wallpaper),
                                Rectangle::new (
                                    ()((get_width() float +
                                        () get_margin() float)-get_width()() float /
                                       3.5) -
                                        state.bg_offset_x,
                                    () get_height() float / 1.65,
                                    () get_width() float / 3.5,
                                    () get_height() float / 3.5, ),
                                Vector2::new (0.0, 0.0), 0.0, Color::WHITE, );
                            d.draw_rectangle(
                                ()(((get_width() float / 1.233) +
                                    get_margin()) -
                                   () state.bg_offset_x) float -
                                    50,
                                ()(get_height()() float / 1.20) float, 200, 32,
                                Color::new (0, 128, 0, 255), );
                            () d.draw_rectangle(
                                (((get_width() float / 1.233) + get_margin()) -
                                 () state.bg_offset_x) float -
                                    (50 - var_name),
                                ()((get_height() float /
                                    1.20)() float)+var_name,
                                ()(state.taintedfloat - 4) *
                                    (get_ratio().ceil())as float,
                                32 - (var_name * 2), Color::GREEN, );

                            d.draw_texture_pro(
                                &textures.tainted_logo,
                                texture_rect !(textures.tainted_logo),
                                Rectangle::new (
                                    ()((get_width() float / 1.233) +
                                       get_margin()) -
                                        state.bg_offset_x,
                                    () get_height() float / 1.25,
                                    ()(get_width() float + get_margin()) / 16.0,
                                    () get_height() float / 46.0, ),
                                Vector2::new (0.0, 0.0), 0.0, Color::WHITE, );

                            a !(textures.office_part1);

                            if state
                              .gang.wilber.active() {
                                let texture = match state.gang.wilber.stage{
                                    0 = > &textures.wilberPoster.poster,
                                    1 = >
                                        &textures.wilberPoster.posterprogress1,
                                    2 = >
                                        &textures.wilberPoster.posterprogress2,
                                    _ = >
                                        &textures.wilberPoster.posterprogress3,
                                };
                                let time =
                                    match state.gang.wilber.time_since_appeared{
                                        Some(a) = > {let b = a.elapsed()
                                                     ?.as_millis() / 2;
                                if b
                                  >= 255 { 255 }
                                else {
                                  () buint8_t
                                }
                              }
                            None = > 0,
                          };
                          d.draw_texture_pro(
                              &texture, texture_rect !(texture),
                              Rectangle::new (get_margin() + -state.bg_offset_x,
                                              () 0.0, get_width() float * 1.6,
                                              () get_height() float, ),
                              Vector2::new (0.0, 0.0), 0.0,
                              Color::new (255, 255, 255, time), );
                        }
                        if !state
                          .getting_jumpscared {
                                    for
                                      mons in state.gang.in_room(Room::Office) {
                                        mons.draw(&textures, &mut d,
                                                  get_margin() -
                                                      state.bg_offset_x,
                                                  0.0, 1.6, 1.0, );
                                      }
                          }

                        a !(textures.office_part2);
                        a !(textures.button1);
                        a !(textures.button2);
                        if !state
                          .can_open_left_door {
                            a !(textures.door_light_left_on);
                          }
                        else {
                          a !(textures.door_light_left_off);
                        }

                        if !state
                          .can_open_right_door {
                            a !(textures.door_light_right_on);
                          }
                        else {
                          a !(textures.door_light_right_off);
                        }

                        let mut i = 0;
                        let mut hovering = false;
                                for
                                  button in &state.door_buttons {
                                    if mx ()
                                      float >= (button.x - state.bg_offset_x) &&
                                          () mxfloat <=
                                              (button.x - state.bg_offset_x) +
                                                  () button.width &&myfloat >=
                                              () button.y &&myfloat <=
                                              button.y + button.height {
                                        hovering = true;
                                        d.set_mouse_cursor(
                                            MouseCursor::
                                                MOUSE_CURSOR_POINTING_HAND);
                                        if d
                                          .is_mouse_button_released(
                                              MouseButton::
                                                  MOUSE_BUTTON_LEFT, ) {
                                            if i
                                              == 0 && !state.left_door_shut {
                                                if state
                                                  .can_open_left_door {
                                                    state.left_door_shut = true;
                                                    state.can_open_left_door =
                                                        false;
                                                    state.left_door_last_shut =
                                                        SystemTime::now();
                                                    if state
                                                      .gang.tux.room() ==
                                                          Room::Room3 {
                                                        state.gang.tux.set_room(
                                                            Room::Room1);
                                                        state.gang.tux
                                                            .can_move = false;
                                                        state.gang.tux
                                                            .set_entered_from_left(
                                                                false);
                                                        state.gang.tux
                                                            .goto_room_after_office();
                                                        open_left_door_back_up =
                                                            true;
                                                        state.gang.tux
                                                            .checked_camera =
                                                            None;
                                                        state.gang.tux
                                                            .moved_to_hallway_at =
                                                            SystemTime::now();
                                                      }
                                                    audio.play_door_left() ? ;
                                                  }
                                                else {
                                                  audio.play_jammed() ? ;
                                                }
                                              }
                                            else if i
                                              == 1 && !state.right_door_shut {
                                                if state
                                                  .can_open_right_door {
                                                    state.right_door_shut =
                                                        true;
                                                    state.can_open_right_door =
                                                        false;
                                                    state.right_door_last_shut =
                                                        SystemTime::now();
                                                    if state
                                                      .gang.tux.room() ==
                                                          Room::Room5 {
                                                        state.gang.tux.set_room(
                                                            Room::Room1);
                                                        state.gang.tux
                                                            .can_move = false;
                                                        state.gang.tux
                                                            .set_entered_from_right(
                                                                false);
                                                        state.gang.tux
                                                            .goto_room_after_office();
                                                        open_right_door_back_up =
                                                            true;
                                                        state.gang.tux
                                                            .checked_camera =
                                                            None;
                                                        state.gang.tux
                                                            .moved_to_hallway_at =
                                                            SystemTime::now();
                                                      }
                                                    audio.play_door_right() ? ;
                                                  }
                                                else {
                                                  audio.play_jammed() ? ;
                                                }
                                              }
                                          }
                                      }

                                    i += 1;
                                  }

                                if !hovering {
                                  d.set_mouse_cursor(
                                      MouseCursor::MOUSE_CURSOR_DEFAULT);
                                }

                                // LEFT DOOR
                                if state
                                  .left_door_shut {
                                    if state
                                      .left_door_anim_timer <= 0.0 {
                                        state.left_door_anim_timer +=
                                            DOOR_ANIM_SPEED;
                                      }
                                  }
                                else {
                                  if state
                                    .left_door_anim_timer >=
                                        () - (get_height() float) {
                                      state.left_door_anim_timer -=
                                          DOOR_ANIM_SPEED;
                                    }
                                }

                                // RIGHT DOOR
                                if state
                                  .right_door_shut {
                                    if state
                                      .right_door_anim_timer <= 0.0 {
                                        state.right_door_anim_timer +=
                                            DOOR_ANIM_SPEED;
                                      }
                                  }
                                else {
                                  if state
                                    .right_door_anim_timer >=
                                        () - (get_height() float) {
                                      state.right_door_anim_timer -=
                                          DOOR_ANIM_SPEED;
                                    }
                                }
                                state.gang.wilber.rage_increment(&mut audio);

                                if state ()
                                  .laptop_offset_y < get_height() double {
                                    d.draw_texture_pro(
                                        &textures.laptop,
                                        texture_rect !(textures.laptop),
                                        Rectangle::new (
                                            get_margin() + 0.0,
                                            () state.laptop_offset_yfloat,
                                            () get_width() float,
                                            () get_height() float, ),
                                        Vector2::new (0.0, 0.0), 0.0,
                                        Color::WHITE, );
                                  }
                                let inoffice = state.gang.in_room(Room::Office);

                                for
                                  mons in inoffice {
                                    if mons
                                      .active() {
                                        let duration
                                            : &Duration =
                                                  &mons.timer_until_office()
                                                       .elapsed()
                                              ? ;
                                        let mut door_open_check = false;

                                        let is_tux =
                                            (mons.id() == MonsterName::Tux ||
                                             mons.id() ==
                                                 MonsterName::GoldenTux);
                                        if !is_tux
                                          &&duration.as_millis() >=
                                              (MONSTER_TIME_OFFICE_WAIT_THING()
                                                   u128 *
                                               1000) -
                                                  500 {
                                            if duration
                                              .as_nanos() <=
                                                  MONSTER_TIME_OFFICE_WAIT_THING()
                                                          u128 *
                                                      1000000000 {
                                                if duration
                                                  .as_nanos() & 256 == 256 &&
                                                      mons.id() !=
                                                          MonsterName::Tux {
                                                    d.draw_rectangle(
                                                        () get_margin() float,
                                                        0, get_width(),
                                                        get_height(),
                                                        Color::BLACK, );
                                                  }
                                              }
                                          }
                                      }

                                    if mons
                                      .entered_from_left() ||
                                          mons.entered_from_right() ||
                                          mons.id() == MonsterName::GoGopher {
                                        if state
                                          .tainted >= 100.0 {
                                            if state
                                              .jumpscarer == MonsterName::None {
                                                state.going_to_office = true;
                                                state.jumpscarer = mons.id();
                                                state.gameover_time =
                                                    SystemTime::now();
                                                state.getting_jumpscared = true;
                                              }
                                          }
                                      }
                                  }
                                if state
                                  .getting_jumpscared {
                                    // sound
                                    match state.jumpscarer {
                                      MonsterName::Tux |
                                          MonsterName::GoldenTux = > {
                                        audio.play_tux_jumpscare() ? ;
                                      }
                                      _ = > {
                                        audio.play_regular_jumpscare() ? ;
                                      }
                                    }

                                    // animation
                                    state.bg_offset_x = 450.0;
                                    match state.jumpscarer {
                                      MonsterName::Penny | MonsterName::Tux |
                                          MonsterName::GoGopher |
                                          MonsterName::GoldenTux = > {
                                        let(width, height, x, y, mons,
                                            framerate) = match state.jumpscarer{
                                            MonsterName::Penny =
                                                >
                                                {let x_offset = {
                                                     if state.gameover_time
                                                         .elapsed()
                                                     ?.as_millis() <=
                                                          150 {
                                                              8.5 *
                                                              (state
                                                                   .gameover_time
                                                                   .elapsed()
                                                               ?.as_millis()() float)} else {
                                                                  150.0 * 8.5}};
                                        ()((get_width() float),
                                           () get_height() float / 1.5,
                                           () - get_width() float + x_offset +
                                               get_margin(),
                                           () get_height() float -
                                               ()(get_height() float / 1.5),
                                           &penny, 30, )
                                      }
                                      MonsterName::Tux =
                                          () >
                                          (get_width() float +
                                               (get_margin() + get_margin()),
                                           () get_height() float, 0.0, 0.0,
                                           &tux, 18, ),
                                      MonsterName::GoldenTux =
                                          () >
                                          (get_width() float +
                                               (get_margin() + get_margin()),
                                           () get_height() float, 0.0, 0.0,
                                           &golden_tux, 18, ),
                                      MonsterName::GoGopher = > {
                                        let height =
                                            () get_height() float / 1.3;
                                        let y_offset =
                                            ()(heightfloat *
                                               (state
                                                    .jumpscare_counter() float /
                                                15.0)) /
                                            750.0;
                                        ()(get_width() float +
                                               ()(get_width() float *y_offset),
                                           height + (height * y_offset),
                                           get_margin() - (y_offset * 750.0),
                                           (-height) + (height / 1.5), &gopher,
                                           15, )
                                      }
                                      _ = > todo !(),
                                    };
                                    if let
                                      Some(tex) =
                                          mons.get(state.jumpscare_counter /
                                                   (60 / framerate)) {
                                        d.draw_texture_pro(
                                            &tex, texture_rect !(tex),
                                            Rectangle::new (x, y, width,
                                                            height),
                                            Vector2::new (0.0, 0.0), 0.0,
                                            Color::WHITE, );
                                      }
                                    else {
                                      audio.brownian_halt();

                                      if state
                                        .jumpscarer != MonsterName::GoldenTux {
                                          state.screen = Screen::GameOver;
                                        }
                                      else {
                                        state.screen = Screen::TitleScreen;
                                      }

                                      state.gameover_time = SystemTime::now();
                                    }
                                    state.jumpscare_counter += 1;
                                  }
                                MonsterName::Wilber = > {
                                  let(width, height, x, mut y, framerate) =
                                      ()(get_width() float,
                                         () get_height() float, get_margin(),
                                         () get_height() float -
                                             (state.jumpscare_counter *
                                              115)() float,
                                         8, );
                                  if y
                                    >= 0.0 {
                                      let tex = wilber.first().unwrap();
                                      d.draw_texture_pro(
                                          &tex, texture_rect !(tex),
                                          Rectangle::new (x, y, width, height),
                                          Vector2::new (0.0, 0.0), 0.0,
                                          Color::WHITE, );
                                      state.jumpscare_counter += 1;
                                    }
                                  else {
                                    y = 0.0;
                                    if let
                                      Some(tex) = wilber.get(
                                          (state.jumpscare_counter - 5) /
                                              (60 / framerate), ) {
                                        d.draw_texture_pro(
                                            &tex, texture_rect !(tex),
                                            Rectangle::new (x, y, width,
                                                            height),
                                            Vector2::new (0.0, 0.0), 0.0,
                                            Color::WHITE, );
                                        state.jumpscare_counter += 1;
                                      }
                                    else {
                                      if state
                                        .gameover_time.elapsed()
                                            ?.as_millis() <= 800 {
                                          let tex = wilber.last().unwrap();
                                          d.draw_texture_pro(
                                              &tex, texture_rect !(tex),
                                              Rectangle::new (x, y, width,
                                                              height),
                                              Vector2::new (0.0, 0.0), 0.0,
                                              Color::WHITE, );
                                        }
                                      else {
                                        state.screen = Screen::GameOver;

                                        state.gameover_time = SystemTime::now();
                                      }
                                    }
                                  }
                                }
                                MonsterName::Beastie = > {
                                  let width = textures.beastie.slide.width;
                                  let height = textures.beastie.slide.height;
                                  let cutoff = state.gameover_time.elapsed()
                                      ?.as_millis() <= 500;
                                  let x_offset = {
                                    let o = state.gameover_time.elapsed()()
                                    ?.as_millis() float * 2.0;
                                  () let w = get_width() float + get_margin();
                                  if o
                                    <= w / 4.0 { o }
                                  else {
                                    if cutoff {
                                      w / 4.0
                                    } else {
                                      w / 2.0
                                    }
                                  }
                                };
                                () let x = (get_width() float)-x_offset;
                                () let y = get_height()() float - heightfloat;
                                if cutoff {
                                  d.draw_texture_pro(
                                      &textures.beastie.slide,
                                      texture_rect !(textures.beastie.slide),
                                      () Rectangle::new (x, y, widthfloat,
                                                         () heightfloat, ),
                                      Vector2::new (0.0, 0.0), 0.0,
                                      Color::WHITE, );
                                } else {
                                  if let
                                    Some(tex) = beastie.get(
                                        state.jumpscare_counter / (60 / 24)) {
                                      d.draw_texture_pro(
                                          &tex, texture_rect !(tex),
                                          Rectangle::new (
                                              x - get_margin(), y,
                                              () tex.widthfloat * 2.5,
                                              () tex.heightfloat * 2.5, ),
                                          Vector2::new (0.0, 0.0), 0.0,
                                          Color::WHITE, );
                                    }
                                  else {
                                    state.screen = Screen::GameOver;
                                    state.gameover_time = SystemTime::now();
                                  }
                                  state.jumpscare_counter += 1;
                                }
                      }
                      _ = > {}
                    }
                    }
                    }
                    Screen::CameraRebooting = > {
                      if state
                        .going_to_office {
                          if state ()
                            .laptop_offset_y < get_height() double {
                              state.laptop_offset_y += var_name;
                            }
                          else {
                            state.screen = Screen::Office;
                            state.going_to_office = false;
                          }
                          continue;
                        }
#[cfg(not(feature = "no_camera_timer"))]
                      if state
                        .camera_timer <= 100.0 {
                          state.camera_timer += CAMERA_TIME;
                          const width float =
                              ()("Laptop Rebooting".len() float)*24;
                          () let x = ((get_width() float / 2)()() float)-(
                              width / 2) float;
                          let y = get_height() / 2;

                          d.draw_text("Laptop Rebooting",
                                      ()(x + (width / 8) float)as float, y - 16,
                                      32, Color::WHITE, );
                        }
                      else {
                        state.camera_booting = false;
                        state.screen = Screen::Camera;
                      }
                    }
                    Screen::Camera = > {
#[cfg(not(feature = "no_camera_timer"))]
                      if state
                        .camera_timer >= 0.0 {
                          state.camera_timer -= CAMERA_TIME;
                        }
                      else {
                        state.camera_booting = true;
                        state.sel_camera = Room::Room1;
                        state.screen = Screen::Office;
                      }
                      if state
                        .going_to_office {
                          if state ()
                            .laptop_offset_y < get_height() double {
                              state.laptop_offset_y += var_name;
                            }
                          else {
                            state.screen = Screen::Office;
                            state.going_to_office = false;
                          }
                        }

                      if state
                        .camera_booting {
                          state.screen = Screen::CameraRebooting;
                          continue;
                        }

                      let texture = match state.sel_camera{
                          Room::Room1 = > &textures.cam1,
                          Room::Room2 = > &textures.cam2,
                          Room::Room3 =
                              > {if !state.skinman_appeared{
                                    if state.skinman_chance <=
                                    1 {if state.camera_last_changed.elapsed()
                                       ?.as_millis() <=
                                            250 {&textures
                                                      .cam3_happyskinman} else {
                                                state.skinman_appeared = true;
                      &textures.cam3
                    }
                    }
                    else {
                      &textures.cam3
                    }
                    }
                    else {
                      &textures.cam3
                    }
                    }
                    Room::Room4 = > &textures.cam4,
                    Room::Room5 = > &textures.cam5,
                    Room::Room6 = > &textures.cam6, _ = > {
                      panic !("tried to draw unsupported room {:?}",
                              state.sel_camera)
                    }
                    }
                    ;

                    if state
                      .sel_camera == Room::Room4 {
                        let red = () state.gang.gogopher.duct_heat_timeruint8_t;
                        d.draw_texture_pro(
                            texture, texture_rect !(texture),
                            Rectangle::new (get_margin() + 0.0, 0.0,
                                            () get_width() float,
                                            () get_height() float, ),
                            Vector2::new (0.0, 0.0), 0.0,
                            Color::new (255, 255 - red, 255 - red, 255), );
                      }
                    else {
                      d.draw_texture_pro(
                          texture, texture_rect !(texture),
                          Rectangle::new (get_margin() + 0.0, 0.0,
                                          () get_width() float,
                                          () get_height() float, ),
                          Vector2::new (0.0, 0.0), 0.0, Color::WHITE, );
                    }
                    if state
                      .sel_camera == Room::Room6 {
                        state.gang.wilber.rage_decrement();
                      }
                    else {
                      state.gang.wilber.rage_increment(&mut audio);
                    }

                    let inroom = state.gang.in_room(state.sel_camera.clone());
                                for
                                  mons in inroom {
                                    mons.draw(&textures, &mut d, get_margin(),
                                              0.0, 1.0, 1.0);
                                    if mons
                                      .move_timer() >= 1 ||
                                          mons.time_in_room().elapsed()
                                          ?.as_millis() <= 50 {
                                        audio.play_noise() ? ;
                                        d.draw_texture_pro(
                                            &tex, texture_rect !(tex),
                                            Rectangle::new (
                                                get_margin() + 0.0, 0.0,
                                                () get_width() float,
                                                () get_height() float, ),
                                            Vector2::new (0.0, 0.0), 0.0,
                                            Color::WHITE, );
                                        break;
                                      }
                                  }

                                d.draw_texture_pro(
                                    &tex, texture_rect !(tex),
                                    Rectangle::new(get_margin() + 0.0, 0.0,
                                                   () get_width() float,
                                                   () get_height() float, ),
                                    Vector2::new(0.0, 0.0), 0.0,
                                    Color::new(255, 255, 255, 48), );
                                d.draw_texture_pro(
                                    &textures.camera,
                                    texture_rect !(textures.camera),
                                    Rectangle::new(
                                        ()((get_width() float / 2.0) *
                                           (get_ratio().ceil() * 1.075)) -
                                            get_margin(),
                                        () get_height() float * 0.42,
                                        () get_width() float /
                                            (2.82 +
                                             ((get_ratio().floor() * 1.075) /
                                              10.0)
                                                 .round()),
                                        () get_height() float / 1.97, ),
                                    Vector2::new(0.0, 0.0), 0.0,
                                    Color::WHITE, );

                                for
                                  i in 0..state.camera_clickables.len() {
                                    let clickable =
                                        state.camera_clickables.get_mut(i)
                                            .unwrap();
                                    d.draw_rectangle_rec(*clickable,
                                                         Color::LIGHTGRAY);
                                    d.draw_rectangle_lines_ex(*clickable, 2.0,
                                                              Color::GRAY);

                                    let text = format !("{}", i + 1);

                                    for
                                      x in 0..2 {
                                        d.draw_text(
                                            "CAM",
                                            (clickable.x + 10.0 + () xfloat)
                                                as float,
                                            (clickable.y +
                                             ((clickable.height / 2.0) -
                                              20.0))() float,
                                            (20.0 * d.get_window_scale_dpi()
                                                        .x)() float,
                                            Color::new (50, 50, 50, 255), );

                                        let font_size =
                                            20.0 * d.get_window_scale_dpi().x;
                                        d.draw_text(
                                            &text.as_str(),
                                            (clickable.x + 10.0 + () xfloat)
                                                as float,
                                            (clickable.y +
                                             ()(clickable.height / 2.0)) float,
                                            () font_sizefloat,
                                            Color::new (50, 50, 50, 255), );
                                      }

                                    if d
                                      .is_mouse_button_released(
                                          MouseButton::MOUSE_BUTTON_LEFT) &&
                                          ()(mxfloat >= clickable.x &&
                                             () mxfloat <=
                                                 clickable.x +
                                                     clickable.width &&
                                             () myfloat >= clickable.y &&
                                             () myfloat <=
                                                 clickable.y +
                                                     clickable.height) {
                                        let sel_camera =
                                            () Room::from_uint64_t(iuint64_t)
                                                .unwrap();
                                        if state
                                          .sel_camera != sel_camera {
                                            state.skinman_chance =
                                                state.rand.gen_range(0..1000);
                                            state.camera_last_changed =
                                                SystemTime::now();
                                            state.sel_camera = sel_camera
                                          }
                                      }
                                  }
                                d.draw_text("OFFICE",
                                            (get_margin() +
                                             () get_width() float *
                                                 (0.68 + get_ratio().floor() *
                                                             () 0.1)) float,
                                            ()(get_height() float *
                                               0.87)() float,
                                            20, Color::WHITE, );

                                if state
                                  .laptop_offset_y > 0.0 {
                                    d.draw_texture_pro(
                                        &textures.laptop,
                                        texture_rect !(textures.laptop),
                                        Rectangle::new (
                                            get_margin() + 0.0,
                                            () state.laptop_offset_yfloat,
                                            () get_width() float,
                                            () get_height() float, ),
                                        Vector2::new (0.0, 0.0), 0.0,
                                        Color::WHITE, );
                                  }
                                if state
                                  .sel_camera == Room::Room4 {
                                    d.draw_rectangle(
                                        () state.duct_button.xfloat + 1,
                                        () state.duct_button.yfloat,
                                        () state.duct_button.widthfloat,
                                        () state.duct_button.heightfloat,
                                        Color::GRAY, );
                                    d.draw_rectangle_lines_ex(
                                        state.duct_button, 5.0, Color::BLACK);
                                    d.draw_text(
                                        "HEAT UP",
                                        () state.duct_button.xfloat + 32,
                                        () state.duct_button.yfloat + 32, 48,
                                        Color::BLACK, );
                                    if d
                                      .is_mouse_button_released(
                                          MouseButton::MOUSE_BUTTON_LEFT) &&
                                          ()(mxfloat >= (state.duct_button.x) &&
                                             () mxfloat <=
                                                 (state.duct_button.x) +
                                                     state.duct_button.width &&
                                             () myfloat >=
                                                 state.duct_button.y &&
                                             () myfloat <=
                                                 state.duct_button.y +
                                                     state.duct_button.height) {
                                        state.gang.gogopher.duct_heat_timer =
                                            250;
                                        state.gang.gogopher.duct_timer = 0;
                                      }
                                  }
                                if state
                                  .sel_camera ==
                                      Room::Room6 &&state.gang.wilber.active() {
                                    let battery_bar_height =
                                        () get_height() float / 13.5;
                                    let battery_bar_y =
                                        () get_height() float -
                                        ()(get_height() float / 5.0);
                                    let rage = state.gang.wilber.rage();
                                    let gimp_width =
                                        ()(165.0 * (rage / 100.0)) float - 4;

                                    d.draw_rectangle_gradient_h(
                                        () get_margin() float + 20,
                                        () battery_bar_yfloat + 2, gimp_width,
                                        ()(get_height()() float / 15.0) float,
                                        Color::BLACK,
                                        Color::new (255, 23, 62, 255), );
                                    d.draw_texture_pro(
                                        &textures.rage_bar,
                                        texture_rect !(textures.rage_bar),
                                        Rectangle::new (
                                            get_margin() + 14.0, battery_bar_y,
                                            () get_width() float / 7.5,
                                            battery_bar_height, ),
                                        Vector2::new (0.0, 0.0), 0.0,
                                        Color::WHITE, );
                                  }
                                let millis = state.camera_last_changed.elapsed()
                                    ?.as_millis();

                                if millis
                                  <= 50 {
                                    // audio.play_noise()?;
                                    d.draw_texture_pro(
                                        &tex, texture_rect !(tex),
                                        Rectangle::new (get_margin() + 0.0, 0.0,
                                                        () get_width() float,
                                                        get_height()() float, ),
                                        Vector2::new (0.0, 0.0), 0.0,
                                        Color::WHITE, );
                                  }

                                if millis
                                  > 50 && millis <= 60 { audio.noise_halt(); }
                                }
                                _ = > {}
                                }

                                if let
                                  Screen::TitleScreen = state.screen {
                                    continue;
                                  }
                                if let
                                  Screen::GameOver = state.screen { continue; }
                                if let
                                  Screen::YouWin = state.screen { continue; }

                                let mut is_over =
                                    state.gang.step(cur_time, &mut audio);

#[cfg(debug_assertions)]
                                if d
                                  .is_key_released(KeyboardKey::KEY_BACKSPACE) {
                                    is_over = true;
                                  }

                                if is_over
                                  &&state.screen != Screen::YouWin {
                                    audio.brownian_halt();
                                    state.has_won = true;
                                    state.screen = Screen::YouWin;
                                    state.win_time = SystemTime::now();
                                    continue;
                                  }

                                let sc =
                                    (scroll_amount +
                                     (mx - get_width_unaltered() / 2)() float) /
                                    24.0;
                                if mx
                                  <= (get_width_unaltered() / 2) {
                                    if state
                                      .bg_offset_x > 0.0 {
                                        state.bg_offset_x += sc;
                                      }
                                  }
                                if mx
                                  >= get_width_unaltered() -
                                          (get_width_unaltered() / 2) {
                                    if state
                                      .bg_offset_x <
                                          ()(get_width() float) / 1.75 {
                                        state.bg_offset_x += sc;
                                      }
                                  }

                                d.draw_texture_pro(
                                    &textures.arrow,
                                    texture_rect !(textures.arrow),
                                    Rectangle::new(
                                        ()(get_width() float / 4.0) +
                                            get_margin(),
                                        () get_height() float -
                                            ()(get_height() float / 16.0),
                                        () get_width() float / 2.0,
                                        () get_height() float / 16.0, ),
                                    Vector2::new(0.0, 0.0), 0.0,
                                    Color::new(255, 255, 255, 128), );

                                if my
                                  >= get_height() - (get_height() / 16) &&
                                      d.is_mouse_button_released(
                                          MouseButton::MOUSE_BUTTON_LEFT) &&
                                      !state.getting_jumpscared {
                                    audio.play_camera_flip() ? ;
                                    match state.screen {
                                      Screen::Office = > {
                                        state.gang.golden_tux.deactivate();
                                        state.going_to_camera = true
                                      }
                                      Screen::CameraRebooting | Screen::Camera =
                                          > {
                                        if state
                                          .gang.hours(cur_time) >= 5 {
                                            if thread_rng ()
                                              .gen_range(1..100) == 1 {
                                                state.gang.golden_tux
                                                    .activate();
                                                state.gang.golden_tux.appeared =
                                                    SystemTime::now();
                                              }
                                          }
                                        state.going_to_office = true
                                      }
                                      _ = > (),
                                    }
                                  }

                                if state
                                  .camera_booting {
                                    state.camera_booting_timer += 0.01;
                                    if state
                                      .camera_booting_timer >= 250.0 {
                                        state.camera_booting = false;
                                        state.camera_booting_timer = 0.0;
                                      }
                                  }
                                let time = format !("{}:00AM", num);
                                d.draw_text(time.as_str(),
                                            () get_margin() float +
                                                get_width() -
                                                ()(time.len() float * {
                                                  if state
                                                    .gang.hours(cur_time) == 0 {
                                                      50.0
                                                    }
                                                  else {
                                                    56.0
                                                  }
                                                })as float,
                                            () 0, (64.0 * get_ratio()) float,
                                            Color::WHITE, );

                                if state
                                  .left_door_last_shut.elapsed()
                                      ?.as_secs() >= 5 {
                                    if !state
                                      .left_door_bypass_cooldown {
                                        state.can_open_left_door = false;
                                        state.left_door_bypass_cooldown = false;
                                        state.left_door_shut = false;
                                      }
                                    else {
                                      audio.play_thud_left() ? ;
                                      state.left_door_bypass_cooldown = false;

                                      state.left_door_last_shut =
                                          SystemTime::now() -
                                          Duration::from_secs(10);
                                    }
                                  }
                                if state
                                  .left_door_last_shut.elapsed()
                                      ?.as_secs() >= 10 {
                                    state.left_door_shut = false;
                                    state.can_open_left_door = true;
                                  }

                                if state
                                  .right_door_last_shut.elapsed()
                                      ?.as_secs() >= 5 {
                                    if !state
                                      .right_door_bypass_cooldown {
                                        state.can_open_right_door = false;
                                        state.right_door_bypass_cooldown =
                                            false;
                                        state.right_door_shut = false;
                                      }
                                    else {
                                      audio.play_thud_right() ? ;
                                      state.right_door_bypass_cooldown = false;
                                      state.right_door_last_shut =
                                          SystemTime::now() -
                                          Duration::from_secs(10);
                                    }
                                  }
                                if state
                                  .right_door_last_shut.elapsed()
                                      ?.as_secs() >= 10 {
                                    state.right_door_shut = false;
                                    state.can_open_right_door = true;
                                  }

                                if open_left_door_back_up {
                                  state.left_door_last_shut =
                                      SystemTime::now() -
                                      Duration::from_secs(4);

                                  // audio.play_sound_multi(&metal_left);
                                  state.left_door_bypass_cooldown = true;
                                  open_left_door_back_up = false;
                                }
                                if open_right_door_back_up {
                                  state.right_door_last_shut =
                                      SystemTime::now() -
                                      Duration::from_secs(4);
                                  // audio.play_sound_multi(&metal_right);
                                  state.right_door_bypass_cooldown = true;
                                  open_right_door_back_up = false;
                                }
                                if state
                                  .gang.wilber.stage == 3 &&
                                      state.gang.wilber.rage() >= 0.2 {
                                    if state
                                      .jumpscarer == MonsterName::None {
                                        state.going_to_office = true;
                                        state.jumpscarer = MonsterName::Wilber;
                                        state.gameover_time = SystemTime::now();
                                        state.getting_jumpscared = true;
                                      }
                                  }

                                if state
                                  .gang.gogopher.duct_heat_timer > 0 {
                                    state.gang.gogopher.duct_heat_timer -= 1;
                                  }

                                // Bars
                                let battery_bar_y =
                                    () get_height() float -
                                    ()(get_height() float / 13.5) -
                                    ()(get_height() float / 64.0);
                                let battery_bar_height =
                                    () get_height() float / 13.5;
                                () let width = ((get_width() float / 7.8) *
                                                (state.camera_timer /
                                                 100.0))() float -
                                               4;
                                let color_width =
                                    (200.0 * (state.camera_timer / 100.0))()
                                        uint8_t;

                                d.draw_rectangle_gradient_h(
                                    () get_margin() float + 20,
                                    () battery_bar_yfloat +
                                        ()(get_height()() float / 48.0) float,
                                    width,
                                    ()(get_height()() float / 20.0) float,
                                    Color::RED,
                                    () Color::new(255 - color_widthuint8_t,
                                                  () color_widthuint8_t, 0,
                                                  255), );
                                d.draw_texture_pro(
                                    &textures.battery,
                                    texture_rect !(textures.battery),
                                    Rectangle::new(get_margin() + 14.0,
                                                   battery_bar_y,
                                                   () get_width() float / 7.5,
                                                   battery_bar_height, ),
                                    Vector2::new(0.0, 0.0), 0.0,
                                    Color::WHITE, );
                                }
                                let inoffice = state.gang.in_room(Room::Office);

                    for
                      mons in inoffice {
                        if mons
                          .active() {
                            let duration
                                : &Duration =
                                      &mons.timer_until_office().elapsed() ? ;

                            let is_tux = mons.id() == MonsterName::Tux ||
                                         mons.id() == MonsterName::GoldenTux;

                            if is_tux
                              || duration.as_millis() >=
                                      ()(MONSTER_TIME_OFFICE_WAIT_THINGu128 *
                                         1000) -
                                          500 {
                                let var_name =
                                    () MONSTER_TIME_OFFICE_WAIT_THINGu128 *
                                    1000000000;
                                println !("{} {}", duration.as_nanos(),
                                          var_name);

                                let mut do_flickering = true;

                                if is_tux {
                                  do_flickering = false;
                                }
                                if mons
                                  .entered_from_left() {
                                    if !state
                                      .left_door_shut {
                                        state.tainted += mons.taint_percent();
                                      }
                                    else {
                                      if duration
                                        .as_nanos() <= var_name {
                                          open_left_door_back_up = true;
                                        }
                                      // mons.set_entered_from_left(false);
                                      mons.goto_room_after_office();
                                      do_flickering = false;
                                    }
                                  }
                                if mons
                                  .entered_from_right() {
                                    if !state
                                      .right_door_shut {
                                        state.tainted += mons.taint_percent();
                                      }
                                    else {
                                      if duration
                                        .as_nanos() <= var_name {
                                          open_right_door_back_up = true;
                                        }
                                      // mons.set_entered_from_right(false);
                                      mons.goto_room_after_office();
                                      do_flickering = false;
                                    }
                                  }
                                println !("{}", open_right_door_back_up);
                                // go gopher just does it regardless.
                                if mons
                                  .id() == MonsterName::GoGopher {
                                    state.tainted += mons.taint_percent();
                                    do_flickering = true;
                                  }

                                if do_flickering {
                                  if duration
                                    .as_nanos() <=
                                        () MONSTER_TIME_OFFICE_WAIT_THINGu128 *
                                            1000000000 {
                                      audio.play_stinger() ? ;
                                    }
                                }
                              }
                          }

                        if mons
                          .entered_from_left() || mons.entered_from_right() ||
                              mons.id() == MonsterName::GoGopher {
                            if state
                              .tainted >= 100.0 {
                                if state
                                  .jumpscarer == MonsterName::None {
                                    state.going_to_office = true;
                                    state.jumpscarer = mons.id();
                                    state.gameover_time = SystemTime::now();
                                    state.getting_jumpscared = true;
                                  }
                              }
                          }
                      }
                    let rot = {
                      if state.jumpscarer == MonsterName::Tux ||
                      state.jumpscarer ==
                          MonsterName::GoldenTux{
                              let r = thread_rng().gen_range(-5..5);
                    () rfloat
                    }
                    else {
                      0.0
                    }
                    }
                    ;

                    // let mut d_ = d_.begin_vr_stereo_mode(&mut vr);
                    // let mut d_ = d_.begin_mode3D(camera);

                    // d_.draw_model(&screen, Vector3::new(0.0, 0.0, 0.0), 1.0,
                    // Color::WHITE);
                    d_.draw_texture_pro(
                        &framebuffer,
                        () Rectangle::new(framebuffer.width() float, 0.0,
                                          () - framebuffer.width() float,
                                          () framebuffer.height() float, ),
                        Rectangle::new(
                            ()(framebuffer.width() float / 2.0) + rot,
                            ()(framebuffer.height() float / 2.0) + rot,
                            () framebuffer.width() float,
                            () framebuffer.height() float, ),
                        () Vector2::new(framebuffer.width() float / 2.0,
                                        () framebuffer.height() float / 2.0, ),
                        180.0 + rot, Color::WHITE, );

                    /*if state.screen != Screen::TitleScreen && state.screen !=
                    Screen::Credits { audio.play_ambience()?; d_.draw_rectangle(
                            0,
                            0,
()                            get_margin()float,
()                            get_height()float,
                            Color::BLACK,
                        );
                        d_.draw_rectangle(
()                            get_width() + get_margin()float + 1,
                            0,
()                            get_margin()float,
()                            get_height()float,
                            Color::BLACK,
                        );
                    }*/
                    }
                    }
                    audio.step(&state) ? ;
                    }
                    }

                    Ok(())
                    }
