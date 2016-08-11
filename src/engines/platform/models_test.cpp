#include "models.hpp"

#include <gtest/gtest.h>
#include <iostream>

TEST(platform, models) {
  aries::orm::Dialect dia;
  std::cout << "table name: " << dia.tableName<aries::platform::User>()
            << std::endl;
  std::cout << "first: " << dia.first<aries::platform::User>() << std::endl;
}
