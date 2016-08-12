/*
* https://www.gnu.org/software/libmicrohttpd/tutorial.html
*/
#include "microhttpd.hpp"

#include <arpa/inet.h>
#include <boost/log/trivial.hpp>
#include <cstdio>
#include <cstdlib>
#include <cstring>
#include <iostream>
#include <microhttpd.h>

#define PAGE "hello, aries!"

namespace aries {
namespace web {
namespace microhttpd {
static int on_client_connect(void *cls, const sockaddr *addr,
                             socklen_t addrlen) {
  char buf[INET6_ADDRSTRLEN];
  BOOST_LOG_TRIVIAL(info) << "remote ip: "
                          << inet_ntop(addr->sa_family, addr->sa_data + 2, buf,
                                       INET6_ADDRSTRLEN);
  return MHD_YES;
}

static int parse_params(void *cls, enum MHD_ValueKind kind, const char *key,
                        const char *value) {
  BOOST_LOG_TRIVIAL(info) << key << ": " << value;
  return MHD_YES;
}

static int handle(void *cls, struct MHD_Connection *connection, const char *url,
                  const char *method, const char *version,
                  const char *upload_data, size_t *upload_data_size,
                  void **ptr) {
  BOOST_LOG_TRIVIAL(info) << version << " " << method << " " << url;
  MHD_get_connection_values(connection, MHD_HEADER_KIND, &parse_params, NULL);

  static int dummy;
  const char *page = (char *)cls;
  struct MHD_Response *response;
  int ret;

  if (0 != strcmp(method, "GET"))
    return MHD_NO; /* unexpected method */
  if (&dummy != *ptr) {
    /* The first time only the headers are valid,
       do not respond in the first round... */
    *ptr = &dummy;
    return MHD_YES;
  }
  if (0 != *upload_data_size)
    return MHD_NO; /* upload data in a GET!? */
  *ptr = NULL;     /* clear context pointer */
  response = MHD_create_response_from_buffer(strlen(page), (void *)page,
                                             MHD_RESPMEM_PERSISTENT);
  ret = MHD_queue_response(connection, MHD_HTTP_OK, response);
  MHD_destroy_response(response);
  return ret;
}
}

void MicroHttpd::start(int port, size_t threads) {
  MHD_Daemon *d = MHD_start_daemon(
      MHD_USE_SELECT_INTERNALLY, port, microhttpd::on_client_connect, NULL,
      &microhttpd::handle, (void *)PAGE, MHD_OPTION_END);
  if (d == NULL) {
    BOOST_LOG_TRIVIAL(info) << "failed to create new thread";
    return;
  }
  (void)getc(stdin);
  MHD_stop_daemon(d);
}
}
}
