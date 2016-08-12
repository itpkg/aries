#pragma once

#include <string>

namespace aries {
namespace web {
typedef enum { GET, POST, PUT, PATCH, DELETE } METHOD;

class Request {
public:
  Request(char *method, char *path);
  ~Request();
  std::string header(std::string k);

private:
  METHOD method;
  std::string path;
};
}
}
