#pragma once

#include "../../web/engine.hpp"

namespace aries {
namespace platform {

class Engine : public web::Engine {
public:
  void scheme(orm::Dialect *dia);
  void mount(web::Router *rt);
  YAML::Node config();
  void init();
};
}
}
