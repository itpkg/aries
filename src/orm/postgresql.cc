#include "postgresql.hpp"

#include <boost/log/trivial.hpp>

namespace aries {
namespace orm {

std::string PostgreSql::console() {
  std::ostringstream buf;
  buf << "psql -h " << this->host << " -p " << this->port << " -U "
      << this->user << " " << this->name;

  return buf.str();
}
std::string PostgreSql::create() {
  std::ostringstream buf;
  buf << "psql -h " << this->host << " -p " << this->port << " -U "
      << this->user << " -c "
      << "\"CREATE DATABASE " << this->name << " WITH ENCODING='UTF8'\"";

  return buf.str();
}

std::string PostgreSql::drop() {
  std::ostringstream buf;
  buf << "psql -h " << this->host << " -p " << this->port << " -U "
      << this->user << " -c "
      << "\"DROP DATABASE " << this->name << "\"";

  return buf.str();
}
std::string PostgreSql::type() { return dialect::postgresql; }

void PostgreSql::open() {
  std::ostringstream buf;
  buf << "host=" << this->host << " port=" << this->port
      << " dbname=" << this->name << " user=" << this->user
      << " password=" << this->password << " sslmode=" << this->ssl_mode
      << " connect_timeout=" << this->timeout;
  const char *path = buf.str().c_str();
  BOOST_LOG_TRIVIAL(debug) << "open database: " << path;
  this->db = PQconnectdb(path);

  if (PQstatus(this->db) != CONNECTION_OK) {
    BOOST_LOG_TRIVIAL(error) << PQerrorMessage(db);
    PQfinish(this->db);
  }
}

PostgreSql::PostgreSql(std::string host, int port, std::string name,
                       std::string user, std::string password,
                       std::string ssl_mode, uint timeout) {
  this->host = host;
  this->port = port;
  this->name = name;
  this->user = user;
  this->password = password;
  this->ssl_mode = ssl_mode;
  this->timeout = timeout;
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
}
}
}
