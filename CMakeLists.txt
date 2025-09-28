cmake_minimum_required(VERSION 4.0)
project(simkv)

set(CMAKE_CXX_STANDARD 20)

add_executable(simkv main.cpp
        posix/server.cpp
        posix/server.h
        parser/cnf_parser.cpp
        parser/cnf_parser.h)