#pragma once

#include <iostream>
#include <sstream>
#include <stdarg.h>
#include <vector>

namespace aries {
class DB {
public:
  virtual std::vector<const char *>
  query(const char *sql, std::initializer_list<const char *> params) = 0;
};
}
