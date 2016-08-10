/**
* https://www.postgresql.org/docs/9.5/static/libpq-connect.html#LIBPQ-PARAMKEYWORDS
*/

#ifndef AIRES_IMPL_POSTGRESQL_H
#define AIRES_IMPL_POSTGRESQL_H

#include "../database.hpp"

#include <cstdlib>
#include <libpq-fe.h>

namespace aries {
namespace database {
class PostgreSql : public DB {
public:
  PostgreSql(const char *host, int port, const char *name, const char *user,
             const char *password, const char *mode, uint timeout);
  ~PostgreSql();
  void call(const char *sql, std::initializer_list<const char *> params);

private:
  PGconn *db;
};
}
}

#endif
