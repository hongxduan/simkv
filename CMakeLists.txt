cmake_minimum_required(VERSION 4.0)
project(simkv)

set(CMAKE_CXX_STANDARD 23)

add_executable(simkv main.cpp
        posix/server.cpp
        posix/server.h
        util/string_util.cpp
        util/string_util.h
        config/config.cpp
        config/config.h
        db/value.cpp
        db/value.h
        executor/executor.cpp
        executor/executor.h
        executor/command.h
        kvtp/request.cpp
        kvtp/request.h
        kvtp/response.cpp
        kvtp/response.h
        util/byte_util.cpp
        util/byte_util.h
        inc/type.h
        db/db.cpp
        db/db.h
        db/key.h
        db/key.cpp
        kvtp/kvtp.h
)