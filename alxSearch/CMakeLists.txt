cmake_minimum_required(VERSION 3.10)
project(WortTokenExtractor)

set(CMAKE_CXX_STANDARD 17)

include_directories(include)

add_executable(WortTokenExtractor
    src/main.cpp
    src/TokenExtractor.cpp
    src/FileUtils.cpp
    src/CSVWriter.cpp
)

# Binärdatei in bin/ statt build/
set_target_properties(WortTokenExtractor PROPERTIES
    RUNTIME_OUTPUT_DIRECTORY ${CMAKE_CURRENT_SOURCE_DIR}/bin
)
