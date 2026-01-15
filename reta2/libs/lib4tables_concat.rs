// Cargo.toml dependencies:
// [dependencies]
// regex = "1.10"
// itertools = "0.12"
// num = "0.4"
// num-rational = "0.4"
// ordered-float = "3.9"
// enum-as-inner = "0.6"
// serde = { version = "1.0", features = ["derive"] }
// anyhow = "1.0"
// thiserror = "1.0"
// once_cell = "1.19"
// csv = "1.3"
// fractions = "0.5"
// strum = { version = "0.25", features = ["derive"] }

use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet, LinkedList, VecDeque};
use std::fmt::{self, Display, Formatter};
use std::fs::File;
use std::io::{self, BufReader};
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};

use anyhow::{Context, Result};
use csv::ReaderBuilder;
use enum_as_inner::EnumAsInner;
use fractions::Fraction;
use itertools::Itertools;
use num::Integer;
use ordered_float::OrderedFloat;
use regex::Regex;
use serde::{Deserialize, Serialize};
use strum::{Display, EnumIter, EnumString, IntoEnumIterator};

// Import from previously created modules
mod center {
    pub use super::*;
}

mod lib4tables {
    pub use super::*;
}

mod lib4tables_enum {
    pub use super::*;
}

// I18n module
mod i18n {
    use super::*;
    use once_cell::sync::Lazy;
    
    pub static INSTANCE: Lazy<I18n> = Lazy::new(|| I18n::new());
    
    #[derive(Debug, Clone)]
    pub struct I18n {
        pub csv_file_names: CsvFileNames,
        pub concat: ConcatI18n,
    }
    
    impl I18n {
        pub fn new() -> Self {
            I18n {
                csv_file_names: CsvFileNames::new(),
                concat: ConcatI18n::new(),
            }
        }
    }
    
    #[derive(Debug, Clone)]
    pub struct CsvFileNames {
        pub prim: String,
        pub bruch13: String,
        pub bruch15: String,
        pub bruch7: String,
        pub bruch_struk_groesse: String,
    }
    
    impl CsvFileNames {
        pub fn new() -> Self {
            CsvFileNames {
                prim: "prim.csv".to_string(),
                bruch13: "bruch13.csv".to_string(),
                bruch15: "bruch15.csv".to_string(),
                bruch7: "bruch7.csv".to_string(),
                bruch_struk_groesse: "bruchStrukGroesse.csv".to_string(),
            }
        }
    }
    
    #[derive(Debug, Clone)]
    pub struct ConcatI18n {
        pub polygon1: HashMap<String, String>,
        pub polygon2: HashMap<String, String>,
        pub gleichheit_freiheit_vergleich: HashMap<String, String>,
        pub energietopologie1: HashMap<String, String>,
        pub ausgabe_string: HashMap<String, String>,
        pub krea_zahl: HashMap<String, String>,
        pub mond_exp_log1: HashMap<String, String>,
        pub mond_exp_log2: HashMap<String, String>,
        pub modal_b: HashMap<String, String>,
        pub modal_c: HashMap<String, String>,
        pub modal_d: HashMap<String, String>,
        pub generiert_wort: HashMap<String, String>,
        pub alles_nur_bezogen_auf_satz: HashMap<String, String>,
        pub headline1: String,
        pub gegen: HashMap<String, String>,
        pub pro: HashMap<String, String>,
        pub pro_ist: HashMap<String, String>,
        pub contra_ist: HashMap<String, String>,
        pub hineinversetzen: HashMap<String, String>,
        pub hineinversetzen_satz: String,
        pub multiplikationen_: HashMap<String, String>,
        pub n_wichtigste: HashMap<String, String>,
        pub faktorenbla: HashMap<String, String>,
        pub polygone: HashMap<String, String>,
        pub kombis_namen: HashMap<String, String>,
        pub gen_mul: HashMap<String, String>,
        pub ausserdem: HashMap<String, String>,
        pub meta_or_what: HashMap<String, String>,
        pub thema_wort: String,
        pub meta_konkret: HashMap<String, String>,
        pub spalten_namen: HashMap<String, String>,
        pub prim_richtung: HashMap<String, String>,
        pub letzt_end: HashMap<String, String>,
        pub gal_or_uni_or_fehler: HashMap<String, String>,
        pub prim_viel_gen: HashMap<String, String>,
        pub polygon1_text: HashMap<String, String>,
        pub polygon2_text: HashMap<String, String>,
        pub krea_zahl_types: HashMap<String, String>,
        pub modal_b_text: HashMap<String, String>,
        pub modal_c_text: HashMap<String, String>,
        pub modal_d_text: HashMap<String, String>,
        pub generiert_wort_text: HashMap<String, String>,
        pub alles_nur_bezogen_auf_satz_text: String,
    }
    
