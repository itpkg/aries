cmake_minimum_required (VERSION 2.8)
project (aries CXX)

aux_source_directory(src LIB_SRCS)
list (REMOVE_ITEM LIB_SRCS src/main.cpp)

# link_directories(/usr/local/lib)
# include_directories(${CMAKE_CURRENT_BINARY_DIR}/include)
# link_directories(${CMAKE_CURRENT_BINARY_DIR}/lib)

#-------boost--------------------------
ADD_DEFINITIONS(-DBOOST_LOG_DYN_LINK)
ADD_DEFINITIONS(-DBOOST_LOG_USE_NATIVE_SYSLOG)

#-------library------------------------
add_library(aries SHARED ${LIB_SRCS})
target_compile_features(aries PRIVATE cxx_range_for)

#-------binary-------------------------

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
set_target_properties(demo PROPERTIES OUTPUT_NAME aries)
#--------------3rd library-------------
# include(ExternalProject)
#
# ExternalProject_Add(fruit
#   # DOWNLOAD_NAME fruit-2.0.4.tar.gz
#   # URL https://github.com/google/fruit/archive/v2.0.4.tar.gz
#   GIT_REPOSITORY https://github.com/google/fruit.git
#   # INSTALL_DIR ${PROJECT_BINARY_DIR}
#   # CMAKE_ARGS -DCMAKE_INSTALL_PREFIX:PATH=<INSTALL_DIR>
# )
# # ExternalProject_Get_Property(fruit install_dir)
# add_dependencies(aries fruit)
#--------install-----------------------

install (TARGETS aries demo
 RUNTIME DESTINATION bin
 ARCHIVE DESTINATION lib
 LIBRARY DESTINATION lib
 CONFIGURATIONS Release
)

install (DIRECTORY src/
 DESTINATION include/aries
 FILES_MATCHING PATTERN "*.h"
)
# install (DIRECTORY ${CMAKE_CURRENT_BINARY_DIR}/include/
#   DESTINATION include
# )
# install (DIRECTORY ${CMAKE_CURRENT_BINARY_DIR}/lib/
#   DESTINATION lib
#   #FILE_PERMISSIONS "0755"
# )

#--------uninstall---------------------

configure_file(
   "${CMAKE_CURRENT_SOURCE_DIR}/cmake_uninstall.cmake.in"
   "${CMAKE_CURRENT_BINARY_DIR}/cmake_uninstall.cmake"
   IMMEDIATE @ONLY)

add_custom_target(uninstall
   COMMAND ${CMAKE_COMMAND} -P ${CMAKE_CURRENT_BINARY_DIR}/cmake_uninstall.cmake)

#--------------testing-----------------
enable_testing()
find_package(GTest REQUIRED)
#find_package(GMock REQUIRED)
include_directories(
    ${GTEST_INCLUDE_DIRS}
  #  ${GMOCK_INCLUDE_DIRS}
)
