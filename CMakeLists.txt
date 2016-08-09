cmake_minimum_required (VERSION 2.8)
project (aries CXX)

ADD_DEFINITIONS(-DBOOST_LOG_DYN_LINK)

aux_source_directory(src ARIES_SRCS)

find_package(gflags REQUIRED)

add_executable(aries ${ARIES_SRCS})
target_compile_features(aries PRIVATE cxx_range_for)
target_link_libraries(aries
  pthread
  boost_program_options
  boost_filesystem
  boost_system
  boost_log
  boost_thread
  boost_log_setup
  boost_date_time
)
