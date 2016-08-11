#pragma once

#include "app.hpp"

#define ARIES_REGISTER_ENGINE(x)                                               \
  BOOST_LOG_TRIVIAL(info) << "load engine " << #x;                             \
  aries::engines.push_back(new x);

namespace aries {
int main(int argc, char **argv);
}
