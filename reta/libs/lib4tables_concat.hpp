// lib4tables_concat.hpp
#pragma once

#include <vector>
#include <string>
#include <map>
#include <set>
#include <unordered_map>
#include <memory>
#include <fstream>
#include <sstream>
#include <algorithm>
#include <cmath>

// Hilfsstrukturen für Python-Äquivalente
namespace PyEquiv {
    // OrderedSet als std::set (behält Reihenfolge bei Einfügung)
    template<typename T>
    class OrderedSet : public std::set<T> {
    private:
        std::vector<T> insertionOrder;
    public:
        void insert(const T& value) {
            if (this->find(value) == this->end()) {
                std::set<T>::insert(value);
                insertionOrder.push_back(value);
            }
        }
        
        auto begin() const { return insertionOrder.begin(); }
        auto end() const { return insertionOrder.end(); }
    };
    
    // OrderedDict als std::map (sortiert nach Schlüssel)
    template<typename K, typename V>
    using OrderedDict = std::map<K, V>;
    
    // Fraction-Klasse
    class Fraction {
    private:
        int numerator;
        int denominator;
        
        void simplify() {
            int gcd = std::gcd(numerator, denominator);
            numerator /= gcd;
            denominator /= gcd;
            if (denominator < 0) {
                numerator = -numerator;
                denominator = -denominator;
            }
        }
        
    public:
        Fraction(int num = 0, int den = 1) : numerator(num), denominator(den) {
            if (denominator == 0) throw std::runtime_error("Denominator cannot be zero");
            simplify();
        }
        
        Fraction operator*(const Fraction& other) const {
            return Fraction(numerator * other.numerator, 
                          denominator * other.denominator);
        }
        
        Fraction operator/(const Fraction& other) const {
            return Fraction(numerator * other.denominator,
                          denominator * other.numerator);
        }
        
        double toDouble() const {
            return static_cast<double>(numerator) / denominator;
        }
        
        bool operator==(const Fraction& other) const {
            return numerator == other.numerator && 
                   denominator == other.denominator;
        }
        
        bool operator<(const Fraction& other) const {
            return numerator * other.denominator < other.numerator * denominator;
        }
        
        int getNumerator() const { return numerator; }
        int getDenominator() const { return denominator; }
    };
}

// Forward-Deklarationen
class Tables;  // Muss definiert werden
class I18n;    // Internationalisierung

// Enum für Tags
enum class ST {
    sternPolygon,
    galaxie,
    gleichfoermigesPolygon,
    universum,
    gebrRat
};

// Hauptklasse Concat
class Concat {
private:
    Tables* tables;
    
    // Python: OrderedSet() -> C++: OrderedSet<T>
    PyEquiv::OrderedSet<int> ones;
    PyEquiv::OrderedDict<int, std::vector<int>> CSVsSame;
    PyEquiv::OrderedDict<int, std::vector<std::vector<std::string>>> CSVsAlreadRead;
    
    // Brueche-Sets
    PyEquiv::OrderedSet<PyEquiv::Fraction> BruecheUni;
    PyEquiv::OrderedSet<PyEquiv::Fraction> BruecheGal;
    
    // Weitere Sets
    PyEquiv::OrderedSet<PyEquiv::Fraction> gebrRatMulSternUni;
    PyEquiv::OrderedSet<PyEquiv::Fraction> gebrRatDivSternUni;
    // ... weitere Sets
    
    // Hilfsmethoden
    std::vector<std::string> readCSV(const std::string& filename);
    PyEquiv::OrderedSet<PyEquiv::Fraction> getAllBrueche(
        const std::vector<std::vector<std::string>>& table);
    
public:
    // Konstruktor
    Concat(Tables* tables) : tables(tables) {
        // Initialisierung
        CSVsSame = {
            {1, {1}},
            {2, {2, 4}},
            {3, {3, 5}},
            {4, {2, 4}},
            {5, {3, 5}}
        };
    }
    
    // Methoden aus Python-Klasse
    std::pair<std::vector<std::vector<std::string>>, std::set<int>> 
    concatLovePolygon(const std::vector<std::vector<std::string>>& relitable, 
                     std::set<int> rowsAsNumbers);
    
    std::string gleichheitFreiheitVergleich(int zahl);
    
    std::string geistEmotionEnergieMaterieTopologie(int zahl);
    
    std::pair<std::vector<std::vector<std::string>>, std::set<int>> 
    concatGleichheitFreiheitDominieren(
        const std::vector<std::vector<std::string>>& relitable,
        std::set<int> rowsAsNumbers);
    
    // Weitere Methoden...
    
private:
    // Interne Hilfsmethoden
    PyEquiv::OrderedDict<int, PyEquiv::OrderedSet<std::pair<PyEquiv::Fraction, PyEquiv::Fraction>>>
    convertSetOfPaarenToDictOfNumToPaareMul(
        const PyEquiv::OrderedSet<std::pair<PyEquiv::Fraction, PyEquiv::Fraction>>& paareSet,
        bool gleichf = false);
    
    std::string spalteMetaKonkretTheorieAbstrakt_getGebrRatUnivStrukturalie(
        const PyEquiv::Fraction& koord,
        const std::pair<int, int>& n_and_invers_spalten,
        const std::vector<std::vector<std::string>>& gebrTable,
        bool isNotUniverse = true);
};
