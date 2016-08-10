#pragma once

namespace aries {
namespace orm {
class Dialect {
public:
  template <class C> static std::string getClassName();
};
}
}
