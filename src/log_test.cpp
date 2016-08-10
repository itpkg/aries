#include "log.hpp"

#include <gtest/gtest.h>

TEST(log, file) {
  aries::log::use_native_syslog_backend();
  BOOST_LOG_TRIVIAL(info) << "test file backend ";
}

TEST(log, syslog) {
  aries::log::use_file_backend("test");
  BOOST_LOG_TRIVIAL(info) << "test syslog backend";
}
