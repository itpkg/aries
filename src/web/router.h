#ifndef AIRES_WEB_ROUTER_H
#define AIRES_WEB_ROUTER_H

namespace aries{
  namespace web{
    class Router{
    public:
      void get();
      void post();
      void put();
      void patch();
      void _delete();
    };
  }
}


#endif
