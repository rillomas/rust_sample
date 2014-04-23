cmake_minimum_required(VERSION 2.8)
project(winapi)
set(CMAKE_C_FLAGS_RELEASE "-Wall -O2")
set(CMAKE_C_FLAGS_DEBUG "-Wall -g")
set(CMAKE_BUILD_TYPE Debug)

set(ROOT_DIR winapi)
set(SRC_FILES ${ROOT_DIR}/winapi.c)

add_library(winapi STATIC ${SRC_FILES})

add_executable(winapi_sample winapi_sample.c)
target_link_libraries(winapi_sample winapi)
