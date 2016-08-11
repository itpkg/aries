#include "console.hpp"
#include "web/engine.hpp"

#include <fstream>
#include <iostream>

namespace aries {
namespace console {

void do_init(std::string config) {
  BOOST_LOG_TRIVIAL(info) << "generate file " << config;

  if (std::ifstream(config)) {
    throw std::invalid_argument("file already exists");
  }

  std::ofstream fout(config);
  for (auto en : aries::engines) {
    fout << en->config();
    fout << std::endl;
  }
  fout.close();
}

void do_version() { std::cout << "2016.08.09" << std::endl; }
}
}
