#pragma once
/**
* https://www.postgresql.org/docs/9.5/static/libpq-connect.html#LIBPQ-PARAMKEYWORDS
*/

#include "database.hpp"

#include <cstdlib>
#include <libpq-fe.h>

namespace aries {
namespace orm {
class PostgreSql : public orm::Driver {
public:
  PostgreSql(std::string host, const int port, std::string name,
             std::string user, std::string password, std::string ssl_mode,
             uint timeout);
  ~PostgreSql();
  void open();
  inline std::string console();
  inline std::string create();
  inline std::string drop();
  inline std::string type();
  std::vector<const char *> query(const char *sql,
                                  std::initializer_list<const char *> params);

private:
  void parse(PGresult *res, std::vector<const char *> &val);
  PGconn *db;

  std::string host;
  std::string name;
  std::string user;
  std::string password;
  std::string ssl_mode;
  int port;
  int timeout;
};
}
}
