cmake_minimum_required(VERSION 3.10)
project(tbb_1)

list(APPEND CMAKE_MODULE_PATH "${PROJECT_SOURCE_DIR}/..")

find_package(TBB COMPONENTS tbbmalloc tbbmalloc_proxy tbb_preview)

set(CMAKE_CXX_STANDARD 17)

add_executable(tbb_1 main.cpp)
target_link_libraries(tbb_1 tbb)