#ifndef AIRES_ENGINES_BOOKS_H
#define AIRES_ENGINES_BOOKS_H

#include "../web/engine.h"

namespace aries{
    namespace books{
        class Engine : public web::Engine{
        public:
          std::vector<orm::Migration*> migrations();
          std::vector<std::string> seed();
          std::map<std::string, std::string> queries();
          void mount(web::Router* rt);
          YAML::Node config();
          void init();
        };
    }
}
#endif
