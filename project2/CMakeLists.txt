cmake_minimum_required(VERSION 3.10)
project(CgiExplorer VERSION 1.0 LANGUAGES CXX)

# C++17 Standard erzwingen
set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

# Header-Verzeichnis einbinden
include_directories(include)

# Alle Quelldateien sammeln
file(GLOB SOURCES "src/*.cpp")

# Das Executable erstellen
add_executable(explorer.cgi ${SOURCES})

# Filesystem-Bibliothek linken (für ältere g++ Versionen notwendig)
target_link_libraries(explorer.cgi stdc++fs)

# Optional: Installationsziel festlegen (z.B. nach /usr/lib/cgi-bin/)
# install(TARGETS explorer.cgi DESTINATION /usr/lib/cgi-bin)

