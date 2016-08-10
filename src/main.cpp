#include "app.h"

#include "log.h"
#include "utils.h"

#include <boost/program_options.hpp>
#include <boost/filesystem.hpp>

#include <iostream>


void init(std::string appName){
  #ifdef NDEBUG
    aries::log::init_native_syslog();
    //aries::log::init_file(appName);
  #else

  #endif
}

namespace po = boost::program_options;

int main(int argc, char** argv){

  try
    {
      std::string appName = boost::filesystem::basename(argv[0]);
      init(appName);

      po::options_description nginx("Nginx options");
      nginx.add_options()
        ("ssl", "enable ssl certs.")
      ;

      po::options_description server("Server options");

      server.add_options()
        ("host,H", po::value<std::string>()->default_value("localhost"), "listening port.")
        ("port,p", po::value<uint>()->default_value(8080), "listening port.")
        ("jobs,j", po::value<uint>()->default_value(4), "allow N worker jobs at once.")
        ("daemon,D", "daemon mode.")
      ;

      po::options_description database("Database options");
      database.add_options()
        ("db-create", "create new database.")
        ("db-connect", "connect to database.")
        ("db-migrate", "migrate database.")
        ("db-seed", "insert seed data into database.")
        ("db-rollback", "rollback database changes.")
        ("db-drop", "drop database.")
      ;

      po::options_description cache("cache options");
      cache.add_options()
        ("cache-list", "list all items.")
        ("cache-flush", "clear items.")
        ("cache-connect", "connect cache.")
      ;

      po::options_description desc("Global options");
      desc
        .add(server)
        .add(database)
        .add(cache)
        .add(nginx)
        .add_options()
        ("init,i", "generate config file.")
        ("config,c", po::value<std::string>()->default_value("config.yaml"), "load config from file.")

        ("help,h", "print help messages.")
        ("version,v", "print application version.")
      ;



      po::variables_map vm;
      std::string config;
      try
      {
        po::store(po::parse_command_line(argc, argv, desc), vm);

        config = vm["config"].as<std::string>();
        if ( vm.count("help")  )
        {
          std::cout << appName
            << " is build by aries web framework(https://github.com/itpkg/aries)."
            << std::endl
            << std::endl;
          std::cout << desc
            <<std::endl;
          return EXIT_SUCCESS;
        }

        if ( vm.count("version")  )
        {
          std::cout << "2016.08.09" << std::endl;
          return EXIT_SUCCESS;
        }

        if ( vm.count("init")  )
        {
          BOOST_LOG_TRIVIAL(info) << "generate file " << config;
          YAML::Node node;
          auto buf = aries::random_bytes(512);
          std::string secret(buf.begin(),buf.end());
          node["secret"] = aries::to_base64(secret);
          node["database"]["driver"] = "postgres";
          node["database"]["url"] = "postgres@localhost:5432/aries";
          node["cache"]["driver"] = "redis";
          node["cache"]["url"] = "tcp://localhost:6379/2";

          if ( std::ifstream(config) ) {
            throw std::invalid_argument("file already exists");
          }

          std::ofstream fout(config);
          fout << node;
          fout.close();
          return EXIT_SUCCESS;
        }


        po::notify(vm);
      }
      catch(po::error& e)
      {
        std::cerr << "ERROR: " << e.what() << std::endl << std::endl;
        std::cerr << desc << std::endl;
        return EXIT_FAILURE;
      }

      // application code here //
      fruit::Injector<aries::App> injector(aries::getApp(config));
      aries::App* app = injector.get<aries::App*>();
      //app->start();
      throw std::invalid_argument("not support");
    }
    catch(std::exception& e)
    {
      std::cerr << "Unhandled exception: "
                << e.what() << ", application will now exit." << std::endl;
      return EXIT_FAILURE;

    }

    return EXIT_SUCCESS;
}
