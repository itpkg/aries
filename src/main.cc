#include "main.hpp"

#include <boost/filesystem.hpp>
#include <boost/program_options.hpp>

#include <iostream>

namespace po = boost::program_options;

namespace aries {

App *getApp(std::string file) {
  fruit::Component<App> root = fruit::createComponent();
  BOOST_LOG_TRIVIAL(info) << "load config from file " << file;
  YAML::Node node = YAML::LoadFile(file);

  // BOOST_LOG_TRIVIAL(debug) << node["database"]["driver"].as<std::string>();

  // TODO
  fruit::Injector<aries::App> injector(root);
  return injector.get<aries::App *>();
}

void init(std::string appName) {
#ifdef NDEBUG
  aries::log::show_debug(false);
  aries::log::use_native_syslog_backend();
#else

#endif
}

int main(int argc, char **argv) {

  try {
    std::string appName = boost::filesystem::basename(argv[0]);
    init(appName);

    po::options_description nginx("Nginx options");
    nginx.add_options()("nginx", "[TODO] generate nginx.config(http).")(
        "nginx-ssl", "[TODO] generate nginx.config(https).");

    po::options_description server("Server options");

    server.add_options()("host,H",
                         po::value<std::string>()->default_value("localhost"),
                         "[TODO] listening host.")(
        "port,p", po::value<uint>()->default_value(8080),
        "[TODO] listening port.")("jobs,j", po::value<uint>()->default_value(4),
                                  "[TODO] allow N worker jobs at once.")(
        "daemon,D", "[TODO] daemon mode.");

    po::options_description database("Database options");
    database.add_options()("db-create", "[TODO] create new database.")(
        "db-connect", "[TODO] connect to database.")(
        "db-migrate", "[TODO] migrate database.")(
        "db-seed", "[TODO] insert seed data into database.")(
        "db-rollback", "[TODO] rollback database changes.")(
        "db-drop", "[TODO] drop database.");

    po::options_description cache("Cache options");
    cache.add_options()("cache-list", "[TODO] list all items.")(
        "cache-flush", "[TODO] clear items.")("cache-connect",
                                              "[TODO] connect cache.");

    po::options_description desc("Global options");
    desc.add(server).add(database).add(cache).add(nginx).add_options()(
        "init,i", "generate config file.")(
        "config,c", po::value<std::string>()->default_value("config.yaml"),
        "load config from file.")

        ("help,h", "print help messages.")("version,v",
                                           "print application version.");

    po::variables_map vm;
    std::string config;
    try {
      po::store(po::parse_command_line(argc, argv, desc), vm);

      config = vm["config"].as<std::string>();
      if (vm.count("help")) {
        std::cout << appName << " is build by aries web "
                                "framework(https://github.com/itpkg/aries)."
                  << std::endl
                  << std::endl;
        std::cout << desc << std::endl;
        return EXIT_SUCCESS;
      }

      if (vm.count("version")) {
        std::cout << "2016.08.09" << std::endl;
        return EXIT_SUCCESS;
      }

      if (vm.count("init")) {
        BOOST_LOG_TRIVIAL(info) << "generate file " << config;

        if (std::ifstream(config)) {
          throw std::invalid_argument("file already exists");
        }

        std::ofstream fout(config);
        for (auto en : engines) {
          fout << en->config();
          fout << std::endl;
        }
        fout.close();
        return EXIT_SUCCESS;
      }

      po::notify(vm);
    } catch (po::error &e) {
      std::cerr << "ERROR: " << e.what() << std::endl << std::endl;
      std::cerr << desc << std::endl;
      return EXIT_FAILURE;
    }

    // application code here //
    auto app = getApp(config);
    // TODO
    throw std::invalid_argument("not support");
  } catch (std::exception &e) {
    std::cerr << "Unhandled exception: " << e.what()
              << ", application will now exit." << std::endl;
    return EXIT_FAILURE;
  }

  return EXIT_SUCCESS;
}
}
