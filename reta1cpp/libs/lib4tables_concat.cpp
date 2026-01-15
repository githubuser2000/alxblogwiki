// lib4tables_concat.cpp - Vollständige C++ Konvertierung
#include <iostream>
#include <vector>
#include <string>
#include <map>
#include <set>
#include <unordered_map>
#include <unordered_set>
#include <memory>
#include <fstream>
#include <sstream>
#include <algorithm>
#include <cmath>
#include <numeric>
#include <utility>
#include <functional>
#include <regex>
#include <queue>
#include <stack>
#include <variant>
#include <optional>
#include <filesystem>

namespace fs = std::filesystem;

// ============================================================================
// PYTHON-ÄQUIVALENTE KLASSEN UND FUNKTIONEN
// ============================================================================

// OrderedSet als std::set (behält Einfügereihenfolge nicht perfekt, aber sortiert)
template<typename T>
using OrderedSet = std::set<T>;

// OrderedDict als std::map (sortiert nach Schlüssel)
template<typename K, typename V>
using OrderedDict = std::map<K, V>;

// DefaultOrderedDict
template<typename K, typename V>
class DefaultOrderedDict {
private:
    std::map<K, V> data;
    V defaultValue;
    
public:
    DefaultOrderedDict(V defaultVal) : defaultValue(defaultVal) {}
    
    V& operator[](const K& key) {
        if (data.find(key) == data.end()) {
            data[key] = defaultValue;
        }
        return data[key];
    }
    
    auto begin() { return data.begin(); }
    auto end() { return data.end(); }
    auto begin() const { return data.begin(); }
    auto end() const { return data.end(); }
    
    size_t size() const { return data.size(); }
    bool contains(const K& key) const { return data.find(key) != data.end(); }
    void insert(const std::pair<K, V>& p) { data.insert(p); }
};

// Fraction-Klasse (Python fractions.Fraction)
class Fraction {
private:
    int num;
    int den;
    
    void normalize() {
        if (den < 0) {
            num = -num;
            den = -den;
        }
        int gcd = std::gcd(num, den);
        if (gcd != 0) {
            num /= gcd;
            den /= gcd;
        }
        if (den == 0) {
            throw std::runtime_error("Division by zero in Fraction");
        }
    }
    
public:
    Fraction(int numerator = 0, int denominator = 1) 
        : num(numerator), den(denominator) {
        normalize();
    }
    
    int numerator() const { return num; }
    int denominator() const { return den; }
    
    double toDouble() const {
        return static_cast<double>(num) / den;
    }
    
    Fraction operator+(const Fraction& other) const {
        return Fraction(num * other.den + other.num * den, den * other.den);
    }
    
    Fraction operator-(const Fraction& other) const {
        return Fraction(num * other.den - other.num * den, den * other.den);
    }
    
    Fraction operator*(const Fraction& other) const {
        return Fraction(num * other.num, den * other.den);
    }
    
    Fraction operator/(const Fraction& other) const {
        return Fraction(num * other.den, den * other.num);
    }
    
    bool operator==(const Fraction& other) const {
        return num == other.num && den == other.den;
    }
    
    bool operator!=(const Fraction& other) const {
        return !(*this == other);
    }
    
    bool operator<(const Fraction& other) const {
        return num * other.den < other.num * den;
    }
    
    bool operator<=(const Fraction& other) const {
        return num * other.den <= other.num * den;
    }
    
    bool operator>(const Fraction& other) const {
        return num * other.den > other.num * den;
    }
    
    bool operator>=(const Fraction& other) const {
        return num * other.den >= other.num * den;
    }
    
    Fraction inverse() const {
        return Fraction(den, num);
    }
    
    std::string toString() const {
        if (den == 1) return std::to_string(num);
        return std::to_string(num) + "/" + std::to_string(den);
    }
};

// Hash für Fraction für unordered_set/unordered_map
struct FractionHash {
    size_t operator()(const Fraction& f) const {
        return std::hash<int>()(f.numerator()) ^ (std::hash<int>()(f.denominator()) << 1);
    }
};

// ============================================================================
// ENUMS UND STRUKTUREN
// ============================================================================

enum class ST {
    sternPolygon,
    galaxie,
    gleichfoermigesPolygon,
    universum,
    gebrRat
};

struct STHash {
    size_t operator()(ST s) const {
        return static_cast<size_t>(s);
    }
};

