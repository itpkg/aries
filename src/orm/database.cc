#include "database.hpp"

namespace aries {
namespace orm {
void DB::addMigration(std::string name, std::initializer_list<std::string> up,
                      std::initializer_list<std::string> down) {
  Migration mig;
  mig.name = name;
  mig.up.insert(mig.up.end(), up.begin(), up.end());
  mig.down.insert(mig.down.end(), down.begin(), down.end());
  this->migrations.push_back(&mig);
}

void DB::setQuery(std::string name, std::string sql) {
  this->queries[name] = sql;
}

std::string DB::getQuery(std::string name) { return this->queries[name]; }
}
}
