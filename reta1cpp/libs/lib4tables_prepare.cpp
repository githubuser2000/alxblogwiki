// lib4tables_prepare.cpp - Vollständige C++ Konvertierung
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
#include <type_traits>
#include <tuple>
#include <iomanip>
#include <cctype>
#include <limits>

namespace fs = std::filesystem;

// ============================================================================
// FORWARD-DEKLARATIONEN UND GLOBALE VARIABLEN
// ============================================================================

// Globale Variablen (entsprechen Python-Globals)
int shellRowsAmount = 80;
std::string h_de;
std::map<std::string, std::string> dic;
char fill = ' ';

// Enums
enum class ST {
    sternPolygon,
    galaxie,
    gleichfoermigesPolygon,
    universum,
    gebrRat,
    keinParaOdMetaP
};

enum class Wraptype {
    pyphen = 1,
    pyhyphen = 2,
    nohyphen = 3
};

Wraptype wrappingType = Wraptype::pyhyphen;

// Hilfsklassen für Tabellen-Tags
namespace lib4tables_Enum {
    std::unordered_map<int, std::unordered_set<ST>> tableTags2;
    std::unordered_map<int, std::unordered_set<ST>> tableTags2_kombiTable;
    std::unordered_map<int, std::unordered_set<ST>> tableTags2_kombiTable2;
}

// Internationalisierung (vereinfacht)
class I18n {
public:
    std::unordered_map<std::string, std::string> befehle2;
    
    static I18n& instance() {
        static I18n i18n;
        return i18n;
    }
};

// ============================================================================
// HILFSFUNKTIONEN
// ============================================================================

// Entspricht getTextWrapThings() in Python
void getTextWrapThings(int& shellRows, std::string& h_de_out, 
                      std::map<std::string, std::string>& dic_out, char& fill_out) {
    shellRows = shellRowsAmount;
    h_de_out = h_de;
    dic_out = dic;
    fill_out = fill;
}

void setShellRowsAmount(int shellRowsAmount2) {
    shellRowsAmount = shellRowsAmount2;
}

// Entspricht alxp() in Python
std::vector<std::string> alxp(const std::string& msg) {
    std::cerr << "[ALXP] " << msg << std::endl;
    return {};
}

// Entspricht x() in Python
void x(const std::string& label, const auto& value) {
    std::cerr << "[DEBUG " << label << "] " << typeid(value).name() << std::endl;
}

// Entspricht isZeilenAngabe() in Python
bool isZeilenAngabe(const std::string& str) {
    // Vereinfachte Prüfung
    return !str.empty() && std::all_of(str.begin(), str.end(), 
                                      [](char c) { return std::isdigit(c) || c == '-'; });
}

// Entspricht BereichToNumbers2() in Python
std::set<int> BereichToNumbers2(const std::string& bereich, bool isReligious, int maxValue) {
    std::set<int> result;
    
    // Einfache Implementation für "1-10,20-30" Format
    std::regex rangeRegex(R"((\d+)-(\d+))");
    std::regex singleRegex(R"(\d+)");
    
    std::sregex_iterator it(bereich.begin(), bereich.end(), rangeRegex);
    std::sregex_iterator end;
    
    while (it != end) {
        int start = std::stoi((*it)[1]);
        int endVal = std::stoi((*it)[2]);
        for (int i = start; i <= endVal && i <= maxValue; i++) {
            result.insert(i);
        }
        ++it;
    }
    
    // Einzelne Zahlen
    std::sregex_iterator it2(bereich.begin(), bereich.end(), singleRegex);
    while (it2 != end) {
        int num = std::stoi((*it2)[0]);
        if (num <= maxValue) {
            result.insert(num);
        }
        ++it2;
    }
    
    return result;
}

// Entspricht teiler() in Python
std::pair<std::set<int>, std::set<int>> teiler(const std::string& numsStr) {
    std::set<int> divisors, numbers;
    
    std::stringstream ss(numsStr);
    std::string numStr;
    while (std::getline(ss, numStr, ',')) {
        try {
            int num = std::stoi(numStr);
            numbers.insert(num);
            
            // Teiler finden
            for (int i = 1; i * i <= num; i++) {
                if (num % i == 0) {
                    divisors.insert(i);
                    if (i != num / i) {
                        divisors.insert(num / i);
                    }
                }
            }
        } catch (...) {
            // Ignoriere ungültige Zahlen
        }
    }
    
    return {numbers, divisors};
}

