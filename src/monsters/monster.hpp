#include <cstdint>
#include <format>
#include <optional>
#include <raylib.h>
#include <stdexcept>
#include <string>

#include "../core/screen.hpp"
#include "../game/enums.hpp"
#include "../textures/textures.hpp"
#include "../utils.hpp"

#define PENNY_START true
#define BEASTIE_START true
#define WILBER_START false
#define GO_GOPHER_START false
#define TUX_START false
#define NOLOK_START false
#define GOLDEN_TUX_START false
#define MONSTER_TIME_OFFICE_WAIT_THING 5
#define DEFAULT_AI_LEVEL 2

enum class MonsterName {
  Penny,
  Beastie,
  Wilber,
  GoGopher,
  Tux,
  Nolok,
  GoldenTux,
  None,
};

class Monster {
public:
  MonsterName id;
  Room room;
  Room next_room;
  uint8_t ai_level;
  bool entered_from_left;
  bool entered_from_right;
  SystemTime last_scared_at;
  uint8_t move_timer;
  SystemTime timer_until_office;
  SystemTime time_in_room;
  bool move_after_timer;
  int8_t progress_to_hallway;
  bool active;

  // void activate() { this->active = true; };
  // void deactivate() { this->active = false; };

  void reset_time_in_room();

  void begin_move_timer() { this->move_timer = 5; }

  bool _end_move_timer() {
    if (this->move_timer >= 1) {
      this->move_timer = this->move_timer - 1;
      if (this->move_timer == 0) {
        return true;
      }
    }
    return false;
  }
  void end_move_timer() {
    if (this->_end_move_timer()) {
      this->reset_time_in_room();
      this->next();
    };
  }

  virtual std::optional<Texture2D> get_texture(Textures &textures) {
    return {};
  };

  void _draw(Textures &textures, float x_offset, float y_offset,
             float width_offset, float height_offset) {
    auto t = this->get_texture(textures);
    if (t.has_value()) {
      DrawTexturePro(t, TEXTURE_RECT(t),
                     (Rectangle){
                         x_offset,
                         y_offset,
                         (float)SCREEN.width() * width_offset,
                         SCREEN.height() * height_offset,
                     },
                     (Vector2){0.0, 0.0}, 0.0, WHITE);
    }
  }
  void draw(Textures &textures, float x_offset, float y_offset,
            float width_offset, float height_offset) {
    this->_draw(textures, x_offset, y_offset, width_offset, height_offset);
  }

  std::string name() { return std::format("{}", this->id); }
  float taint_percent() { return 0.2; }

  bool _try_move() {
    auto chance = GetRandomValue(0, 20);
    // if any of them are in the hallways, have them move in.
    if (this->room == Room::Room3 || this->room == Room::Room5) {
      this->next();
    } else {
      if (chance <= this->ai_level) {
        this->begin_move_timer();
        return true;
      }
    }
    return false;
  }
  bool try_move() { return this->_try_move(); }

  void _step() { this->end_move_timer(); }
  void step() { this->_step(); }

  Room _next() { return room_next(this->room); }
  void next() { this->room = _next(); }

  virtual Room room_after_office() { return room_random(); };

  Room goto_room_after_office() {
    this->last_scared_at = SystemTime();
    this->progress_to_hallway = 0;
    this->room = this->room_after_office();
    return this->room_after_office();
  }
};

class HallwayMonster : public Monster {
  virtual Room hallway_room();
  virtual void set_door();

  void end_move_timer() {
    if (this->_end_move_timer()) {
      this->reset_time_in_room();
      Monster::next();
    }
  }
  void next() {
    switch (this->progress_to_hallway) {
    case 0:
      this->progress_to_hallway += 1;
      this->move_after_timer = false;
      this->reset_time_in_room();
      break;
    case 1:
      this->progress_to_hallway += 1;
      this->move_after_timer = false;
      this->begin_move_timer();
      this->reset_time_in_room();
      break;
    case 2:
      this->reset_time_in_room();
      this->move_after_timer = true;

      this->room = this->_next();
      if (this->room == Room::Office) {
        this->timer_until_office = SystemTime();
        this->set_door();
      }
      this->progress_to_hallway = 0;
      break;
    default:
      throw std::runtime_error("Invalid progress to hallway value");
    }
  }
};

class Beastie : public HallwayMonster {
public:
  bool door_shut;
  Beastie();
  std::optional<Texture2D> get_texture(Textures &textures) override;
  Room room_after_office() override;
  Room hallway_room() override;
  void set_door() override;
};

class GoldenTux : public Monster {
public:
  SystemTime appeared;
};

class GoGopher : public Monster {
public:
  uint16_t duct_timer;
  uint16_t duct_heat_timer;
  SystemTime appeared;
};

class Nolok : public Monster {};

class Penny : public HallwayMonster {
public:
  bool door_shut;
};

class Tux : public Monster {
public:
  SystemTime time_since_entered_hallway;
  SystemTime time_since_last_attempt;
  bool can_move;
  SystemTime moved_to_hallway_at;
  std::optional<SystemTime> checked_camera;
};

class Wilber : public Monster {
public:
  float rage;
  uint8_t stage;
  std::optional<SystemTime> time_since_appeared;
};

class Gang {
public:
  Penny penny;
  Beastie beastie;
  Wilber wilber;
  GoGopher gogopher;
  Tux tux;
  Nolok nolok;
  GoldenTux golden_tux;

  SystemTime since_last_move;
  bool moved;
  bool one_am_checked;
  bool two_am_checked;
  bool three_am_checked;
  bool four_am_checked;
  bool five_am_checked;
  bool tux_moved;
  uint64_t hour_offset;
};