    impl ConcatI18n {
        pub fn new() -> Self {
            let mut polygon1 = HashMap::new();
            polygon1.insert(" der eigenen Strukturgröße (".to_string(), " der eigenen Strukturgröße (".to_string());
            
            let mut polygon2 = HashMap::new();
            polygon2.insert(") auf dich bei gleichförmigen Polygonen".to_string(), ") auf dich bei gleichförmigen Polygonen".to_string());
            
            let mut gleichheit_freiheit_vergleich = HashMap::new();
            gleichheit_freiheit_vergleich.insert("Dominieren, Unterordnen".to_string(), "Dominieren, Unterordnen".to_string());
            gleichheit_freiheit_vergleich.insert("Freiheit".to_string(), "Freiheit".to_string());
            gleichheit_freiheit_vergleich.insert("Einschränkung der Freiheit".to_string(), "Einschränkung der Freiheit".to_string());
            gleichheit_freiheit_vergleich.insert("Gleichheit".to_string(), "Gleichheit".to_string());
            gleichheit_freiheit_vergleich.insert("den anderen überbieten wollen".to_string(), "den anderen überbieten wollen".to_string());
            gleichheit_freiheit_vergleich.insert("den anderen unterbieten wollen".to_string(), "den anderen unterbieten wollen".to_string());
            gleichheit_freiheit_vergleich.insert("Gleichheit, Freiheit, Dominieren (Ordnungen [12]) Generiert".to_string(), "Gleichheit, Freiheit, Dominieren (Ordnungen [12]) Generiert".to_string());
            
            let mut energietopologie1 = HashMap::new();
            energietopologie1.insert("eine Denkart".to_string(), "eine Denkart".to_string());
            energietopologie1.insert("eine Gefühlsart".to_string(), "eine Gefühlsart".to_string());
            energietopologie1.insert("total eine Art, etwas geistig zu erzeugen".to_string(), "total eine Art, etwas geistig zu erzeugen".to_string());
            energietopologie1.insert("total eine Art zu erleben".to_string(), "total eine Art zu erleben".to_string());
            energietopologie1.insert("total eine Energie-Art".to_string(), "total eine Energie-Art".to_string());
            energietopologie1.insert("etwas eine Art zu erleben".to_string(), "etwas eine Art zu erleben".to_string());
            energietopologie1.insert("etwas eine Art, etwas geistig zu erzeugen".to_string(), "etwas eine Art, etwas geistig zu erzeugen".to_string());
            energietopologie1.insert("wenig eine Art, etwas geistig zu erzeugen".to_string(), "wenig eine Art, etwas geistig zu erzeugen".to_string());
            energietopologie1.insert("einigermaßen eine Energie-Art".to_string(), "einigermaßen eine Energie-Art".to_string());
            energietopologie1.insert("kaum eine Energie-Art".to_string(), "kaum eine Energie-Art".to_string());
            energietopologie1.insert("kaum eine Art, etwas geistig zu erzeugen".to_string(), "kaum eine Art, etwas geistig zu erzeugen".to_string());
            
            let mut ausgabe_string = HashMap::new();
            ausgabe_string.insert("Energie oder Denkart oder Gefühlsart oder Materie-Art oder Topologie-Art".to_string(), "Energie oder Denkart oder Gefühlsart oder Materie-Art oder Topologie-Art".to_string());
            
            let mut krea_zahl = HashMap::new();
            krea_zahl.insert("Evolutions-Züchtungs-Kreativität".to_string(), "Evolutions-Züchtungs-Kreativität".to_string());
            krea_zahl.insert("0. Primzahl 1".to_string(), "0. Primzahl 1".to_string());
            krea_zahl.insert("1. Primzahl und Sonnenzahl".to_string(), "1. Primzahl und Sonnenzahl".to_string());
            krea_zahl.insert("2. Sonnenzahl, aber keine Primzahl".to_string(), "2. Sonnenzahl, aber keine Primzahl".to_string());
            krea_zahl.insert("3. Mondzahl".to_string(), "3. Mondzahl".to_string());
            
            let mut mond_exp_log1 = HashMap::new();
            mond_exp_log1.insert("Mond-Typ eines Sternpolygons".to_string(), "Mond-Typ eines Sternpolygons".to_string());
            mond_exp_log1.insert("Mond-Typ eines gleichförmigen Polygons".to_string(), "Mond-Typ eines gleichförmigen Polygons".to_string());
            
            let mut mond_exp_log2 = HashMap::new();
            mond_exp_log2.insert("kein Mond".to_string(), "kein Mond".to_string());
            
            let mut modal_b = HashMap::new();
            modal_b.insert("mittelstark überdurchschnittlich: ".to_string(), "mittelstark überdurchschnittlich: ".to_string());
            modal_b.insert("überdurchschnittlich: ".to_string(), "überdurchschnittlich: ".to_string());
            modal_b.insert("mittelleicht überdurchschnittlich: ".to_string(), "mittelleicht überdurchschnittlich: ".to_string());
            modal_b.insert("sehr: ".to_string(), "sehr: ".to_string());
            modal_b.insert("sehr leicht überdurchschnittlich: ".to_string(), "sehr leicht überdurchschnittlich: ".to_string());
            
            let mut modal_c = HashMap::new();
            modal_c.insert("intrinsisch".to_string(), "intrinsisch".to_string());
            modal_c.insert("zuerst".to_string(), "zuerst".to_string());
            modal_c.insert("extrinsisch".to_string(), "extrinsisch".to_string());
            modal_c.insert("als zweites".to_string(), "als zweites".to_string());
            
            let mut modal_d = HashMap::new();
            modal_d.insert(", nicht: ".to_string(), ", nicht: ".to_string());
            modal_d.insert(" (das alles nicht): ".to_string(), " (das alles nicht): ".to_string());
            
            let mut generiert_wort = HashMap::new();
            generiert_wort.insert("Generiert: ".to_string(), "Generiert: ".to_string());
            
            let alles_nur_bezogen_auf_satz_text = " alles nur bezogen auf: ".to_string();
            
            let headline1 = "Primzahlkreuz pro contra".to_string();
            
            let mut gegen = HashMap::new();
            gegen.insert("gegen ".to_string(), "gegen ".to_string());
            
            let mut pro = HashMap::new();
            pro.insert("pro ".to_string(), "pro ".to_string());
            
            let mut pro_ist = HashMap::new();
            pro_ist.insert("pro dieser Zahl sind: ".to_string(), "pro dieser Zahl sind: ".to_string());
            pro_ist.insert("pro dieser Zahl ist ".to_string(), "pro dieser Zahl ist ".to_string());
            
            let mut contra_ist = HashMap::new();
            contra_ist.insert("contra dieser Zahl sind: ".to_string(), "contra dieser Zahl sind: ".to_string());
            contra_ist.insert("contra dieser Zahl ist ".to_string(), "contra dieser Zahl ist ".to_string());
            
            let mut hineinversetzen = HashMap::new();
            hineinversetzen.insert(" Darin kann sich die ".to_string(), " Darin kann sich die ".to_string());
            hineinversetzen.insert(" am Besten hineinversetzen.".to_string(), " am Besten hineinversetzen.".to_string());
            
            let hineinversetzen_satz = " hineinversetzen.".to_string();
            
            let mut multiplikationen_ = HashMap::new();
            multiplikationen_.insert("Multiplikationen".to_string(), "Multiplikationen".to_string());
            
            let mut n_wichtigste = HashMap::new();
            n_wichtigste.insert("Wichtigstes_zum_verstehen".to_string(), "Wichtigstes_zum_verstehen".to_string());
            n_wichtigste.insert("Viertwichtigste".to_string(), "Viertwichtigste".to_string());
            
            let mut faktorenbla = HashMap::new();
            faktorenbla.insert(", mit Faktoren aus gebrochen-rationalen Zahlen".to_string(), ", mit Faktoren aus gebrochen-rationalen Zahlen".to_string());
            
            let mut polygone = HashMap::new();
            polygone.insert("Sternpolygone".to_string(), "Sternpolygone".to_string());
            polygone.insert("gleichförmige Polygone".to_string(), "gleichförmige Polygone".to_string());
            
            let mut kombis_namen = HashMap::new();
            kombis_namen.insert("Motiv -> Motiv".to_string(), "Motiv -> Motiv".to_string());
            kombis_namen.insert("Motiv -> Strukur".to_string(), "Motiv -> Strukur".to_string());
            kombis_namen.insert("Struktur -> Motiv".to_string(), "Struktur -> Motiv".to_string());
            kombis_namen.insert("Struktur -> Strukur".to_string(), "Struktur -> Strukur".to_string());
            
            let mut gen_mul = HashMap::new();
            gen_mul.insert("generierte Multiplikationen ".to_string(), "generierte Multiplikationen ".to_string());
            
            let mut ausserdem = HashMap::new();
            ausserdem.insert(", außerdem: ".to_string(), ", außerdem: ".to_string());
            ausserdem.insert("| außerdem: ".to_string(), "| außerdem: ".to_string());
            
            let mut meta_or_what = HashMap::new();
            meta_or_what.insert("Meta-Thema: ".to_string(), "Meta-Thema: ".to_string());
            meta_or_what.insert("Konkretes: ".to_string(), "Konkretes: ".to_string());
            meta_or_what.insert("Meta-".to_string(), "Meta-".to_string());
            meta_or_what.insert("Konkret-".to_string(), "Konkret-".to_string());
            meta_or_what.insert("Theorie-Thema: ".to_string(), "Theorie-Thema: ".to_string());
            meta_or_what.insert("Praxis: ".to_string(), "Praxis: ".to_string());
            meta_or_what.insert("Theorie-".to_string(), "Theorie-".to_string());
            meta_or_what.insert("Praxis-".to_string(), "Praxis-".to_string());
            meta_or_what.insert("Planungs-Thema: ".to_string(), "Planungs-Thema: ".to_string());
            meta_or_what.insert("Umsetzungs-Thema: ".to_string(), "Umsetzungs-Thema: ".to_string());
            meta_or_what.insert("Planung-".to_string(), "Planung-".to_string());
            meta_or_what.insert("Umsetzung-".to_string(), "Umsetzung-".to_string());
            meta_or_what.insert("Anlass-Thema: ".to_string(), "Anlass-Thema: ".to_string());
            meta_or_what.insert("Wirkungs-Thema: ".to_string(), "Wirkungs-Thema: ".to_string());
            meta_or_what.insert("Anlass-".to_string(), "Anlass-".to_string());
            meta_or_what.insert("wirkung-".to_string(), "wirkung-".to_string());
            meta_or_what.insert("Kraft-Gebung: ".to_string(), "Kraft-Gebung: ".to_string());
            meta_or_what.insert("Verstärkungs-Thema: ".to_string(), "Verstärkungs-Thema: ".to_string());
            meta_or_what.insert("Kraft-geben-".to_string(), "Kraft-geben-".to_string());
            meta_or_what.insert("Verstärkung-".to_string(), "Verstärkung-".to_string());
            meta_or_what.insert("Beherrschung: ".to_string(), "Beherrschung: ".to_string());
            meta_or_what.insert("Richtung-Thema: ".to_string(), "Richtung-Thema: ".to_string());
            meta_or_what.insert("beherrschend-".to_string(), "beherrschend-".to_string());
            meta_or_what.insert("Richtung-".to_string(), "Richtung-".to_string());
            
            let thema_wort = " Thema: ".to_string();
            
            let mut meta_konkret = HashMap::new();
            meta_konkret.insert("Meta".to_string(), "Meta".to_string());
            meta_konkret.insert("Theorie".to_string(), "Theorie".to_string());
            meta_konkret.insert("Management".to_string(), "Management".to_string());
            meta_konkret.insert("ganzheitlich".to_string(), "ganzheitlich".to_string());
            meta_konkret.insert("Verwertung, Unternehmung, Geschäft".to_string(), "Verwertung, Unternehmung, Geschäft".to_string());
            meta_konkret.insert("regieren, beherrschen".to_string(), "regieren, beherrschen".to_string());
            meta_konkret.insert("Konkretes".to_string(), "Konkretes".to_string());
            meta_konkret.insert("Praxis".to_string(), "Praxis".to_string());
            meta_konkret.insert("verändernd".to_string(), "verändernd".to_string());
            meta_konkret.insert("darüber hinaus gehend".to_string(), "darüber hinaus gehend".to_string());
            meta_konkret.insert("wertvoll".to_string(), "wertvoll".to_string());
            meta_konkret.insert("Richtung".to_string(), "Richtung".to_string());
            meta_konkret.insert(" für 1/n statt n".to_string(), " für 1/n statt n".to_string());
            meta_konkret.insert(" für n".to_string(), " für n".to_string());
            
            let mut spalten_namen = HashMap::new();
            spalten_namen.insert("Transzendentalien, Strukturalien, Universum n".to_string(), "Transzendentalien, Strukturalien, Universum n".to_string());
            spalten_namen.insert("Galaxie n".to_string(), "Galaxie n".to_string());
            spalten_namen.insert("Galaxie 1/n".to_string(), "Galaxie 1/n".to_string());
            spalten_namen.insert("Transzendentalien, Strukturalien, Universum 1/n".to_string(), "Transzendentalien, Strukturalien, Universum 1/n".to_string());
            spalten_namen.insert("Dagegen-Gegen-Transzendentalien, Gegen-Strukturalien, Universum n".to_string(), "Dagegen-Gegen-Transzendentalien, Gegen-Strukturalien, Universum n".to_string());
            spalten_namen.insert("neutrale Gegen-Transzendentalien, Gegen-Strukturalien, Universum n".to_string(), "neutrale Gegen-Transzendentalien, Gegen-Strukturalien, Universum n".to_string());
            spalten_namen.insert("Richtung-Richtung".to_string(), "Richtung-Richtung".to_string());
            
            let mut prim_richtung = HashMap::new();
            prim_richtung.insert("Primzahlwirkung (7, Richtung) ".to_string(), "Primzahlwirkung (7, Richtung) ".to_string());
            
            let mut letzt_end = HashMap::new();
            letzt_end.insert("] * letztendlich: ".to_string(), "] * letztendlich: ".to_string());
            
            let mut gal_or_uni_or_fehler = HashMap::new();
            gal_or_uni_or_fehler.insert("Fehler".to_string(), "Fehler".to_string());
            gal_or_uni_or_fehler.insert("Universum".to_string(), "Universum".to_string());
            gal_or_uni_or_fehler.insert("Galaxie".to_string(), "Galaxie".to_string());
            gal_or_uni_or_fehler.insert("Emotion".to_string(), "Emotion".to_string());
            gal_or_uni_or_fehler.insert("Strukturgroesse".to_string(), "Strukturgroesse".to_string());
            
            let mut prim_viel_gen = HashMap::new();
            prim_viel_gen.insert("Primzahlvielfache, nicht generiert".to_string(), "Primzahlvielfache, nicht generiert".to_string());
            
            ConcatI18n {
                polygon1,
                polygon2,
                gleichheit_freiheit_vergleich,
                energietopologie1,
                ausgabe_string,
                krea_zahl,
                mond_exp_log1,
                mond_exp_log2,
                modal_b,
                modal_c,
                modal_d,
                generiert_wort,
                alles_nur_bezogen_auf_satz: HashMap::new(),
                headline1,
                gegen,
                pro,
                pro_ist,
                contra_ist,
                hineinversetzen,
                hineinversetzen_satz,
                multiplikationen_,
                n_wichtigste,
                faktorenbla,
                polygone,
                kombis_namen,
                gen_mul,
                ausserdem,
                meta_or_what,
                thema_wort,
                meta_konkret,
                spalten_namen,
                prim_richtung,
                letzt_end,
                gal_or_uni_or_fehler,
                prim_viel_gen,
                polygon1_text: HashMap::new(),
                polygon2_text: HashMap::new(),
                krea_zahl_types: HashMap::new(),
                modal_b_text: HashMap::new(),
                modal_c_text: HashMap::new(),
                modal_d_text: HashMap::new(),
                generiert_wort_text: HashMap::new(),
                alles_nur_bezogen_auf_satz_text,
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Display, EnumIter, EnumString)]
pub enum NPmEnum {
    #[strum(serialize = "galN")]
    GalN = 2,
    #[strum(serialize = "gal1pN")]
    Gal1pN = 3,
    #[strum(serialize = "uniN")]
    UniN = 4,
    #[strum(serialize = "uni1pN")]
    Uni1pN = 5,
    #[strum(serialize = "emoN")]
    EmoN = 6,
    #[strum(serialize = "emo1pN")]
    Emo1pN = 7,
    #[strum(serialize = "groeN")]
    GroeN = 8,
    #[strum(serialize = "groe1pN")]
    Groe1pN = 9,
}

impl NPmEnum {
    pub fn gal() -> Vec<Self> {
        vec![Self::GalN, Self::Gal1pN]
    }
    