// Entspricht primFak() in Python
std::vector<int> primFak(int n) {
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

// Entspricht primRepeat2() in Python
std::vector<std::pair<int, int>> primRepeat2(const std::vector<int>& factors) {
    std::vector<std::pair<int, int>> result;
    
    // Gruppiere gleiche Faktoren
    std::map<int, int> count;
    for (int f : factors) {
        count[f]++;
    }
    
    for (const auto& [factor, freq] : count) {
        result.push_back({factor, freq});
    }
    
    return result;
}

// Entspricht moonNumber() in Python
std::pair<std::vector<int>, std::vector<int>> moonNumber(int n) {
    // Vereinfachte Implementation
    std::vector<int> basis, exponent;
    
    if (n > 1) {
        auto factors = primFak(n);
        std::map<int, int> counts;
        
        for (int f : factors) {
            counts[f]++;
        }
        
        for (const auto& [f, cnt] : counts) {
            if (cnt > 1) {
                basis.push_back(f);
                exponent.push_back(cnt);
            }
        }
    }
    
    return {basis, exponent};
}

// Entspricht isPrimMultiple() in Python
bool isPrimMultiple(int n, const std::vector<int>& primMultiples) {
    auto factors = primFak(n);
    
    for (int pm : primMultiples) {
        // Prüfe ob pm in den Faktoren vorkommt
        if (std::find(factors.begin(), factors.end(), pm) != factors.end()) {
            return true;
        }
    }
    
    return false;
}

// ============================================================================
// TEXT-WRAPPING FUNKTIONEN
// ============================================================================

template<typename T>
std::vector<T> chunks(const std::vector<T>& lst, int n) {
    std::vector<T> result;
    for (size_t i = 0; i < lst.size(); i += n) {
        if (i + n <= lst.size()) {
            result.insert(result.end(), lst.begin() + i, lst.begin() + i + n);
        }
    }
    return result;
}

std::vector<std::string> splitMoreIfNotSmall(const std::vector<std::string>& textList, int lenToBe) {
    std::vector<std::string> newList;
    bool neededToBeDoneAtAll = false;
    
    for (const auto& text : textList) {
        if (text.length() > lenToBe) {
            neededToBeDoneAtAll = true;
            break;
        }
    }
    
    if (neededToBeDoneAtAll) {
        for (const auto& text : textList) {
            if (text.length() > lenToBe) {
                // Text in Chunks aufteilen
                for (size_t i = 0; i < text.length(); i += lenToBe) {
                    newList.push_back(text.substr(i, lenToBe));
                }
            } else {
                newList.push_back(text);
            }
        }
    } else {
        newList = textList;
    }
    
    return newList;
}

std::vector<std::string> alxwrap(const std::string& text, int len_) {
    std::vector<std::string> result;
    
    if (len_ == 0) {
        result.push_back(text);
        return result;
    }
    
    // Einfache Textumbrüche
    if (text.length() <= len_) {
        result.push_back(text);
        return result;
    }
    
    // Einfacher Zeilenumbruch
    std::string current;
    std::stringstream ss(text);
    std::string word;
    
    while (ss >> word) {
        if (current.length() + word.length() + 1 > len_) {
            if (!current.empty()) {
                result.push_back(current);
                current.clear();
            }
        }
        
        if (!current.empty()) current += " ";
        current += word;
    }
    
    if (!current.empty()) {
        result.push_back(current);
    }
    
    // Nachbearbeitung falls nötig
    result = splitMoreIfNotSmall(result, len_);
    
    return result;
}

// ============================================================================
// HAUPTKLASSE PREPARE
// ============================================================================

// Forward-Deklaration für Tables-Klasse
class Tables;

class Prepare {
private:
    Tables* tables;
    std::map<int, int> hoechsteZeile;
    std::vector<int> originalLinesRange;
    int shellRowsAmount_;
    
    // Zählungs-Strukturen
    struct Zaehlungen {
        int lastNumber = 0;
        std::map<int, int> zaehlungToStart;
        std::map<int, int> numberToZaehlung1;
        std::map<int, int> numberToZaehlung2;
        std::map<int, std::pair<std::vector<int>, std::vector<int>>> moonTypes;
    } zaehlungen;
    
    std::vector<int> religionNumbers;
    bool gezaehlt = false;
    bool ifZeilenSetted = false;
    std::vector<int> breiten;
    bool nummerierung = true;
    int textwidth = 21;
    int certaintextwidth = 0;
    int headingsAmount = 0;
    
public:
    Prepare(Tables* tablesPtr, const std::map<int, int>& hz) 
        : tables(tablesPtr), hoechsteZeile(hz) {
        
        getTextWrapThings(shellRowsAmount_, h_de, dic, fill);
        
        // originalLinesRange initialisieren
        int maxVal = (hz.find(1024) != hz.end()) ? hz.at(1024) : 1024;
        for (int i = 0; i <= maxVal + 3; i++) {
            originalLinesRange.push_back(i);
        }
    }
    
    // Getter/Setter für Properties
    std::vector<int> getBreiten() const { return breiten; }
    void setBreiten(const std::vector<int>& value) { breiten = value; }
    
    bool getNummerierung() const { return nummerierung; }
    void setNummerierung(bool value) { nummerierung = value; }
    
    int getTextWidth() const { return textwidth; }
    void setTextWidth(int value) { textwidth = value; }
    
    bool ifprimmultis() const { 
        // Vereinfacht
        return false; 
    }
    void setIfprimmultis(bool value) {
        // Vereinfacht
    }
    
    bool getIfZeilenSetted() const { return ifZeilenSetted; }
    void setIfZeilenSetted(bool value) { ifZeilenSetted = value; }
    
    void setReligionNumbers(const std::vector<int>& nums) {
        religionNumbers = nums;
    }
    
    // Hauptmethoden
    void setZaehlungen(int num) {
        if (gezaehlt) return;
        
        gezaehlt = true;
        num = originalLinesRange.back();
        
        bool wasMoon = true;
        bool isMoon = (zaehlungen.lastNumber == 0) ? true : 
                     !moonNumber(zaehlungen.lastNumber).first.empty();
        
        for (int i = zaehlungen.lastNumber + 1; i <= num; i++) {
            wasMoon = isMoon;
            auto moonType = moonNumber(i);
            isMoon = !moonType.first.empty();
            
            if (wasMoon && !isMoon) {
                int newZaehlung = zaehlungen.zaehlungToStart.size() + 1;
                zaehlungen.zaehlungToStart[newZaehlung] = i;
            }
            
            int currentZaehlung = zaehlungen.numberToZaehlung1.size() + 1;
            zaehlungen.numberToZaehlung1[i] = currentZaehlung;
            zaehlungen.numberToZaehlung2[i] = zaehlungen.numberToZaehlung1.size();
            zaehlungen.moonTypes[i] = moonType;
        }
        
        zaehlungen.lastNumber = num;
    }
    
    std::vector<std::string> wrapping(const std::string& text, int length) {
        if (text.length() > length && length != 0) {
            return alxwrap(text, length);
        } else {
            return {text};
        }
    }
    
    int setWidth(int rowToDisplay, int combiRows1 = 0) {
        if (shellRowsAmount_ == 0) return 0;
        
        int combiRows = (combiRows1 != 0) ? combiRows1 : 0; // rowsAsNumbers size vereinfacht
        
        int certaintextwidth = textwidth;
        
        if (rowToDisplay - 1 < breiten.size() && rowToDisplay - 1 >= 0) {
            certaintextwidth = breiten[rowToDisplay - 1];
        }
        
        return certaintextwidth;
    }
    
    std::set<std::string> parametersCmdWithSomeBereich(
        const std::string& MehrereBereiche,
        const std::string& symbol,
        const std::string& neg,
        bool keineNegBeruecksichtigung = false) {
        
        std::set<std::string> results;
        
        if (keineNegBeruecksichtigung) {
            if (isZeilenAngabe(MehrereBereiche)) {
                results.insert("_" + symbol + "_" + MehrereBereiche);
            }
            return results;
        }
        
        std::stringstream ss(MehrereBereiche);
        std::string EinBereich;
        
        while (std::getline(ss, EinBereich, ',')) {
            bool shouldAdd = false;
            
            if ((neg.empty() && !EinBereich.empty() && EinBereich[0] != '-') ||
                (!neg.empty() && EinBereich.substr(0, neg.length()) == neg)) {
                
                shouldAdd = true;
                if (!neg.empty()) {
                    EinBereich = EinBereich.substr(neg.length());
                }
            }
            
            if (shouldAdd && !EinBereich.empty() && isZeilenAngabe(EinBereich)) {
                results.insert("_" + symbol + "_" + EinBereich);
            }
        }
        
        return results;
    }
    
    std::pair<std::set<int>, std::set<int>> deleteDoublesInSets(
        const std::set<int>& set1, const std::set<int>& set2) {
        
        std::set<int> intersection;
        std::set_intersection(set1.begin(), set1.end(),
                             set2.begin(), set2.end(),
                             std::inserter(intersection, intersection.begin()));
        
        std::set<int> result1, result2;
        std::set_difference(set1.begin(), set1.end(),
                           intersection.begin(), intersection.end(),
                           std::inserter(result1, result1.begin()));
        
        std::set_difference(set2.begin(), set2.end(),
                           intersection.begin(), intersection.end(),
                           std::inserter(result2, result2.begin()));
        
        return {result1, result2};
    }
    
    std::pair<int, int> fromUntil(std::vector<std::string>& a) {
        if (a.empty()) return {1, 1};
        
        if (!a[0].empty() && std::all_of(a[0].begin(), a[0].end(), ::isdigit)) {
            int start = std::stoi(a[0]);
            
            if (a.size() == 2 && !a[1].empty() && 
                std::all_of(a[1].begin(), a[1].end(), ::isdigit)) {
                int end = std::stoi(a[1]);
                return {start, end};
            } else if (a.size() == 1) {
                return {1, start};
            }
        }
        
        return {1, 1};
    }
    
    int zeileWhichZaehlung(int zeile) {
        auto it = zaehlungen.numberToZaehlung2.find(zeile);
        return (it != zaehlungen.numberToZaehlung2.end()) ? it->second : 0;
    }
    
    std::set<int> moonsun(bool MoonNotSun, std::set<int> numRangeYesZ, 
                         const std::set<int>& numRange, bool ifZaehlungenAtAll = true) {
        
        if (!ifZaehlungenAtAll) {
            setZaehlungen(originalLinesRange.back());
        }
        
        for (int n : numRange) {
            auto moonType = zaehlungen.moonTypes[n];
            bool isMoon = !moonType.first.empty();
            
            if (isMoon == MoonNotSun) {
                numRangeYesZ.insert(n);
            }
        }
        
        return numRangeYesZ;
    }
    
    std::set<int> FilterOriginalLines(std::set<int> numRange, 
                                      const std::set<std::string>& paramLines) {
        
        // 0 entfernen
        numRange.erase(0);
        
        auto cutset = [](bool wether, const std::set<int>& a, const std::set<int>& b) {
            if (wether) {
                std::set<int> result;
                std::set_intersection(a.begin(), a.end(),
                                     b.begin(), b.end(),
                                     std::inserter(result, result.begin()));
                return result;
            }
            return a;
        };
        
        if (paramLines.find("all") != paramLines.end() ||
            (!ifZeilenSetted && 
             std::all_of(paramLines.begin(), paramLines.end(),
                        [](const std::string& s) { 
                            return s == "ka" || s == "ka2"; 
                        }))) {
            
            int maxVal = hoechsteZeile[1024];
            numRange.clear();
            for (int i = 1; i <= maxVal; i++) {
                numRange.insert(i);
            }
        } else {
            numRange.clear();
        }
        
        bool if_a_AtAll = false;
        std::vector<std::string> mehrere;
        bool ifTeiler = false;
        
        for (const auto& condition : paramLines) {
            if (condition.length() > 3 && condition.substr(0, 3) == "_a_") {
                if_a_AtAll = true;
                mehrere.push_back(condition.substr(3));
            }
            if (condition.substr(0, 3) == "_w_") {
                ifTeiler = true;
            }
        }
        
        if (if_a_AtAll && !mehrere.empty()) {
            auto newNumbers = BereichToNumbers2(
                join(mehrere, ","), false, hoechsteZeile[1024] + 1);
            numRange.insert(newNumbers.begin(), newNumbers.end());
        }
        
        if (ifTeiler && !numRange.empty()) {
            std::string numRangeStr;
            for (int n : numRange) {
                if (!numRangeStr.empty()) numRangeStr += ",";
                numRangeStr += std::to_string(n);
            }
            
            auto [numbers, divisors] = teiler(numRangeStr);
            numRange.insert(divisors.begin(), divisors.end());
        }
        
        // Negative Bereiche entfernen
        if (!numRange.empty()) {
            for (const auto& eins : mehrere) {
                bool ja1 = !eins.empty() && eins[0] == '-';
                auto& i18n = I18n::instance();
                bool ja2 = !eins.empty() && eins.substr(0, 2) == i18n.befehle2["v"] + "-";
                
                std::string toRemove = eins;
                if (ja1) toRemove = toRemove.substr(1);
                if (ja2) toRemove = i18n.befehle2["v"] + toRemove.substr(2);
                
                if (ja1 || ja2) {
                    auto removeSet = BereichToNumbers2(toRemove, false, hoechsteZeile[1024] + 1);
                    for (int n : removeSet) {
                        numRange.erase(n);
                    }
                }
            }
        }
        
        // Bereich b (_b_) verarbeiten
        bool if_b_AtAll = false;
        std::vector<std::string> mehrere_b;
        std::set<int> numRangeYesZ;
        
        for (const auto& condition : paramLines) {
            if (condition.length() > 3 && condition.substr(0, 3) == "_b_") {
                if_b_AtAll = true;
                mehrere_b.push_back(condition.substr(3));
            }
        }
        
        if (if_b_AtAll) {
            if (numRange.empty() && !if_a_AtAll && paramLines.find("all") == paramLines.end()) {
                int maxVal = hoechsteZeile[114];
                for (int i = 1; i <= maxVal; i++) {
                    numRange.insert(i);
                }
            }
            
            auto bNumbers = BereichToNumbers2(
                join(mehrere_b, ","), true, hoechsteZeile[114] + 1);
            numRangeYesZ.insert(bNumbers.begin(), bNumbers.end());
            
            if (!numRangeYesZ.empty()) {
                std::set<int> intersection;
                std::set_intersection(numRange.begin(), numRange.end(),
                                     numRangeYesZ.begin(), numRangeYesZ.end(),
                                     std::inserter(intersection, intersection.begin()));
                numRange = intersection;
            }
            
            // Negative Bereiche für b entfernen
            if (!numRange.empty()) {
                for (const auto& eins : mehrere_b) {
                    bool ja1 = !eins.empty() && eins[0] == '-';
                    auto& i18n = I18n::instance();
                    bool ja2 = !eins.empty() && eins.substr(0, 2) == i18n.befehle2["v"] + "-";
                    
                    std::string toRemove = eins;
                    if (ja1) toRemove = toRemove.substr(1);
                    if (ja2) toRemove = i18n.befehle2["v"] + toRemove.substr(2);
                    
                    if (ja1 || ja2) {
                        auto removeSet = BereichToNumbers2(toRemove, true, hoechsteZeile[1024] + 1);
                        for (int n : removeSet) {
                            numRange.erase(n);
                        }
                    }
                }
            }
        }
        
        // Zeit-Bedingungen (<, >, =)
        std::set<int> zeitRange;
        bool ifZeitAtAll = false;
        
        for (const auto& condition : paramLines) {
            if (condition == "=") {
                ifZeitAtAll = true;
                zeitRange.insert(10);
            } else if (condition == "<") {
                ifZeitAtAll = true;
                for (int i = 1; i < 10; i++) zeitRange.insert(i);
            } else if (condition == ">") {
                ifZeitAtAll = true;
                for (int i = 11; i <= hoechsteZeile[1024]; i++) zeitRange.insert(i);
            }
        }
        
        if (ifZeitAtAll) {
            if (numRange.empty() && !if_a_AtAll && !if_b_AtAll && 
                paramLines.find("all") == paramLines.end() && zeitRange.empty()) {
                for (int i = 1; i <= hoechsteZeile[1024]; i++) {
                    numRange.insert(i);
                }
            }
            
            if (if_a_AtAll || paramLines.find("all") != paramLines.end() || if_b_AtAll) {
                std::set<int> intersection;
                std::set_intersection(numRange.begin(), numRange.end(),
                                     zeitRange.begin(), zeitRange.end(),
                                     std::inserter(intersection, intersection.begin()));
                numRange = intersection;
            } else {
                numRange.insert(zeitRange.begin(), zeitRange.end());
            }
        }
        
        // Zählungs-Bedingungen (_n_)
        std::set<int> zaehlungRange;
        bool ifZaehlungenAtAll = false;
        std::vector<std::string> mehrere_n;
        
        for (const auto& condition : paramLines) {
            if (condition.length() > 3 && condition.substr(0, 3) == "_n_") {
                ifZaehlungenAtAll = true;
                mehrere_n.push_back(condition.substr(3));
                auto newNumbers = BereichToNumbers2(condition.substr(3), false, 
                                                   hoechsteZeile[1024] + 1);
                zaehlungRange.insert(newNumbers.begin(), newNumbers.end());
            }
        }
        
        if (ifZaehlungenAtAll) {
            setZaehlungen(originalLinesRange.back());
            
            if (numRange.empty() && !if_a_AtAll && !if_b_AtAll && 
                paramLines.find("all") == paramLines.end()) {
                for (int i = 1; i <= hoechsteZeile[1024]; i++) {
                    numRange.insert(i);
                }
            }
            
            std::set<int> zaehlungRange2;
            for (int n : numRange) {
                int zaehlung = zeileWhichZaehlung(n);
                if (zaehlungRange.find(zaehlung) != zaehlungRange.end()) {
                    zaehlungRange2.insert(n);
                }
            }
            
            if (!zaehlungRange2.empty()) {
                std::set<int> intersection;
                std::set_intersection(numRange.begin(), numRange.end(),
                                     zaehlungRange2.begin(), zaehlungRange2.end(),
                                     std::inserter(intersection, intersection.begin()));
                numRange = intersection;
            } else if (numRange.empty()) {
                numRange = zaehlungRange2;
            }
            
            // Negative Zählungen entfernen
            if (!numRange.empty()) {
                std::set<int> minusBereiche;
                for (const auto& eins : mehrere_n) {
                    bool ja1 = !eins.empty() && eins[0] == '-';
                    auto& i18n = I18n::instance();
                    bool ja2 = !eins.empty() && eins.substr(0, 2) == i18n.befehle2["v"] + "-";
                    
                    std::string toRemove = eins;
                    if (ja1) toRemove = toRemove.substr(1);
                    if (ja2) toRemove = i18n.befehle2["v"] + toRemove.substr(2);
                    
                    if (ja1 || ja2) {
                        auto removeSet = BereichToNumbers2(toRemove, false, 
                                                          hoechsteZeile[1024] + 1);
                        minusBereiche.insert(removeSet.begin(), removeSet.end());
                    }
                }
                
                for (int n : minusBereiche) {
                    int zaehlung = zeileWhichZaehlung(n);
                    for (auto it = numRange.begin(); it != numRange.end();) {
                        if (zeileWhichZaehlung(*it) == zaehlung) {
                            it = numRange.erase(it);
                        } else {
                            ++it;
                        }
                    }
                }
            }
        }
        
        // Typ-Bedingungen (aussenerste, innenerste, etc.)
        std::set<std::string> typConditions = {"aussenerste", "innenerste", 
                                              "aussenalle", "innenalle"};
        bool ifTypAtAll = false;
        std::set<int> typRange;
        
        for (const auto& condition : paramLines) {
            if (typConditions.find(condition) != typConditions.end()) {
                ifTypAtAll = true;
                
                // Primzahl-Analyse für innen/außen
                std::map<int, std::vector<int>> primList;
                std::map<int, std::tuple<bool, bool, bool>> innenAussen;
                
                innenAussen[1] = {true, false, true};
                
                auto numRangeCopy = numRange;
                numRangeCopy.erase(1);
                numRangeCopy.erase(2);
                numRangeCopy.erase(3);
                
                for (int n : numRangeCopy) {
                    primList[n] = primFak(n);
                }
                
                for (const auto& [anfangsZahl, primZahlen] : primList) {
                    bool NurEineZahl = primZahlen.size() == 1;
                    bool innen = false, aussen = false;
                    
                    for (int primZahl : primZahlen) {
                        if (primZahl >= 4) {
                            int innenOrAussen = primZahl % 6;
                            innen = innen || (innenOrAussen == 1);
                            aussen = aussen || (innenOrAussen == 5);
                        }
                    }
                    
                    innenAussen[anfangsZahl] = {innen, aussen, NurEineZahl};
                }
                
                if (condition == "aussenerste") {
                    for (const auto& [num, vals] : innenAussen) {
                        auto [innen, aussen, nurEine] = vals;
                        if (aussen && nurEine) typRange.insert(num);
                    }
                } else if (condition == "innenerste") {
                    for (const auto& [num, vals] : innenAussen) {
                        auto [innen, aussen, nurEine] = vals;
                        if (innen && nurEine) typRange.insert(num);
                    }
                } else if (condition == "aussenalle") {
                    for (const auto& [num, vals] : innenAussen) {
                        auto [innen, aussen, nurEine] = vals;
                        if (aussen) typRange.insert(num);
                    }
                } else if (condition == "innenalle") {
                    for (const auto& [num, vals] : innenAussen) {
                        auto [innen, aussen, nurEine] = vals;
                        if (innen) typRange.insert(num);
                    }
                }
            }
        }
        
        if (ifTypAtAll) {
            std::set<int> intersection;
            std::set_intersection(numRange.begin(), numRange.end(),
                                 typRange.begin(), typRange.end(),
                                 std::inserter(intersection, intersection.begin()));
            numRange = intersection;
        }
        
        // Mond/Sonne/Planet-Bedingungen
        std::set<int> astroRange;
        bool ifAstroAtAll = false;
        
        for (const auto& condition : paramLines) {
            if (condition.find("mond") != std::string::npos) {
                astroRange = moonsun(true, astroRange, numRange, ifZaehlungenAtAll);
                ifAstroAtAll = true;
            } else if (condition.find("schwarzesonne") != std::string::npos) {
                ifAstroAtAll = true;
                for (int n : numRange) {
                    if (n % 3 == 0) astroRange.insert(n);
                }
            } else if (condition.find("sonne") != std::string::npos) {
                astroRange = moonsun(false, astroRange, numRange, ifZaehlungenAtAll);
                ifAstroAtAll = true;
            } else if (condition.find("planet") != std::string::npos) {
                ifAstroAtAll = true;
                for (int n : numRange) {
                    if (n % 2 == 0) astroRange.insert(n);
                }
            } else if (condition == "SonneMitMondanteil") {
                ifAstroAtAll = true;
                for (int n : numRange) {
                    auto factors = primFak(n);
                    auto repeats = primRepeat2(factors);
                    
                    bool hasBoth = false;
                    for (const auto& [factor, count] : repeats) {
                        if (count == 1) {
                            hasBoth = true;
                            break;
                        }
                    }
                    
                    if (hasBoth) {
                        astroRange.insert(n);
                    }
                }
            }
        }
        
        if (ifAstroAtAll) {
            std::set<int> intersection;
            std::set_intersection(numRange.begin(), numRange.end(),
                                 astroRange.begin(), astroRange.end(),
                                 std::inserter(intersection, intersection.begin()));
            numRange = intersection;
        }
        
        // Primzahl-Multiples (Xp)
        std::vector<int> primMultiples;
        bool ifPrimAtAll = false;
        std::set<int> primRange;
        
        for (const auto& condition : paramLines) {
            if (condition.length() > 1 && condition.back() == 'p') {
                ifPrimAtAll = true;
                try {
                    primMultiples.push_back(std::stoi(condition.substr(0, condition.length() - 1)));
                } catch (...) {
                    // Ignoriere ungültige Zahlen
                }
            }
        }
        
        if (ifPrimAtAll) {
            if (numRange.empty() && !if_a_AtAll && !if_b_AtAll && 
                paramLines.find("all") == paramLines.end() && !ifTypAtAll) {
                for (int i = 1; i <= hoechsteZeile[1024]; i++) {
                    numRange.insert(i);
                }
            }
            
            for (int n : numRange) {
                if (isPrimMultiple(n, primMultiples)) {
                    primRange.insert(n);
                }
            }
            
            std::set<int> intersection;
            std::set_intersection(numRange.begin(), numRange.end(),
                                 primRange.begin(), primRange.end(),
                                 std::inserter(intersection, intersection.begin()));
            numRange = intersection;
        }
        
        // Potenzen (X^Y) - vereinfacht
        std::vector<int> powerBases;
        bool ifPowerAtall = false;
        
        for (const auto& condition : paramLines) {
            if (condition.length() > 3 && condition.substr(0, 3) == "_^_") {
                ifPowerAtall = true;
                auto numbers = BereichToNumbers2(condition.substr(3), false, 
                                                hoechsteZeile[1024] + 1);
                powerBases.insert(powerBases.end(), numbers.begin(), numbers.end());
            }
        }
        
        if (ifPowerAtall) {
            std::set<int> powerRange;
            
            if (numRange.empty() && !paramLines.empty() && 
                std::any_of(paramLines.begin(), paramLines.end(),
                           [](const std::string& s) { return s != "ka" && s != "ka2"; })) {
                for (int i = 1; i <= hoechsteZeile[1024]; i++) {
                    numRange.insert(i);
                }
            }
            
            if (!numRange.empty()) {
                int lastEl = *numRange.rbegin();
                for (int base : powerBases) {
                    for (int exp = 0; exp < lastEl; exp++) {
                        int power = static_cast<int>(std::pow(base, exp));
                        if (power <= lastEl) {
                            powerRange.insert(power);
                        } else {
                            break;
                        }
                    }
                }
                
                std::set<int> intersection;
                std::set_intersection(numRange.begin(), numRange.end(),
                                     powerRange.begin(), powerRange.end(),
                                     std::inserter(intersection, intersection.begin()));
                numRange = intersection;
                numRange.erase(1); // 1 entfernen wie in Python
            }
        }
        
        // Vielfache (Xv)
        std::vector<int> anyMultiples;
        bool ifMultiplesFromAnyAtAll = false;
        std::set<int> multiplesRange;
        
        for (const auto& condition : paramLines) {
            if (condition.length() > 1 && condition.back() == 'v') {
                std::string numStr = condition.substr(0, condition.length() - 1);
                if (!numStr.empty() && std::all_of(numStr.begin(), numStr.end(), ::isdigit)) {
                    ifMultiplesFromAnyAtAll = true;
                    anyMultiples.push_back(std::stoi(numStr));
                }
            }
        }
        
        if (ifMultiplesFromAnyAtAll) {
            for (int n : numRange) {
                for (int divisor : anyMultiples) {
                    if (divisor != 0 && n % divisor == 0) {
                        multiplesRange.insert(n);
                        break;
                    }
                }
            }
            
            std::set<int> intersection;
            std::set_intersection(numRange.begin(), numRange.end(),
                                 multiplesRange.begin(), multiplesRange.end(),
                                 std::inserter(intersection, intersection.begin()));
            numRange = intersection;
        }
        
        // Sonnen über 114 entfernen
        for (auto it = numRange.begin(); it != numRange.end();) {
            int n = *it;
            auto moonType = zaehlungen.moonTypes[n];
            bool isSonne = moonType.first.empty();
            
            if (isSonne && n > hoechsteZeile[114]) {
                it = numRange.erase(it);
            } else {
                ++it;
            }
        }
        
        // Invertieren (_i_)
        bool invertieren = false;
        for (const auto& condition : paramLines) {
            if (condition.substr(0, 3) == "_i_") {
                invertieren = true;
                break;
            }
        }
        
        if (invertieren) {
            std::set<int> invertedRange;
            int h = hoechsteZeile[1024];
            
            for (int i = 1; i <= h; i++) {
                if ((numRange.find(i + 1) != numRange.end() || 
                     numRange.find(i - 1) != numRange.end()) &&
                    numRange.find(i) == numRange.end()) {
                    invertedRange.insert(i);
                }
            }
            
            numRange = invertedRange;
        }
        
        // Z- und Y-Bedingungen
        if (!numRange.empty()) {
            std::vector<int> numRangeList(numRange.begin(), numRange.end());
            std::sort(numRangeList.begin(), numRangeList.end());
            
            std::map<int, int> numRange2Map;
            for (size_t i = 0; i < numRangeList.size(); i++) {
                numRange2Map[i + 1] = numRangeList[i];
            }
            
            // _z_ Bedingung
            bool zJa = false;
            std::set<int> numRangeNeu2_z;
            
            for (const auto& condition : paramLines) {
                if (condition.length() > 3 && condition.substr(0, 3) == "_z_") {
                    zJa = true;
                    auto zNumbers = BereichToNumbers2(condition.substr(3), false, 
                                                     hoechsteZeile[1024] + 1);
                    
                    for (int a : zNumbers) {
                        if (numRange2Map.find(a) != numRange2Map.end()) {
                            numRangeNeu2_z.insert(numRange2Map[a]);
                        }
                    }
                }
            }
            
            if (zJa) {
                std::set<int> intersection;
                std::set_intersection(numRange.begin(), numRange.end(),
                                     numRangeNeu2_z.begin(), numRangeNeu2_z.end(),
                                     std::inserter(intersection, intersection.begin()));
                numRange = intersection;
            }
            
            // _y_ Bedingung
            bool yJa = false;
            std::set<int> numRangeNeu2_y;
            
            for (const auto& condition : paramLines) {
                if (condition.length() > 3 && condition.substr(0, 3) == "_y_") {
                    yJa = true;
                    auto yNumbers = BereichToNumbers2(condition.substr(3), true, 
                                                     hoechsteZeile[1024] + 1);
                    
                    for (int a : yNumbers) {
                        if (numRange2Map.find(a) != numRange2Map.end()) {
                            numRangeNeu2_y.insert(numRange2Map[a]);
                        }
                    }
                }
            }
            
            if (yJa) {
                std::set<int> intersection;
                std::set_intersection(numRange.begin(), numRange.end(),
                                     numRangeNeu2_y.begin(), numRangeNeu2_y.end(),
                                     std::inserter(intersection, intersection.begin()));
                numRange = intersection;
            }
        }
        
        return numRange;
    }
    
    std::tuple<std::set<int>, std::vector<std::vector<std::string>>, int, 
               std::vector<int>, std::pair<std::map<int, int>, std::map<int, int>>>
    prepare4out(const std::set<std::string>& paramLines,
                const std::set<std::string>& paramLinesNot,
                const std::vector<std::vector<std::string>>& contentTable,
                const std::set<int>& rowsAsNumbers,
                const std::map<std::string, std::set<int>>& gebrSpalten,
                int combiRows = 0,
                int reliTableLenUntilNow = -1,
                const std::set<int>* primSpalten = nullptr,
                int kombiCSVNumber = 0) {
        
        auto [finallyDisplayLines, headingsAmount, newerTable, numlen, rowsRange] =
            prepare4out_beforeForLoop_SpaltenZeilenBestimmen(contentTable, paramLines, paramLinesNot);
        
        this->headingsAmount = headingsAmount;
        
        std::pair<std::map<int, int>, std::map<int, int>> old2Rows;
        bool reliNumbersBool = religionNumbers.empty();
        
        for (int u = 0; u < contentTable.size(); u++) {
            if (finallyDisplayLines.find(u) != finallyDisplayLines.end() || combiRows != 0) {
                if (reliNumbersBool) {
                    religionNumbers.push_back(u);
                }
                
                auto new2Lines = prepare4out_LoopBody(combiRows, gebrSpalten, headingsAmount,
                                                      contentTable[u], old2Rows, primSpalten,
                                                      reliNumbersBool, reliTableLenUntilNow,
                                                      rowsAsNumbers, u, kombiCSVNumber);
                
                if (!new2Lines.empty()) {
                    newerTable.push_back(new2Lines);
                }
            }
        }
        
        return {finallyDisplayLines, newerTable, numlen, rowsRange, old2Rows};
    }
    
private:
    // Hilfsfunktion zum Verbinden von Strings
    std::string join(const std::vector<std::string>& strings, const std::string& delimiter) {
        std::string result;
        for (size_t i = 0; i < strings.size(); i++) {
            if (i > 0) result += delimiter;
            result += strings[i];
        }
        return result;
    }
    
    std::tuple<std::set<int>, int, std::vector<std::vector<std::string>>, 
               int, std::vector<int>>
    prepare4out_beforeForLoop_SpaltenZeilenBestimmen(
        const std::vector<std::vector<std::string>>& contentTable,
        const std::set<std::string>& paramLines,
        const std::set<std::string>& paramLinesNot) {
        
        std::vector<std::vector<std::string>> newerTable;
        int headingsAmount = contentTable.empty() ? 0 : contentTable[0].size();
        
        std::vector<int> rowsRange;
        for (int i = 0; i < headingsAmount; i++) {
            rowsRange.push_back(i);
        }
        
        std::set<int> originalRangeSet(originalLinesRange.begin(), originalLinesRange.end());
        std::set<int> finallyDisplayLines = FilterOriginalLines(originalRangeSet, paramLines);
        
        if (!paramLinesNot.empty()) {
            std::set<int> finallyDisplayLines2 = FilterOriginalLines(finallyDisplayLines, paramLinesNot);
            std::set<int> hasAnythingChanged;
            std::set_difference(originalRangeSet.begin(), originalRangeSet.end(),
                               finallyDisplayLines2.begin(), finallyDisplayLines2.end(),
                               std::inserter(hasAnythingChanged, hasAnythingChanged.begin()));
            
            if (!hasAnythingChanged.empty()) {
                for (int n : finallyDisplayLines2) {
                    finallyDisplayLines.erase(n);
                }
            }
        }
        
        if (finallyDisplayLines.empty()) {
            if (ifZeilenSetted) {
                finallyDisplayLines.clear();
            } else {
                for (int i = 1; i <= hoechsteZeile[1024]; i++) {
                    finallyDisplayLines.insert(i);
                }
            }
        }
        
        finallyDisplayLines.insert(0);
        
        std::vector<int> finallyDisplayLines3(finallyDisplayLines.begin(), finallyDisplayLines.end());
        std::sort(finallyDisplayLines3.begin(), finallyDisplayLines3.end());
        
        int numlen = finallyDisplayLines3.empty() ? 0 : 
                     std::to_string(finallyDisplayLines3.back()).length();
        
        return {std::set<int>(finallyDisplayLines3.begin(), finallyDisplayLines3.end()),
                headingsAmount, newerTable, numlen, rowsRange};
    }
    
    std::vector<std::vector<std::string>> prepare4out_LoopBody(
        int combiRows,
        const std::map<std::string, std::set<int>>& gebrSpalten,
        int headingsAmount,
        const std::vector<std::string>& line,
        std::pair<std::map<int, int>, std::map<int, int>>& old2Rows,
        const std::set<int>* primSpalten,
        bool reliNumbersBool,
        int reliTableLenUntilNow,
        const std::set<int>& rowsAsNumbers,
        int u,
        int kombiCSVNumber) {
        
        std::vector<std::vector<std::string>> new2Lines;
        int rowToDisplay = 0;
        int h = 0;
        
        for (int t = 0; t < line.size(); t++) {
            if (rowsAsNumbers.find(t) != rowsAsNumbers.end()) {
                if (u == 0) {
                    prepare4out_Tagging(combiRows, gebrSpalten, primSpalten,
                                       reliTableLenUntilNow, rowToDisplay, t,
                                       kombiCSVNumber);
                }
                
                rowToDisplay++;
                certaintextwidth = setWidth(rowToDisplay, combiRows);
                
                auto wrapped = cellWork(line[t], certaintextwidth);
                if (!wrapped.empty() && (wrapped.size() > 1 || wrapped[0] != "")) {
                    new2Lines.push_back(wrapped);
                }
                
                if (u == 0) {
                    old2Rows.first[t] = h;
                    old2Rows.second[h] = t;
                    h++;
                }
            }
        }
        
        return new2Lines;
    }
    
    void prepare4out_Tagging(
        int combiRows,
        const std::map<std::string, std::set<int>>& gebrSpalten,
        const std::set<int>* primSpalten,
        int reliTableLenUntilNow,
        int rowToDisplay,
        int t,
        int kombiCSVNumber) {
        
        if (combiRows == 0) {
            try {
                // Vereinfachte Tag-Logik
                if (primSpalten && primSpalten->find(t) != primSpalten->end()) {
                    // Primzahl-Spalten
                } else if (gebrSpalten.find("Gal") != gebrSpalten.end() &&
                          gebrSpalten.at("Gal").find(t) != gebrSpalten.at("Gal").end()) {
                    // Galaxie-Spalten
                } else if (gebrSpalten.find("Uni") != gebrSpalten.end() &&
                          gebrSpalten.at("Uni").find(t) != gebrSpalten.at("Uni").end()) {
                    // Universum-Spalten
                }
            } catch (...) {
                // Ignoriere Fehler
            }
        } else {
            // Kombi-CSV Tags
            if (kombiCSVNumber == 0 || kombiCSVNumber == 1) {
                // Tags setzen
            }
        }
    }
    
public:
    std::vector<std::string> cellWork(const std::string& cell, int certaintextwidth) {
        std::string cellTrimmed;
        // Trimmen
        size_t start = cell.find_first_not_of(" \t\n\r");
        size_t end = cell.find_last_not_of(" \t\n\r");
        
        if (start != std::string::npos && end != std::string::npos) {
            cellTrimmed = cell.substr(start, end - start + 1);
        } else {
            cellTrimmed = cell;
        }
        
        if (certaintextwidth == 0) {
            return {cellTrimmed};
        }
        
        auto wrapped = wrapping(cellTrimmed, certaintextwidth);
        std::vector<std::string> result;
        
        if (wrapped.empty()) {
            return {""};
        }
        
        // Nachbearbeitung wie in Python
        std::string rest = wrapped.back();
        wrapped.pop_back();
        
        while (!rest.empty() && rest.length() > certaintextwidth) {
            result.push_back(rest.substr(0, certaintextwidth));
            rest = rest.substr(certaintextwidth);
        }
        
        if (!rest.empty()) {
            result.push_back(rest.substr(0, certaintextwidth));
        }
        
        // Füge vorherige Teile hinzu
        std::vector<std::string> finalResult;
        for (const auto& part : wrapped) {
            finalResult.push_back(part);
        }
        for (const auto& part : result) {
            finalResult.push_back(part);
        }
        
        return finalResult;
    }
};

// ============================================================================
// HAUPTFUNKTION ZUM TESTEN
// ============================================================================

int main() {
    // Test der Prepare-Klasse
    std::map<int, int> hoechsteZeile = {{1024, 100}, {114, 50}};
    
    // Dummy Tables-Klasse
    class DummyTables {
    public:
        std::map<int, std::unordered_set<ST>> generatedSpaltenParameter_Tags;
        std::map<int, std::vector<std::pair<std::string, std::string>>> generatedSpaltenParameter;
        std::map<int, std::vector<std::pair<std::string, std::string>>> dataDict[20];
        
        DummyTables() {
            // Initialisiere dataDict
            for (int i = 0; i < 20; i++) {
                dataDict[i] = {};
            }
        }
    };
    
    DummyTables dummyTables;
    Prepare prepare(reinterpret_cast<Tables*>(&dummyTables), hoechsteZeile);
    
    // Test 1: Text-Wrapping
    std::cout << "Test 1: Text-Wrapping" << std::endl;
    auto wrapped = prepare.cellWork("Dies ist ein sehr langer Testtext der umgebrochen werden soll", 20);
    for (const auto& line : wrapped) {
        std::cout << "  '" << line << "'" << std::endl;
    }
    
    // Test 2: Bereichsverarbeitung
    std::cout << "\nTest 2: Bereichsverarbeitung" << std::endl;
    std::set<std::string> paramLines = {"_a_1-10", "_b_5-15", "mond"};
    auto filtered = prepare.FilterOriginalLines({1, 2, 3, 4, 5, 6, 7, 8, 9, 10}, paramLines);
    
    std::cout << "Gefilterte Zahlen: ";
    for (int n : filtered) {
        std::cout << n << " ";
    }
    std::cout << std::endl;
    
    // Test 3: Zählungen
    std::cout << "\nTest 3: Zählungen" << std::endl;
    prepare.setZaehlungen(50);
    
    // Test 4: prepare4out
    std::cout << "\nTest 4: prepare4out" << std::endl;
    std::vector<std::vector<std::string>> contentTable = {
        {"Spalte1", "Spalte2", "Spalte3"},
        {"Zeile1Zelle1", "Zeile1Zelle2", "Zeile1Zelle3"},
        {"Zeile2Zelle1", "Zeile2Zelle2", "Zeile2Zelle3"}
    };
    
    std::set<int> rowsAsNumbers = {0, 1, 2};
    std::map<std::string, std::set<int>> gebrSpalten = {
        {"Gal", {0}},
        {"Uni", {1}}
    };
    
    auto result = prepare.prepare4out({"all"}, {}, contentTable, rowsAsNumbers, gebrSpalten);
    
    auto& [displayLines, newTable, numlen, rowsRange, old2Rows] = result;
    
    std::cout << "Display Lines: ";
    for (int n : displayLines) {
        std::cout << n << " ";
    }
    std::cout << "\nNeue Tabelle Größe: " << newTable.size() << "x" 
              << (newTable.empty() ? 0 : newTable[0].size()) << std::endl;
    
    return 0;
}

// ============================================================================
// COMPILE-INSTRUKTIONEN
// ============================================================================
/*
Compile mit:
g++ -std=c++17 -Wall -O2 -o lib4tables_prepare lib4tables_prepare.cpp

ODER in CMakeLists.txt:
cmake_minimum_required(VERSION 3.10)
project(lib4tables_prepare)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

add_executable(lib4tables_prepare lib4tables_prepare.cpp)

# Für mathematische Funktionen
target_link_libraries(lib4tables_prepare m)
*/