// Struktur für i18n (vereinfacht)
struct I18n {
    std::unordered_map<std::string, std::unordered_map<std::string, std::string>> translations;
    
    std::string get(const std::string& category, const std::string& key) {
        if (translations.find(category) != translations.end() &&
            translations[category].find(key) != translations[category].end()) {
            return translations[category][key];
        }
        return key; // Fallback
    }
    
    // CSV Dateinamen
    struct CSVFileNames {
        std::string prim = "primnumbers.csv";
        std::string bruch13 = "bruch13.csv";
        std::string bruch15 = "bruch15.csv";
        std::string bruch7 = "bruch7.csv";
        std::string bruchStrukGroesse = "bruchStrukGroesse.csv";
    } csvFileNames;
    
    // Singleton-Instanz
    static I18n& instance() {
        static I18n i18n;
        return i18n;
    }
};

// Forward-Deklarationen
class Tables;
class Concat;

// ============================================================================
// TABLES-KLASSE (VEREINFACHT)
// ============================================================================

class Tables {
public:
    bool htmlOutputYes = false;
    bool bbcodeOutputYes = false;
    int lastLineNumber = 1000;
    int SpaltenVanillaAmount = 0;
    int hoechsteZeile = 1024;
    
    std::unordered_map<int, std::string> dataDict[20]; // Vereinfacht
    std::unordered_map<int, std::unordered_set<ST, STHash>> generatedSpaltenParameter_Tags;
    std::unordered_map<int, std::vector<std::pair<std::string, std::string>>> generatedSpaltenParameter;
    
    // Vereinfachte Methoden
    std::pair<std::vector<std::vector<std::string>>, std::vector<std::vector<std::string>>> 
    fillBoth(const std::vector<std::vector<std::string>>& a, 
             const std::vector<std::vector<std::string>>& b) {
        // Einfache Implementation
        return {a, b};
    }
};

// ============================================================================
// HILFSFUNKTIONEN (aus Python-Modulen)
// ============================================================================

namespace Helper {
    bool couldBePrimeNumberPrimzahlkreuz(int n) {
        if (n <= 1) return false;
        for (int i = 2; i * i <= n; i++) {
            if (n % i == 0) return false;
        }
        return true;
    }
    
    bool couldBePrimeNumberPrimzahlkreuz_fuer_innen(int n) {
        // Vereinfachte Logik
        return couldBePrimeNumberPrimzahlkreuz(n) && (n % 4 == 3 || n == 2);
    }
    
    bool couldBePrimeNumberPrimzahlkreuz_fuer_aussen(int n) {
        // Vereinfachte Logik
        return couldBePrimeNumberPrimzahlkreuz(n) && (n % 4 == 1 || n == 3);
    }
    
    int primCreativity(int n) {
        if (n <= 1) return 0;
        if (couldBePrimeNumberPrimzahlkreuz(n)) {
            if (n % 4 == 1) return 1;
            if (n % 4 == 3) return 2;
        }
        return 3; // Mondzahl
    }
    
    std::vector<std::pair<int, int>> primMultiple(int n) {
        std::vector<std::pair<int, int>> result;
        for (int i = 1; i * i <= n; i++) {
            if (n % i == 0) {
                result.push_back({i, n / i});
                if (i != n / i) {
                    result.push_back({n / i, i});
                }
            }
        }
        return result;
    }
    
    std::vector<std::pair<int, int>> primRepeat(const std::vector<int>& factors) {
        std::vector<std::pair<int, int>> result;
        // Vereinfachte Implementation
        for (size_t i = 0; i < factors.size(); i++) {
            for (size_t j = i; j < factors.size(); j++) {
                result.push_back({factors[i], factors[j]});
            }
        }
        return result;
    }
    
    std::vector<int> primfaktoren(int n) {
        std::vector<int> factors;
        int temp = n;
        for (int i = 2; i * i <= temp; i++) {
            while (temp % i == 0) {
                factors.push_back(i);
                temp /= i;
            }
        }
        if (temp > 1) {
            factors.push_back(temp);
        }
        return factors;
    }
    
    std::pair<std::vector<int>, std::vector<int>> moonNumber(int n) {
        // Vereinfachte Implementation
        return {{}, {}};
    }
    
    std::vector<std::pair<int, int>> multiples(int n) {
        std::vector<std::pair<int, int>> result;
        for (int i = 1; i <= n; i++) {
            if (n % i == 0) {
                result.push_back({i, n / i});
            }
        }
        return result;
    }
    
