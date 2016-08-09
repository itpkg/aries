cmake_minimum_required (VERSION 2.8)
project (aries)

aux_source_directory(src ARIES_SRCS)

find_package(gflags REQUIRED)


add_executable(aries ${ARIES_SRCS})
target_link_libraries(aries gflags pthread)
