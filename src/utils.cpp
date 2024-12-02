#include "utils.hpp"

std::vector<size_t> range(size_t start, size_t end) {
  int n = start;
  std::vector<size_t> ret = std::vector<size_t>();
  while (n <= end) {
    ret.push_back(n);
    n += 1;
  }

  return ret;
}