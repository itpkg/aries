#ifndef AIRES_WEB_ENGINE_H
#define AIRES_WEB_ENGINE_H

#include "router.h"
#include "../log.h"
#include "../orm/migration.h"

#include <yaml-cpp/yaml.h>
#include <map>
#include <vector>


namespace aries{

  namespace web{
    class Engine{
    public:
      virtual std::vector<orm::Migration*> migrations() = 0;
      virtual std::vector<std::string> seed() = 0;
      virtual std::map<std::string, std::string> queries() = 0;

      virtual void mount(Router* rt) = 0;
      virtual YAML::Node config() = 0;
      virtual void init() = 0;
    private:
    };
  }
}

#endif
