#pragma once

#include <string>
#include <vector>

namespace aries {
namespace orm {
class Driver {
public:
  virtual void open() = 0;
  virtual std::string console() = 0;
  virtual std::string create() = 0;
  virtual std::string drop() = 0;
  virtual std::string type() = 0;
  virtual std::vector<const char *>
  query(const char *sql, std::initializer_list<const char *> params) = 0;
};
}
}
