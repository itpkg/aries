#pragma once

#include <string>

namespace aries {
namespace web {
typedef enum { OK = 200 } STATUS;
class Response {
public:
  Response();
  ~Response();
  void header(std::string k, std::string v);
  void json();
  void xml();
  void html();
  void text();
  void data();
  void abort(STATUS s);

private:
};
}
}
