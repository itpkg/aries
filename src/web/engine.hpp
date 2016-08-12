#pragma once

#include "../cache/cache.hpp"
#include "../orm/database.hpp"
#include "router.hpp"

#include <boost/log/trivial.hpp>
#include <map>
#include <mstch/mstch.hpp>
#include <vector>
#include <yaml-cpp/yaml.h>

namespace aries {

namespace web {
class Engine {
public:
  virtual void scheme(orm::Dialect *dia) = 0;
  virtual void mount(Router *rt) = 0;
  virtual YAML::Node config() = 0;
  virtual void init() = 0;

private:
};
}

extern std::vector<web::Engine *> engines;
}