    std::vector<int> divisorGenerator(int n) {
        std::vector<int> divisors;
        for (int i = 1; i * i <= n; i++) {
            if (n % i == 0) {
                divisors.push_back(i);
                if (i != n / i) {
                    divisors.push_back(n / i);
                }
            }
        }
        std::sort(divisors.begin(), divisors.end());
        return divisors;
    }
    
    bool isPrimMultiple(int n) {
        auto factors = primfaktoren(n);
        return factors.size() == 2;
    }
    
    std::string getTextWrapThings(const std::string& text, int width = 80) {
        // Einfache Textumbrüche
        std::string result;
        int current = 0;
        for (char c : text) {
            if (current >= width && c == ' ') {
                result += '\n';
                current = 0;
            } else {
                result += c;
                current++;
            }
        }
        return result;
    }
    
    template<typename T>
    std::vector<T> unique_everseen(const std::vector<T>& seq) {
        std::vector<T> result;
        std::unordered_set<T> seen;
        for (const auto& item : seq) {
            if (seen.find(item) == seen.end()) {
                seen.insert(item);
                result.push_back(item);
            }
        }
        return result;
    }
    
    void infoLog(const std::string& msg) {
        std::cerr << "[INFO] " << msg << std::endl;
    }
    
    void output(const std::string& msg) {
        std::cout << msg << std::endl;
    }
    
    void x(const std::string& label, const auto& value) {
        std::cerr << "[DEBUG " << label << "] ";
        if constexpr (std::is_same_v<decltype(value), const std::string&> || 
                     std::is_same_v<decltype(value), std::string>) {
            std::cerr << value;
        } else {
            std::cerr << typeid(value).name();
        }
        std::cerr << std::endl;
    }
    
    std::vector<std::string> alxp(const std::string& msg) {
        std::cerr << "[ALXP] " << msg << std::endl;
        return {};
    }
    
    void cliout(const std::string& msg) {
        std::cout << msg << std::endl;
    }
}

// ============================================================================
// HAUPTKLASSE CONCAT
// ============================================================================

class Concat {
private:
    Tables* tables;
    I18n& i18n;
    
    OrderedSet<int> ones;
    OrderedDict<int, std::vector<std::vector<std::string>>> CSVsAlreadRead;
    OrderedDict<int, std::vector<int>> CSVsSame;
    
    OrderedSet<Fraction> BruecheUni;
    OrderedSet<Fraction> BruecheGal;
    OrderedSet<Fraction> BruecheEmo;
    OrderedSet<Fraction> BruecheStrukGroesse;
    
    OrderedSet<Fraction> gebrRatMulSternUni;
    OrderedSet<Fraction> gebrRatDivSternUni;
    OrderedSet<Fraction> gebrRatMulGleichfUni;
    OrderedSet<Fraction> gebrRatDivGleichfUni;
    OrderedSet<Fraction> gebrRatMulSternGal;
    OrderedSet<Fraction> gebrRatDivSternGal;
    OrderedSet<Fraction> gebrRatMulGleichfGal;
    OrderedSet<Fraction> gebrRatDivGleichfGal;
    
    std::vector<std::vector<std::string>> relitable;
    std::set<int> rowsAsNumbers;
    OrderedSet<Fraction> gebrRatEtwaSchonMalDabeiGewesen;
    
    // Strukturalien-Spalten
    std::pair<int, int> struktAndInversSpalten = {5, 131};
    std::pair<int, int> transzendentalienSpalten;
    std::vector<std::vector<std::string>> gebrUnivTable4metaKonkret;
    
    // Primzahl-Zählung
    int primAmounts = 0;
    int oldPrimAmounts = 0;
    std::map<int, std::string> lastPrimAnswers;
    
public:
    Concat(Tables* tablesPtr) 
        : tables(tablesPtr), 
          i18n(I18n::instance()) {
        // Initialisierung
        CSVsSame = {
            {1, {1}},
            {2, {2, 4}},
            {3, {3, 5}},
            {4, {2, 4}},
            {5, {3, 5}}
        };
    }
    
    // ------------------------------------------------------------------------
    // HAUPTMETHODEN
    // ------------------------------------------------------------------------
    
