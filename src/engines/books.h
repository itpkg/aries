#ifndef AIRES_ENGINES_BOOKS_H
#define AIRES_ENGINES_BOOKS_H

#include "../web/engine.h"

namespace aries{
    namespace books{
        class Engine : public web::Engine{
        public:
          void mount(web::Router* rt);
          YAML::Node config();
          void init();
        };
    }
}
#endif
