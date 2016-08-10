#include "main.h"
#include "engines/platform.h"
#include "engines/books.h"

#include <iostream>

int main(int argc, char** argv){
  aries::engines.push_back(new aries::platform::Engine);
  aries::engines.push_back(new aries::books::Engine);
  aries::main(argc, argv);
}
