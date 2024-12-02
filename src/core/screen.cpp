#include "screen.hpp"
#include <raylib.h>

ScreenInfo SCREEN = ScreenInfo();

ScreenInfo::ScreenInfo() {
  InitWindow(0, 0, "ONAT");
  this->_width = GetMonitorWidth(GetCurrentMonitor());
  this->_height = GetMonitorHeight(GetCurrentMonitor());

  auto default_ratio = (float)this->_width / (float)this->_height;
  auto desired_ratio = 4.0 / 3.0;
  auto ratio = 1.0 + (default_ratio - desired_ratio);

  this->_margin = (float)_width - (((float)_width) / ratio);
  if (this->_margin < 0.0) {
    this->_margin = 0.0;
  }
}

void ScreenInfo::update() {
  auto monitor_width = GetScreenWidth();
  auto monitor_height = GetScreenHeight();
  auto default_ratio = (float)monitor_width / (float)monitor_height;
  auto desired_ratio = 4.0 / 3.0;
  auto ratio = 1.0 + (default_ratio - desired_ratio);

  auto margin = (float)monitor_width - (((float)monitor_width) / ratio);
  if ((margin < 0.0)) {
    margin = 0.0;
  }

  this->_width = monitor_width;
  this->_height = monitor_height;
  this->_ratio = ratio;
  this->_margin = margin;
}

float ScreenInfo::width() { return (float)this->_width / (float)this->_ratio; }

float ScreenInfo::width_unaltered() { return (float)this->_width; }
float ScreenInfo::height() { return (float)this->_height; }

float ScreenInfo::margin() { return this->_margin / 2.0; }

float ScreenInfo::ratio() { return this->_ratio; }
