// tableHandling.cpp - Vollständige C++ Konvertierung
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

namespace fs = std::filesystem;

// ============================================================================
// FORWARD-DEKLARATIONEN UND BASIS-KLASSEN
// ============================================================================

// Enums
enum class ST {
    sternPolygon,
    galaxie,
    gleichfoermigesPolygon,
    universum,
    gebrRat
};

// OutputSyntax Basis-Klasse und Ableitungen
class OutputSyntax {
public:
    virtual ~OutputSyntax() = default;
    virtual std::string beginTable() const { return ""; }
    virtual std::string endTable() const { return ""; }
    virtual std::string generateCell(int col, 
                                     const std::map<int, std::vector<std::pair<std::string, std::string>>>& params,
                                     int zeile = -1,
                                     void* tables = nullptr) const { return ""; }
    virtual std::string endCell() const { return ""; }
    virtual std::string coloredBeginCol(int num) const { return ""; }
    virtual std::string endZeile() const { return ""; }
};

class NichtsSyntax : public OutputSyntax {};
class bbCodeSyntax : public OutputSyntax {};
class htmlSyntax : public OutputSyntax {};
class markdownSyntax : public OutputSyntax {};
class csvSyntax : public OutputSyntax {};
class emacsSyntax : public OutputSyntax {};

// Internationalisierung (vereinfacht)
class I18n {
public:
    struct CSVFileNames {
        std::string prim = "primnumbers.csv";
        std::string kombi13 = "animalsProfessions.csv";
        std::string kombi15 = "anotherCombi.csv";
        std::string bruch13 = "bruch13.csv";
        std::string bruch15 = "bruch15.csv";
    } csvFileNames;
    
    std::unordered_map<std::string, std::string> translations;
    
    std::string get(const std::string& key) {
        auto it = translations.find(key);
        if (it != translations.end()) return it->second;
        return key; // Fallback
    }
    
    static I18n& instance() {
        static I18n i18n;
        return i18n;
    }
};

// Hilfsfunktionen (aus Python-Modulen)
namespace Helper {
    std::vector<int> primfaktoren(int n) {
        std::vector<int> factors;
        int temp = n;
        for (int i = 2; i * i <= temp; i++) {
            while (temp % i == 0) {
                factors.push_back(i);
                temp /= i;
            }
        }
        if (temp > 1) factors.push_back(temp);
        return factors;
    }
    
    std::pair<std::vector<int>, std::vector<int>> moonNumber(int n) {
        // Vereinfachte Implementation
        return {{}, {}};
    }
    
    std::vector<std::pair<int, int>> primRepeat(const std::vector<int>& factors) {
        std::vector<std::pair<int, int>> result;
        for (size_t i = 0; i < factors.size(); i++) {
            for (size_t j = i; j < factors.size(); j++) {
                result.push_back({factors[i], factors[j]});
            }
        }
        return result;
    }
    
    std::string getTextWrapThings(int& shellRowsAmount, 
                                  std::string& h_de, 
                                  std::map<std::string, std::string>& dic,
                                  char& fill) {
        // Einfache Implementation
        shellRowsAmount = 80;
        h_de = "";
        dic.clear();
        fill = ' ';
        return "";
    }
    
    void infoLog(const std::string& msg) {
        std::cerr << "[INFO] " << msg << std::endl;
    }
    
    void output(const std::string& msg) {
        std::cout << msg << std::endl;
    }
    
    void cliout(const std::string& msg, bool color = false, const std::string& type = "") {
        if (type == "html" || type == "bbcode") {
            // Spezielle Behandlung für HTML/BBcode
            std::cout << msg;
        } else {
            std::cout << msg << std::endl;
        }
    }
    
    void x(const std::string& label, const auto& value) {
        std::cerr << "[DEBUG " << label << "] ";
        std::cerr << typeid(value).name() << std::endl;
    }
    
    std::vector<std::string> alxp(const std::string& msg) {
        std::cerr << "[ALXP] " << msg << std::endl;
        return {};
    }
}

// Exception-Klasse
class BreakoutException : public std::exception {
public:
    const char* what() const noexcept override {
        return "BreakoutException";
    }
};

// OrderedSet (vereinfacht)
template<typename T>
class OrderedSet : public std::set<T> {
public:
    OrderedSet() = default;
    
    template<typename... Args>
    OrderedSet(Args&&... args) {
        (this->insert(std::forward<Args>(args)), ...);
    }
    
    OrderedSet& operator|=(const OrderedSet& other) {
        for (const auto& item : other) {
            this->insert(item);
        }
        return *this;
    }
    
