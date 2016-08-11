#pragma once

#include "dialect.hpp"
#include "migration.hpp"

#include <iostream>
#include <sstream>

namespace aries {
namespace orm {
class DB {
public:
  virtual std::vector<const char *>
  query(const char *sql, std::initializer_list<const char *> params) = 0;
};
}
}
