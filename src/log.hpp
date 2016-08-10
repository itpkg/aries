#ifndef AIRES_LOG_H
#define AIRES_LOG_H

#include <boost/log/trivial.hpp>
#include <string>

namespace aries {
namespace log {
void show_debug(bool b);
void use_native_syslog_backend();
void use_file_backend(std::string appName);
}
}

#endif