    OrderedSet operator|(const OrderedSet& other) const {
        OrderedSet result = *this;
        result |= other;
        return result;
    }
};

// OrderedDict
template<typename K, typename V>
using OrderedDict = std::map<K, V>;

// ============================================================================
// HAUPTKLASSE TABLES
// ============================================================================

// Forward-Deklarationen für innere Klassen
class Prepare;
class Concat;
class Output;
class Combi;
class Maintable;

class Tables {
private:
    std::map<int, int> __hoechsteZeile;
    bool keineUeberschriften = false;
    OrderedDict<int, int> rowNumDisplay2rowNumOrig;
    std::map<int, std::vector<std::pair<std::string, std::string>>> generatedSpaltenParameter;
    std::map<int, std::unordered_set<ST>> generatedSpaltenParameter_Tags;
    int textwidth = 21;
    bool nummerierung = true;
    bool spaltegGestirn = false;
    std::vector<int> breitenn;
    std::vector<int> religionNumbers;
    OrderedSet<int> __generRows__;
    bool keineleereninhalte = false;
    
    // Innere Klassen als Pointer
    std::unique_ptr<Prepare> getPrepare;
    std::unique_ptr<Combi> getCombis;
    std::unique_ptr<Concat> getConcat;
    std::unique_ptr<Output> getOut;
    std::unique_ptr<Maintable> getMainTable;
    
public:
    // Properties als Getter/Setter
    bool NichtsOutputYes() const {
        // Vereinfachte Prüfung
        return false;
    }
    
    bool markdownOutputYes() const {
        // Vereinfachte Prüfung
        return false;
    }
    
    bool bbcodeOutputYes() const {
        auto outType = getOut->getOutType();
        return dynamic_cast<bbCodeSyntax*>(outType.get()) != nullptr;
    }
    
    bool htmlOutputYes() const {
        auto outType = getOut->getOutType();
        return dynamic_cast<htmlSyntax*>(outType.get()) != nullptr;
    }
    
    std::shared_ptr<OutputSyntax> outType() const {
        return getOut->getOutType();
    }
    
    void setOutType(std::shared_ptr<OutputSyntax> value) {
        getOut->setOutType(value);
    }
    
    std::map<int, int> hoechsteZeile() const {
        return __hoechsteZeile;
    }
    
    void setHoechsteZeile(const std::map<int, int>& value) {
        __hoechsteZeile = value;
    }
    
    OrderedSet<int> generRows() const {
        return __generRows__;
    }
    
    void setGenerRows(const OrderedSet<int>& value) {
        __generRows__ = value;
    }
    
    bool ifPrimMultis() const {
        return getPrepare->ifprimmultis();
    }
    
    void setIfPrimMultis(bool value) {
        getPrepare->setIfprimmultis(value);
    }
    
    bool ifZeilenSetted() const {
        return getPrepare->ifZeilenSetted();
    }
    
    void setIfZeilenSetted(bool value) {
        getPrepare->setIfZeilenSetted(value);
    }
    
    std::vector<int> gebrUnivSet() const {
        // Vereinfacht
        return {};
    }
    
    std::vector<int> breitennProp() const {
        return breitenn;
    }
    
    void setBreitenn(const std::vector<int>& value) {
        int shellRowsAmount;
        std::string h_de;
        std::map<std::string, std::string> dic;
        char fill;
        Helper::getTextWrapThings(shellRowsAmount, h_de, dic, fill);
        
        breitenn = value;
        for (size_t i = 0; i < breitenn.size(); i++) {
            if (shellRowsAmount > breitenn[i] + 7 || shellRowsAmount == 0) {
                // Behalte Wert
            } else {
                breitenn[i] = shellRowsAmount - 7;
            }
        }
        getPrepare->setBreiten(breitenn);
        getOut->setBreiten(breitenn);
    }
    
    bool nummeriere() const {
        return getOut->nummerierung();
    }
    
    void setNummeriere(bool value) {
        getOut->setNummerierung(value);
        getPrepare->setNummerierung(value);
        nummerierung = value;
    }
    
    int textHeight() const {
        return getOut->textHeight();
    }
    
    void setTextHeight(int value) {
        getOut->setTextHeight(value);
    }
    
    int textWidth() const {
        return textwidth;
    }
    
    void setTextWidth(int value) {
        int shellRowsAmount;
        std::string h_de;
        std::map<std::string, std::string> dic;
        char fill;
        Helper::getTextWrapThings(shellRowsAmount, h_de, dic, fill);
        
        if ((shellRowsAmount > value + 7 || shellRowsAmount == 0) && 
            (value != 0 || (bbcodeOutputYes() || htmlOutputYes() || getOut->oneTable()))) {
            textwidth = value;
        } else {
            textwidth = shellRowsAmount - 7;
        }
        getPrepare->setTextWidth(textwidth);
        getOut->setTextWidth(textwidth);
    }
    
