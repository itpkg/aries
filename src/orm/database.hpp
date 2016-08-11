#pragma once

#include "dialect.hpp"
#include "migration.hpp"

#include <iostream>
#include <map>
#include <sstream>

namespace aries {
namespace orm {

const std::string migration_last = "migrations.last";
const std::string migration_add = "migrations.add";
const std::string migration_del = "migrations.del";
const std::string migration_init = "scheme-migrations.init";

class DB {
public:
  virtual void init() = 0;
  virtual std::vector<const char *>
  query(const char *sql, std::initializer_list<const char *> params) = 0;
  void addMigration(std::string name, std::initializer_list<std::string> up,
                    std::initializer_list<std::string> down);
  void migrate();
  void rollback();
  void setQuery(std::string name, std::string sql);
  std::string getQuery(std::string name);

protected:
  std::vector<Migration *> migrations;
  std::map<std::string, std::string> queries;
};
}
}
