#pragma once

#include "cache/cache.hpp"
#include "orm/database.hpp"

#include <boost/program_options/options_description.hpp>
#include <string>

namespace aries {
namespace console {
void show_help(std::string app_name,
               const boost::program_options::options_description &desc);
void show_version();

void init_config(std::string name);

void db_migrate(std::string cfg);
void db_console(std::string cfg);
void db_create(std::string cfg);
void db_drop(std::string cfg);
void db_rollback(std::string cfg);

void cache_console(std::string cfg);
void cache_clear(std::string cfg);

orm::DB *openDB(std::string file);
cache::Cache *openCache(std::string file);
orm::Driver *getDbDriver(std::string file);
}
}
