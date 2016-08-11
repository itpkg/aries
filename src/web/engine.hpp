#pragma once

#include "../log.hpp"
#include "../orm/migration.hpp"
#include "router.hpp"

#include <map>
#include <vector>
#include <yaml-cpp/yaml.h>

namespace aries {

namespace web {
class Engine {
public:
  virtual std::vector<Migration *> migrations() = 0;
  virtual std::vector<std::string> seed() = 0;
  virtual std::map<std::string, std::string> queries() = 0;

  virtual void mount(Router *rt) = 0;
  virtual YAML::Node config() = 0;
  virtual void init() = 0;

private:
};
}
}