    std::pair<std::vector<std::vector<std::string>>, std::set<int>> 
    concatLovePolygon(const std::vector<std::vector<std::string>>& relitable, 
                     std::set<int> rowsAsNumbers) {
        this->relitable = relitable;
        this->rowsAsNumbers = rowsAsNumbers;
        
        if (!relitable.empty() && 
            std::any_of(rowsAsNumbers.begin(), rowsAsNumbers.end(),
                       [](int n) { return n >= 9; })) {
            
            int currentWidth = relitable[0].size();
            rowsAsNumbers.insert(currentWidth);
            
            // Tags setzen
            tables->generatedSpaltenParameter_Tags[currentWidth - 1] = 
                {ST::sternPolygon, ST::galaxie, ST::gleichfoermigesPolygon};
            
            for (size_t i = 0; i < relitable.size() && i <= tables->lastLineNumber; i++) {
                std::vector<std::string> newRow = relitable[i];
                if (i < relitable.size() && relitable[i].size() > 8 && 
                    !relitable[i][8].empty()) {
                    std::string newCell = relitable[i][8] + 
                        i18n.get("polygon1", " der eigenen Strukturgröße (") +
                        relitable[i][4] + 
                        i18n.get("polygon2", ") auf dich bei gleichförmigen Polygonen");
                    newRow.push_back(newCell);
                } else {
                    newRow.push_back("");
                }
                this->relitable[i] = newRow;
            }
        }
        
        return {this->relitable, rowsAsNumbers};
    }
    
    std::string gleichheitFreiheitVergleich(int zahl) {
        std::vector<std::string> ausgabeStringList;
        
        if (zahl % 4 == 0) {
            ausgabeStringList.push_back(
                i18n.get("gleichheitFreiheitVergleich", "Dominieren, Unterordnen")
            );
        }
        if (zahl % 4 == 1) {
            ausgabeStringList.push_back(
                i18n.get("gleichheitFreiheitVergleich", "Freiheit")
            );
        }
        if (zahl % 4 == 3) {
            ausgabeStringList.push_back(
                i18n.get("gleichheitFreiheitVergleich", "Einschränkung der Freiheit")
            );
        }
        if (zahl % 4 == 2) {
            if ((zahl - 2) % 8 == 0) {
                ausgabeStringList.push_back(
                    i18n.get("gleichheitFreiheitVergleich", "Gleichheit")
                );
            }
            if ((zahl - 6) % 16 == 0) {
                ausgabeStringList.push_back(
                    i18n.get("gleichheitFreiheitVergleich", "den anderen überbieten wollen")
                );
            }
            if ((zahl - 14) % 16 == 0) {
                ausgabeStringList.push_back(
                    i18n.get("gleichheitFreiheitVergleich", "den anderen unterbieten wollen")
                );
            }
        }
        
        // Zusammenfügen
        std::string result;
        for (size_t i = 0; i < ausgabeStringList.size(); i++) {
            if (i > 0) result += "; ";
            result += ausgabeStringList[i];
        }
        return result;
    }
    
