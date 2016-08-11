#pragma once

#include <string>

namespace aries {
namespace orm {
class Dialect {
public:
  template <class T> std::string tableName();
  template <class T> std::string first();
};
}
}
