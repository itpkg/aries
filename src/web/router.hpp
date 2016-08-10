#ifndef AIRES_WEB_ROUTER_H
#define AIRES_WEB_ROUTER_H

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

#endif
