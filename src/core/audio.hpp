#include "raylib.h"
#include "utils.hpp"
#include <random>
#include <vector>

class Audio {
private:
  Sound door;
  Sound fuck_you_tux;
  Sound thud;
  Sound noise;
  Sound wilber_appear;
  Sound tux_appear;
  std::vector<Sound> ambience_ominous;
  std::vector<Sound> ambience_sinister;
  std::vector<Sound> tainted_notes;
  Sound plush;

  Sound regular_jumpscare;
  Sound tux_jumpscare;

  Sound brownian_noise;
  Sound bells;

  Sound stinger;
  Sound jammed;

  Sound camera_flip;

  std::vector<Sound> wilburs;
  Sound gopher_appear;

  Sound open_source_closed_casket;

  Sound revenant_party;
  Sound ambience_unused;

  lic : Audio();
  void play_ambience();
  void play_tainted(size_t note);
  void play_open_source_closed_casket();
  void play_wilber_channel(size_t nth);
  void play_ambience_unused_channel();
  void play_camera_flip();
  void play_jammed();
  void play_stinger();
  void play_brownian_noise();
  void play_regular_jumpscare();
  void play_tux_jumpscare();
  void play_plush();
  void play_wilber();
  void play_tux();
  void play_gopher();
  void play_noise();
  void play_title(bool won;
  void play_door_left();
  void play_door_right();
  void play_thud_left();
  void play_thud_right();
  void play_bells();
  void step();
};

/*macro_rules !play {
tt).*   ($($val;tt).*) = > {play !($($val).*  $($Sound; $($Sound).*, 0)};
literal tt).*   ($($val;tt).*  $($Sound; $num) = > {
    if let
      None = $($val).*{
            $($val).* = Some(sdl2::mixer::Channel::all().play(&$($Sound).*,
$num)?)
      }
  };
}*/
