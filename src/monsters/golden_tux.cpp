

impl GoldenTux {
  fn new ()->Self {
    Self {
    name:
      MonsterName::GoldenTux, Room::Office room;

      Room::None next_room;
      DEFAULT_AI_LEVEL ai_level;
      GOLDEN_TUX_START active;
      entered_from_left false;
      false entered_from_right;
      1 progress_to_hallway;

      SystemTime::now() last_scared_at;
      timer_until_office SystemTime::now();
      0 move_timer;

      SystemTime::now() appeared;
      SystemTime::now() time_in_room;
      true move_after_timer;
    }
  }
}

impl Monster for GoldenTux {
  monster_function_macro !();

  // Golden Tux has special rules.
  fn taint_percent(&self) -> float{0.0}

  fn get_texture<
      'a>(&' a self,
      &'a Textures                  textures) -> Option<&' a Texture2D> {
    if self
      .active { Some(&textures.golden_tux) }
    else {
      None
    }
  }
}
