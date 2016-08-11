#pragma once

#include <string>

namespace aries {
namespace orm {
class Dialect {
public:
  template <class T> static std::string tableName();
};
}
}
