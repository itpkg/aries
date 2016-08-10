#ifndef AIRES_ENGINES_PLATFORM_H
#define AIRES_ENGINES_PLATFORM_H

#include "../web/engine.h"

namespace aries{
    namespace platform{
        class Engine : public web::Engine{
        public:
          void mount(web::Router* rt);
          YAML::Node config();
          void init();
        };


    }
}
#endif
