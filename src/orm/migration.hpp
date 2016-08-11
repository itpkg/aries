#pragma once

#include <vector>

namespace aries {
namespace orm {
class Migration {
public:
  std::vector<const char *> up;
  std::vector<const char *> down;
  const char *version;
};
}
}