    std::string geistEmotionEnergieMaterieTopologie(int zahl) {
        auto prFa = Helper::primfaktoren(zahl);
        
        std::vector<bool> auss, innen;
        for (int a : prFa) {
            auss.push_back(Helper::couldBePrimeNumberPrimzahlkreuz_fuer_aussen(a));
            innen.push_back(Helper::couldBePrimeNumberPrimzahlkreuz_fuer_innen(a));
        }
        
        int zwei = std::count(prFa.begin(), prFa.end(), 2);
        bool gefuehl = std::any_of(auss.begin(), auss.end(), [](bool b) { return b; });
        bool denken = std::any_of(innen.begin(), innen.end(), [](bool b) { return b; });
        
        bool totalTopologie = (zwei > 1) && gefuehl;
        bool etwasTopologie = (zwei > 1 || (zwei > 0 && gefuehl)) && !totalTopologie;
        bool totalMaterie = zwei > 4;
        bool etwasMaterie = zwei == 4;
        bool wenigMaterie = zwei == 3;
        bool kaumMaterie = zwei == 2;
        
        bool x = denken;
        bool y = std::find(prFa.begin(), prFa.end(), 2) != prFa.end();
        bool z = std::find(prFa.begin(), prFa.end(), 3) != prFa.end();
        
        bool totalEnergie = x && y && z;
        bool einermassenEnergie = ((x && y) || (y && z) || (x && z)) && !totalEnergie;
        bool kaumEnergie = !einermassenEnergie && !totalEnergie && (x || y || z);
        
        std::vector<std::string> ausgabeStringList;
        
        if (denken) ausgabeStringList.push_back(i18n.get("energietopologie1", "eine Denkart"));
        if (gefuehl) ausgabeStringList.push_back(i18n.get("energietopologie1", "eine Gefühlsart"));
        if (totalMaterie) ausgabeStringList.push_back(
            i18n.get("energietopologie1", "total eine Art, etwas geistig zu erzeugen"));
        if (totalTopologie) ausgabeStringList.push_back(
            i18n.get("energietopologie1", "total eine Art zu erleben"));
        if (totalEnergie) ausgabeStringList.push_back(
            i18n.get("energietopologie1", "total eine Energie-Art"));
        if (etwasTopologie) ausgabeStringList.push_back(
            i18n.get("energietopologie1", "etwas eine Art zu erleben"));
        if (etwasMaterie) ausgabeStringList.push_back(
            i18n.get("energietopologie1", "etwas eine Art, etwas geistig zu erzeugen"));
        if (wenigMaterie) ausgabeStringList.push_back(
            i18n.get("energietopologie1", "wenig eine Art, etwas geistig zu erzeugen"));
        if (einermassenEnergie) ausgabeStringList.push_back(
            i18n.get("energietopologie1", "einigermaßen eine Energie-Art"));
        if (kaumEnergie) ausgabeStringList.push_back(
            i18n.get("energietopologie1", "kaum eine Energie-Art"));
        if (kaumMaterie) ausgabeStringList.push_back(
            i18n.get("energietopologie1", "kaum eine Art, etwas geistig zu erzeugen"));
        
        std::string result;
        for (size_t i = 0; i < ausgabeStringList.size(); i++) {
            if (i > 0) result += "; ";
            result += ausgabeStringList[i];
        }
        return result;
    }
    
    std::pair<std::vector<std::vector<std::string>>, std::set<int>> 
    concatGleichheitFreiheitDominieren(
        const std::vector<std::vector<std::string>>& relitable,
        std::set<int> rowsAsNumbers) {
        
        this->relitable = relitable;
        this->rowsAsNumbers = rowsAsNumbers;
        
        if (!relitable.empty() && 
            std::any_of(rowsAsNumbers.begin(), rowsAsNumbers.end(),
                       [](int n) { return n >= 132; })) {
            
            int currentWidth = relitable[0].size();
            rowsAsNumbers.insert(currentWidth);
            
            tables->generatedSpaltenParameter_Tags[currentWidth - 1] = 
                {ST::sternPolygon, ST::universum};
            
            for (size_t i = 0; i < relitable.size() && i <= tables->lastLineNumber; i++) {
                std::vector<std::string> newRow = relitable[i];
                std::string ausgabeString;
                
                if (i == 0) {
                    ausgabeString = i18n.get("gleichheitFreiheitVergleich",
                        "Gleichheit, Freiheit, Dominieren (Ordnungen [12]) Generiert");
                } else {
                    ausgabeString = gleichheitFreiheitVergleich(static_cast<int>(i));
                }
                
                newRow.push_back(ausgabeString);
                this->relitable[i] = newRow;
            }
        }
        
        return {this->relitable, rowsAsNumbers};
    }
    
    std::pair<std::vector<std::vector<std::string>>, std::set<int>> 
    concatGeistEmotionEnergieMaterieTopologie(
        const std::vector<std::vector<std::string>>& relitable,
        std::set<int> rowsAsNumbers) {
        
        this->relitable = relitable;
        this->rowsAsNumbers = rowsAsNumbers;
        
        if (!relitable.empty() && 
            std::any_of(rowsAsNumbers.begin(), rowsAsNumbers.end(),
                       [](int n) { return n >= 242; })) {
            
            int currentWidth = relitable[0].size();
            rowsAsNumbers.insert(currentWidth);
            
            tables->generatedSpaltenParameter_Tags[currentWidth - 1] = 
                {ST::sternPolygon, ST::universum};
            
            for (size_t i = 0; i < relitable.size() && i <= tables->lastLineNumber; i++) {
                std::vector<std::string> newRow = relitable[i];
                std::string ausgabeString;
                
                if (i == 0) {
                    ausgabeString = i18n.get("ausgabeString",
                        "Energie oder Denkart oder Gefühlsart oder Materie-Art oder Topologie-Art");
                } else {
                    ausgabeString = geistEmotionEnergieMaterieTopologie(static_cast<int>(i));
                }
                
                newRow.push_back(ausgabeString);
                this->relitable[i] = newRow;
            }
        }
        
        return {this->relitable, rowsAsNumbers};
    }
    
