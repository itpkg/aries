#pragma once
#include "../../orm/model.hpp"

namespace aries {
namespace platform {
class User {
public:
  template <class T> void hibernate(T &t);
  uint id;
  std::string username;
  std::string password;
  std::string uid;
};
}
}
