#include "cache/redis.hpp"
#include "console.hpp"
#include "orm/postgresql.hpp"
#include "utils.hpp"
#include "web/engine.hpp"

#include <fstream>
#include <iostream>

namespace aries {
namespace console {

void db_migrate(std::string cfg) {
  auto db = openDB(cfg);
  db->initScheme();
  db->migrate();
}
void db_rollback(std::string cfg) {
  auto db = openDB(cfg);
  db->initScheme();
  db->rollback();
}
void db_console(std::string cfg) {
  auto db = getDbDriver(cfg);
  utils::shell(db->console());
}
void db_create(std::string cfg) {
  auto db = getDbDriver(cfg);
  utils::shell(db->create());
}
void db_drop(std::string cfg) {
  auto db = getDbDriver(cfg);
  utils::shell(db->drop());
}

void cache_console(std::string cfg) {
  auto ch = openCache(cfg);
  utils::shell(ch->console());
}
void cache_clear(std::string cfg) {
  auto ch = openCache(cfg);
  ch->clear();
}

void init_config(std::string name) {
  BOOST_LOG_TRIVIAL(info) << "generate file " << name;

  if (std::ifstream(name)) {
    throw std::invalid_argument("file already exists");
  }

  std::ofstream fout(name);
  for (auto en : aries::engines) {
    fout << en->config();
    fout << std::endl;
  }
  fout.close();
}

void show_help(std::string appName,
               const boost::program_options::options_description &desc) {
  std::cout << appName << " is build by (https://github.com/itpkg/aries)."
            << std::endl
            << std::endl;
  std::cout << desc << std::endl;
}

void show_version() { std::cout << "2016.08.09" << std::endl; }

inline YAML::Node readConfig(std::string file) { return YAML::LoadFile(file); }

orm::Driver *getDbDriver(std::string file) {
  auto cfg = readConfig(file);
  orm::Driver *drv;
  auto type = cfg["database"]["driver"].as<std::string>();
  if (type == orm::dialect::postgresql) {
    drv = new orm::PostgreSql(cfg["database"]["host"].as<std::string>(),
                              cfg["database"]["port"].as<int>(),
                              cfg["database"]["name"].as<std::string>(),
                              cfg["database"]["user"].as<std::string>(),
                              cfg["database"]["password"].as<std::string>(),
                              cfg["database"]["ssl_mode"].as<std::string>(),
                              cfg["database"]["timeout"].as<uint>());
  } else {
    throw std::invalid_argument("unsupport database driver " + type);
  }

  return drv;
}

orm::DB *openDB(std::string file) {
  auto dia = new orm::Dialect;
  for (auto en : aries::engines) {
    en->scheme(dia);
  }
  auto drv = getDbDriver(file);
  drv->open();
  return new orm::DB(drv, dia);
}

cache::Cache *openCache(std::string file) {
  auto cfg = readConfig(file);
  cache::Cache *drv;
  auto type = cfg["cache"]["driver"].as<std::string>();
  if (type == cache::dialect::redis) {
    drv = new cache::Redis(
        cfg["cache"]["prefix"].as<std::string>(),
        cfg["cache"]["host"].as<std::string>(), cfg["cache"]["port"].as<int>(),
        cfg["cache"]["db"].as<int>(), cfg["cache"]["timeout"].as<int>());
  } else {
    throw std::invalid_argument("unsupport cache driver " + type);
  }
  return drv;
}
}
}
