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
    src/media_view.cpp
    src/fileview.cpp
    src/pdf_template.cpp
)

target_link_libraries(docs.cgi stdc++)

# PDFs werden direkt aus templates/ gelesen – kein Copy nötig!
# Wenn du trotzdem eine Kopie in bin/ willst, aktiviere nur, wenn Schreibrechte passen
# add_custom_command(TARGET docs.cgi POST_BUILD
#     COMMAND ${CMAKE_COMMAND} -E copy_if_different
#         "${CMAKE_SOURCE_DIR}/templates/pdf_template.html"
#         "${CMAKE_RUNTIME_OUTPUT_DIRECTORY}/pdf_template.html"
# )