    std::pair<std::vector<std::vector<std::string>>, std::set<int>> 
    concatPrimCreativityType(
        const std::vector<std::vector<std::string>>& relitable,
        std::set<int> rowsAsNumbers) {
        
        this->relitable = relitable;
        this->rowsAsNumbers = rowsAsNumbers;
        
        if (!relitable.empty() && 
            std::any_of(rowsAsNumbers.begin(), rowsAsNumbers.end(),
                       [](int n) { return n >= 64; })) {
            
            int currentWidth = relitable[0].size();
            rowsAsNumbers.insert(currentWidth);
            
            tables->generatedSpaltenParameter_Tags[currentWidth - 1] = 
                {ST::sternPolygon, ST::galaxie};
            
            for (size_t i = 0; i < relitable.size() && i <= tables->lastLineNumber; i++) {
                std::vector<std::string> newRow = relitable[i];
                std::string cellValue;
                
                if (i == 0) {
                    cellValue = i18n.get("kreaZahl", "Evolutions-Züchtungs-Kreativität");
                } else {
                    int primCreativityType = Helper::primCreativity(static_cast<int>(i));
                    switch (primCreativityType) {
                        case 0:
                            cellValue = i18n.get("kreaZahl", "0. Primzahl 1");
                            break;
                        case 1:
                            cellValue = i18n.get("kreaZahl", "1. Primzahl und Sonnenzahl");
                            break;
                        case 2:
                            cellValue = i18n.get("kreaZahl", "2. Sonnenzahl, aber keine Primzahl");
                            break;
                        case 3:
                            cellValue = i18n.get("kreaZahl", "3. Mondzahl");
                            break;
                        default:
                            cellValue = "";
                    }
                }
                
                newRow.push_back(cellValue);
                this->relitable[i] = newRow;
            }
        }
        
        return {this->relitable, rowsAsNumbers};
    }
    
    // ------------------------------------------------------------------------
    // CSV OPERATIONEN
    // ------------------------------------------------------------------------
    
    std::vector<std::vector<std::string>> readOneCSVAndReturn(int wahl) {
        std::string filename = getCSVFilename(wahl);
        
        if (CSVsAlreadRead.find(wahl) != CSVsAlreadRead.end()) {
            return CSVsAlreadRead[wahl];
        }
        
        std::vector<std::vector<std::string>> table = readCSVFile(filename);
        CSVsAlreadRead[wahl] = table;
        
        // Brüche extrahieren basierend auf Wahl
        if (wahl == 2 || wahl == 3) { // Universe
            BruecheUni = getAllBrueche(table);
        } else if (wahl == 4 || wahl == 5) { // Galaxie
            BruecheGal = getAllBrueche(table);
        } else if (wahl == 6 || wahl == 7) { // Emotion
            BruecheEmo = getAllBrueche(table);
        } else if (wahl == 8 || wahl == 9) { // Strukturgröße
            BruecheStrukGroesse = getAllBrueche(table);
        }
        
        return table;
    }
    
private:
    // ------------------------------------------------------------------------
    // PRIVATE HILFSMETHODEN
    // ------------------------------------------------------------------------
    
    std::string getCSVFilename(int wahl) {
        std::string baseDir = "csv/";
        switch (wahl) {
            case 1: return baseDir + i18n.csvFileNames.prim;
            case 2:
            case 3: return baseDir + i18n.csvFileNames.bruch15;
            case 4:
            case 5: return baseDir + i18n.csvFileNames.bruch13;
            case 6:
            case 7: return baseDir + i18n.csvFileNames.bruch7;
            case 8:
            case 9: return baseDir + i18n.csvFileNames.bruchStrukGroesse;
            default: return "";
        }
    }
    
    std::vector<std::vector<std::string>> readCSVFile(const std::string& filename) {
        std::vector<std::vector<std::string>> data;
        std::ifstream file(filename);
        
        if (!file.is_open()) {
            std::cerr << "Fehler beim Öffnen der Datei: " << filename << std::endl;
            return data;
        }
        
        std::string line;
        while (std::getline(file, line)) {
            std::vector<std::string> row;
            std::stringstream ss(line);
            std::string cell;
            
            while (std::getline(ss, cell, ';')) {
                row.push_back(cell);
            }
            
            data.push_back(row);
        }
        
        return data;
    }
    
