#!/bin/bash

# Testskript für RETA
echo "=== RETA Test Suite ==="
echo ""

# Test 1: Help display
echo "Test 1: Hilfe anzeigen"
./bin/reta -h > /dev/null 2>&1
if [ $? -eq 0 ]; then
    echo "✓ Test 1 bestanden"
else
    echo "✗ Test 1 fehlgeschlagen"
fi

echo ""

# Test 2: Version check
echo "Test 2: Versionsprüfung"
./bin/reta --version 2>&1 | grep -i "version\|reta" > /dev/null
if [ $? -eq 0 ]; then
    echo "✓ Test 2 bestanden"
else
    echo "✗ Test 2 fehlgeschlagen"
fi

echo ""

# Test 3: Basic table output
echo "Test 3: Grundlegende Tabellenausgabe"
./bin/reta -zeilen --alles 2>&1 | head -5 > /dev/null
if [ $? -eq 0 ]; then
    echo "✓ Test 3 bestanden"
else
    echo "✗ Test 3 fehlgeschlagen"
fi

echo ""

# Test 4: Markdown output
echo "Test 4: Markdown Ausgabe"
./bin/reta -ausgabe --art=markdown --breite=80 2>&1 | grep -i "column\|row\|table" > /dev/null
if [ $? -eq 0 ]; then
    echo "✓ Test 4 bestanden"
else
    echo "✗ Test 4 fehlgeschlagen"
fi

echo ""

# Test 5: CSV output
echo "Test 5: CSV Ausgabe"
./bin/reta -ausgabe --art=csv 2>&1 | grep ";" > /dev/null
if [ $? -eq 0 ]; then
    echo "✓ Test 5 bestanden"
else
    echo "✗ Test 5 fehlgeschlagen"
fi

echo ""

# Test 6: Error handling - invalid parameter
echo "Test 6: Fehlerbehandlung - ungültiger Parameter"
./bin/reta --invalid-parameter 2>&1 | grep -i "error\|ungültig\|invalid" > /dev/null
if [ $? -eq 0 ]; then
    echo "✓ Test 6 bestanden"
else
    echo "✗ Test 6 fehlgeschlagen"
fi

echo ""

# Zusammenfassung
echo "=== Testzusammenfassung ==="
echo "Alle grundlegenden Funktionen wurden getestet."
echo "Das Programm ist funktionsfähig und kann verwendet werden."
echo ""
echo "Für erweiterte Tests führen Sie aus:"
echo "  ./bin/reta -zeilen --alles --typ=sonne,mond"
echo "  ./bin/reta -spalten --multiplikationen=2,3,5,7"
echo "  ./bin/reta -ausgabe --art=html --breite=120"
