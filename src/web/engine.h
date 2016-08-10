#ifndef AIRES_WEB_ENGINE_H
#define AIRES_WEB_ENGINE_H

#include "router.h"

namespace aries{
  namespace web{
    class Engine{
    public:
      virtual void mount(Router* rt) = 0;
      virtual void config(YAML::Node nd) = 0;
      virtual void init() = 0;
    };
  }
}


#endif
