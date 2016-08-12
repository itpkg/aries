#pragma once

#include "request.hpp"
#include "response.hpp"

namespace aries {
namespace web {
class Handler {
public:
  virtual STATUS handle(Request *req, Response *res) = 0;
};
}
}
