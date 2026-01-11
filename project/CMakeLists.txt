cmake_minimum_required(VERSION 3.16)
project(docview LANGUAGES CXX)

set(CMAKE_CXX_STANDARD 20)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

# Binary neben build/
set(CMAKE_RUNTIME_OUTPUT_DIRECTORY
    ${CMAKE_SOURCE_DIR}/bin
)

include_directories(include)


add_executable(docs.cgi
    src/main.cpp
    src/html.cpp
    src/query.cpp
    src/renderer.cpp
    src/listview.cpp
    src/csv.cpp
    src/pandoc.cpp
    src/media_view.cpp  # <- wichtig
    src/fileview.cpp
)

# GCC 8 braucht das evtl. noch
target_link_libraries(docs.cgi stdc++fs)
