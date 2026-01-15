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
        
        for (size_t i = 0; i < line.length(); ++i) {
            char c = line[i];
            
            if (c == '"') {
                // Handle escaped quotes
                if (i + 1 < line.length() && line[i + 1] == '"') {
                    field += '"';
                    ++i;
                } else {
                    in_quotes = !in_quotes;
                }
            } else if (c == delimiter && !in_quotes) {
                row.push_back(utils::trim(field));
                field.clear();
            } else {
                field += c;
            }
        }
        
        // Add last field
        row.push_back(utils::trim(field));
        data.push_back(row);
    }
    
    return data;
}

// Language handling
std::map<std::string, std::map<std::string, std::string>> get_translations() {
    static std::map<std::string, std::map<std::string, std::string>> translations = {
        {"de", {
            {"help", "Hilfe"},
            {"error", "Fehler"},
            {"warning", "Warnung"},
            {"success", "Erfolg"},
            {"table", "Tabelle"},
            {"row", "Zeile"},
            {"column", "Spalte"},
            {"cell", "Zelle"},
            {"header", "Kopfzeile"},
            {"footer", "Fußzeile"},
            {"print", "Drucken"},
            {"export", "Exportieren"},
            {"import", "Importieren"},
            {"save", "Speichern"},
            {"load", "Laden"},
            {"quit", "Beenden"}
        }},
        {"en", {
            {"help", "Help"},
            {"error", "Error"},
            {"warning", "Warning"},
            {"success", "Success"},
            {"table", "Table"},
            {"row", "Row"},
            {"column", "Column"},
            {"cell", "Cell"},
            {"header", "Header"},
            {"footer", "Footer"},
            {"print", "Print"},
            {"export", "Export"},
            {"import", "Import"},
            {"save", "Save"},
            {"load", "Load"},
            {"quit", "Quit"}
        }},
        {"fr", {
            {"help", "Aide"},
            {"error", "Erreur"},
            {"warning", "Avertissement"},
            {"success", "Succès"},
            {"table", "Tableau"},
            {"row", "Ligne"},
            {"column", "Colonne"},
            {"cell", "Cellule"},
            {"header", "En-tête"},
            {"footer", "Pied de page"},
            {"print", "Imprimer"},
            {"export", "Exporter"},
            {"import", "Importer"},
            {"save", "Sauvegarder"},
            {"load", "Charger"},
            {"quit", "Quitter"}
        }},
        {"es", {
            {"help", "Ayuda"},
            {"error", "Error"},
            {"warning", "Advertencia"},
            {"success", "Éxito"},
            {"table", "Tabla"},
            {"row", "Fila"},
            {"column", "Columna"},
            {"cell", "Celda"},
            {"header", "Encabezado"},
            {"footer", "Pie de página"},
            {"print", "Imprimir"},
            {"export", "Exportar"},
            {"import", "Importar"},
            {"save", "Guardar"},
            {"load", "Cargar"},
            {"quit", "Salir"}
        }}
    };
    
    return translations;
}

std::string translate(const std::string& key, const std::string& language) {
    auto translations = get_translations();
    
    if (translations.find(language) != translations.end()) {
        const auto& lang_dict = translations[language];
        if (lang_dict.find(key) != lang_dict.end()) {
            return lang_dict.at(key);
        }
    }
    
    // Fallback to English
    if (language != "en") {
        const auto& en_dict = translations["en"];
        if (en_dict.find(key) != en_dict.end()) {
            return en_dict.at(key);
        }
    }
    
    // Return key if not found
    return key;
}

// Output format strings
std::map<SyntaxType, std::string> get_output_format_strings() {
    return {
        {SyntaxType::Default, "Standard Shell Ausgabe"},
        {SyntaxType::Nichts, "Keine Ausgabe"},
        {SyntaxType::Markdown, "Markdown Format"},
        {SyntaxType::BBCode, "BBCode Format"},
        {SyntaxType::Html, "HTML Format"},
        {SyntaxType::Csv, "CSV Format"},
        {SyntaxType::Emacs, "Emacs Org-Mode Format"}
    };
}

std::string get_output_format_name(SyntaxType type) {
    auto formats = get_output_format_strings();
    if (formats.find(type) != formats.end()) {
        return formats[type];
    }
    return "Unbekanntes Format";
}

// Table handling strings
std::map<std::string, std::string> get_table_handling_strings() {
    return {
        {"Gestirn", "Gestirn"},
        {"Sonne (keine Potenzen)", "Sonne (keine Potenzen)"},
        {"Mond (Potenzen)", "Mond (Potenzen)"},
        {"Planet (2*n)", "Planet (2*n)"},
        {"wäre eine schwarze Sonne (-3*n), wenn ins Negative durch eine Typ 13 verdreht", 
         "wäre eine schwarze Sonne (-3*n), wenn ins Negative durch eine Typ 13 verdreht"},
        {"und außerdem", ", und außerdem "},
        {"Kombination_(Galaxie_und_schwarzes_Loch)_(14_mit_13)", 
         "Kombination (Galaxie und schwarzes Loch) (14 mit 13)"},
        {"Wichtigstes_zum_gedanklich_einordnen", "Wichtigstes zum gedanklich einordnen"},
        {"Zweitwichtigste", "Zweitwichtigste"},
        {"tiere", "Tiere"},
        {"berufe", "Berufe"},
        {"intelligenz", "Intelligenz"},
        {"Kombination_(Universum_und_Galaxie)_(14_mit_15)", 
         "Kombination (Universum und Galaxie) (14 mit 15)"}
    };
}

