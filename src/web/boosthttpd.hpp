#pragma once

#include "server.hpp"

namespace aries {
namespace web {
class BoostHttpd : public Server {
public:
  void start(int port, size_t threads);
};
}
}
