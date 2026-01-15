#include "i18n.hpp"
#include <iostream>

namespace i18n {

// Static instances
ParametersMain& get_parameters_main() {
    static ParametersMain instance;
    return instance;
}

ZeilenParas& get_zeilen_paras() {
    static ZeilenParas instance;
    return instance;
}

AusgabeParas& get_ausgabe_paras() {
    static AusgabeParas instance;
    return instance;
}

KombiMainParas& get_kombi_main_paras() {
    static KombiMainParas instance;
    return instance;
}

I18n& get_i18n() {
    static I18n instance;
    return instance;
}

// RetaStrings implementation
struct RetaStrings {
    std::string keine_num_wort = "keinenummerierung";
    std::vector<std::string> cliout_saetze = {
        "Fehler: ",
        " ist kein gültiger Parameter für ",
        "Mögliche Parameter: ",
        "Verfügbare Werte: ",
        "Unbekannter Parameter: ",
        "Gültige Parameter sind: ",
        "Verwendung: ",
        "Beispiele: ",
        "Für Hilfe verwenden Sie: ",
        "Unbekannter Befehl: "
    };
    
    std::vector<std::string> ausgabe_art_strings = {
        "shell", "nichts", "csv", "bbcode", "html", "emacs", "markdown"
    };
    
