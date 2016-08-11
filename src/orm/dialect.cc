#include "dialect.hpp"

namespace aries {
namespace orm {
template <class T> std::string Dialect::first() {
  return "SELECT * FROM " + this->tableName<T>() + " ORDER BY id DESC LIMIT 1";
}
}
}
