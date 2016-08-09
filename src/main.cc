#include <docopt.h>

#include <iostream>

static const char USAGE[] =
R"(A c++ web framwork.

    Usage:
      aries ship new <name>...
      aries ship <name> move <x> <y> [--speed=<kn>]
      aries ship shoot <x> <y>
      aries mine (set|remove) <x> <y> [--moored | --drifting]
      aries (-h | --help)
      aries (-v | --version)

    Options:
      -h --help     Show this screen.
      -v --version  Show version.
      --speed=<kn>  Speed in knots [default: 10].
      --moored      Moored (anchored) mine.
      --drifting    Drifting mine.
)";

int main(int argc, char** argv){
  std::map<std::string, docopt::value> args
        = docopt::docopt(USAGE,
                         { argv + 1, argv + argc },
                         true,
                         "2016.08.09");

    for(auto const& arg : args) {
        std::cout << arg.first <<  arg.second << std::endl;
    }

    return 0;
}
