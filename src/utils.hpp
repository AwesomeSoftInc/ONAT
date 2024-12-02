/**
    Utility functions to clean up the process of Rust -> C++.

    The contents of macros.rs were also ported here.
*/

extern "C" {
#include <stddef.h>
}

#include <vector>

/**
  Return an iterable vector for loops.
*/
std::vector<size_t> range(size_t start, size_t end);

/**
  Rectangle that covers the entire surface of `tex`.
*/
#define TEXTURE_RECT(tex)                                                      \
  (Rectangle) { 0, 0, tex.width, tex.height }

#include "rs_systemtime.hpp"
