#pragma once

#include <initializer_list>
#include <string>

namespace aries {
namespace orm {

namespace migration {
const std::string last = "migrations.last";
const std::string add = "migrations.add";
const std::string exist = "migrations.exist";
const std::string del = "migrations.del";
}

class Migration {
public:
  std::initializer_list<const char *> up;
  std::initializer_list<const char *> down;
  const char *version;
  std::string driver;
};
}
}
