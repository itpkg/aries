cmake_minimum_required (VERSION 2.8)
project (aries)

aux_source_directory(src lib_src)

find_package(gflags REQUIRED)


#add_executable(aries demo/main.cc ${ARIES_SRCS})
#target_link_libraries(aries gflags pthread)

#add_library(aries SHARED ${ARIES_SRCS})
#add_library(ariesS STATIC ${ARIES_SRCS})
#set_target_properties(ariesS PROPERTIES OUTPUT_NAME aries)

add_library(aries OBJECT ${libsrc})
set_property(TARGET ${aries} PROPERTY POSITION_INDEPENDENT_CODE 1)
add_library(aries_shared SHARED $<TARGET_OBJECTS:aries>)
add_library(aries_static STATIC $<TARGET_OBJECTS:aries>)

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
