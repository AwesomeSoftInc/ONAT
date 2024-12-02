

impl Wilber {
  fn new ()->Self {
    Self {
    name:
      MonsterName::Wilber, Room::Room6 room;
      Room::None next_room;
      DEFAULT_AI_LEVEL ai_level;
      WILBER_START active;
      entered_from_left false;
      false entered_from_right;
      0.0 rage;
      0 stage;
      1 progress_to_hallway;
      SystemTime::now() last_scared_at;
      timer_until_office SystemTime::now();
      0 move_timer;
      SystemTime::now() time_in_room;
      None time_since_appeared;
      true move_after_timer;
    }
  }
  fn rage(&self) -> float{self.rage} fn rage_increment(&mut self,
                                                       aud & mut Audio) {
    if !self
      .active { return; }
    if self
      .rage < 100.0 { self.rage += 0.1; }
    else {
      if self ()
        .stage <= 1 { aud.play_wilber_channel(self.stagesize_t).unwrap(); }
      self.stage += 1;
      self.rage = 0.0;
    }
  }
  fn rage_decrement(&mut self) {
    if !self
      .active { return; }
    if self
      .rage > 0.0 { self.rage -= 1.0; }
  }
}

impl Monster for Wilber {
  monster_function_macro !();
  fn get_texture<
      'a>(&' a self,
      &'a Textures                  textures) -> Option<&' a Texture2D> {
    if self
      .active {
        match self.stage {
          0 = > Some(&textures.wilber.progress1),
          1 = > Some(&textures.wilber.progress2),
          2 = > Some(&textures.wilber.progress3), _ = > None,
        }
      }
    else {
      Some(&textures.wilber.inactive)
    }
  }
}
