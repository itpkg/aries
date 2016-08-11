#pragma once

#include "dialect.hpp"
#include "driver.hpp"

#include <boost/log/trivial.hpp>
#include <cstdint>
#include <iostream>
#include <sstream>

namespace aries {
namespace orm {

class DB {
public:
  DB(Driver *drv, Dialect *dia);

  void initScheme();
  void migrate();
  void rollback();

  std::vector<const char *> query(std::string name,
                                  std::initializer_list<const char *> params);

  inline uint32_t toUint(const char *v);

private:
  Driver *driver;
  Dialect *dialect;
};
}
}
