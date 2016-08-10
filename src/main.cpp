#include "engines/books.hpp"
#include "engines/platform/engine.hpp"
#include "main.hpp"

#include <iostream>

int main(int argc, char **argv) {
  aries::engines.push_back(new aries::platform::Engine);
  aries::engines.push_back(new aries::books::Engine);
  aries::main(argc, argv);
}
