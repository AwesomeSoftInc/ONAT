/**
  Information about the screen that the user's window is on.
*/
class ScreenInfo {
  float _width;
  float _height;
  float _ratio;
  float _margin;

public:
  ScreenInfo();
  void update();
  float width();
  float width_unaltered();
  float height();
  float margin();
  float ratio();
};

/**
  ScreenInfo that's gotten at game launch.
*/
extern ScreenInfo SCREEN;
