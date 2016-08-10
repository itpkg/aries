#ifndef AIRES_ORM_CACHE_CACHE_H
#define AIRES_ORM_CACHE_CACHE_H

#include <vector>
#include <cstdlib>

namespace aries{
  class Cache{
  public:
    virtual void set(const char* key, const char* val, uint ttl) = 0;
    virtual const char* get(const char* key) = 0;
    virtual void del(const char* key) = 0;
    virtual std::vector<const char*> keys() = 0;
  };
}

#endif
