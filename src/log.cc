#include "log.hpp"

#include <boost/log/attributes/timer.hpp>
#include <boost/log/expressions.hpp>
#include <boost/log/sinks/sync_frontend.hpp>
#include <boost/log/sinks/syslog_backend.hpp>
#include <boost/log/support/date_time.hpp>
#include <boost/log/utility/setup/common_attributes.hpp>
#include <boost/log/utility/setup/file.hpp>

namespace logging = boost::log;
namespace sinks = boost::log::sinks;
namespace keywords = boost::log::keywords;
namespace expr = boost::log::expressions;

typedef sinks::synchronous_sink<sinks::syslog_backend> sink_t;

namespace aries {
namespace log {
void show_debug(bool b) {
  if (!b) {
    logging::core::get()->set_filter(logging::trivial::severity >=
                                     logging::trivial::info);
  }
}

void use_native_syslog_backend() {
  boost::shared_ptr<logging::core> core = logging::core::get();

  // Create a backend
  boost::shared_ptr<sinks::syslog_backend> backend(
      new sinks::syslog_backend(keywords::facility = sinks::syslog::user,
                                keywords::use_impl = sinks::syslog::native));

  // Set the straightforward level translator for the "Severity" attribute of
  // type int
  backend->set_severity_mapper(
      sinks::syslog::direct_severity_mapping<int>("Severity"));

  // Wrap it into the frontend and register in the core.
  // The backend requires synchronization in the frontend.
  core->add_sink(boost::make_shared<sink_t>(backend));
}

void use_file_backend(std::string appName) {

  logging::add_file_log(
      keywords::file_name = appName + "_%N.log",
      keywords::rotation_size = 10 * 1024 * 1024,
      keywords::time_based_rotation =
          sinks::file::rotation_at_time_point(0, 0, 0),
      keywords::auto_flush = true,
      keywords::open_mode = (std::ios::out | std::ios::app),
      keywords::format =
          (expr::stream << expr::format_date_time<boost::posix_time::ptime>(
                               "TimeStamp", "%Y-%m-%d %H:%M:%S")
                        << " [" << logging::trivial::severity << "] "
                        << expr::smessage));

  logging::add_common_attributes();
  expr::attr<logging::trivial::severity_level>("Severity");
  logging::core::get()->add_global_attribute("Uptime",
                                             logging::attributes::timer());
}
}
}
