#pragma once

#include <cstdlib>

namespace aries {
namespace web {
class Server {
public:
  void start(int port, size_t threads);
};
}
}
