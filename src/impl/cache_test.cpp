#include "redis.hpp"
#include <gtest/gtest.h>

bool test_cache(aries::Cache *c) {
  char key[] = "hi";
  char val[] = "Hello, Aries!";
  c->set(key, val, 60 * 60);
  auto tmp = c->get(key);
  std::cout << "GET: " << key << " =>" << tmp << std::endl;
  if (strcmp(val, tmp) != 0) {
    return false;
  }

  std::cout << "KEYS: ";
  for (auto k : c->keys()) {
    std::cout << k << " ";
  }
  std::cout << std::endl;
  c->del(key);
  return true;
}

TEST(cache, redis) {
  ASSERT_TRUE(test_cache(new aries::cache::Redis("localhost", 6379, 1, 5)));
}
