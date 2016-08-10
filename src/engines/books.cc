#include "books.h"

namespace aries{
    namespace books{

        void Engine::mount(web::Router* rt){

        }

        YAML::Node Engine::config(){
          YAML::Node node;
          node["books"]["store"] = "tmp/books";
          return node;
        }

        void Engine::init(){

        }

    }

}
