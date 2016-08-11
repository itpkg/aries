#include "postgresql.hpp"

#include <gtest/gtest.h>
#include <iostream>

bool test_db(aries::orm::DB *db) {
  db->query("SELECT CURRENT_TIMESTAMP", {});

  db->query("CREATE TABLE IF NOT EXISTS tests(id SERIAL, msg1 VARCHAR(255), "
            "msg2 VARCHAR(255), msg3 VARCHAR(255))",
            {});

  db->query("INSERT INTO tests(msg1, msg2, msg3) VALUES($1, $2, $3)",
            {"111", "222", "333"});

  auto val = db->query("SELECT * FROM tests", {});

  std::cout << "ITEMS: ";
  for (auto v : val) {
    std::cout << v << " ";
  }
  std::cout << std::endl;
  return true;
}

TEST(db, postgresql) {
  ASSERT_TRUE(test_db(new aries::orm::PostgreSql(
      "localhost", 5432, "aries_t", "postgres", "", "disable", 10)));
}
