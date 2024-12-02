#include "rs_systemtime.hpp"
#include "utils.hpp"
#include <raylib.h>
#include <stdint.h>
#include <vector>

class State {
  Screen screen;
  float bg_offset_x;
  double laptop_offset_y;
  std::vector<Rectangle> camera_clickables;
  Rectangle plush_clickable;
  std::vector<Rectangle> door_buttons;
  Rectangle duct_button;
  Room sel_camera;
  SystemTime timer;

  SystemTime ingame_time;
  Gang gang;
  float tainted;
  float tainted_cache;

  float camera_timer;
  bool camera_booting;
  float camera_booting_timer;

  SystemTime gameover_time;
  SystemTime win_time;

  SystemTime camera_last_changed;

  bool can_open_left_door;
  bool can_open_right_door;

  float left_door_anim_timer;
  float right_door_anim_timer;

  bool left_door_shut;
  bool right_door_shut;

  bool left_door_bypass_cooldown;
  bool right_door_bypass_cooldown;
  SystemTime left_door_last_shut;
  SystemTime right_door_last_shut;

  double duct_heat_timer;

  ThreadRng rand;
  uint32_t skinman_chance;
  bool skinman_appeared;

  bool going_to_office;
  bool going_to_camera;
  bool going_to_office_from_title;
  SystemTime title_clicked;
  bool going_to_youwin;

  bool wilber_snd_played;
  bool tux_snd_played;
  bool gopher_snd_played;

  size_t jumpscare_counter;
  bool getting_jumpscared;
  MonsterName jumpscarer;
  bool has_won;
}
