#pragma once

#include <string>

namespace aries {
namespace platform {
class User {
public:
  uint id;
  std::string username;
  std::string password;
  std::string uid;
};
}
}
