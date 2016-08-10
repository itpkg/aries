#include "postgresql.hpp"

#include <gtest/gtest.h>

bool test_db(aries::DB *db) {
  db->call("SELECT CURRENT_TIMESTAMP", {});
  db->call("CREATE TABLE IF NOT EXISTS tests(id SERIAL, msg1 VARCHAR(255), "
           "msg2 VARCHAR(255), msg3 VARCHAR(255))",
           {});

  db->call("INSERT INTO tests(msg1, msg2, msg3) VALUES($1, $2, $3)",
           {"111", "222", "333"});
  return true;
}

TEST(db, postgresql) {
  ASSERT_TRUE(test_db(new aries::database::PostgreSql(
      "localhost", 5432, "aries_t", "postgres", "", "disable", 10)));
}