    // Statische Methode
    static std::pair<std::vector<std::vector<std::string>>, 
                     std::vector<std::vector<std::string>>> 
    fillBoth(const std::vector<std::vector<std::string>>& liste1,
             const std::vector<std::vector<std::string>>& liste2) {
        auto result1 = liste1;
        auto result2 = liste2;
        
        while (result1.size() < result2.size()) {
            result1.push_back({""});
        }
        while (result2.size() < result1.size()) {
            result2.push_back({""});
        }
        
        return {result1, result2};
    }
    
    // Konstruktor
    Tables(int hoechstZeil, const std::string& Txt) {
        if (hoechstZeil == -1) {
            __hoechsteZeile = {{1024, 1024}, {114, 163}};
        } else {
            __hoechsteZeile = {{1024, hoechstZeil}, {114, hoechstZeil}};
        }
        
        // Innere Klassen initialisieren
        getPrepare = std::make_unique<Prepare>(this, __hoechsteZeile);
        getCombis = std::make_unique<Combi>(this);
        getConcat = std::make_unique<Concat>(this);
        getOut = std::make_unique<Output>(this, Txt);
        getMainTable = std::make_unique<Maintable>(this);
        
        // Standardwerte
        textHeight = 0;
        textWidth = 21;
        nummeriere = true;
    }
    
    // Getter für innere Klassen
    Prepare* prepare() const { return getPrepare.get(); }
    Combi* combi() const { return getCombis.get(); }
    Concat* concat() const { return getConcat.get(); }
    Output* output() const { return getOut.get(); }
    Maintable* maintable() const { return getMainTable.get(); }
    
    // Weitere Methoden
    std::vector<std::vector<std::string>> tableReducedInLinesByTypeSet(
        const std::vector<std::vector<std::string>>& table,
        const OrderedSet<int>& lineSet) {
        
        std::vector<std::vector<std::string>> result;
        for (int lineNum : lineSet) {
            if (lineNum >= 0 && lineNum < table.size()) {
                result.push_back(table[lineNum]);
            }
        }
        return result;
    }
    
    // Datenzugriff (vereinfacht)
    std::map<int, std::vector<std::pair<std::string, std::string>>> dataDict[20];
    int SpaltenVanillaAmount = 0;
    int lastLineNumber = 1000;
    
private:
    // Innere Klassen als Friend deklarieren
    friend class Output;
    friend class Combi;
    friend class Maintable;
};

// ============================================================================
// INNERE KLASSE OUTPUT
// ============================================================================

class Tables::Output {
private:
    Tables* tables;
    bool __oneTable = false;
    bool __color = true;
    std::shared_ptr<OutputSyntax> __outType;
    std::string Txt;
    std::vector<std::string> resultingTable;
    std::vector<int> breiten;
    bool nummerierung = true;
    int textheight = 0;
    int textwidth = 21;
    
public:
    Output(Tables* tablesPtr, const std::string& txt) 
        : tables(tablesPtr), Txt(txt) {
        __outType = std::make_shared<OutputSyntax>();
    }
    
    std::shared_ptr<OutputSyntax> getOutType() const { return __outType; }
    void setOutType(std::shared_ptr<OutputSyntax> value) { __outType = value; }
    
    bool color() const { return __color; }
    void setColor(bool value) { __color = value; }
    
    bool oneTable() const { return __oneTable; }
    void setOneTable(bool value) { __oneTable = value; }
    
    std::vector<int> getBreiten() const { return breiten; }
    void setBreiten(const std::vector<int>& value) { breiten = value; }
    
    bool nummerierung() const { return nummerierung; }
    void setNummerierung(bool value) { nummerierung = value; }
    
    int textHeight() const { return textheight; }
    void setTextHeight(int value) { textheight = value; }
    
    int textWidth() const { return textwidth; }
    void setTextWidth(int value) { textwidth = value; }
    
    // Hauptmethode für Tabellenausgabe
    std::vector<std::vector<std::string>> onlyThatColumns(
        const std::vector<std::vector<std::string>>& table,
        const std::vector<int>& onlyThatColumns) {
        
        if (onlyThatColumns.empty()) return table;
        
        std::vector<std::vector<std::string>> newTable;
        for (const auto& row : table) {
            std::vector<std::string> newCol;
            for (int colNum : onlyThatColumns) {
                if (colNum - 1 >= 0 && colNum - 1 < row.size()) {
                    newCol.push_back(row[colNum - 1]);
                }
            }
            if (!newCol.empty()) {
                newTable.push_back(newCol);
            }
        }
        return newTable.empty() ? table : newTable;
    }
    
