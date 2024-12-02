#include "monster.hpp"
#include "rs_systemtime.hpp"

Gang::Gang() {
  this->penny = Penny();
  this->beastie = Beastie();
  this->wilber = Wilber();
  this->gogopher = GoGopher();
  this->tux = Tux();
  this->nolok = Nolok();
  this->golden_tux = GoldenTux();
  this->since_last_move = SystemTime();
  this->moved = true;
  this->one_am_checked = false;
  this->two_am_checked = false;
  this->three_am_checked = false;
  this->four_am_checked = false;
  this->five_am_checked = false;
  this->tux_moved = false;
  this->hour_offset = 0;
}

fn hours(&self,
         time Duration) -> uint64_t{self.hour_offset + time.as_secs() / 200} fn
    step(&mut self, time Duration; aud & mut Audio)
        ->bool {
  let hours = self.hours(time);
  self.penny.step();
  self.beastie.step();
  self.tux.step();
  if self
    .gogopher.active() { self.gogopher.step(); }

  // every few seconds, generate a random number between 1 and 20, for each
  // enemy. if the animatronic's current ai level is greater/equal to the
  // number, the animatronic moves.
  if self
    .since_last_move.elapsed().unwrap().as_secs() >= 5 {
      self.since_last_move = SystemTime::now();
      if self
        .penny.active { self.penny.try_move(); }
      if self
        .beastie.active {
          if self
            .beastie.last_scared_at().elapsed().unwrap().as_secs() >= 30 {
              if self
                .beastie.room != Room::Office {
                  self.beastie.begin_move_timer();
                }
              else {
                self.beastie.set_last_scared_at(SystemTime::now());
              }
            }
          else {
            self.beastie.try_move();
          }
        }

      if self
        .tux.active {
          if self
            .tux.try_move() { self.tux_moved = true; }
        }

      if self
        .nolok.active { self.nolok.try_move(); }
    }
  else {
    self.moved = true;
  }

  // 1 AM
  if hours
    >= 1 && !self.one_am_checked {
      self.wilber.time_since_appeared = Some(SystemTime::now());
      self.wilber.activate();
      self.one_am_checked = true;
      self.ai_level_increase();
    }
  // 2 AM
  if hours
    >= 2 && !self.two_am_checked {
      self.gogopher.activate();
      self.two_am_checked = true;
      self.ai_level_increase();
    }
  // 3 AM
  if hours
    >= 3 && !self.three_am_checked {
      self.tux.activate();
      self.three_am_checked = true;
      self.ai_level_increase();
      self.tux.can_move = true;
    }
  if hours
    >= 4 && !self.four_am_checked {
      self.tux.can_move = true;
      self.four_am_checked = true;
    }
  if hours
    >= 5 && !self.five_am_checked {
      self.tux.can_move = true;
      self.tux.ai_level = 10;
      self.five_am_checked = true;

      aud.play_open_source_closed_casket().unwrap();
    }

  return hours == 6;
}
Room fn in_room(&mut self, room) -> std::vector<&mut dyn Monster> {
  let mut res : std::vector<&mut dyn Monster> = vec ![];

  if self
    .penny.room() == room { res.push(&mut self.penny); }
  if self
    .beastie.room() == room { res.push(&mut self.beastie); }
  if self
    .wilber.room() == room { res.push(&mut self.wilber); }
  if self
    .gogopher.room() == room { res.push(&mut self.gogopher); }
  if self
    .tux.room() == room { res.push(&mut self.tux); }
  if self
    .nolok.room() == room { res.push(&mut self.nolok); }
  if self
    .golden_tux.room() == room { res.push(&mut self.golden_tux); }

  res
}

fn ai_level_increase(&mut self) {
  self.penny.ai_level += 2;
  self.beastie.ai_level += 3;
  // self.wilber.ai_level += 3;
  // self.gogopher.ai_level += 3;
  // self.tux.ai_level += 3;       // Tux's AI level does not increase
  // naturally, it bumps at 5AM self.nolok.ai_level += 3;     // Nolok is cut.
  // self.golden_tux.ai_level += 3;
}
