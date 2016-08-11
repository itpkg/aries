#pragma once

#include "cache.hpp"

#include <hiredis/hiredis.h>

namespace aries {
namespace cache {
class Redis : public Cache {
public:
  Redis(std::string prefix, std::string host, int port, int db, int timeout);
  ~Redis();
  std::string console();
  void set(const char *key, const char *val, uint ttl);
  const char *get(const char *key);
  void del(const char *key);
  void clear();
  std::vector<const char *> keys();

private:
  redisContext *ctx;
  const char *prefix;
  std::string host;
  int port;
  int db;
};
}
}
