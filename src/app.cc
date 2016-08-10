#include "app.h"
#include "log.h"

namespace aries{
  void App::start(){
    //TODO
  }

  fruit::Component<App> getApp(std::string file) {
    BOOST_LOG_TRIVIAL(info) << "load config from file " << file;
    YAML::Node config = YAML::LoadFile(file);
    //TODO
    return fruit::createComponent();
  }
}
