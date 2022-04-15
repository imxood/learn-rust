cmake_minimum_required(VERSION 3.10)

project(hello C)

set(CMAKE_ARCHIVE_OUTPUT_DIRECTORY ${PROJECT_SOURCE_DIR}/out)

add_library(add_static STATIC hello.c)
