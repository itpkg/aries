#pragma once

#include "migration.hpp"

#include <map>
#include <vector>

namespace aries {
namespace orm {

namespace dialect {
const std::string postgresql = "postgres";
const std::string mysql = "mysql";
const std::string sqlite3 = "sqlite3";
const std::string oracle = "oracle";
const std::string mssql = "mssql";
}

class Dialect {
public:
  friend class DB;
  void addMigration(std::string driver, const char *version,
                    std::initializer_list<const char *> up,
                    std::initializer_list<const char *> down);

  void set(std::string driver, std::string name, const char *sql);
  const char *get(std::string driver, std::string name);

private:
  std::string key(std::string driver, std::string name);
  std::vector<Migration> migrations;
  std::map<std::string, const char *> queries;
};
}
}
