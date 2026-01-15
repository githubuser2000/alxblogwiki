#ifndef RETA_I18N_HPP
#define RETA_I18N_HPP

#include "types.hpp"
#include <map>
#include <string>
#include <vector>
#include <memory>

namespace i18n {

// CSV file names
struct CsvFileNames {
    static constexpr const char* religion = "religion.csv";
    static constexpr const char* kombi13 = "kombi13.csv";
    static constexpr const char* kombi15 = "kombi15.csv";
    
    static const char* filename(const char* name) {
        return name;
    }
};

// Parameters main
struct ParametersMain {
    Vector<std::string> multiplikationen = {"multiplikationen"};
    Vector<std::string> gebrochenuniversum = {"gebrochenuniversum", "gebrochenuniversum2"};
    Vector<std::string> gebrochengalaxie = {"gebrochengalaxie", "gebrochengalaxie2"};
    Vector<std::string> gebrochenemotion = {"gebrochenemotion", "gebrochenemotion2"};
    Vector<std::string> gebrochengroesse = {"gebrochengroesse", "gebrochengroesse2"};
    Vector<std::string> primvielfache = {"primvielfache"};
    std::string alles = "alles";
};

// Zeilen parameters
struct ZeilenParas {
    std::string alles = "alles";
    std::string heute = "heute";
    std::string gestern = "gestern";
    std::string morgen = "morgen";
    std::string oberesmaximum = "oberesmaximum";
    std::string vorhervonausschnitt = "vorhervonausschnitt";
    std::string sonne = "sonne";
    std::string schwarzesonne = "schwarzesonne";
    std::string planet = "planet";
    std::string mond = "mond";
    std::string aussenerste = "aussenerste";
    std::string innenerste = "innenerste";
    std::string aussenalle = "aussenalle";
    std::string innenalle = "innenalle";
    std::string invertieren = "invertieren";
    std::string zaehlung = "zaehlung";
    std::string hoehemaximal = "hoehemaximal";
    std::string typ = "typ";
    std::string primzahlen = "primzahlen";
    std::string potenzenvonzahlen = "potenzenvonzahlen";
    std::string vielfachevonzahlen = "vielfachevonzahlen";
    std::string primzahlvielfache = "primzahlvielfache";
    std::string nachtraeglichneuabzaehlung = "nachtraeglichneuabzaehlung";
    std::string nachtraeglichneuabzaehlungvielfache = "nachtraeglichneuabzaehlungvielfache";
};

// Ausgabe parameters
struct AusgabeParas {
    std::string breite = "breite";
    std::string breiten = "breiten";
    std::string keineueberschriften = "keineueberschriften";
    std::string keinenummerierung = "keinenummerierung";
    std::string keineleereninhalte = "keineleereninhalte";
    std::string spaltenreihenfolgeundnurdiese = "spaltenreihenfolgeundnurdiese";
    std::string art = "art";
    std::string nocolor = "nocolor";
    std::string justtext = "justtext";
    std::string endlessscreen = "endlessscreen";
    std::string endless = "endless";
    std::string dontwrap = "dontwrap";
    std::string onetable = "onetable";
};

// Kombi parameters
struct KombiMainParas {
    std::string galaxie = "galaxie";
    std::string universum = "universum";
};

// Global constants
constexpr int GEBROCHEN_SPALTEN_MAXIMUM_PLUS_1 = 100;
constexpr const char* RETA_HILFE = "Hilfetext für RETA";

// Global instances (simulating lazy_static)
inline ParametersMain& get_parameters_main() {
    static ParametersMain instance;
    return instance;
}

inline ZeilenParas& get_zeilen_paras() {
    static ZeilenParas instance;
    return instance;
}

inline AusgabeParas& get_ausgabe_paras() {
    static AusgabeParas instance;
    return instance;
}

inline KombiMainParas& get_kombi_main_paras() {
    static KombiMainParas instance;
    return instance;
}

// Data matrix functions
using ParaNDataMatrixEntry = std::tuple<
    Vector<std::string>,  // parameterMainNames
    Vector<std::string>,  // parameterNames  
    Vector<Vector<int32_t>> // datas
>;

inline Vector<ParaNDataMatrixEntry> get_para_n_data_matrix() {
    return {
        {
            {"multiplikationen"},
            {},
            {
                {1, 2, 3, 4, 5},  // ordinary
                {},               // generated1
                {},               // concat1
                {},               // kombi1
                {},               // boolAndTupleSet1
                {},               // gebroUni1
                {},               // gebrGal1
                {},               // generated2
                {},               // kombi2
                {},               // gebrEmo1
                {},               // gebrGroe1
                {},               // metakonkret
            }
        },
        {
            {"gebrochenuniversum", "gebrochenuniversum2"},
            {"2", "3", "5", "7"},
            {
                {},
                {},
                {},
                {},
                {},
                {2, 3, 5, 7},    // gebroUni1
                {},
                {},
                {},
                {},
                {},
                {},
            }
        },
        // Add more entries as needed
    };
}

// I18n main structure
struct I18n {
    std::string sprachen_wahl = "de";
    Map<std::string, std::string> sprachen;
    
    I18n() {
        sprachen["de"] = "de";
        sprachen["en"] = "en";
        sprachen["fr"] = "fr";
        sprachen["es"] = "es";
    }
};

inline I18n& get_i18n() {
    static I18n instance;
    return instance;
}

} // namespace i18n

#endif // RETA_I18N_HPP
