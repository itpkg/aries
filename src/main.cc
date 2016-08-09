#include <boost/program_options.hpp>
#include <boost/filesystem.hpp>

#include <boost/log/trivial.hpp>
#include <boost/log/utility/setup/file.hpp>
#include <boost/log/attributes/timer.hpp>
#include <boost/log/utility/setup/common_attributes.hpp>

#include <iostream>
#include <string>
#include <cstdlib>

namespace logging = boost::log;

void init(){
  #ifdef NDEBUG

    logging::core::get()->set_filter
    (
        logging::trivial::severity >= logging::trivial::info
    );



    logging::add_file_log(
            logging::keywords::file_name = "out.log",
            logging::keywords::auto_flush = true,
            logging::keywords::open_mode = (std::ios::out | std::ios::app),
            //logging::keywords::format = "%TimeStamp% [%Uptime%] (%LineID%) <%Severity%>: %Message%"
            logging::keywords::format = "%TimeStamp%: %Message%"
            );
    logging::add_common_attributes();
    //logging::expressions::attr< logging::trivial::severity_level >("Severity");
    //logging::core::get()->add_global_attribute("Uptime", logging::attributes::timer());

  #else

  #endif
}



int main(int argc, char** argv){
  init();

  try
    {
      std::string appName = boost::filesystem::basename(argv[0]);
      uint jobs = 0;
      bool server = false;
      bool daemon = false;
      std::string config = "config.toml";


      namespace po = boost::program_options;
      po::options_description desc("Options");
      desc.add_options()
        ("init,i", "init config file.")
        ("config,c", po::value<std::string>(&config), "load config from file, default: \"config.toml\".")
        ("server,s", "start web server.")
        ("jobs,j", po::value<uint>(&jobs), "allow N worker jobs at once, default: \"0\".")
        ("daemon,d", "daemon mode.")
        ("help,h", "print help messages.")
        ("version,v", "print application version.");

      po::variables_map vm;
      try
      {
        po::store(po::parse_command_line(argc, argv, desc),
                  vm);

        if ( vm.count("help")  )
        {
          std::cout << appName << " is a web framework for c++." << std::endl
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
          //TODO generate config file
          BOOST_LOG_TRIVIAL(info) << "generate file " << config << ".";
          return EXIT_SUCCESS;
        }

        if ( vm.count("daemon")  )
        {
          daemon = true;
        }

        if ( vm.count("server")  )
        {
          server = true;
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

    }
    catch(std::exception& e)
    {
      std::cerr << "Unhandled exception reached the top of main: "
                << e.what() << ", application will now exit" << std::endl;
      return EXIT_FAILURE;

    }

    return EXIT_SUCCESS;
}
