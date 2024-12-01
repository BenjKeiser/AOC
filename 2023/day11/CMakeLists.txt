cmake_minimum_required(VERSION 3.10.1)

project(aoc_galaxy)

include_directories(src)

add_executable (${CMAKE_PROJECT_NAME} src/main.cpp src/elf.cpp)