#include "redis.h"
#include "../log.h"

namespace aries{
  namespace cache{
      Redis::Redis(const char* host, int port, int db, int timeout){
        prefix = "cache://";
        ctx = redisConnectWithTimeout(host, port, {timeout});
        if(ctx == NULL){
          BOOST_LOG_TRIVIAL(error) << "can't allocate redis context";
        } else{
          if(ctx->err){
            BOOST_LOG_TRIVIAL(error) <<"redis connection error: " << ctx->errstr;
            redisFree(ctx);
          }else{
            auto reply = redisCommand(ctx, "SELECT %d",db);
            freeReplyObject(reply);
          }
        }


      }
      Redis::~Redis(){
        if(ctx!=NULL){
          redisFree(ctx);
        }
      }
      void Redis::set(const char* key, const char* val, uint ttl){
        if(ctx!=NULL){
          auto reply = redisCommand(ctx, "SET %s%s %s EX %d",prefix, key, val, ttl);
          freeReplyObject(reply);
        }
      }
      const char* Redis::get(const char* key){
        if(ctx!=NULL){
          auto reply = (redisReply*)redisCommand(ctx, "GET %s%s",prefix, key);
          char* buf = strdup(reply->str);
          freeReplyObject(reply);
          return buf;
        }
        return NULL;
      }
      void Redis::del(const char* key){
        if(ctx!=NULL){
          auto reply = redisCommand(ctx, "DEL %s%s",prefix, key);
          freeReplyObject(reply);
        }
      }
      std::vector<const char*> Redis::keys(){
        std::vector<const char*> items;
          if(ctx!=NULL){
            auto reply = (redisReply*)redisCommand(ctx, "KEYS %s*",prefix);
            for (size_t j = 0; j < reply->elements; j++) {
              items.push_back(strdup(reply->element[j]->str));
            }
            freeReplyObject(reply);
          }
        return items;
      }

  }
}
