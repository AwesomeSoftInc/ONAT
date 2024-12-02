#include "monster.hpp"
#include "rs_systemtime.hpp"

Beastie::Beastie() {
  this->id = MonsterName::Beastie;
  this->room = Room::Room2;
  this->ai_level = DEFAULT_AI_LEVEL;
  this->active = BEASTIE_START;
  this->entered_from_left = false;
  this->entered_from_right = false;
  this->door_shut = false;
  this->progress_to_hallway = 0;
  this->last_scared_at = SystemTime();
  this->move_timer = 0;
  this->time_in_room = SystemTime();
  this->move_after_timer = true;
}

std::optional<Texture2D> Beastie::get_texture(Textures &textures) {
  if (this->active) {
    switch (this->room) {
    case Room::Room2:
      switch (this->progress_to_hallway) {
      case 0:
        return textures.beastie.cam2stage1;
      case 1:
        return textures.beastie.cam2stage2;
      }
      break;
    case Room::Room5:
      switch (this->progress_to_hallway) {
      case 0:
        return textures.beastie.cam5stage1;
      case 1:
        return textures.beastie.cam5stage2;
      }
      break;
    case Room::Office:
      break;
    default:
      return {};
    }
  } else {
    return {};
  }
}

Room Beastie::hallway_room() { return Room::Room5; }

void Beastie::set_door() { this->entered_from_right = true; }