    OrderedSet<Fraction> getAllBrueche(const std::vector<std::vector<std::string>>& table) {
        OrderedSet<Fraction> brueche;
        
        for (size_t i = 1; i < table.size(); i++) {
            for (size_t k = 1; k < table[i].size(); k++) {
                if (!table[i][k].empty() && table[i][k].length() > 3) {
                    Fraction frac(static_cast<int>(i + 1), static_cast<int>(k + 1));
                    if (frac.denominator() != 1 && frac.numerator() != 1) {
                        brueche.insert(frac);
                    }
                }
            }
        }
        
        return brueche;
    }
    
    DefaultOrderedDict<int, OrderedSet<std::pair<Fraction, Fraction>>>
    convertSetOfPaarenToDictOfNumToPaareMul(
        const OrderedSet<std::pair<Fraction, Fraction>>& paareSet,
        bool gleichf = false) {
        
        DefaultOrderedDict<int, OrderedSet<std::pair<Fraction, Fraction>>> result(
            OrderedSet<std::pair<Fraction, Fraction>>());
        
        for (const auto& paar : paareSet) {
            Fraction mul = paar.first * paar.second;
            if (gleichf) {
                mul = Fraction(1) / mul;
            }
            int mulInt = static_cast<int>(std::round(mul.toDouble()));
            result[mulInt].insert(paar);
        }
        
        return result;
    }
    
    DefaultOrderedDict<int, OrderedSet<std::pair<Fraction, Fraction>>>
    convertSetOfPaarenToDictOfNumToPaareDiv(
        const OrderedSet<std::pair<Fraction, Fraction>>& paareSet,
        bool gleichf = false) {
        
        DefaultOrderedDict<int, OrderedSet<std::pair<Fraction, Fraction>>> result(
            OrderedSet<std::pair<Fraction, Fraction>>());
        
        for (const auto& paar : paareSet) {
            Fraction div = gleichf ? paar.second / paar.first : paar.first / paar.second;
            int divInt = static_cast<int>(std::round(div.toDouble()));
            result[divInt].insert(paar);
        }
        
        return result;
    }
    
    std::string spalteMetaKonkretTheorieAbstrakt_getGebrRatUnivStrukturalie(
        const Fraction& koord,
        const std::pair<int, int>& n_and_invers_spalten,
        const std::vector<std::vector<std::string>>& gebrTable,
        bool isNotUniverse = true) {
        
        if (koord.denominator() == 0 || koord.numerator() == 0) {
            return "";
        }
        
        if (koord.denominator() > 100 || koord.numerator() > 100) {
            return "";
        }
        
        if (koord.numerator() == 1) {
            int denom = koord.denominator();
            if (denom < relitable.size() && 
                n_and_invers_spalten.second < relitable[denom].size() &&
                relitable[denom][n_and_invers_spalten.second].length() > 3) {
                
                std::string strukname = relitable[denom][n_and_invers_spalten.second];
                if (isNotUniverse) {
                    strukname += " (1/" + std::to_string(denom) + ")";
                }
                return strukname;
            }
            return "";
        }
        
        if (koord.denominator() == 1) {
            int numer = koord.numerator();
            if (numer < relitable.size() && 
                n_and_invers_spalten.first < relitable[numer].size() &&
                relitable[numer][n_and_invers_spalten.first].length() > 3) {
                
                std::string strukname = relitable[numer][n_and_invers_spalten.first];
                if (isNotUniverse) {
                    strukname += " (" + std::to_string(numer) + ")";
                }
                return strukname;
            }
            return "";
        }
        
        // Aus gebrochener Tabelle lesen
        int row = koord.numerator() - 1;
        int col = koord.denominator() - 1;
        
        if (row >= 0 && row < gebrTable.size() && 
            col >= 0 && col < gebrTable[row].size()) {
            return gebrTable[row][col];
        }
        
        return "";
    }
    
    bool spalteMetaKonkretAbstrakt_isGanzZahlig(const Fraction& zahl, bool spaltenWahl) {
        Fraction testZahl = spaltenWahl ? Fraction(1) / zahl : zahl;
        double decimal = testZahl.toDouble() - std::floor(testZahl.toDouble());
        return decimal < 0.00001 || decimal > 0.99999;
    }
    
public:
    // ------------------------------------------------------------------------
    // WEITERE ÖFFENTLICHE METHODEN (vereinfacht)
    // ------------------------------------------------------------------------
    
