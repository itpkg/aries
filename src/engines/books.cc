#include "books.h"

namespace aries{
    namespace books{
        std::vector<orm::Migration*> Engine::migrations(){
          std::vector<orm::Migration*> items;
          return items;
        }
        std::vector<std::string> Engine::seed(){
          std::vector<std::string> items;
          return items;
        }
        std::map<std::string, std::string> Engine::queries(){
          std::map <std::string, std::string> items;
          return items;
        }
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