    pub fn uni() -> Vec<Self> {
        vec![Self::UniN, Self::Uni1pN]
    }
    
    pub fn emo() -> Vec<Self> {
        vec![Self::EmoN, Self::Emo1pN]
    }
    
    pub fn groe() -> Vec<Self> {
        vec![Self::GroeN, Self::Groe1pN]
    }
    
    pub fn n() -> Vec<Self> {
        vec![Self::GalN, Self::UniN, Self::EmoN, Self::GroeN]
    }
    
    pub fn eins_pn() -> Vec<Self> {
        vec![Self::Gal1pN, Self::Uni1pN, Self::Emo1pN, Self::Groe1pN]
    }
    
    pub fn is_gal(&self) -> bool {
        matches!(self, Self::GalN | Self::Gal1pN)
    }
    
    pub fn is_uni(&self) -> bool {
        matches!(self, Self::UniN | Self::Uni1pN)
    }
    
    pub fn is_emo(&self) -> bool {
        matches!(self, Self::EmoN | Self::Emo1pN)
    }
    
    pub fn is_groe(&self) -> bool {
        matches!(self, Self::GroeN | Self::Groe1pN)
    }
    
    pub fn is_n(&self) -> bool {
        matches!(self, Self::GalN | Self::UniN | Self::EmoN | Self::GroeN)
    }
    
