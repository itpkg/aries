#include "platform.h"
#include "../utils.h"



namespace aries{
    namespace platform{

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
