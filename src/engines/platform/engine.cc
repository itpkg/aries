#include "../../utils.hpp"
#include "engine.hpp"
#include "models.hpp"

namespace aries {
namespace platform {

void Engine::scheme(orm::Dialect *dia) {
  // TODO
  dia->addMigration(orm::dialect::postgresql, "platform.create_users",
                    {"CREATE TABLE IF NOT EXISTS users(id SERIAL)"},
                    {"DROP TABLE IF EXISTS users"});

  dia->set(orm::dialect::postgresql, "platform.user.find_by_id",
           "SELECT * FROM USERS WHERE ID = $1");
}

void Engine::mount(web::Router *rt) {
  // TODO
}

YAML::Node Engine::config() {
  YAML::Node node;
  auto buf = random_bytes(512);
  std::string secret(buf.begin(), buf.end());
  node["host"] = "http://localhost";
  node["secret"] = to_base64(secret);

  node["database"]["driver"] = "postgres";
  node["database"]["host"] = "localhost";
  node["database"]["port"] = 5432;
  node["database"]["name"] = "aries_d";
  node["database"]["user"] = "postgres";
  node["database"]["password"] = "";
  node["database"]["timeout"] = 5;

  node["cache"]["driver"] = "redis";
  node["cache"]["host"] = "localhost";
  node["cache"]["port"] = 6379;
  node["cache"]["db"] = 0;
  node["cache"]["timeout"] = 5;

  return node;
}

void Engine::init() {}
}
}