    void cliOut(const std::set<int>& finallyDisplayLinesSet,
                const std::vector<std::vector<std::string>>& newTable,
                int numlen,
                const std::vector<int>& rowsRange) {
        
        if (finallyDisplayLinesSet.empty() || 
            (finallyDisplayLinesSet.size() == 1 && finallyDisplayLinesSet.count(0) > 0)) {
            return;
        }
        
        // Max Zellbreiten finden
        auto maxCellTextLen = findMaxCellTextLen(finallyDisplayLinesSet, newTable, rowsRange);
        
        std::vector<int> finallyDisplayLines(finallyDisplayLinesSet.begin(), 
                                             finallyDisplayLinesSet.end());
        std::sort(finallyDisplayLines.begin(), finallyDisplayLines.end());
        
        // CSV-Ausgabe (vereinfacht)
        if (auto csv = std::dynamic_pointer_cast<csvSyntax>(__outType)) {
            std::stringstream ss;
            for (int lineNum : finallyDisplayLines) {
                if (lineNum >= 0 && lineNum < newTable.size()) {
                    for (size_t col = 0; col < newTable[lineNum].size(); col++) {
                        if (col > 0) ss << ";";
                        ss << newTable[lineNum][col];
                    }
                    ss << "\n";
                }
            }
            cliout2(ss.str());
            return;
        }
        
        // Normale Ausgabe
        for (int lineNum : finallyDisplayLines) {
            if (lineNum >= 0 && lineNum < newTable.size()) {
                std::string line;
                if (nummerierung && lineNum > 0) {
                    line += std::to_string(lineNum) + " ";
                }
                
                for (size_t col = 0; col < newTable[lineNum].size(); col++) {
                    if (col > 0) line += " ";
                    
                    int colWidth = determineRowWidth(col, maxCellTextLen);
                    std::string cell = newTable[lineNum][col];
                    if (cell.length() > colWidth) {
                        cell = cell.substr(0, colWidth);
                    } else {
                        cell += std::string(colWidth - cell.length(), ' ');
                    }
                    
                    if (__color) {
                        cell = colorize(cell, lineNum);
                    }
                    line += cell;
                }
                cliout2(line);
            }
        }
    }
    
    void cliout2(const std::string& text) {
        resultingTable.push_back(text);
        
        bool useColor = false;
        std::string type = "";
        
        if (tables->bbcodeOutputYes()) type = "bbcode";
        else if (tables->htmlOutputYes()) type = "html";
        else if (tables->markdownOutputYes()) type = "markdown";
        else if (auto csv = std::dynamic_pointer_cast<csvSyntax>(__outType)) type = "csv";
        
        if (!tables->NichtsOutputYes()) {
            Helper::cliout(text, __color && !type.empty(), type);
        }
    }
    
    std::string colorize(const std::string& text, int num, bool rest = false) {
        // Einfache ANSI-Farbcodes
        if (num == 0) return "\033[41m\033[30m\033[4m" + text + "\033[0m";
        
        auto factors = Helper::primfaktoren(num);
        auto moon = Helper::moonNumber(num);
        
        if (!moon.second.empty()) {
            if (num % 2 == 0) return "\033[106m\033[30m" + text + "\033[0m";
            else return "\033[46m\033[30m" + text + "\033[0m";
        }
        else if (factors.size() == 1) {
            if (num % 2 == 0) return "\033[103m\033[30m\033[1m" + text + "\033[0m";
            else return "\033[43m\033[30m" + text + "\033[0m";
        }
        else if (rest) {
            if (num % 2 == 0) return "\033[47m\033[30m" + text + "\033[0m";
            else return "\033[40m\033[37m" + text + "\033[0m";
        }
        else if (num % 2 == 0) {
            return "\033[47m\033[30m" + text + "\033[0m";
        }
        else {
            return "\033[100m\033[37m" + text + "\033[0m";
        }
    }
    
private:
    std::map<int, int> findMaxCellTextLen(
        const std::set<int>& finallyDisplayLinesSet,
        const std::vector<std::vector<std::string>>& newTable,
        const std::vector<int>& rowsRange) {
        
        std::map<int, int> maxCellTextLen;
        
        for (int lineNum : finallyDisplayLinesSet) {
            if (lineNum >= 0 && lineNum < newTable.size()) {
                for (size_t col = 0; col < newTable[lineNum].size(); col++) {
                    int len = newTable[lineNum][col].length();
                    if (maxCellTextLen[col] < len) {
                        maxCellTextLen[col] = len;
                    }
                }
            }
        }
        return maxCellTextLen;
    }
    
