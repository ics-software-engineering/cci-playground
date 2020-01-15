cmake_minimum_required(VERSION 3.14)
project(cpp)

set(CMAKE_CXX_STANDARD 14)

include_directories(src)
include_directories(test)

add_executable(cpp
        src/IsUnique.cpp
        src/IsUnique.hpp
        test/TestIsUnique.cpp
        test/TestIsUnique.hpp
        test/UnitTests.cpp)
