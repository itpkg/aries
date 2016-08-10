#ifndef AIRES_APP_H
#define AIRES_APP_H

#include <fruit/fruit.h>
#include <yaml-cpp/yaml.h>

namespace aries{
  class App{
  public:
    INJECT(App()) = default;

    void start();
  private:
    YAML::Node config;
  };

  fruit::Component<App> getApp(std::string file);
}

#endif
