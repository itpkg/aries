#pragma once

namespace aries {
namespace web {

class Router {
public:
  void GET();
  void POST();
  void PUT();
  void PATCH();
  void DELETE();
  void REST();
};
}
}
