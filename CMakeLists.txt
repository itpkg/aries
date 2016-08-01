cmake_minimum_required (VERSION 2.8)
project (aries)

aux_source_directory(src ARIES_SRCS)

add_executable(demo demo/main.cc ${ARIES_SRCS})
add_library(aries ${ARIES_SRCS})

install (TARGETS aries
 ARCHIVE DESTINATION lib
 LIBRARY DESTINATION lib
 CONFIGURATIONS Release
)

INSTALL (DIRECTORY src/
 DESTINATION include/aries
 FILES_MATCHING PATTERN "*.h"
 )

# uninstall target
configure_file(
   "${CMAKE_CURRENT_SOURCE_DIR}/cmake_uninstall.cmake.in"
   "${CMAKE_CURRENT_BINARY_DIR}/cmake_uninstall.cmake"
   IMMEDIATE @ONLY)

add_custom_target(uninstall
   COMMAND ${CMAKE_COMMAND} -P ${CMAKE_CURRENT_BINARY_DIR}/cmake_uninstall.cmake)
