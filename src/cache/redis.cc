#include "redis.hpp"
#include <boost/log/trivial.hpp>

namespace aries {
namespace cache {

Redis::Redis(std::string prefix, std::string host, int port, int db,
             int timeout) {
  this->prefix = prefix.c_str();
  this->host = host;
  this->port = port;
  this->db = db;

  BOOST_LOG_TRIVIAL(debug) << "open redis: tcp://" << host << ":" << port << "/"
                           << db;
  this->ctx = redisConnectWithTimeout(host.c_str(), port, {timeout});
  if (this->ctx == NULL) {
    BOOST_LOG_TRIVIAL(error) << "can't allocate redis context";
  } else {
    if (this->ctx->err) {
      BOOST_LOG_TRIVIAL(error) << "redis connection error: "
                               << this->ctx->errstr;
      redisFree(this->ctx);
    } else {
      auto reply = redisCommand(this->ctx, "SELECT %d", db);
      freeReplyObject(reply);
    }
  }
}

Redis::~Redis() {
  if (this->ctx != NULL) {
    redisFree(this->ctx);
  }
}

std::string Redis::console() {
  std::ostringstream buf;
  buf << "redis-cli -h " << this->host << " -p " << this->port << " -n "
      << this->db;
  return buf.str();
}

void Redis::set(const char *key, const char *val, uint ttl) {
  if (this->ctx != NULL) {
    auto reply = redisCommand(this->ctx, "SET %s%s %s EX %d", this->prefix, key,
                              val, ttl);
    freeReplyObject(reply);
  }
}

const char *Redis::get(const char *key) {
  if (this->ctx != NULL) {
    auto reply =
        (redisReply *)redisCommand(this->ctx, "GET %s%s", this->prefix, key);
    char *buf = strdup(reply->str);
    freeReplyObject(reply);
    return buf;
  }
  return NULL;
}

void Redis::del(const char *key) {
  if (this->ctx != NULL) {
    auto reply = redisCommand(this->ctx, "DEL %s%s", this->prefix, key);
    freeReplyObject(reply);
  }
}
void Redis::clear() {
  BOOST_LOG_TRIVIAL(info) << "clear all cache items.";
  if (this->ctx != NULL) {
    auto keys = this->keys();
    if (!keys.empty()) {
      std::ostringstream buf;
      for (auto k : keys) {
        buf << " " << k;
      }
      auto reply = redisCommand(this->ctx, "DEL %s", buf.str().c_str());
      freeReplyObject(reply);
    }
  }
}

std::vector<const char *> Redis::keys() {
  std::vector<const char *> items;
  if (this->ctx != NULL) {
    auto reply =
        (redisReply *)redisCommand(this->ctx, "KEYS %s*", this->prefix);
    for (size_t j = 0; j < reply->elements; j++) {
      items.push_back(strdup(reply->element[j]->str));
    }
    freeReplyObject(reply);
  }
  return items;
}
}
}
