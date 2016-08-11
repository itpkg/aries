#include "database.hpp"
#include <arpa/inet.h>

namespace aries {
namespace orm {

void DB::addMigration(const char *version,
                      std::initializer_list<const char *> up,
                      std::initializer_list<const char *> down) {
  Migration mig;
  mig.version = version;
  mig.up.insert(mig.up.end(), up.begin(), up.end());
  mig.down.insert(mig.down.end(), down.begin(), down.end());
  this->migrations.push_back(mig);

  // BOOST_LOG_TRIVIAL(debug) << "after add " << version;
  // for (auto m : this->migrations) {
  //   BOOST_LOG_TRIVIAL(debug) << m.version;
  // }
}

void DB::migrate() {
  for (auto mig : this->migrations) {
    auto rst = this->query(this->getQuery(migration_exist), {mig.version});

    // BOOST_LOG_TRIVIAL(debug) << "version: " << mig.version << " "
    // << this->toUint(rst.front());
    // for (auto r : rst) {
    //   BOOST_LOG_TRIVIAL(debug) << "VAL: " << r;
    // }

    if (this->toUint(rst.front()) == 0) {
      this->query(this->getQuery(migration_add), {mig.version});
    }
  }
}

void DB::rollback() {}

void DB::setQuery(std::string name, const char *sql) {
  this->queries[name] = sql;
}

const char *DB::getQuery(std::string name) { return this->queries[name]; }

uint32_t DB::toUint(const char *v) { return ntohl(*(uint32_t *)v); }
}
}