// Main help text
const char* RETA_HILFE = R"(
RETA - Tabellenverarbeitungsprogramm
=====================================

RETA ist ein leistungsstarkes Programm zur Verarbeitung und Darstellung von
Tabellendaten mit Unterstützung für verschiedene Ausgabeformate und erweiterte
Funktionen wie Tabellenkombinationen und benutzerdefinierte Spalten.

HAUPTBEFEHLE:
  -zeilen          Zeilenparameter festlegen
  -spalten         Spaltenparameter festlegen  
  -ausgabe         Ausgabeparameter festlegen
  -kombination     Tabellenkombinationen
  -h, --help       Diese Hilfe anzeigen

ZEILENPARAMETER:
  --alles                    Alle Zeilen anzeigen
  --zeit=heute,gestern,morgen  Zeitbasierte Filter
  --zaehlung=1-10,20-30      Numerische Bereiche
  --typ=sonne,mond,planet    Zeilentypen filtern
  --primzahlen=aussenerste,innenerste  Primzahlenfilter
  --potenzenvonzahlen=2,3,5  Potenzen von Zahlen
  --vielfachevonzahlen=7,11,13 Vielfache von Zahlen
  --primzahlvielfache=3,5,7  Primzahlvielfache
  --invertieren              Auswahl invertieren

SPALTENPARAMETER:
  --multiplikationen=2,3,5,7     Multiplikationstabellen
  --gebrochenuniversum=11,13,17  Gebrochenes Universum
  --gebrochengalaxie=19,23,29    Gebrochene Galaxie  
  --gebrochenemotion=31,37,41    Gebrochene Emotion
  --gebrochengroesse=43,47,53    Gebrochene Größe
  --primvielfache=59,61,67       Primzahlvielfache
  --alles                        Alle verfügbaren Spalten

AUSGABEPARAMETER:
  --art=shell|html|bbcode|markdown|csv|emacs  Ausgabeformat
  --breite=80                                 Ausgabebreite
  --breiten=20,30,40,50                       Individuelle Spaltenbreiten
  --keineueberschriften                       Überschriften ausblenden
  --keinenummerierung                         Zeilennummerierung ausblenden
  --keineleereninhalte                        Leere Inhalte ausblenden
  --spaltenreihenfolgeundnurdiese=1,3,5,7     Spaltenreihenfolge festlegen
  --nocolor|justtext                          Farbausgabe deaktivieren
  --endlessscreen|onetable                    Kontinuierliche Ausgabe

KOMBINATIONSPARAMETER:
  --galaxie=Tiere,Berufe,Intelligenz     Galaxiekombinationen
  --universum=Universum,Dimension,Energie Universumskombinationen

BEISPIELE:
  reta -zeilen --alles --typ=sonne,mond
  reta -spalten --multiplikationen=2,3,5 --gebrochenuniversum=11,13
  reta -ausgabe --art=html --breite=80 --nocolor
  reta -kombination --galaxie=Tiere,Berufe --universum=Dimension,Zeit

KONFIGURATION:
  Die Konfiguration erfolgt über Kommandozeilenparameter. Für komplexe
  Anwendungsfälle können Skripte mit mehreren Aufrufen kombiniert werden.

  Standardmäßig wird die religion.csv Datei im csv/ Verzeichnis verwendet.
  Zusätzliche CSV-Dateien können für Kombinationen und Konkatenationen
  bereitgestellt werden.

  Die Ausgabe kann in verschiedene Formate konvertiert werden, einschließlich
  HTML für Webanwendungen, BBCode für Foren, Markdown für Dokumentation
  und CSV für weitere Verarbeitung.

  Das Programm unterstützt Farbausgabe im Terminal, die mit --nocolor
  deaktiviert werden kann. Bei breiten Tabellen wird automatisch ein
  Zeilenumbruch durchgeführt, der mit --endlessscreen deaktiviert werden kann.

FEHLERBEHANDLUNG:
  Bei Syntaxfehlern oder ungültigen Parametern werden detaillierte
  Fehlermeldungen angezeigt. Dateifehler führen zu entsprechenden
  Hinweisen mit Dateipfadinformationen.

  Für Debug-Informationen können zusätzliche Parameter aktiviert werden,
  um den Verarbeitungsprozess zu verfolgen.

VERSION:
  RETA 1.0.0 - C++ Implementierung
  Basierend auf der originalen Python/Rust Implementierung

LIZENZ:
  Proprietäre Software - Alle Rechte vorbehalten
)";

} // namespace i18n
