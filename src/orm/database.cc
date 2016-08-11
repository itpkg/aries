#include "database.hpp"

#include <arpa/inet.h>

namespace aries {
namespace orm {

DB::DB(Driver *drv, Dialect *dia) {
  this->driver = drv;
  this->dialect = dia;
}

std::vector<const char *>
DB::query(std::string name, std::initializer_list<const char *> params) {
  auto sql = this->dialect->get(this->driver->type(), name);
  return this->driver->query(sql, params);
}

void DB::initScheme() {
  this->driver->query("CREATE TABLE IF NOT EXISTS schema_migrations(version "
                      "VARCHAR(255) NOT NULL UNIQUE, created TIMESTAMP "
                      "NOT NULL DEFAULT NOW())",
                      {});

  this->dialect->set(
      this->driver->type(), migration::exist,
      "SELECT count(*)::INT4 FROM schema_migrations WHERE version = $1");

  this->dialect->set(
      this->driver->type(), migration::last,
      "SELECT version FROM schema_migrations ORDER BY created DESC LIMIT 1");
  this->dialect->set(this->driver->type(), migration::del,
                     "DELETE FROM schema_migrations WHERE version = $1");
  this->dialect->set(this->driver->type(), migration::add,
                     "INSERT INTO schema_migrations(version) VALUES($1)");
}

void DB::migrate() {
  for (auto mig : this->dialect->migrations) {
    auto rst = this->driver->query(
        this->dialect->get(this->driver->type(), migration::exist),
        {mig.version});

    if (this->toUint(rst.front()) == 0) {
      for (auto q : mig.up) {
        this->driver->query(q, {});
      }
      this->driver->query(
          this->dialect->get(this->driver->type(), migration::add),
          {mig.version});
    }
  }
}

void DB::rollback() {
  auto rst = this->driver->query(
      this->dialect->get(this->driver->type(), migration::last), {});
  if (rst.size() == 1) {
    for (auto mig : this->dialect->migrations) {
      if (strcmp(rst.front(), mig.version) == 0) {
        for (auto q : mig.down) {
          this->driver->query(q, {});
        }
        break;
      }
    }
    this->driver->query(
        this->dialect->get(this->driver->type(), migration::del),
        {rst.front()});
  }
}

uint32_t DB::toUint(const char *v) { return ntohl(*(uint32_t *)v); }
}
}
