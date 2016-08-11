#include "postgresql.hpp"

#include <gtest/gtest.h>
#include <iostream>

bool test_db(aries::orm::DB *db) {
  db->init();
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

  db->addMigration(
      "test.1", {"create table t11(id int)", "create table t12(id int)",
                 "create table t13(id int)", "create table t14(id int)"},
      {"drop table t14", "drop table t13", "drop table t12", "drop table t11"});
  db->addMigration(
      "test.2", {"create table t21(id int)", "create table t22(id int)",
                 "create table t23(id int)", "create table t24(id int)"},
      {"drop table t24", "drop table t23", "drop table t22", "drop table t21"});
  db->addMigration(
      "test.3", {"create table t31(id int)", "create table t32(id int)",
                 "create table t33(id int)", "create table t34(id int)"},
      {"drop table t34", "drop table t33", "drop table t32", "drop table t31"});

  db->migrate();
  db->rollback();
  return true;
}

TEST(db, postgresql) {
  ASSERT_TRUE(test_db(new aries::orm::PostgreSql(
      "localhost", 5432, "aries_t", "postgres", "", "disable", 10)));
}
