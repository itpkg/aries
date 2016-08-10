#include "platform.h"
#include "../utils.h"



namespace aries{
    namespace platform{

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
          items["user.find_by_id"] = "SELECT * FROM USERS WHERE ID = $1";
          return items;
        }

        void Engine::mount(web::Router* rt){

        }

        YAML::Node Engine::config(){
          YAML::Node node;
          auto buf = random_bytes(512);
          std::string secret(buf.begin(),buf.end());
          node["secret"] = to_base64(secret);
          node["database"]["driver"] = "postgres";
          node["database"]["url"] = "postgres@localhost:5432/aries";
          node["cache"]["driver"] = "redis";
          node["cache"]["url"] = "tcp://localhost:6379/2";
          return node;
        }

        void Engine::init(){

        }

    }

}
