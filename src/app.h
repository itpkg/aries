#ifndef AIRES_APP_H
#define AIRES_APP_H

#include "log.h"

#include <fruit/fruit.h>


namespace aries{
  class App{
  public:
    INJECT(App()) = default;

    void start();
  private:
  };

  fruit::Component<App> getApp(std::string file);
}

#endif
