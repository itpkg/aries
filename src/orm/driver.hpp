#pragma once

#include <string>
#include <vector>

namespace aries {
namespace orm {
class Driver {
public:
  virtual std::string name() = 0;
  virtual std::vector<const char *>
  query(const char *sql, std::initializer_list<const char *> params) = 0;
};
}
}
