# RETA - Tabellenverarbeitungsprogramm

RETA ist ein leistungsstarkes C++ Programm zur Verarbeitung und Darstellung von Tabellendaten mit Unterstützung für verschiedene Ausgabeformate und erweiterte Funktionen.

## Funktionen

- **Mehrere Ausgabeformate**: Shell, HTML, BBCode, Markdown, CSV, Emacs
- **Flexible Filterung**: Zeilen- und Spaltenfilter mit komplexen Bedingungen
- **Tabellenkombinationen**: SQL-ähnliche JOIN-Operationen zwischen Tabellen
- **Internationalisierung**: Mehrsprachige Unterstützung (Deutsch, Englisch, Französisch, Spanisch)
- **Farbausgabe**: Farbige Terminalausgabe mit ANSI Escape Codes
- **CSV-Verarbeitung**: Einlesen und Verarbeiten von CSV-Dateien
- **Mathematische Funktionen**: Primzahlberechnung, Faktorisierung, spezielle Zahlen

## Installation

### Voraussetzungen

- C++17 kompatibler Compiler (g++ 7+ oder clang++ 7+)
- Make (optional, aber empfohlen)

### Kompilierung

```bash
# Mit Make (empfohlen)
make clean
make

# Oder mit Build-Skript
chmod +x build.sh
./build.sh

# Für Windows
build.bat
