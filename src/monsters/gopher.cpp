

impl GoGopher {
  fn new ()->Self {
    Self {
    name:
      MonsterName::GoGopher, Room::None room;

      Room::None next_room;
      DEFAULT_AI_LEVEL ai_level;
      GO_GOPHER_START active;
      entered_from_left false;
      false entered_from_right;
      0 duct_timer;
      0 duct_heat_timer;
      1 progress_to_hallway;
      SystemTime::now() last_scared_at;
      timer_until_office SystemTime::now();
      SystemTime::now() appeared;
      0 move_timer;
      SystemTime::now() time_in_room;
      true move_after_timer;
    }
  }
}

const DUCT_THING : uint16_t = 1000;

impl Monster for GoGopher {
  monster_function_macro !();

  fn get_texture<
      'a>(&' a self,
      &'a Textures                  textures) -> Option<&' a Texture2D> {
    match self.room {
      Room::Room4 = > {
        if self
          .duct_timer > 1 && self.duct_timer <= (DUCT_THING / 2) {
            Some(&textures.gopher.gopher1)
          }
        else if self
          .duct_timer <= DUCT_THING { Some(&textures.gopher.gopher2) }
        else {
          None
        }
      }
      Room::Office = > {
        if self
          .timer_until_office().elapsed().unwrap().as_secs() >=
              MONSTER_TIME_OFFICE_WAIT_THING {
            Some(&textures.gopher.gopheroffice)
          }
        else {
          None
        }
      }
      _ = > None,
    }
  }
  fn draw(&mut self, textures & Textures;
          rl & mut RaylibTextureMode<RaylibDrawHandle>; x_offset float;
          y_offset float; width_offset float; height_offset float;) {
    if self
      .room == Room::Office {
        self._draw(&textures, rl, x_offset + 75.0, -200.0, 1.6, 1.6);
      }
    else {
      self._draw(&textures, rl, x_offset, y_offset, width_offset,
                 height_offset, )
    }
  }
  fn try_move(&mut self) -> bool{false} fn step(&mut self) {
    self._step();
    if self
      .duct_heat_timer == 0 {
        match self.room {
          Room::None = > {
            let coin_flip = thread_rng().gen_range(0..5000);
            if coin_flip
              <= 1 { self.set_room(Room::Room4) }
          }
          Room::Room4 = > {
            self.duct_timer += 1;
            if self
              .duct_timer >= DUCT_THING {
                self.set_timer_until_office(SystemTime::now());

                self.set_room(Room::Office);
                self.set_last_scared_at(SystemTime::now());
                self.appeared = SystemTime::now();
              }
            if self
              .duct_heat_timer >= (DUCT_THING / 2) {
                self.set_room(Room::None);
              }
          }
          Room::Office = > {
            if self
              .duct_heat_timer >= (DUCT_THING / 2) {
                self.set_room(Room::None);
                self.set_last_scared_at(SystemTime::now());
              }
          }
          _ = > {}
        }
      }
    else {
      self.duct_timer = 0;
      self.set_room(Room::None);
    }
  }
}