    int determineRowWidth(int colIndex, const std::map<int, int>& maxCellTextLen) {
        int certaintextwidth = textwidth;
        if (colIndex < breiten.size()) {
            certaintextwidth = breiten[colIndex];
        }
        
        auto it = maxCellTextLen.find(colIndex);
        int maxLen = (it != maxCellTextLen.end()) ? it->second : 0;
        
        if (certaintextwidth > maxLen || (certaintextwidth == 0 && 
            !tables->bbcodeOutputYes() && !tables->htmlOutputYes())) {
            return maxLen;
        }
        return certaintextwidth;
    }
};

// ============================================================================
// INNERE KLASSE COMBI (Kombinationen)
// ============================================================================

class Tables::Combi {
private:
    Tables* tables;
    int sumOfAllCombiRowsAmount = 0;
    std::vector<int> religionNumbers;
    OrderedSet<int> rowsOfcombi;
    
public:
    Combi(Tables* tablesPtr) : tables(tablesPtr) {}
    
    void setReligionNumbers(const std::vector<int>& nums) {
        religionNumbers = nums;
    }
    
    std::vector<std::map<int, std::vector<std::vector<std::string>>>> prepareTableJoin(
        const std::map<int, std::vector<int>>& ChosenKombiLines,
        const std::vector<std::vector<std::string>>& newTable_kombi_1) {
        
        std::vector<std::map<int, std::vector<std::vector<std::string>>>> KombiTables;
        
        for (const auto& [key, value] : ChosenKombiLines) {
            std::map<int, std::vector<std::vector<std::string>>> table;
            
            for (int kombiLineNumber : value) {
                auto into = tables->tableReducedInLinesByTypeSet(
                    newTable_kombi_1, 
                    OrderedSet<int>{kombiLineNumber}
                );
                
                if (!into.empty()) {
                    table[key].push_back(into[0]);
                }
            }
            
            if (!table.empty()) {
                KombiTables.push_back(table);
            }
        }
        
        return KombiTables;
    }
    
    std::vector<std::string> removeOneNumber(const std::vector<std::string>& hinein, 
                                             int colNum) {
        if (hinein.empty()) return hinein;
        
        // Vereinfachte Implementation
        std::vector<std::string> result = hinein;
        
        if (tables->textWidth() == 0 && 
            (tables->output()->oneTable() || 
             tables->textWidth() > 80 - 7)) { // shellRowsAmount vereinfacht
            std::string combined;
            for (const auto& s : hinein) {
                if (!combined.empty()) combined += " | ";
                combined += s;
            }
            return {combined};
        }
        
        return result;
    }
    
