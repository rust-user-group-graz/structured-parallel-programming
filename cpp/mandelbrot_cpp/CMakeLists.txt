cmake_minimum_required(VERSION 3.10)
project(mandelbrot_cpp)

list(APPEND CMAKE_MODULE_PATH "${PROJECT_SOURCE_DIR}/..")

find_package(TBB COMPONENTS tbbmalloc tbbmalloc_proxy tbb_preview)
find_package(Qt5Test)

set(CMAKE_AUTOMOC ON)
set(CMAKE_AUTOUIC ON)
set(CMAKE_INCLUDE_CURRENT_DIR ON)

set(CMAKE_CXX_STANDARD 17)

add_executable(mandelbrot_cpp main.cpp TestMandelbrot.cpp TestMandelbrot.h)
target_link_libraries(mandelbrot_cpp tbb)
target_link_libraries (mandelbrot_cpp Qt5::Test)