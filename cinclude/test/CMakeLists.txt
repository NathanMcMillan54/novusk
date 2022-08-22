cmake_minimum_required(VERSION ${CMAKE_VERSION})

project(TestStd
        LANGUAGES C
        DESCRIPTION "Test my custom Standard Library, with static libraries in ../lib, and headers in ../")

# Link to custom standard library
set(CMAKE_C_STANDARD_INCLUDE_DIRECTORIES ${CMAKE_CURRENT_SOURCE_DIR}/../inc)
set(CMAKE_C_STANDARD_LIBRARIES ${CMAKE_CURRENT_SOURCE_DIR}/../lib/libstdlib.a)
set(CMAKE_CXX_STANDARD_INCLUDE_DIRECTORIES ${CMAKE_CURRENT_SOURCE_DIR}/../inc)
set(CMAKE_CXX_STANDARD_LIBRARIES ${CMAKE_CURRENT_SOURCE_DIR}/../lib/libstdlib.a)

add_executable(stdlib_test ${CMAKE_CURRENT_SOURCE_DIR}/stdlib_test.c)