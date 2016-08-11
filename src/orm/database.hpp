#pragma once

#include "../log.hpp"
#include "dialect.hpp"
#include "driver.hpp"

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

  inline uint32_t toUint(const char *v);

private:
  Driver *driver;
  Dialect *dialect;
};
}
}
