cmake_minimum_required (VERSION 2.8)
project (aries CXX)


file(GLOB LIB_SRCS
  RELATIVE "${CMAKE_CURRENT_SOURCE_DIR}/"
  "src/*.cc"
  "src/**/*.cc"
  "src/**/**/*.cc"
)

file(GLOB TEST_SRCS
  RELATIVE "${CMAKE_CURRENT_SOURCE_DIR}"
  "src/*_test.cpp"
  "src/**/*_test.cpp"
  "src/**/**/*_test.cpp"
)

find_package(Boost 1.60 REQUIRED)
find_package(mstch REQUIRED)


# MESSAGE(STATUS "${LIB_SRCS}")

# FIXME
# find_package ( cppnetlib REQUIRED )
# include_directories (${CPPNETLIB_INCLUDE_DIRS} )

set(LIB_LINKS
  pthread
  jemalloc
  yaml-cpp
  ssl
  crypto

  hiredis
  pq

  boost_program_options
  boost_filesystem
  boost_system
  boost_log
  boost_thread
  boost_log_setup
  boost_date_time
  # ${CPPNETLIB_LIBRARIES}

  microhttpd
  fruit
  aries
)

set(LIB_INS
  libmstch.a
)


#-------boost--------------------------
ADD_DEFINITIONS(-DBOOST_LOG_DYN_LINK)
ADD_DEFINITIONS(-DBOOST_LOG_USE_NATIVE_SYSLOG)

#-------library------------------------
add_library(aries SHARED ${LIB_SRCS})
target_compile_features(aries PRIVATE cxx_range_for)
set_target_properties(aries PROPERTIES VERSION 1.0 SOVERSION 1)

#-------binary-------------------------

add_executable(demo src/main.cpp)
target_compile_features(demo PRIVATE cxx_range_for)

target_link_libraries(demo ${LIB_LINKS} ${LIB_INS})
set_target_properties(demo PROPERTIES OUTPUT_NAME aries)

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
add_executable(unit_tests ${TEST_SRCS})
target_compile_features(unit_tests PRIVATE cxx_range_for)
target_link_libraries(unit_tests ${LIB_LINKS} ${LIB_INS} GTest::GTest GTest::Main)

add_test(runUnitTests unit_tests)
