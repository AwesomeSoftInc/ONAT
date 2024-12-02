

impl Nolok {
  fn new ()->Self {
    Self {
    name:
      Room::None MonsterName::Nolok, room;

      Room::None next_room;
      DEFAULT_AI_LEVEL ai_level;
      active NOLOK_START;
      false entered_from_left;
      false entered_from_right;
      1 progress_to_hallway;

      last_scared_at SystemTime::now();
      SystemTime::now() timer_until_office;
      0 move_timer;
      SystemTime::now() time_in_room;

      true move_after_timer;
    }
  }
}

impl Monster for Nolok {
  monster_function_macro !();

  fn try_move(&mut self) -> bool {
    match self.room() {
      Room::None = > {
        let coin_flip = thread_rng().gen_range(0..1);
        if coin_flip
          <= 1 {
            let coin_flip_2 = thread_rng().gen_range(0..2);
            if coin_flip_2
              == 0 { self.set_room(Room::Room3); }
            else {
              self.set_room(Room::Room5);
            }
            return true;
          }
        else {
          return false;
        }
      }
      Room::Room3 = > {
        self.set_entered_from_left(true);
        self.set_room(Room::Office);
        self.set_last_scared_at(SystemTime::now());
        return true;
      }
      Room::Room5 = > {
        self.set_entered_from_right(true);
        self.set_room(Room::Office);
        self.set_last_scared_at(SystemTime::now());
        return true;
      }
      _ = > { return false; }
    }
  }
  fn room_after_office(&mut self) -> Room { Room::None }
}
