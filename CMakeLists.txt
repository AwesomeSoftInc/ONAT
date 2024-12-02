project(onat)

cmake_minimum_required(VERSION 3.30)

set(CMAKE_EXPORT_COMPILE_COMMANDS ON)
set(CMAKE_CXX_STANDARD 20)
set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} -std=c++20")

file(GLOB_RECURSE sources 
    src/*.hpp src/*.cpp
)

# TODO: This is a Rust library! Having it here defeats the purpose of us using C++.
# Begrudingly (C++'s time functions suck dick) we must replace it with something native to C++.
include(FetchContent)
FetchContent_Declare(
    rs_systemtime
    GIT_REPOSITORY "https://github.com/IoIxD/rs_systemtime.git"
    GIT_PROGRESS TRUE
)
FetchContent_MakeAvailable(rs_systemtime)
include_directories(${RS_SYSTEMTIME_INCLUDE_DIRECTORY})


add_executable(${PROJECT_NAME} ${sources})