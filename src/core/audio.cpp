#include "audio.hpp"
#include "state.hpp"
#include "utils.hpp"
#include <cstddef>
#include <format>
#include <raylib.h>
#include <vector>

Audio::Audio() {
  this->door = LoadSound("./audio/door.mp3");
  this->fuck_you_tux = LoadSound("./audio/fuck_you_tux.mp3");
  this->thud = LoadSound("./audio/thud.mp3");
  this->noise = LoadSound("./audio/noise.mp3");
  this->wilber_appear = LoadSound("./audio/wilber_appear.mp3");
  this->tux_appear = LoadSound("./audio/tux_appears.mp3");
  this->gopher_appear = LoadSound("./audio/gopher.mp3");
  this->open_source_closed_casket =
      LoadSound("./audio/open_source_closed_casket.mp3");
  this->plush = LoadSound("./audio/plush.mp3");

  this->regular_jumpscare = LoadSound("./audio/regular_jumpscare.mp3");
  this->tux_jumpscare = LoadSound("./audio/tux_jumpscare.mp3");
  this->revenant_party = LoadSound("./audio/revenant_party.mp3");
  this->brownian_noise = LoadSound("./audio/brownian_noise.mp3");
  this->ambience_ominous = std::vector<Sound>();
  ambience_ominous.push_back(LoadSound("./audio/ominous_ambient_1.mp3"));
  ambience_ominous.push_back(LoadSound("./audio/ominous_ambient_3.mp3"));
  this->ambience_unused = LoadSound("./audio/ominous_ambient_2.mp3");

  this->ambience_sinister = std::vector<Sound>();
  ambience_sinister.push_back(LoadSound("./audio/sinister_ambient_1.mp3"));
  ambience_sinister.push_back(LoadSound("./audio/sinister_ambient_2.mp3"));
  ambience_sinister.push_back(LoadSound("./audio/sinister_ambient_3.mp3"));

  this->bells = LoadSound("./audio/bells.mp3");

  this->stinger = LoadSound("./audio/stinger.mp3");
  this->jammed = LoadSound("./audio/jammed.mp3");
  this->camera_flip = LoadSound("./audio/camera_flip.mp3");

  this->wilburs = std::vector<Sound>();
  wilburs.push_back(LoadSound("./audio/wilbur1.mp3"));
  wilburs.push_back(LoadSound("./audio/wilbur3.mp3"));
  wilburs.push_back(LoadSound("./audio/wilbur2.mp3"));

  this->tainted_notes = std::vector<Sound>();
  for (auto n : range(1, 37)) {
    tainted_notes.push_back(
        LoadSound(std::format("./audio/tainted/note{}.mp3", n).c_str()));
  }
}

void Audio::play_ambience() {

  auto chance_to_play = GetRandomValue(1, 1000);
  if (chance_to_play <= 1) {
    auto chance = GetRandomValue(1, 2000);
    std::vector<Sound> *vec;
    if (chance <= 1) {
      vec = &this->ambience_ominous;
    } else {
      vec = &this->ambience_sinister;
    }
    chance = GetRandomValue(1, vec->size());
    auto snd = vec->at(chance - 1);

    PlaySound(snd);
  }
}

void Audio::play_tainted(size_t note) {
  if (note >= 36) {
    note = 0;
  }
  PlaySound(this->tainted_notes.at(note));
}
void Audio::play_open_source_closed_casket() {
  PlaySound(this->open_source_closed_casket);
}
void Audio::play_wilber_channel(size_t nth) {
  PlaySound(this->wilburs.at(nth));
}
void Audio::play_ambience_unused_channel() { PlaySound(this->ambience_unused); }
void Audio::play_camera_flip() { PlaySound(this->camera_flip); }
void Audio::play_jammed() { PlaySound(this->jammed); }
void Audio::play_stinger() { PlaySound(this->stinger); }
void Audio::play_brownian_noise() { PlaySound(this->brownian_noise); }

