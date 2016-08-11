#include "engines/books.hpp"
#include "engines/platform/engine.hpp"
#include "main.hpp"

#include <iostream>

int main(int argc, char **argv) {
  ARIES_REGISTER_ENGINE(aries::platform::Engine)
  ARIES_REGISTER_ENGINE(aries::books::Engine)

  aries::main(argc, argv);
}