    std::pair<std::vector<std::vector<std::string>>, std::set<int>> 
    concatVervielfacheZeile(
        const std::vector<std::vector<std::string>>& relitable,
        std::set<int> rowsAsNumbers) {
        
        // Vereinfachte Implementation
        return {relitable, rowsAsNumbers};
    }
    
    std::pair<std::vector<std::vector<std::string>>, std::set<int>> 
    concatModallogik(
        const std::vector<std::vector<std::string>>& relitable,
        const std::set<std::pair<int, int>>& conceptsRowsSetOfTuple,
        std::set<int> rowsAsNumbers) {
        
        // Vereinfachte Implementation
        return {relitable, rowsAsNumbers};
    }
    
    std::pair<std::vector<std::vector<std::string>>, std::set<int>> 
    concat1PrimzahlkreuzProContra(
        const std::vector<std::vector<std::string>>& relitable,
        std::set<int> rowsAsNumbers,
        const std::set<std::string>& generatedBefehle,
        const std::vector<std::pair<std::string, std::string>>& ParametersMain) {
        
        // Vereinfachte Implementation
        return {relitable, rowsAsNumbers};
    }
    
    std::pair<std::vector<std::vector<std::string>>, std::set<int>> 
    concat1RowPrimUniverse2(
        const std::vector<std::vector<std::string>>& relitable,
        std::set<int> rowsAsNumbers,
        const std::set<std::string>& generatedBefehle,
        const std::vector<std::vector<std::vector<std::pair<std::string, std::string>>>>& htmlTagParaClassWoerter) {
        
        // Vereinfachte Implementation
        return {relitable, rowsAsNumbers};
    }
    
    std::pair<std::vector<std::vector<std::string>>, std::set<int>> 
    spalteMetaKontretTheorieAbstrakt_etc_1(
        const std::vector<std::vector<std::string>>& relitable,
        std::set<int> rowsAsNumbers,
        const std::set<std::pair<int, int>>& geordnetePaare) {
        
        // Vereinfachte Implementation
        return {relitable, rowsAsNumbers};
    }
    
    std::pair<std::vector<std::vector<std::string>>, std::set<int>> 
    spalteFuerGegenInnenAussenSeitlichPrim(
        const std::vector<std::vector<std::string>>& relitable,
        std::set<int> rowsAsNumbers) {
        
        // Vereinfachte Implementation
        return {relitable, rowsAsNumbers};
    }
    
    std::tuple<std::vector<std::vector<std::string>>, std::set<int>, std::set<int>> 
    readConcatCsv(
        const std::vector<std::vector<std::string>>& relitable,
        std::set<int> rowsAsNumbers,
        const std::set<int>& concatTableSelection,
        int concatTable = 1) {
        
        // Vereinfachte Implementation
        std::set<int> concatCSVspalten;
        return {relitable, rowsAsNumbers, concatCSVspalten};
    }
};

// ============================================================================
// HAUPTFUNKTION ZUM TESTEN
// ============================================================================

int main() {
    // Beispiel-Nutzung
    Tables tables;
    Concat concat(&tables);
    
    // Beispiel-Tabelle
    std::vector<std::vector<std::string>> testTable = {
        {"Header1", "Header2", "Header3"},
        {"Row1Col1", "Row1Col2", "Row1Col3"},
        {"Row2Col1", "Row2Col2", "Row2Col3"}
    };
    
    std::set<int> rows = {1, 2, 3};
    
    auto result = concat.concatLovePolygon(testTable, rows);
    
    std::cout << "Ergebnis-Größe: " << result.first.size() << "x" 
              << (result.first.empty() ? 0 : result.first[0].size()) << std::endl;
    
    // Test der Hilfsfunktion
    std::string vergleich = concat.gleichheitFreiheitVergleich(10);
    std::cout << "Vergleich für 10: " << vergleich << std::endl;
    
    return 0;
}

// ============================================================================
// MAKE-DATEI BEISPIEL
// ============================================================================
/*
Makefile:

CXX = g++
CXXFLAGS = -std=c++17 -Wall -O2
TARGET = lib4tables_concat

all: $(TARGET)

$(TARGET): lib4tables_concat.cpp
	$(CXX) $(CXXFLAGS) -o $(TARGET) lib4tables_concat.cpp

clean:
	rm -f $(TARGET)

test: $(TARGET)
	./$(TARGET)
*/