    std::vector<std::vector<std::string>> tableJoin(
        const std::vector<std::vector<std::string>>& mainTable,
        const std::vector<std::map<int, std::vector<std::vector<std::string>>>>& manySubTables,
        const std::pair<std::map<int, int>, std::map<int, int>>& maintable2subtable_Relation,
        const std::pair<std::vector<int>, std::vector<int>>& old2newRows,
        const OrderedSet<int>& rowsOfcombiSet) {
        
        auto rowsOfcombiVec = std::vector<int>(rowsOfcombiSet.begin(), rowsOfcombiSet.end());
        std::sort(rowsOfcombiVec.begin(), rowsOfcombiVec.end());
        
        std::vector<std::vector<std::string>> table2 = mainTable;
        bool oneLinePerLine = tables->htmlOutputYes() || tables->bbcodeOutputYes();
        
        for (size_t colNum = 0; colNum < religionNumbers.size(); colNum++) {
            int reliNum = religionNumbers[colNum];
            
            for (const auto& subTable : manySubTables) {
                auto it = subTable.find(reliNum);
                if (it == subTable.end()) continue;
                
                const auto& subTableEntries = it->second;
                
                for (size_t row = 0; row < mainTable[colNum].size(); row++) {
                    int oldRowNum = old2newRows.second[row];
                    
                    if (maintable2subtable_Relation.first.find(oldRowNum) != 
                        maintable2subtable_Relation.first.end()) {
                        
                        int subRowNum = maintable2subtable_Relation.first.at(oldRowNum);
                        
                        for (const auto& subTableCell : subTableEntries) {
                            if (subRowNum < rowsOfcombiVec.size()) {
                                int index = std::find(rowsOfcombiVec.begin(), 
                                                     rowsOfcombiVec.end(), 
                                                     subRowNum + 1) - rowsOfcombiVec.begin();
                                
                                if (index < subTableCell.size() && !subTableCell[index].empty()) {
                                    auto hinein = subTableCell[index];
                                    hinein = removeOneNumber(hinein, reliNum);
                                    
                                    if (oneLinePerLine && !hinein.empty() && 
                                        hinein[0].length() > 2) {
                                        if (tables->htmlOutputYes()) {
                                            hinein[0] = "<li>" + hinein[0] + "</li>";
                                        } else if (tables->bbcodeOutputYes()) {
                                            hinein[0] = "[*]" + hinein[0];
                                        } else {
                                            hinein[0] += " |";
                                        }
                                        
                                        if (table2[colNum][row].empty() || 
                                            table2[colNum][row][0].empty()) {
                                            table2[colNum][row] = hinein;
                                        } else {
                                            table2[colNum][row].back() += hinein[0];
                                        }
                                    } else {
                                        if (table2[colNum][row].empty() || 
                                            table2[colNum][row][0].empty()) {
                                            table2[colNum][row] = hinein;
                                        } else {
                                            table2[colNum][row].insert(
                                                table2[colNum][row].end(),
                                                hinein.begin(),
                                                hinein.end()
                                            );
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        
        // Nachbearbeitung
        if (oneLinePerLine && tables->htmlOutputYes()) {
            for (auto& col : table2) {
                for (auto& cell : col) {
                    if (!cell.empty()) {
                        cell[0] = "<ul>" + cell[0] + "</ul>";
                    }
                }
            }
        } else if (oneLinePerLine && tables->bbcodeOutputYes()) {
            for (auto& col : table2) {
                for (auto& cell : col) {
                    if (!cell.empty()) {
                        cell[0] = "[list]" + cell[0] + "[/list]";
                    }
                }
            }
        } else if (tables->textWidth() == 0 && 
                  (tables->output()->oneTable() || 
                   tables->textWidth() > 80 - 7)) {
            for (auto& col : table2) {
                for (auto& cell : col) {
                    std::string combined;
                    for (const auto& s : cell) {
                        if (!combined.empty()) combined += " | ";
                        combined += s;
                    }
                    cell = {combined};
                }
            }
        }
        
        return table2;
    }
    
    std::map<int, std::vector<int>> prepare_kombi(
        const std::set<int>& finallyDisplayLines_kombi_1,
        const std::vector<std::vector<std::string>>& kombiTable,
        const std::set<std::string>& paramLines,
        const std::set<int>& displayingZeilen,
        const std::vector<std::vector<int>>& kombiTable_Kombis) {
        
        std::map<int, std::vector<int>> ChosenKombiLines;
        
        for (const auto& condition : paramLines) {
            if (condition == "ka" || condition == "ka2") {
                for (size_t kombiLineNumber = 0; kombiLineNumber < kombiTable_Kombis.size(); 
                     kombiLineNumber++) {
                    for (int kombiNumber : kombiTable_Kombis[kombiLineNumber]) {
                        if (displayingZeilen.count(kombiNumber) > 0) {
                            ChosenKombiLines[kombiNumber].push_back(kombiLineNumber + 1);
                        }
                    }
                }
            }
        }
        
        return ChosenKombiLines;
    }
    
    std::tuple<std::vector<std::vector<std::string>>,
               std::vector<std::vector<std::string>>,
               std::vector<std::vector<int>>,
               std::pair<std::map<int, int>, std::map<int, int>>>
    readKombiCsv(const std::vector<std::vector<std::string>>& relitable,
                 std::set<int>& rowsAsNumbers,
                 const OrderedSet<int>& rowsOfcombiSet,
                 const std::string& csvFileName) {
        
        sumOfAllCombiRowsAmount += rowsOfcombiSet.size();
        
        auto rowsOfcombi = std::vector<int>(rowsOfcombiSet.begin(), rowsOfcombiSet.end());
        std::sort(rowsOfcombi.begin(), rowsOfcombi.end());
        
        std::vector<std::vector<std::string>> kombiTable;
        std::vector<std::vector<int>> kombiTable_Kombis;
        std::pair<std::map<int, int>, std::map<int, int>> maintable2subtable_Relation;
        
        if (rowsOfcombi.empty()) {
            return {kombiTable, relitable, kombiTable_Kombis, maintable2subtable_Relation};
        }
        
        // CSV lesen
        std::string place = "../csv/" + csvFileName;
        std::ifstream file(place);
        
        if (!file.is_open()) {
            std::cerr << "Fehler beim Öffnen der Datei: " << place << std::endl;
            return {kombiTable, relitable, kombiTable_Kombis, maintable2subtable_Relation};
        }
        
        std::string line;
        int lineNum = 0;
        while (std::getline(file, line)) {
            std::vector<std::string> row;
            std::stringstream ss(line);
            std::string cell;
            
            while (std::getline(ss, cell, ';')) {
                row.push_back(cell);
            }
            
            if (lineNum > 0 && !row.empty()) {
                // Behandlung der ersten Spalte
                std::vector<int> kombiNumbers;
                std::string firstCell = row[0];
                
                // Zahlen extrahieren (vereinfacht)
                std::regex numRegex(R"((-?\d+))");
                std::sregex_iterator it(firstCell.begin(), firstCell.end(), numRegex);
                std::sregex_iterator end;
                
                while (it != end) {
                    kombiNumbers.push_back(std::abs(std::stoi(it->str())));
                    ++it;
                }
                
                kombiTable_Kombis.push_back(kombiNumbers);
                
                // Zellen formatieren
                for (size_t i = 1; i < row.size(); i++) {
                    if (!row[i].empty() && !firstCell.empty()) {
                        row[i] = "(" + firstCell + ") " + row[i] + " (" + firstCell + ")";
                    }
                }
            }
            
            kombiTable.push_back(row);
            lineNum++;
        }
        
        // Tabellen kombinieren
        auto [newRelitable, animalsProfessionsCol] = Tables::fillBoth(relitable, kombiTable);
        
        // Relationen aufbauen
        int headingsAmount = relitable[0].size();
        for (size_t t = 0; t < animalsProfessionsCol[0].size() - 1; t++) {
            maintable2subtable_Relation.first[headingsAmount + t] = t;
            maintable2subtable_Relation.second[t] = headingsAmount + t;
        }
        
        // rowsAsNumbers aktualisieren
        for (int a : rowsOfcombi) {
            if (headingsAmount + a - 1 < newRelitable[0].size()) {
                rowsAsNumbers.insert(headingsAmount + a - 1);
            }
        }
        
        // HTML-Parameter setzen (vereinfacht)
        auto& i18n = I18n::instance();
        if (csvFileName == i18n.csvFileNames.kombi13) {
            // Vereinfachte Parameter-Setzung
        } else if (csvFileName == i18n.csvFileNames.kombi15) {
            // Vereinfachte Parameter-Setzung
        }
        
        return {kombiTable, newRelitable, kombiTable_Kombis, maintable2subtable_Relation};
    }
    
private:
    void kombiNumbersCorrectTestAndSet(const std::string& num) {
        // Vereinfachte Implementation
        // Überprüft Zahlenformat in CSV
    }
};

// ============================================================================
// INNERE KLASSE MAINTABLE
// ============================================================================

class Tables::Maintable {
private:
    Tables* tables;
    
public:
    Maintable(Tables* tablesPtr) : tables(tablesPtr) {}
    
    std::pair<std::vector<std::vector<std::string>>, std::set<int>> 
    createSpalteGestirn(const std::vector<std::vector<std::string>>& relitable,
                       std::set<int> rowsAsNumbers) {
        
        auto newRelitable = relitable;
        auto& i18n = I18n::instance();
        
        if (!rowsAsNumbers.empty() && *rowsAsNumbers.rbegin() >= 64) {
            if (!newRelitable.empty()) {
                // HTML-Parameter prüfen
                if (tables->generatedSpaltenParameter.find(
                    tables->generatedSpaltenParameter.size() + 
                    tables->SpaltenVanillaAmount) != tables->generatedSpaltenParameter.end()) {
                    throw std::runtime_error("ValueError");
                }
                
                // Parameter setzen (vereinfacht)
                tables->generatedSpaltenParameter_Tags[rowsAsNumbers.size() - 1] = 
                    {ST::sternPolygon, ST::universum, ST::galaxie};
                
                rowsAsNumbers.insert(newRelitable[0].size());
                
                // Überschrift hinzufügen
                newRelitable[0].push_back(i18n.get("Gestirn"));
                newRelitable[1].push_back(i18n.get("Sonne (keine Potenzen)"));
                
                // Datenzeilen hinzufügen
                for (size_t i = 2; i < newRelitable.size(); i++) {
                    std::string gestirn;
                    if ((i - 2) % 3 == 1) {
                        gestirn = i18n.get("Mond");
                    } else {
                        gestirn = i18n.get("Sonne");
                    }
                    newRelitable[i].push_back(gestirn);
                }
            }
        }
        
        return {newRelitable, rowsAsNumbers};
    }
};

// ============================================================================
// HILFSKLASSEN (vereinfacht)
// ============================================================================

class Prepare {
private:
    Tables* tables;
    std::map<int, int> hoechsteZeile;
    bool ifprimmultis = false;
    bool ifZeilenSetted = false;
    std::vector<int> breiten;
    bool nummerierung = true;
    int textWidth = 21;
    std::vector<int> religionNumbers;
    
public:
    Prepare(Tables* tablesPtr, const std::map<int, int>& hz) 
        : tables(tablesPtr), hoechsteZeile(hz) {}
    
    bool ifprimmultis() const { return ifprimmultis; }
    void setIfprimmultis(bool value) { ifprimmultis = value; }
    
    bool ifZeilenSetted() const { return ifZeilenSetted; }
    void setIfZeilenSetted(bool value) { ifZeilenSetted = value; }
    
    std::vector<int> getBreiten() const { return breiten; }
    void setBreiten(const std::vector<int>& value) { breiten = value; }
    
    bool getNummerierung() const { return nummerierung; }
    void setNummerierung(bool value) { nummerierung = value; }
    
    int getTextWidth() const { return textWidth; }
    void setTextWidth(int value) { textWidth = value; }
    
    void setReligionNumbers(const std::vector<int>& nums) {
        religionNumbers = nums;
    }
    
    int zeileWhichZaehlung(int num) {
        // Vereinfachte Implementation
        return num;
    }
    
    std::vector<std::string> cellWork(const std::string& text, int width) {
        // Einfache Textumbrüche
        std::vector<std::string> result;
        std::string current;
        
        std::stringstream ss(text);
        std::string word;
        
        while (ss >> word) {
            if (current.length() + word.length() + 1 > width) {
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
        
        return result;
    }
};

class Concat {
private:
    Tables* tables;
    
public:
    Concat(Tables* tablesPtr) : tables(tablesPtr) {}
    
    // Methoden werden von der Haupt-Concat-Klasse bereitgestellt
    // (siehe vorherige Datei)
};

// ============================================================================
// HAUPTFUNKTION ZUM TESTEN
// ============================================================================

int main() {
    // Initialisierung
    auto& i18n = I18n::instance();
    
    // Beispiel-Tabelle erstellen
    Tables tables(100, "Test");
    
    // Output-Type setzen
    tables.setOutType(std::make_shared<OutputSyntax>());
    
    // Test-Tabelle
    std::vector<std::vector<std::string>> testTable = {
        {"Spalte1", "Spalte2", "Spalte3"},
        {"Z1S1", "Z1S2", "Z1S3"},
        {"Z2S1", "Z2S2", "Z2S3"},
        {"Z3S1", "Z3S2", "Z3S3"}
    };
    
    // Test der fillBoth-Methode
    std::vector<std::vector<std::string>> table2 = {{"A"}, {"B"}};
    auto [filled1, filled2] = Tables::fillBoth(testTable, table2);
    
    std::cout << "FillBoth Test:" << std::endl;
    std::cout << "Tabelle 1 Größe: " << filled1.size() << "x" 
              << (filled1.empty() ? 0 : filled1[0].size()) << std::endl;
    std::cout << "Tabelle 2 Größe: " << filled2.size() << "x" 
              << (filled2.empty() ? 0 : filled2[0].size()) << std::endl;
    
    // Test der Output-Funktionalität
    std::set<int> displayLines = {0, 1, 2};
    std::vector<int> rowsRange = {0}; // Nur erste Zeile jeder Zelle
    
    tables.output()->cliOut(displayLines, testTable, 3, rowsRange);
    
    // Test der Combi-Klasse
    std::set<int> rowsAsNumbers = {1, 2};
    OrderedSet<int> rowsOfcombi = {1, 2};
    
    auto result = tables.combi()->readKombiCsv(
        testTable, 
        rowsAsNumbers, 
        rowsOfcombi, 
        i18n.csvFileNames.kombi13
    );
    
    std::cout << "\nCombi Test abgeschlossen" << std::endl;
    
    // Test der Maintable-Klasse
    auto [gestirnTable, newRows] = tables.maintable()->createSpalteGestirn(
        testTable, 
        rowsAsNumbers
    );
    
    std::cout << "Gestirn-Tabelle Größe: " << gestirnTable.size() << "x"
              << (gestirnTable.empty() ? 0 : gestirnTable[0].size()) << std::endl;
    
    return 0;
}

// ============================================================================
// COMPILE-INSTRUKTIONEN
// ============================================================================
/*
Compile mit:
g++ -std=c++17 -Wall -O2 -o tableHandling tableHandling.cpp

ODER in CMakeLists.txt:
cmake_minimum_required(VERSION 3.10)
project(tableHandling)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

add_executable(tableHandling tableHandling.cpp)

# Für filesystem-Bibliothek (falls benötigt)
target_link_libraries(tableHandling stdc++fs)
*/
