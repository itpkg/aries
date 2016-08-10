#ifndef AIRES_WEB_ENGINE_H
#define AIRES_WEB_ENGINE_H

#include <yaml-cpp/yaml.h>

#include "router.h"
#include "../log.h"



namespace aries{

  namespace web{
    class Engine{
    public:
      virtual void mount(Router* rt) = 0;
      virtual YAML::Node config() = 0;
      virtual void init() = 0;
    private:
    };
  }
}

#endif
