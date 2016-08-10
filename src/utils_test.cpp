#include "utils.hpp"

#include <gtest/gtest.h>
#include <iomanip>

TEST(utils, random_and_base64) {
  auto v = aries::random_bytes(8);
  std::cout << "random bytes: " << std::hex;
  for (auto const &c : v) {
    std::cout << int(c) << ' ';
  }
  std::cout << std::dec << std::endl;

  std::string s1(v.begin(), v.end());
  auto s2 = aries::to_base64(s1);
  std::cout << "base64: " << s2 << std::endl;

  auto s3 = aries::from_base64(s2);
  EXPECT_EQ(s1, s3);
}
