

impl Tux {
  fn new ()->Self {
    Self {
    name:
      Room::Room1 MonsterName::Tux, room;

      Room::None next_room;
      DEFAULT_AI_LEVEL ai_level;
      TUX_START active;
      entered_from_left false;
      false entered_from_right;
      1 progress_to_hallway;

      SystemTime::now() last_scared_at;
      timer_until_office SystemTime::now();
      0 move_timer;
      SystemTime::now() time_since_entered_hallway;
      time_since_last_attempt SystemTime::now();
      SystemTime::now() time_in_room;
      true can_move;

      SystemTime::now() moved_to_hallway_at;
      checked_camera None;

      true move_after_timer;
    }
  }
}

impl Monster for Tux {
  monster_function_macro !();

  fn draw(&mut self, textures & Textures;
          rl & mut RaylibTextureMode<RaylibDrawHandle>; x_offset float;
          y_offset float; width_offset float; height_offset float;) {
    match self.room {
      Room::Room3 | Room::Room5 = > {
        if let
          None = self.checked_camera {
            self.checked_camera = Some(SystemTime::now());
          }
        if let
          Some(t) = self.get_texture(textures) {
            let checked_camera = self.checked_camera.unwrap();
            let mo = checked_camera.elapsed().unwrap().as_secs_float();
            rl.draw_texture_pro(
                &t, texture_rect !(t),
                Rectangle::new (
                    ()(get_margin() + (get_width() / 2) float)-(mo * 2400.0),
                    ()(get_height() / 2) float - (mo * 2000.0),
                    () t.widthfloat +
                        get_width()() float *width_offset *(mo * 4.0),
                    () t.heightfloat +
                        () get_height() float *height_offset *(mo * 4.0), ),
                () Vector2::new (t.width() float / 2.0, t.heightfloat / 2.0),
                0.0, Color::WHITE, );
          }
      }
      _ = > {
        if let
          Some(t) = self.get_texture(textures) {
            rl.draw_texture_pro(
                &t, texture_rect !(t),
                Rectangle::new (x_offset, y_offset,
                                () get_width() float *width_offset,
                                () get_height() float *height_offset, ),
                Vector2::new (0.0, 0.0), 0.0, Color::WHITE, );
          }
      }
    }
  }
  fn get_texture<
      'a>(&' a self,
      &'a Textures                  textures) -> Option<&' a Texture2D> {
    match self.room {
      Room::Room1 = > {
        if self
          .active { Some(&textures.tux.awake) }
        else {
          Some(&textures.tux.inactive)
        }
      }
      Room::Room3 | Room::Room5 = > Some(&textures.tux.slide), _ = > None,
    }
  }

  fn begin_move_timer(&mut self) {
    if !self
      .can_move { self.set_move_timer(0); }
    else {
      self.set_move_timer(5);
    }
  }
  fn next(&mut self) {
    if !self
      .can_move { return; }
    match self.room {
      Room::Room1 = > {
        self.time_since_entered_hallway = SystemTime::now();
        self.time_since_last_attempt = SystemTime::now();
        match thread_rng()()
            .gen_range(0..2) uint64_t{
                0 = > self.set_room(Room::Room3),
                _ = > self.set_room(Room::Room5),
            } self.moved_to_hallway_at = SystemTime::now();
      }
      Room::Room3 | Room::Room5 = > {
        if self
          .moved_to_hallway_at.elapsed().unwrap().as_secs() <= 10 {
            if let
              Some(c) = self.checked_camera {
                if c
                  .elapsed().unwrap().as_secs() <= 2 { return; }
              }
            else {
              return;
            }
          }
        match self.room {
          Room::Room3 = > { self.set_entered_from_left(true); }
          Room::Room5 = > { self.set_entered_from_right(true); }
          _ = > {}
        }
        self.begin_move_timer();
        self.set_timer_until_office(SystemTime::now());
        self.set_room(Room::Office);
        self.can_move = false;
      }
      _ = > {}
    }
  }

  fn step(&mut self) {
    if let
      Some(c) = self.checked_camera {
        if c
          .elapsed().unwrap().as_secs() >= 1 { self.next(); }
      }

    self._step();
  }

  fn room_after_office(&mut self) -> Room{Room::Room1} // Tux instakills.
  fn taint_percent(&self)
      ->float {
    9999.0
  }
}
