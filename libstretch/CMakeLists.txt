cmake_minimum_required(VERSION 3.21)
project(LibStretch)

set(CMAKE_CXX_STANDARD 14)

add_library(stretch STATIC stretch.cpp)

install(TARGETS stretch DESTINATION .)
