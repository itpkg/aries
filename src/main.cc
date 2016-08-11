#include "console.hpp"
#include "main.hpp"

#include <boost/filesystem.hpp>
#include <boost/program_options.hpp>

#define ARIES_REGISTER_OPTION(n, f)                                            \
  if (vm.count(n)) {                                                           \
    console::f;                                                                \
    return EXIT_SUCCESS;                                                       \
  }

namespace po = boost::program_options;

namespace aries {

App *getApp(std::string file) {
  fruit::Component<App> root = fruit::createComponent();
  BOOST_LOG_TRIVIAL(info) << "load config from file " << file;
  YAML::Node node = YAML::LoadFile(file);

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

    po::options_description nginx("Nginx files");
    nginx.add_options()("nginx", "[TODO] generate nginx.config(http).")(
        "nginx-with-ssl", "[TODO] generate nginx.config(https).");

    po::options_description server("Server options");

    server.add_options()("port,p", po::value<uint>()->default_value(8080),
                         "[TODO] start web server if > 0.")(
        "jobs,j", po::value<uint>()->default_value(4),
        "[TODO] start N worker jobs at once if > 0.")("daemon,D",
                                                      "[TODO] daemon mode.");

    po::options_description config("Config files operations");
    config.add_options()("init,i", "generate config files if not exists.");

    po::options_description database("Database operations");
    database.add_options()("db-create", "[TODO] create new database.")(
        "db-console", "[TODO] connect to database.")(
        "db-migrate", "[TODO] migrate database.")(
        "db-seed", "[TODO] insert seed data into database.")(
        "db-rollback", "[TODO] rollback database changes.")(
        "db-drop", "[TODO] drop database.");

    po::options_description desc("Global options");
    desc.add(config).add(server).add(database).add(nginx).add_options()(
        "config,c", po::value<std::string>()->default_value("config.yaml"),
        "config file's name.")("help,h", "print help messages.")(
        "version,v", "print application version.");

    po::variables_map vm;
    std::string cfg;
    try {
      po::store(po::parse_command_line(argc, argv, desc), vm);

      cfg = vm["config"].as<std::string>();

      ARIES_REGISTER_OPTION("help", show_help(appName, desc))
      ARIES_REGISTER_OPTION("version", show_version())
      ARIES_REGISTER_OPTION("init", init_config(cfg))
      ARIES_REGISTER_OPTION("db-migrate", db_migrate(cfg))
      ARIES_REGISTER_OPTION("db-create", db_create(cfg))
      ARIES_REGISTER_OPTION("db-drop", db_drop(cfg))
      ARIES_REGISTER_OPTION("db-console", db_console(cfg))
      ARIES_REGISTER_OPTION("db-rollback", db_rollback(cfg))

      po::notify(vm);
    } catch (po::error &e) {
      std::cerr << "ERROR: " << e.what() << std::endl << std::endl;
      std::cerr << desc << std::endl;
      return EXIT_FAILURE;
    }

    console::show_help(appName, desc);

  } catch (std::exception &e) {
    std::cerr << "Unhandled exception: " << e.what()
              << ", application will now exit." << std::endl;
    return EXIT_FAILURE;
  }

  return EXIT_SUCCESS;
}
}
