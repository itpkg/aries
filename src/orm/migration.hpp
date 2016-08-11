#pragma once

#include <string>
#include <vector>

namespace aries {
namespace orm {
class Migration {
public:
  std::vector<std::string> up;
  std::vector<std::string> down;
  std::string name;
};
}
}
