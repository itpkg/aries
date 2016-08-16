/*
* http://www.boost.org/doc/libs/1_61_0/doc/html/boost_asio/examples/cpp11_examples.html
*/
#include "boosthttpd.hpp"

#include <boost/network/protocol/http/server.hpp>
#include <iostream>

struct hello_world;
typedef boost::network::http::server<hello_world> server;

struct hello_world {
  void operator()(boost::network::server::request const &request,
                  boost::network::server::response &response) {
    server::string_type ip = source(request);
    unsigned int port = request.source_port;
    std::ostringstream data;
    data << "Hello, " << ip << ':' << port << '!';
    response = server::response::stock_reply(server::response::ok, data.str());
  }
  void log(const server::string_type &message) {
    std::cerr << "ERROR: " << message << std::endl;
  }
};

void start_httpd(int port) {
  hello_world handler_;
  boost::network::http::server::options options(handler_);
  boost::network::http::server server_(options.address("0.0.0.0").port("8080"));
  server_.run();
}

namespace aries {
namespace web {
struct hello_world;

void BoostHttpd::start(int port, size_t threads) { start_httpd(8080); }
}
}
