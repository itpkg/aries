#include "dialect.hpp"

namespace aries {
namespace orm {

void Dialect::addMigration(std::string driver, const char *version,
                           std::initializer_list<const char *> up,
                           std::initializer_list<const char *> down) {
  Migration mig;
  mig.driver = driver;
  mig.version = version;
  mig.up = up;
  mig.down = down;
  this->migrations.push_back(mig);
}

void Dialect::set(std::string driver, std::string name, const char *sql) {
  this->queries[this->key(driver, name)] = sql;
}

const char *Dialect::get(std::string driver, std::string name) {
  return this->queries[this->key(driver, name)];
}

std::string Dialect::key(std::string driver, std::string name) {
  return driver + "://" + name;
}
}
}
