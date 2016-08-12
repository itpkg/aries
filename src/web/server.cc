#include "server.hpp"

#include <boost/log/trivial.hpp>
#include <iostream>
#include <nghttp2/asio_http2_server.h>

namespace aries {
namespace web {
void Server::start(int port, size_t threads) {

  boost::system::error_code err;
  nghttp2::asio_http2::server::http2 server;

  server.handle("/", [](const nghttp2::asio_http2::server::request &req,
                        const nghttp2::asio_http2::server::response &res) {
    res.write_head(200);
    res.end("hello, world\n");
  });
  // server.num_threads(threads);
  if (server.listen_and_serve(err, "0.0.0.0", std::to_string(port).c_str())) {
    BOOST_LOG_TRIVIAL(error) << err.message();
  }
}
}
}
