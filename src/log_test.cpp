#include "log.h"

#include <gtest/gtest.h>

TEST(log, file)
{
    aries::log::init_native_syslog();
    BOOST_LOG_TRIVIAL(info) << "test file backend ";
}

TEST(log, syslog)
{
  aries::log::init_file("test");
  BOOST_LOG_TRIVIAL(info) << "test syslog backend";
}
