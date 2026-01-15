#!/bin/bash

# RETA Build Script
# Automatische Kompilierung und Installation

set -e  # Exit on error

echo "=== RETA Build Script ==="
echo ""

# Check for required tools
echo "Checking dependencies..."
if ! command -v g++ &> /dev/null; then
    echo "Error: g++ compiler not found!"
    echo "Please install g++ (GNU C++ compiler)"
    exit 1
fi

if ! command -v make &> /dev/null; then
    echo "Warning: make not found, using direct compilation..."
    USE_MAKE=false
else
    USE_MAKE=true
fi

# Create necessary directories
echo "Creating directories..."
mkdir -p obj
mkdir -p bin
mkdir -p csv

# Compile
echo "Compiling RETA..."
if [ "$USE_MAKE" = true ]; then
    make clean
    make
else
    # Manual compilation
    echo "Manual compilation..."
    g++ -std=c++17 -Wall -Wextra -O2 -I. -c main.cpp -o obj/main.o
    g++ -std=c++17 -Wall -Wextra -O2 -I. -c error.cpp -o obj/error.o
    g++ -std=c++17 -Wall -Wextra -O2 -I. -c types.cpp -o obj/types.o
    g++ -std=c++17 -Wall -Wextra -O2 -I. -c utils.cpp -o obj/utils.o
    g++ -std=c++17 -Wall -Wextra -O2 -I. -c i18n.cpp -o obj/i18n.o
    g++ -std=c++17 -Wall -Wextra -O2 -I. -c tables.cpp -o obj/tables.o
    g++ -std=c++17 -Wall -Wextra -O2 -I. -c program.cpp -o obj/program.o
    
    # Link
    g++ -std=c++17 -Wall -Wextra -O2 -o bin/reta \
        obj/main.o obj/error.o obj/types.o obj/utils.o \
        obj/i18n.o obj/tables.o obj/program.o -lpthread
fi

# Create sample data
echo "Creating sample data..."
cat > csv/religion.csv << 'EOF'
Religion;Description;Number;Type;Category
Christentum;Christliche Religion;1;Monotheistisch;Abrahamitisch
Islam;Islamische Religion;2;Monotheistisch;Abrahamitisch
Hinduismus;Hinduistische Religion;3;Polytheistisch;Indisch
Buddhismus;Buddhistische Religion;4;Non-theistisch;Indisch
Judentum;Jüdische Religion;5;Monotheistisch;Abrahamitisch
EOF

# Test the build
echo "Testing the build..."
if [ -f "bin/reta" ]; then
    echo "Build successful!"
    echo ""
    echo "Usage examples:"
    echo "  ./bin/reta -h                          # Show help"
    echo "  ./bin/reta -zeilen --alles             # Show all rows"
    echo "  ./bin/reta --version                   # Show version"
else
    echo "Build failed!"
    exit 1
fi

echo ""
echo "=== Build completed successfully ==="