void Audio::play_regular_jumpscare() { PlaySound(this->regular_jumpscare); }

void Audio::play_tux_jumpscare() { PlaySound(this->tux_jumpscare); }
void Audio::play_plush() { PlaySound(this->plush); }
void Audio::play_wilber() { PlaySound(this->wilber_appear); }
void Audio::play_tux() { PlaySound(this->tux_appear); }
void Audio::play_gopher() { PlaySound(this->gopher_appear); }
void Audio::play_noise() { PlaySound(this->noise); }
void Audio::play_title(bool won) {
  if (won) {
    PlaySound(this->revenant_party);
  } else {
    PlaySound(this->fuck_you_tux);
  }
}
void Audio::play_door_left() { PlaySound(this->door); }
void Audio::play_door_right() { PlaySound(this->door); }
void Audio::play_thud_left() { PlaySound(this->thud); }
void Audio::play_thud_right() { PlaySound(this->thud); }
void Audio::play_bells() { PlaySound(this->bells); }
void Audio::step(State &state) {
  auto var_name = state.bg_offset_x / 3.0;
  auto left = 191.0 - var_name;
  if left
    <= 64.0 { left = 64.0; }
  if left
    >= 191.0 { left = 191.0; }
  auto right = var_name;
  if right
    <= 64.0 { right = 64.0; }
  if right
    >= 191.0 { right = 191.0; }
  () auto left = leftuint8_t;
  () auto right = rightuint8_t;
  if
    auto Some(ch) = this -> left_channel_door {
      ch.set_panning(left, 0);
      if !ch
        .is_playing() { this->left_channel_door = None; }
    }
  if
    auto Some(ch) = this -> right_channel_door {
      ch.set_panning(0, right);
      if !ch
        .is_playing() { this->right_channel_door = None; }
    }
  if
    auto Some(ch) = this -> left_channel_thud {
      ch.set_panning(left, 0);
      if !ch
        .is_playing() { this->left_channel_thud = None; }
    }
  if
    auto Some(ch) = this -> right_channel_thud {
      ch.set_panning(0, right);
      if !ch
        .is_playing() { this->right_channel_thud = None; }
    }
  if
    auto Some(ch) = this -> noise_channel {
      ch.set_volume(100);
      if !ch
        .is_playing() { this->noise_channel = None; }
    }
  if
    auto Some(ch) = this -> monster_appear_channel {
      if !ch
        .is_playing() { this->monster_appear_channel = None; }
    }
  if
    auto Some(ch) = this -> bells_channel {
      if !ch
        .is_playing() { this->bells_channel = None; }
    }
  if
    auto Some(ch) = this -> ambient_channel {
      if !ch
        .is_playing() { this->ambient_channel = None; }
    }
  if
    auto Some(ch) = this -> open_source_channel {
      if !ch
        .is_playing() { this->open_source_channel = None; }
    }
  if
    auto Some(ch) = this -> jammed_channel {
      if !ch
        .is_playing() { this->jammed_channel = None; }
    }
  if
    auto Some(ch) = this -> stinger_channel {
      if !ch
        .is_playing() { this->stinger_channel = None; }
    }
  if
    auto Some(ch) = this -> plush_channel {
      if !ch
        .is_playing() { this->plush_channel = None; }
    }
  if
    auto Some(ch) = this -> jumpscare_channel {
      if !ch
        .is_playing() { this->jumpscare_channel = None; }
    }
  if
    auto Some(ch) = this -> wilber_channel {
      if !ch
        .is_playing() { this->wilber_channel = None; }
    }
  if
    auto Some(ch) = this -> title_channel {
      auto volume = {
        if state.going_to_office_from_title{
            ()(100.0 - (state.title_clicked.elapsed().as_millis() float /
                        ()(4000.0 / 100.0))) float} else {100}
      };
      if volume
        >= 100 { volume = 100; }
      ch.set_volume(volume);
      if !ch
        .is_playing() {
          ch.set_volume(100);
          this->title_channel = None;
        }
    }
}
