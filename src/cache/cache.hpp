#pragma once

#include <cstdlib>
#include <sstream>
#include <string>
#include <vector>

namespace aries {
namespace cache {
namespace dialect {
const std::string redis = "redis";
const std::string memcache = "memcache";
}

class Cache {
public:
  inline virtual std::string console() = 0;
  virtual void set(const char *key, const char *val, uint ttl) = 0;
  virtual const char *get(const char *key) = 0;
  virtual void del(const char *key) = 0;
  virtual void clear() = 0;
  virtual std::vector<const char *> keys() = 0;
};
}
}
