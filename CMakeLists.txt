cmake_minimum_required (VERSION 2.8)
project (aries CXX)

# set(CMAKE_FIND_LIBRARY_SUFFIXES ".a")
# set(BUILD_SHARED_LIBRARIES OFF)
# set(CMAKE_EXE_LINKER_FLAGS "-static")

include_directories ("${PROJECT_SOURCE_DIR}/vendor/docopt")
#add_subdirectory (vendor/docopt)

aux_source_directory(src ARIES_SRCS)

find_package(gflags REQUIRED)
#find_package(docopt REQUIRED)


add_executable(aries ${ARIES_SRCS} vendor/docopt/docopt.cpp)
target_link_libraries(aries pthread)
target_compile_features(aries PRIVATE cxx_range_for)
