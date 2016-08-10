#ifndef AIRES_MIGRATION_H
#define AIRES_MIGRATION_H

#include <string>
#include <vector>

namespace aries {
class Migration {
public:
  virtual std::vector<std::string> up() = 0;
  virtual std::vector<std::string> down() = 0;
  virtual std::string name() = 0;
};
}

#endif