    pub fn is_eins_pn(&self) -> bool {
        matches!(self, Self::Gal1pN | Self::Uni1pN | Self::Emo1pN | Self::Groe1pN)
    }
}

pub struct Concat<'a> {
    tables: &'a Tables,
    ones: BTreeSet<i32>,
    csvs_already_read: BTreeMap<i32, Vec<Vec<String>>>,
    csvs_same: BTreeMap<i32, Vec<i32>>,
    brueche_uni: BTreeSet<Fraction>,
    brueche_gal: BTreeSet<Fraction>,
    gebr_rat_mul_stern_uni: BTreeSet<(Fraction, Fraction)>,
    gebr_rat_div_stern_uni: BTreeSet<(Fraction, Fraction)>,
    gebr_rat_mul_gleichf_uni: BTreeSet<(Fraction, Fraction)>,
    gebr_rat_div_gleichf_uni: BTreeSet<(Fraction, Fraction)>,
    gebr_rat_mul_stern_gal: BTreeSet<(Fraction, Fraction)>,
    gebr_rat_div_stern_gal: BTreeSet<(Fraction, Fraction)>,
    gebr_rat_mul_gleichf_gal: BTreeSet<(Fraction, Fraction)>,
    gebr_rat_div_gleichf_gal: BTreeSet<(Fraction, Fraction)>,
}

impl<'a> Concat<'a> {
    pub fn new(tables: &'a Tables) -> Self {
        let mut csvs_same = BTreeMap::new();
        csvs_same.insert(1, vec![1]);
        csvs_same.insert(2, vec![2, 4]);
        csvs_same.insert(3, vec![3, 5]);
        csvs_same.insert(4, vec![2, 4]);
        csvs_same.insert(5, vec![3, 5]);
        
        Concat {
            tables,
            ones: BTreeSet::new(),
            csvs_already_read: BTreeMap::new(),
            csvs_same,
            brueche_uni: BTreeSet::new(),
            brueche_gal: BTreeSet::new(),
            gebr_rat_mul_stern_uni: BTreeSet::new(),
            gebr_rat_div_stern_uni: BTreeSet::new(),
            gebr_rat_mul_gleichf_uni: BTreeSet::new(),
            gebr_rat_div_gleichf_uni: BTreeSet::new(),
            gebr_rat_mul_stern_gal: BTreeSet::new(),
            gebr_rat_div_stern_gal: BTreeSet::new(),
            gebr_rat_mul_gleichf_gal: BTreeSet::new(),
            gebr_rat_div_gleichf_gal: BTreeSet::new(),
        }
    }
    
