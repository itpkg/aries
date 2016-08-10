
#ifndef AIRES_ORM_CACHE_REDIS_H
#define AIRES_ORM_CACHE_REDIS_H

#include "cache.h"
#include <hiredis/hiredis.h>

namespace aries{
  namespace cache{
    class Redis: public Cache{
    public:
      Redis(const char* host, int port, int db, int timeout);
      ~Redis();
      void set(const char*key, const char* val, uint ttl);
      const char* get(const char* key);
      void del(const char* key);
      std::vector<const char*> keys();
    private:
      redisContext *ctx;
      const char* prefix;
    };
  }
}

#endif
