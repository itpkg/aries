#pragma once
#include "../../orm/model.hpp"

namespace aries {
namespace platform {
class User {
public:
  uint id;
  std::string username;
  std::string password;
  std::string uid;
};
// ORM_EXPORT_CLASS(User);
}
}