    std::map<std::string, std::string> tabelle_handling = {
        {"Gestirn", "Gestirn"},
        {"Sonne (keine Potenzen)", "Sonne (keine Potenzen)"},
        {"Mond (Potenzen)", "Mond (Potenzen)"},
        {"Planet (2*n)", "Planet (2*n)"},
        {"wäre eine schwarze Sonne", "wäre eine schwarze Sonne"},
        {"und außerdem", "und außerdem"}
    };
};

static RetaStrings& get_reta_strings() {
    static RetaStrings instance;
    return instance;
}

// Main parameter commands
std::map<std::string, std::string> get_main_para_cmds() {
    return {
        {"zeilen", "Zeilenparameter"},
        {"spalten", "Spaltenparameter"},
        {"kombination", "Kombinationsparameter"},
        {"ausgabe", "Ausgabeparameter"},
        {"debug", "Debug-Modus"},
        {"h", "Hilfe"},
        {"help", "Hilfe"}
    };
}

// Parameter data matrix
std::vector<ParaNDataMatrixEntry> get_para_n_data_matrix() {
    return {
        // Multiplikationen
        {
            {"multiplikationen"},
            {},
            {
                {1, 2, 3, 4, 5},        // 0: ordinary
                {},                     // 1: generated1
                {},                     // 2: concat1
                {},                     // 3: kombi1
                {},                     // 4: boolAndTupleSet1
                {},                     // 5: gebroUni1
                {},                     // 6: gebrGal1
                {},                     // 7: generated2
                {},                     // 8: kombi2
                {},                     // 9: gebrEmo1
                {},                     // 10: gebrGroe1
                {}                      // 11: metakonkret
            }
        },
        
        // Gebrochenuniversum
        {
            {"gebrochenuniversum", "gebrochenuniversum2"},
            {"2", "3", "5", "7", "11", "13", "17", "19", "23", "29"},
            {
                {},                     // ordinary
                {},                     // generated1
                {},                     // concat1
                {},                     // kombi1
                {},                     // boolAndTupleSet1
                {2, 3, 5, 7, 11, 13, 17, 19, 23, 29}, // gebroUni1
                {},                     // gebrGal1
                {},                     // generated2
                {},                     // kombi2
                {},                     // gebrEmo1
                {},                     // gebrGroe1
                {}                      // metakonkret
            }
        },
        
        // Gebrochengalaxie
        {
            {"gebrochengalaxie", "gebrochengalaxie2"},
            {"2", "3", "5", "7", "11", "13", "17", "19", "23", "29"},
            {
                {},                     // ordinary
                {},                     // generated1
                {},                     // concat1
                {},                     // kombi1
                {},                     // boolAndTupleSet1
                {},                     // gebroUni1
                {2, 3, 5, 7, 11, 13, 17, 19, 23, 29}, // gebrGal1
                {},                     // generated2
                {},                     // kombi2
                {},                     // gebrEmo1
                {},                     // gebrGroe1
                {}                      // metakonkret
            }
        },
        
        // Gebrochenemotion
        {
            {"gebrochenemotion", "gebrochenemotion2"},
            {"2", "3", "5", "7", "11", "13", "17", "19", "23", "29"},
            {
                {},                     // ordinary
                {},                     // generated1
                {},                     // concat1
                {},                     // kombi1
                {},                     // boolAndTupleSet1
                {},                     // gebroUni1
                {},                     // gebrGal1
                {},                     // generated2
                {},                     // kombi2
                {2, 3, 5, 7, 11, 13, 17, 19, 23, 29}, // gebrEmo1
                {},                     // gebrGroe1
                {}                      // metakonkret
            }
        },
        
        // Gebrochengroesse
        {
            {"gebrochengroesse", "gebrochengroesse2"},
            {"2", "3", "5", "7", "11", "13", "17", "19", "23", "29"},
            {
                {},                     // ordinary
                {},                     // generated1
                {},                     // concat1
                {},                     // kombi1
                {},                     // boolAndTupleSet1
                {},                     // gebroUni1
                {},                     // gebrGal1
                {},                     // generated2
                {},                     // kombi2
                {},                     // gebrEmo1
                {2, 3, 5, 7, 11, 13, 17, 19, 23, 29}, // gebrGroe1
                {}                      // metakonkret
            }
        },
        
        // Primvielfache
        {
            {"primvielfache"},
            {"2", "3", "5", "7", "11", "13", "17", "19", "23", "29", "31"},
            {
                {},                     // ordinary
                {},                     // generated1
                {},                     // concat1
                {},                     // kombi1
                {},                     // boolAndTupleSet1
                {},                     // gebroUni1
                {},                     // gebrGal1
                {},                     // generated2
                {},                     // kombi2
                {},                     // gebrEmo1
                {},                     // gebrGroe1
                {}                      // metakonkret
            }
        },
        
        // Alles
        {
            {"alles"},
            {},
            {
                {1, 2, 3, 4, 5, 6, 7, 8, 9, 10}, // ordinary
                {1, 2, 3, 4, 5},                 // generated1
                {1, 2, 3, 4, 5},                 // concat1
                {1, 2, 3},                       // kombi1
                {true, false},                   // boolAndTupleSet1
                {2, 3, 5, 7, 11},               // gebroUni1
                {2, 3, 5, 7, 11},               // gebrGal1
                {1, 2, 3, 4, 5},                 // generated2
                {1, 2, 3},                       // kombi2
                {2, 3, 5, 7, 11},               // gebrEmo1
                {2, 3, 5, 7, 11},               // gebrGroe1
                {1, 2, 3, 4, 5}                  // metakonkret
            }
        }
    };
}

// Kombi parameter data matrices
std::map<int32_t, std::vector<std::string>> get_kombi_para_n_data_matrix() {
    return {
        {1, {"Tiere", "Löwe", "Elefant", "Adler", "Delfin"}},
        {2, {"Berufe", "Arzt", "Ingenieur", "Künstler", "Lehrer"}},
        {3, {"Intelligenz", "hoch", "mittel", "niedrig", "sehr hoch"}},
        {4, {"Farben", "rot", "blau", "grün", "gelb"}},
        {5, {"Elemente", "Feuer", "Wasser", "Luft", "Erde"}}
    };
}

std::map<int32_t, std::vector<std::string>> get_kombi_para_n_data_matrix2() {
    return {
        {1, {"Universum", "Galaxie1", "Galaxie2", "Galaxie3"}},
        {2, {"Dimension", "3D", "4D", "5D", "String"}},
        {3, {"Energie", "dunkel", "hell", "neutral", "quantum"}},
        {4, {"Materie", "fest", "flüssig", "gasförmig", "plasma"}},
        {5, {"Zeit", "linear", "zyklisch", "branching", "multidimensional"}}
    };
}

// CSV file handling
std::vector<std::vector<std::string>> read_csv_file(const std::string& filename, char delimiter) {
    std::vector<std::vector<std::string>> data;
    std::ifstream file(filename);
    
    if (!file.is_open()) {
        throw RETAException::file_not_found("Cannot open CSV file: " + filename);
    }
    
    std::string line;
    while (std::getline(file, line)) {
        // Skip empty lines
        if (line.empty()) continue;
        
        // Parse CSV line
        std::vector<std::string> row;
        std::string field;
        bool in_quotes = false;
        
        for (size_t i = 0; i
