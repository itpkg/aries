#include "../log.hpp"
#include "postgresql.hpp"

namespace aries {
namespace orm {

void PostgreSql::init() {
  Migration mig;
  mig.name = migration_init;
  mig.up.push_back("CREATE TABLE IF NOT EXISTS schema_migrations(id SERIAL, "
                   "version VARCHAR(255) NOT NULL UNIQUE, created TIMESTAME "
                   "NOT NULL DEFAULT NOW)");
  mig.down.push_back("DROP TABLE IF EXISTS schema_migrations");
  this->migrations.insert(this->migrations.begin(), &mig);

  this->setQuery(
      migration_last,
      "SELECT id, version FROM schema_migrations ORDER BY id DESC LIMIT 1");
  this->setQuery(migration_del, "DELETE schema_migrations WHERE id = $1");
  this->setQuery(migration_add,
                 "INSERT INTO schema_migrations(version) VALUES($1)");
}

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

std::vector<const char *>
PostgreSql::query(const char *sql, std::initializer_list<const char *> params) {
  BOOST_LOG_TRIVIAL(debug) << sql;

  PGresult *res;
  std::vector<const char *> items;

  if (params.size() == 0) {
    res = PQexec(this->db, sql);
  } else {
    const char *args[params.size()];
    std::copy(params.begin(), params.end(), args);
    res = PQexecParams(this->db, sql, params.size(), NULL, args, NULL, NULL, 1);
  }

  switch (PQresultStatus(res)) {
  case PGRES_COMMAND_OK:
    break;
  case PGRES_TUPLES_OK:
    this->parse(res, items);
    break;
  default:
    BOOST_LOG_TRIVIAL(error) << PQerrorMessage(db);
    break;
  }
  PQclear(res);
  return items;
}

void PostgreSql::parse(PGresult *res, std::vector<const char *> &val) {
  auto cols = PQnfields(res);
  auto rows = PQntuples(res);

  for (int i = 0; i < rows; i++) {
    for (int j = 0; j < cols; j++) {
      val.push_back(PQgetvalue(res, i, j));
    }
  }
  // std::cout << "COL: ";
  // for (int i = 0; i < cols; i++) {
  //   std::cout << PQfname(res, i) << " ";
  // }
  // std::cout << std::endl;
  // for (int i = 0; i < rows; i++) {
  //   for (int j = 0; j < cols; j++) {
  //     std::cout << "ROW " << PQgetvalue(res, i, j);
  //   }
  //   std::cout << std::endl;
  // }
}
}
}
