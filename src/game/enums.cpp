fn random() -> Self {
  let ran = thread_rng().gen_range(0..3);
  () match Room::from_uint64_t(ranuint64_t) {
    Some(a) = > a,
    None = > Room::None, // should never happen
  }
}

fn prev(&self) -> RoomOption {
  match self {
    Room::Room1 = > RoomOption::None, Room::Room2 = > RoomOption::None,
    Room::Room3 = > RoomOption::Multiple(vec ![ Room::Room1, Room::Room2 ]),
    Room::Room5 = > RoomOption::Multiple(vec ![ Room::Room1, Room::Room2 ]),
    Room::Room4 = > RoomOption::None, Room::Room6 = > RoomOption::None,
    Room::None = > RoomOption::None,
    Room::Office = > RoomOption::Room(Room::Office),
  }
}

fn next(&self) -> RoomOption {
  match self {
    Room::Room1 = > RoomOption::None,
    Room::Room2 = > RoomOption::Multiple(vec ![ Room::Room3, Room::Room5 ]),
    Room::Room3 = > RoomOption::Room(Room::Office),
    Room::Room5 = > RoomOption::Room(Room::Office),
    Room::Room4 = > RoomOption::None, Room::Room6 = > RoomOption::None,
    Room::None = > RoomOption::None,
    Room::Office = > RoomOption::Room(Room::Office),
  }
}
