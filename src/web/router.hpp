#pragma once

#include "handler.hpp"
#include <initializer_list>

namespace aries {
namespace web {

class Router {
public:
  virtual void match(METHOD mth, std::string pat, Request *req,
                     Response *res) = 0;

  Router *GROUP(std::string path, std::initializer_list<Handler *> handlers);
  void GET(std::string path, std::initializer_list<Handler *> handlers);
  void POST(std::string path, std::initializer_list<Handler *> handlers);
  void PUT(std::string path, std::initializer_list<Handler *> handlers);
  void PATCH(std::string path, std::initializer_list<Handler *> handlers);
  void DELETE(std::string path, std::initializer_list<Handler *> handlers);
  void REST(std::string path, std::string name,
            std::initializer_list<Handler *> handlers);
  void ADD(METHOD method, std::string path,
           std::initializer_list<Handler *> handlers);
};
}
}
