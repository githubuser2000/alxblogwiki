cmake_minimum_required(VERSION 3.16)
project(docview LANGUAGES CXX)

set(CMAKE_CXX_STANDARD 20)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

# 👉 bin neben build/, nicht darin
set(CMAKE_RUNTIME_OUTPUT_DIRECTORY
    ${CMAKE_SOURCE_DIR}/bin
)

include_directories(include)

add_executable(docview
    src/main.cpp
    src/html.cpp
    src/query.cpp
    src/renderer.cpp
    src/listview.cpp
    src/csv.cpp
    src/pandoc.cpp
)
