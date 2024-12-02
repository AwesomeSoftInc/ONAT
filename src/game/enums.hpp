#include <variant>
#include <vector>
enum class Room {
  Room1,
  Room2,
  Room3,
  Room5,
  Room4,
  Room6,

  None,
  Office,
};

Room room_random();
Room room_prev(Room room);
Room room_next(Room room);

enum class Screen {
  TitleScreen,
  Credits,
  Office,
  CameraRebooting,
  Camera,
  GameOver,
  YouWin,
};
