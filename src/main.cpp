#include "main.h"
#include "engines/platform.h"

#include <iostream>

int main(int argc, char** argv){
  aries::engines.push_back(new aries::platform::Engine);  
  aries::main(argc, argv);
}
