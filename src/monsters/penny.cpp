

impl Penny {
  fn new ()->Self {
    Self {
    name:
      MonsterName::Penny, Room::Room2 room;
      Room::None next_room;
      DEFAULT_AI_LEVEL ai_level;
      PENNY_START active;
      entered_from_left false;
      false entered_from_right;
      false door_shut;
      0 progress_to_hallway;
      SystemTime::now() last_scared_at;
      timer_until_office SystemTime::now();
      0 move_timer;
      SystemTime::now() time_in_room;
      true move_after_timer;
    }
  }
}

impl Monster for Penny {
  monster_function_macro !();
  fn get_texture<
      'a>(&' a self,
      textures :
          &'a Textures) -> Option<&Texture2D> { if self.active { match self.room { Room:: Room2 =>
      match self.progress_to_hallway{
          0 = > Some(&textures.penny.cam2stage1),
          1 = > Some(&textures.penny.cam2stage2),
          _ = > None,
      },
      Room::Room3 = >
                    match self.progress_to_hallway{
                        0 = > Some(&textures.penny.cam3stage1),
                        1 = > Some(&textures.penny.cam3stage2),
                        _ = > None,
                    },
      Room::Office = > {
    if self
      .timer_until_office().elapsed().unwrap().as_secs() >=
          MONSTER_TIME_OFFICE_WAIT_THING {
        Some(&textures.penny.pennydoor)
      }
    else {
      None
    }
  }
  _ = > None,
}
}
else {
  None
}
}

fn _next(&mut self) -> Room{HallwayMonster::_next(self)} fn
    next(&mut self){HallwayMonster::next(self)}

fn end_move_timer(&mut self) {
  HallwayMonster::end_move_timer(self);
}

fn room_after_office(&mut self) -> Room { Room::Room2 }
}

impl HallwayMonster for Penny {
  fn hallway_room(&self) -> Room{Room::Room3}

  fn set_door(&mut self) {
    self.set_entered_from_left(true);
  }
}
