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


int main(int argc, char** argv){

  try
    {
      std::string appName = boost::filesystem::basename(argv[0]);
      init(appName);

      std::string config = "config.toml";


      namespace po = boost::program_options;
      po::options_description desc("Options");
      desc.add_options()
        ("init,i", "init config file.")
        ("config,c", po::value<std::string>(&config), "load config from file, default: \"config.yaml\".")
        // ("server,s", "start web server.")
        // ("jobs,j", po::value<uint>(&jobs), "allow N worker jobs at once, default: \"0\".")
        // ("daemon,d", "daemon mode.")
        ("help,h", "print help messages.")
        ("version,v", "print application version.");

      po::variables_map vm;
      try
      {
        po::store(po::parse_command_line(argc, argv, desc),
                  vm);

        if ( vm.count("help")  )
        {
          std::cout << appName << " is build by aries web framework(https://github.com/itpkg/aries)." << std::endl
                    << desc << std::endl;
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
          node["secrets"] = aries::random_bytes(512);
          node["http"]["host"]="localhost";
          node["http"]["port"]=8080;
          node["database"]["driver"] = "postgres";
          node["database"]["url"] = "postgres@localhost:5432/aries";
          node["cache"]["driver"] = "redis";
          node["cache"]["url"] = "tcp://localhost:6379/2";
          node["jobs"] = 5;
          node["daemon"] = false;

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
      app->start();
    }
    catch(std::exception& e)
    {
      std::cerr << "Unhandled exception: "
                << e.what() << ", application will now exit" << std::endl;
      return EXIT_FAILURE;

    }

    return EXIT_SUCCESS;
}
