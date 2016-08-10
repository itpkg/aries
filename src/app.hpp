#ifndef AIRES_APP_H
#define AIRES_APP_H

#include "web/engine.hpp"

#include <fruit/fruit.h>
#include <vector>

namespace aries {
extern std::vector<web::Engine *> engines;

class App {
public:
  INJECT(App()) = default;

  void add(web::Engine *en);

  void start(std::string host, int port, int jobs, bool daemon);

  void nginx(bool ssl);

  void db_connect();
  void db_migrate();
  void db_seed();
  void db_rollback();
  void db_drop();

  void cache_list();
  void cache_flush();
  void cache_connect();

private:
  YAML::Node config;
};
}

#endif
