cmake_minimum_required(VERSION 3.10)

project(hello C)

add_library(${PROJECT_NAME} SHARED hello.c)
