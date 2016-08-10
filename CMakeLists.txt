cmake_minimum_required (VERSION 2.8)
project (aries CXX)

aux_source_directory(src LIB_SRCS)
list (REMOVE_ITEM LIB_SRCS src/main.cpp)

include_directories(${CMAKE_CURRENT_BINARY_DIR}/include)
link_directories(${CMAKE_CURRENT_BINARY_DIR}/lib)

#-------boost--------------------------
ADD_DEFINITIONS(-DBOOST_LOG_DYN_LINK)
ADD_DEFINITIONS(-DBOOST_LOG_USE_NATIVE_SYSLOG)

#-------library------------------------
add_library(aries ${LIB_SRCS})
target_compile_features(aries PRIVATE cxx_range_for)

#-------binary-------------------------

#
add_executable(demo ${LIB_SRCS} src/main.cpp)
target_compile_features(demo PRIVATE cxx_range_for)


target_link_libraries(demo
  pthread
  jemalloc
  yaml-cpp

  boost_program_options
  boost_filesystem
  boost_system
  boost_log
  boost_thread
  boost_log_setup
  boost_date_time

  fruit
)

#--------------3rd library-------------
include(ExternalProject)

ExternalProject_Add(fruit
  DOWNLOAD_DIR ${CMAKE_SOURCE_DIR}/vendor
  DOWNLOAD_NAME fruit-2.0.4.tar.gz
  URL https://github.com/google/fruit/archive/v2.0.4.tar.gz
  CMAKE_COMMAND cmake . -DCMAKE_INSTALL_PREFIX=<INSTALL_DIR>
  TEST_BEFORE_INSTALL 0
  BUILD_IN_SOURCE 1
  INSTALL_DIR ${CMAKE_BINARY_DIR}
  BUILD_COMMAND make
  INSTALL_COMMAND make install
)

#--------------testing-----------------
enable_testing()
find_package(GTest REQUIRED)
#find_package(GMock REQUIRED)
include_directories(
    ${GTEST_INCLUDE_DIRS}
  #  ${GMOCK_INCLUDE_DIRS}
)
