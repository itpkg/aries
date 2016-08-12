#pragma once

#include <cstdlib>

namespace aries {
namespace web {
class Server {
public:
  virtual void start(int port, size_t threads) = 0;
};
}
}
