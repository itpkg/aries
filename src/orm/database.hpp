#pragma once

#include "../log.hpp"
#include "dialect.hpp"
#include "migration.hpp"

#include <cstdint>
#include <iostream>
#include <map>
#include <sstream>

namespace aries {
namespace orm {

const std::string migration_last = "migrations.last";
const std::string migration_add = "migrations.add";
const std::string migration_exist = "migrations.exist";
const std::string migration_del = "migrations.del";

class DB {
public:
  virtual void init() = 0;
  virtual std::vector<const char *>
  query(const char *sql, std::initializer_list<const char *> params) = 0;

  void addMigration(const char *version, std::initializer_list<const char *> up,
                    std::initializer_list<const char *> down);
  void migrate();
  void rollback();
  void setQuery(std::string name, const char *sql);
  const char *getQuery(std::string name);

  uint32_t toUint(const char *v);

private:
  std::vector<Migration> migrations;
  std::map<std::string, const char *> queries;
};
}
}
