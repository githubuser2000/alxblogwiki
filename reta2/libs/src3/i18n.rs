use lazy_static::lazy_static;
use std::collections::HashMap;
use serde_json::Value;

lazy_static! {
    pub static ref PARAMETERS_MAIN: ParametersMain = ParametersMain::new();
    pub static ref ZEILEN_PARAS: ZeilenParas = ZeilenParas::new();
    pub static ref AUSGABE_PARAS: AusgabeParas = AusgabeParas::new();
    pub static ref KOMBI_MAIN_PARAS: KombiMainParas = KombiMainParas::new();
    pub static ref KOMBI_PARA_N_DATA_MATRIX: HashMap<i32, Value> = HashMap::new();
    pub static ref KOMBI_PARA_N_DATA_MATRIX2: HashMap<i32, Value> = HashMap::new();
}

pub const GEBROCHEN_SPALTEN_MAXIMUM_PLUS_1: i32 = 100;

pub struct ParametersMain {
    pub multiplikationen: Vec<String>,
    pub gebrochenuniversum: Vec<String>,
    pub gebrochengalaxie: Vec<String>,
    pub gebrochenemotion: Vec<String>,
    pub gebrochengroesse: Vec<String>,
    pub primvielfache: Vec<String>,
    pub alles: String,
}

impl ParametersMain {
    pub fn new() -> Self {
        Self {
            multiplikationen: vec!["multiplikationen".to_string()],
            gebrochenuniversum: vec!["gebrochenuniversum".to_string(), "gebrochenuniversum2".to_string()],
            gebrochengalaxie: vec!["gebrochengalaxie".to_string(), "gebrochengalaxie2".to_string()],
            gebrochenemotion: vec!["gebrochenemotion".to_string(), "gebrochenemotion2".to_string()],
            gebrochengroesse: vec!["gebrochengroesse".to_string(), "gebrochengroesse2".to_string()],
            primvielfache: vec!["primvielfache".to_string()],
            alles: "alles".to_string(),
        }
    }
}

pub struct ZeilenParas {
    pub alles: String,
    pub heute: String,
    pub gestern: String,
    pub morgen: String,
    pub oberesmaximum: String,
    pub vorhervonausschnitt: String,
    pub sonne: String,
    pub schwarzesonne: String,
    pub planet: String,
    pub mond: String,
    pub aussenerste: String,
    pub innenerste: String,
    pub aussenalle: String,
    pub innenalle: String,
    pub invertieren: String,
    pub zaehlung: String,
    pub hoehemaximal: String,
    pub typ: String,
    pub primzahlen: String,
    pub potenzenvonzahlen: String,
    pub vielfachevonzahlen: String,
    pub primzahlvielfache: String,
    pub nachtraeglichneuabzaehlung: String,
    pub nachtraeglichneuabzaehlungvielfache: String,
}

impl ZeilenParas {
    pub fn new() -> Self {
        Self {
            alles: "alles".to_string(),
            heute: "heute".to_string(),
            gestern: "gestern".to_string(),
            morgen: "morgen".to_string(),
            oberesmaximum: "oberesmaximum".to_string(),
            vorhervonausschnitt: "vorhervonausschnitt".to_string(),
            sonne: "sonne".to_string(),
            schwarzesonne: "schwarzesonne".to_string(),
            planet: "planet".to_string(),
            mond: "mond".to_string(),
            aussenerste: "aussenerste".to_string(),
            innenerste: "innenerste".to_string(),
            aussenalle: "aussenalle".to_string(),
            innenalle: "innenalle".to_string(),
            invertieren: "invertieren".to_string(),
            zaehlung: "zaehlung".to_string(),
            hoehemaximal: "hoehemaximal".to_string(),
            typ: "typ".to_string(),
            primzahlen: "primzahlen".to_string(),
            potenzenvonzahlen: "potenzenvonzahlen".to_string(),
            vielfachevonzahlen: "vielfachevonzahlen".to_string(),
            primzahlvielfache: "primzahlvielfache".to_string(),
            nachtraeglichneuabzaehlung: "nachtraeglichneuabzaehlung".to_string(),
            nachtraeglichneuabzaehlungvielfache: "nachtraeglichneuabzaehlungvielfache".to_string(),
        }
    }
}

pub struct AusgabeParas {
    pub breite: String,
    pub breiten: String,
    pub keineueberschriften: String,
    pub keinenummerierung: String,
    pub keineleereninhalte: String,
    pub spaltenreihenfolgeundnurdiese: String,
    pub art: String,
    pub nocolor: String,
    pub justtext: String,
    pub endlessscreen: String,
    pub endless: String,
    pub dontwrap: String,
    pub onetable: String,
}

impl AusgabeParas {
    pub fn new() -> Self {
        Self {
            breite: "breite".to_string(),
            breiten: "breiten".to_string(),
            keineueberschriften: "keineueberschriften".to_string(),
            keinenummerierung: "keinenummerierung".to_string(),
            keineleereninhalte: "keineleereninhalte".to_string(),
            spaltenreihenfolgeundnurdiese: "spaltenreihenfolgeundnurdiese".to_string(),
            art: "art".to_string(),
            nocolor: "nocolor".to_string(),
            justtext: "justtext".to_string(),
            endlessscreen: "endlessscreen".to_string(),
            endless: "endless".to_string(),
            dontwrap: "dontwrap".to_string(),
            onetable: "onetable".to_string(),
        }
    }
}

pub struct KombiMainParas {
    pub galaxie: String,
    pub universum: String,
}

impl KombiMainParas {
    pub fn new() -> Self {
        Self {
            galaxie: "galaxie".to_string(),
            universum: "universum".to_string(),
        }
    }
}

pub fn get_para_n_data_matrix() -> Vec<(Vec<String>, Vec<String>, Vec<Vec<i32>>)> {
    vec![
        (
            vec!["multiplikationen".to_string()],
            vec![],
            vec![
                vec![1, 2, 3, 4, 5],  // ordinary
                vec![],               // generated1
                vec![],               // concat1
                vec![],               // kombi1
                vec![],               // boolAndTupleSet1
                vec![],               // gebroUni1
                vec![],               // gebrGal1
                vec![],               // generated2
                vec![],               // kombi2
                vec![],               // gebrEmo1
                vec![],               // gebrGroe1
                vec![],               // metakonkret
            ],
        ),
        (
            vec!["gebrochenuniversum".to_string(), "gebrochenuniversum2".to_string()],
            vec!["2".to_string(), "3".to_string(), "5".to_string(), "7".to_string()],
            vec![
                vec![],
                vec![],
                vec![],
                vec![],
                vec![],
                vec![2, 3, 5, 7],    // gebroUni1
                vec![],
                vec![],
                vec![],
                vec![],
                vec![],
                vec![],
            ],
        ),
        // Add more parameter entries as needed
    ]
}
