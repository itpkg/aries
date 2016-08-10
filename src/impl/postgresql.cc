#include "../log.hpp"
#include "postgresql.hpp"

namespace aries {
namespace database {

PostgreSql::PostgreSql(const char *host, int port, const char *name,
                       const char *user, const char *password, const char *mode,
                       uint timeout) {
  std::ostringstream buf;
  buf << "host=" << host << " port=" << port << " dbname=" << name
      << " user=" << user << " password=" << password << " sslmode=" << mode
      << " connect_timeout=" << timeout;
  const char *path = buf.str().c_str();
  BOOST_LOG_TRIVIAL(debug) << "open database: " << path;
  this->db = PQconnectdb(path);

  if (PQstatus(this->db) != CONNECTION_OK) {
    BOOST_LOG_TRIVIAL(error) << PQerrorMessage(db);
    PQfinish(this->db);
  }
}

PostgreSql::~PostgreSql() { PQfinish(db); }

void PostgreSql::call(const char *sql,
                      std::initializer_list<const char *> params) {
  BOOST_LOG_TRIVIAL(debug) << sql;

  PGresult *res;

  if (params.size() == 0) {
    res = PQexec(this->db, sql);
  } else {
    const char *args[params.size()];
    std::copy(params.begin(), params.end(), args);
    res = PQexecParams(this->db, sql, params.size(), NULL, args, NULL, NULL, 1);
  }

  int fields;
  switch (PQresultStatus(res)) {
  case PGRES_COMMAND_OK:
    break;
  case PGRES_TUPLES_OK:
    fields = PQnfields(res);
    std::cout << "COL: ";
    for (int i = 0; i < fields; i++) {
      std::cout << PQfname(res, i) << " ";
    }
    std::cout << std::endl;
    for (int i = 0; i < PQntuples(res); i++) {
      for (int j = 0; j < fields; j++) {
        std::cout << "ROW " << PQgetvalue(res, i, j);
      }
      std::cout << std::endl;
    }
    break;
  default:
    BOOST_LOG_TRIVIAL(error) << PQerrorMessage(db);
  }
  PQclear(res);
}
}
}