    pub fn concat_love_polygon(
        &mut self,
        relitable: &mut Vec<Vec<String>>,
        rows_as_numbers: &mut BTreeSet<i32>,
    ) -> Result<()> {
        if rows_as_numbers.contains(&9) {
            rows_as_numbers.insert(relitable[0].len() as i32);
            
            self.tables.generated_spalten_parameter_tags.insert(
                (rows_as_numbers.len() - 1) as i32,
                [ST::SternPolygon, ST::Galaxie, ST::GleichfoermigesPolygon]
                    .iter()
                    .copied()
                    .collect(),
            );
            
            for i in 0..relitable.len() {
                let new_cell = if !relitable[i][8].trim().is_empty() {
                    format!(
                        "{}{}{}{}",
                        relitable[i][8],
                        i18n::INSTANCE.concat.polygon1[" der eigenen Strukturgröße ("],
                        relitable[i][4],
                        i18n::INSTANCE.concat.polygon2[") auf dich bei gleichförmigen Polygonen"]
                    )
                } else {
                    String::new()
                };
                relitable[i].push(new_cell);
            }
            
            let new_index = (self.tables.generated_spalten_parameter.len()
                + self.tables.spalten_vanilla_amount) as i32;
            
            if self.tables.generated_spalten_parameter.contains_key(&new_index) {
                return Err(anyhow::anyhow!("Duplicate generated parameter index"));
            }
            
            if let Some(data) = self.tables.data_dict.get(&0).and_then(|d| d.get(&9)) {
                self.tables.generated_spalten_parameter.insert(new_index, data.clone());
            }
        }
        
        Ok(())
    }
    
