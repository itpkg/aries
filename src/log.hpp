#ifndef AIRES_LOG_H
#define AIRES_LOG_H

#include <boost/log/trivial.hpp>

#include <string>

namespace aries{
  namespace log{
    void init_native_syslog();
    void init_file(std::string appName);
  }
}

#endif
