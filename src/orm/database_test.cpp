#include "postgresql.hpp"

#include <gtest/gtest.h>
#include <iostream>

bool test_pg_db(aries::orm::Driver *drv) {
  auto dia = new aries::orm::Dialect;

  dia->addMigration(
      aries::orm::dialect::postgresql, "test.1",
      {"create table t11(id int)", "create table t12(id int)",
       "create table t13(id int)", "create table t14(id int)"},
      {"drop table t14", "drop table t13", "drop table t12", "drop table t11"});

  dia->addMigration(
      aries::orm::dialect::postgresql, "test.2",
      {"create table t21(id int)", "create table t22(id int)",
       "create table t23(id int)", "create table t24(id int)"},
      {"drop table t24", "drop table t23", "drop table t22", "drop table t21"});

  dia->addMigration(aries::orm::dialect::postgresql, "test.3",
                    {"create table t31(id int)", "create table t32(id int)",
                     "create table t33(id int)", "create table t34(id int)"},
                    {"drop table t34", "drop table t33", "drop table t32",
                     "drop table t31 "});

  auto db = new aries::orm::DB(drv, dia);
  db->initScheme();

  db->migrate();
  db->rollback();
  return true;
}

bool test_driver(aries::orm::Driver *db) {

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

TEST(orm, postgresql) {
  aries::orm::Driver *drv = new aries::orm::PostgreSql(
      "localhost", 5432, "aries_t", "postgres", "", "disable", 10);
  drv->open();
  ASSERT_TRUE(test_driver(drv));
  ASSERT_TRUE(test_pg_db(drv));
}