    pub fn gleichheit_freiheit_vergleich(&self, zahl: i32) -> String {
        let mut ausgabe_string_list = Vec::new();
        let i18n = &i18n::INSTANCE.concat.gleichheit_freiheit_vergleich;
        
        match zahl % 4 {
            0 => ausgabe_string_list.push(i18n["Dominieren, Unterordnen"].clone()),
            1 => ausgabe_string_list.push(i18n["Freiheit"].clone()),
            3 => ausgabe_string_list.push(i18n["Einschränkung der Freiheit"].clone()),
            2 => {
                if (zahl - 2) % 8 == 0 {
                    ausgabe_string_list.push(i18n["Gleichheit"].clone());
                }
                if (zahl - 6) % 16 == 0 {
                    ausgabe_string_list.push(i18n["den anderen überbieten wollen"].clone());
                }
                if (zahl - 14) % 16 == 0 {
                    ausgabe_string_list.push(i18n["den anderen unterbieten wollen"].clone());
                }
            }
            _ => {}
        }
        
        ausgabe_string_list.join("; ")
    }
    
    pub fn geist_emotion_energie_materie_topologie(&self, zahl: i32) -> String {
        let pr_fa = prim_fak(zahl);
        let auss: Vec<bool> = pr_fa.iter().map(|&a| could_be_prime_number_primzahlkreuz_fuer_aussen(a)).collect();
        let innen: Vec<bool> = pr_fa.iter().map(|&a| could_be_prime_number_primzahlkreuz_fuer_innen(a)).collect();
        let zwei = pr_fa.iter().filter(|&&a| a == 2).count();
        
        let gefuehl = auss.iter().any(|&a| a);
        let denken = innen.iter().any(|&a| a);
        
        let total_topologie = zwei > 1 && gefuehl;
        let etwas_topologie = (zwei > 1 || (zwei > 0 && gefuehl)) && !total_topologie;
        let total_materie = zwei > 4;
        let etwas_materie = zwei == 4;
        let wenig_materie = zwei == 3;
        let kaum_materie = zwei == 2;
        
        let (x, y, z) = (denken, pr_fa.contains(&2), pr_fa.contains(&3));
        let total_energie = x && y && z;
        let einermassen_energie = ((x && y) || (y && z) || (x && z)) && !total_energie;
        let kaum_energie = !einermassen_energie && !total_energie && (x || y || z);
        
        let mut ausgabe_string_list = Vec::new();
        let i18n = &i18n::INSTANCE.concat.energietopologie1;
        
        if denken {
            ausgabe_string_list.push(i18n["eine Denkart"].clone());
        }
        if gefuehl {
            ausgabe_string_list.push(i18n["eine Gefühlsart"].clone());
        }
        if total_materie {
            ausgabe_string_list.push(i18n["total eine Art, etwas geistig zu erzeugen"].clone());
        }
        if total_topologie {
            ausgabe_string_list.push(i18n["total eine Art zu erleben"].clone());
        }
        if total_energie {
            ausgabe_string_list.push(i18n["total eine Energie-Art"].clone());
        }
        if etwas_topologie {
            ausgabe_string_list.push(i18n["etwas eine Art zu erleben"].clone());
        }
        if etwas_materie {
            ausgabe_string_list.push(i18n["etwas eine Art, etwas geistig zu erzeugen"].clone());
        }
        if wenig_materie {
            ausgabe_string_list.push(i18n["wenig eine Art, etwas geistig zu erzeugen"].clone());
        }
        if einermassen_energie {
            ausgabe_string_list.push(i18n["einigermaßen eine Energie-Art"].clone());
        }
        if kaum_energie {
            ausgabe_string_list.push(i18n["kaum eine Energie-Art"].clone());
        }
        if kaum_materie {
            ausgabe_string_list.push(i18n["kaum eine Art, etwas geistig zu erzeugen"].clone());
        }
        
        ausgabe_string_list.join("; ")
    }
    
    pub fn concat_gleichheit_freiheit_dominieren(
        &mut self,
        relitable: &mut Vec<Vec<String>>,
        rows_as_numbers: &mut BTreeSet<i32>,
    ) -> Result<()> {
        if rows_as_numbers.contains(&132) {
            rows_as_numbers.insert(relitable[0].len() as i32);
            
            self.tables.generated_spalten_parameter_tags.insert(
                (rows_as_numbers.len() - 1) as i32,
                [ST::SternPolygon, ST::Universum].iter().copied().collect(),
            );
            
            for (i, row) in relitable.iter_mut().enumerate() {
                let ausgabe_string = if i == 0 {
                    i18n::INSTANCE.concat.gleichheit_freiheit_vergleich[
                        "Gleichheit, Freiheit, Dominieren (Ordnungen [12]) Generiert"
                    ].clone()
                } else {
                    self.gleichheit_freiheit_vergleich(i as i32)
                };
                row.push(ausgabe_string);
            }
            
            let new_index = (self.tables.generated_spalten_parameter.len()
                + self.tables.spalten_vanilla_amount) as i32;
            
            if self.tables.generated_spalten_parameter.contains_key(&new_index) {
                return Err(anyhow::anyhow!("Duplicate generated parameter index
