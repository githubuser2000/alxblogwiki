//! Center module - equivalent to Python center module

use std::collections::{HashMap, HashSet};
use indexmap::{IndexSet, IndexMap};
use num_rational::Ratio;
use serde::{Deserialize, Serialize};

pub type DefaultOrderedDict<K, V> = IndexMap<K, V>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Multiplikationen {
    pub values: Vec<(i32, i32)>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrimzahlkreuzProContraStrs {
    pub pro: String,
    pub contra: String,
}

pub fn alxp(msg: &str) {
    println!("ALXP: {}", msg);
}

pub fn cliout(msg: &str) {
    println!("CLIOUT: {}", msg);
}

pub fn get_text_wrap_things(text: &str, width: usize) -> Vec<String> {
    text.chars()
        .collect::<Vec<char>>()
        .chunks(width)
        .map(|chunk| chunk.iter().collect())
        .collect()
}

pub mod i18n {
    use once_cell::sync::Lazy;
    use std::collections::HashMap;
    
    static TRANSLATIONS: Lazy<HashMap<&'static str, &'static str>> = Lazy::new(|| {
        let mut m = HashMap::new();
        m.insert("concat", "Concat");
        m.insert("polygon1", " der eigenen Strukturgröße (");
        m.insert("polygon2", ") auf dich bei gleichförmigen Polygonen");
        m.insert("gleichheitFreiheitVergleich.Dominieren, Unterordnen", "Dominieren, Unterordnen");
        m.insert("gleichheitFreiheitVergleich.Freiheit", "Freiheit");
        m.insert("gleichheitFreiheitVergleich.Einschränkung der Freiheit", "Einschränkung der Freiheit");
        m.insert("gleichheitFreiheitVergleich.Gleichheit", "Gleichheit");
        m.insert("gleichheitFreiheitVergleich.den anderen überbieten wollen", "den anderen überbieten wollen");
        m.insert("gleichheitFreiheitVergleich.den anderen unterbieten wollen", "den anderen unterbieten wollen");
        m.insert("gleichheitFreiheitVergleich.Gleichheit, Freiheit, Dominieren (Ordnungen [12]) Generiert", "Gleichheit, Freiheit, Dominieren (Ordnungen [12]) Generiert");
        m.insert("energietopologie1.eine Denkart", "eine Denkart");
        m.insert("energietopologie1.eine Gefühlsart", "eine Gefühlsart");
        m.insert("energietopologie1.total eine Art, etwas geistig zu erzeugen", "total eine Art, etwas geistig zu erzeugen");
        m.insert("energietopologie1.total eine Art zu erleben", "total eine Art zu erleben");
        m.insert("energietopologie1.total eine Energie-Art", "total eine Energie-Art");
        m.insert("energietopologie1.etwas eine Art zu erleben", "etwas eine Art zu erleben");
        m.insert("energietopologie1.etwas eine Art, etwas geistig zu erzeugen", "etwas eine Art, etwas geistig zu erzeugen");
        m.insert("energietopologie1.wenig eine Art, etwas geistig zu erzeugen", "wenig eine Art, etwas geistig zu erzeugen");
        m.insert("energietopologie1.einigermaßen eine Energie-Art", "einigermaßen eine Energie-Art");
        m.insert("energietopologie1.kaum eine Energie-Art", "kaum eine Energie-Art");
        m.insert("energietopologie1.kaum eine Art, etwas geistig zu erzeugen", "kaum eine Art, etwas geistig zu erzeugen");
        m.insert("ausgabeString.Energie oder Denkart oder Gefühlsart oder Materie-Art oder Topologie-Art", "Energie oder Denkart oder Gefühlsart oder Materie-Art oder Topologie-Art");
        m.insert("kreaZahl.Evolutions-Züchtungs-Kreativität", "Evolutions-Züchtungs-Kreativität");
        m.insert("kreaZahl.0. Primzahl 1", "0. Primzahl 1");
        m.insert("kreaZahl.1. Primzahl und Sonnenzahl", "1. Primzahl und Sonnenzahl");
        m.insert("kreaZahl.2. Sonnenzahl, aber keine Primzahl", "2. Sonnenzahl, aber keine Primzahl");
        m.insert("kreaZahl.3. Mondzahl", "3. Mondzahl");
        m.insert("mondExpLog1.Mond-Typ eines Sternpolygons", "Mond-Typ eines Sternpolygons");
        m.insert("mondExpLog1.Mond-Typ eines gleichförmigen Polygons", "Mond-Typ eines gleichförmigen Polygons");
        m.insert("mondExpLog2.kein Mond", "kein Mond");
        m.insert("modalB.mittelstark überdurchschnittlich: ", "mittelstark überdurchschnittlich: ");
        m.insert("modalB.überdurchschnittlich: ", "überdurchschnittlich: ");
        m.insert("modalB.mittelleicht überdurchschnittlich: ", "mittelleicht überdurchschnittlich: ");
        m.insert("modalB.sehr: ", "sehr: ");
        m.insert("modalB.sehr leicht überdurchschnittlich: ", "sehr leicht überdurchschnittlich: ");
        m.insert("modalC.intrinsisch", "intrinsisch");
        m.insert("modalC.zuerst", "zuerst");
        m.insert("modalC.extrinsisch", "extrinsisch");
        m.insert("modalC.als zweites", "als zweites");
        m.insert("modalD., nicht: ", ", nicht: ");
        m.insert("modalD. (das alles nicht): ", " (das alles nicht): ");
        m.insert("allesNurBezogenAufSatz", "Alles nur bezogen auf Satz: ");
        m.insert("generiertWort.Generiert: ", "Generiert: ");
        m.insert("headline1", "Primzahlkreuz pro contra");
        m.insert("gegen.gegen ", "gegen ");
        m.insert("pro.pro ", "pro ");
        m.insert("hineinversetzen. Darin kann sich die ", " Darin kann sich die ");
        m.insert("hineinversetzen. am Besten hineinversetzen.", " am Besten hineinversetzen.");
        m.insert("proIst.pro dieser Zahl sind: ", "pro dieser Zahl sind: ");
        m.insert("proIst.pro dieser Zahl ist ", "pro dieser Zahl ist ");
        m.insert("contraIst. contra dieser Zahl sind: ", "contra dieser Zahl sind: ");
        m.insert("contraIst. contra dieser Zahl ist ", "contra dieser Zahl ist ");
        m.insert("hineinversetzenSatz", " kann sich in diese Zahl hineinversetzen.");
        m.insert("polygone.Sternpolygone", "Sternpolygone");
        m.insert("polygone.gleichförmige Polygone", "gleichförmige Polygone");
        m.insert("kombisNamen.Motiv -> Motiv", "Motiv -> Motiv");
        m.insert("kombisNamen.Motiv -> Strukur", "Motiv -> Struktur");
        m.insert("kombisNamen.Struktur -> Motiv", "Struktur -> Motiv");
        m.insert("kombisNamen.Struktur -> Strukur", "Struktur -> Struktur");
        m.insert("faktorenbla., mit Faktoren aus gebrochen-rationalen Zahlen", ", mit Faktoren aus gebrochen-rationalen Zahlen");
        m.insert("genMul.generierte Multiplikationen ", "generierte Multiplikationen ");
        m.insert("ausserdem., außerdem: ", ", außerdem: ");
        m.insert("Multiplikationen_.Multiplikationen", "Multiplikationen");
        m.insert("nWichtigste.Wichtigstes_zum_verstehen", "Wichtigstes_zum_verstehen");
        m.insert("nWichtigste.Viertwichtigste", "Viertwichtigste");
        m.insert("metaOrWhat.Meta-Thema: ", "Meta-Thema: ");
        m.insert("metaOrWhat.Konkretes: ", "Konkretes: ");
        m.insert("metaOrWhat.Meta-", "Meta-");
        m.insert("metaOrWhat.Konkret-", "Konkret-");
        m.insert("metaOrWhat.Theorie-Thema: ", "Theorie-Thema: ");
        m.insert("metaOrWhat.Praxis: ", "Praxis: ");
        m.insert("metaOrWhat.Theorie-", "Theorie-");
        m.insert("metaOrWhat.Praxis-", "Praxis-");
        m.insert("metaOrWhat.Planungs-Thema: ", "Planungs-Thema: ");
        m.insert("metaOrWhat.Umsetzungs-Thema: ", "Umsetzungs-Thema: ");
        m.insert("metaOrWhat.Planung-", "Planung-");
        m.insert("metaOrWhat.Umsetzung-", "Umsetzung-");
        m.insert("metaOrWhat.Anlass-Thema: ", "Anlass-Thema: ");
        m.insert("metaOrWhat.Wirkungs-Thema: ", "Wirkungs-Thema: ");
        m.insert("metaOrWhat.Anlass-", "Anlass-");
        m.insert("metaOrWhat.wirkung-", "wirkung-");
        m.insert("metaOrWhat.Kraft-Gebung: ", "Kraft-Gebung: ");
        m.insert("metaOrWhat.Verstärkungs-Thema: ", "Verstärkungs-Thema: ");
        m.insert("metaOrWhat.Kraft-geben-", "Kraft-geben-");
        m.insert("metaOrWhat.Verstärkung-", "Verstärkung-");
        m.insert("metaOrWhat.Beherrschung: ", "Beherrschung: ");
        m.insert("metaOrWhat.Richtung-Thema: ", "Richtung-Thema: ");
        m.insert("metaOrWhat.beherrschend-", "beherrschend-");
        m.insert("metaOrWhat.Richtung-", "Richtung-");
        m.insert("themaWort", " Thema: ");
        m.insert("metaKonkret.Meta", "Meta");
        m.insert("metaKonkret.Theorie", "Theorie");
        m.insert("metaKonkret.Management", "Management");
        m.insert("metaKonkret.ganzheitlich", "ganzheitlich");
        m.insert("metaKonkret.Verwertung, Unternehmung, Geschäft", "Verwertung, Unternehmung, Geschäft");
        m.insert("metaKonkret.regieren, beherrschen", "regieren, beherrschen");
        m.insert("metaKonkret.Konkretes", "Konkretes");
        m.insert("metaKonkret.Praxis", "Praxis");
        m.insert("metaKonkret.verändernd", "verändernd");
        m.insert("metaKonkret.darüber hinaus gehend", "darüber hinaus gehend");
        m.insert("metaKonkret.wertvoll", "wertvoll");
        m.insert("metaKonkret.Richtung", "Richtung");
        m.insert("metaKonkret. für 1/n statt n", " für 1/n statt n");
        m.insert("metaKonkret. für n", " für n");
        m.insert("innenAussen.für innen", "für innen");
        m.insert("innenAussen.für außen", "für außen");
        m.insert("innenAussen.\"für seitlich und gegen Schwächlinge innen\"", "\"für seitlich und gegen Schwächlinge innen\"");
        m.insert("innenAussen.\"gegen seitlich und für Schwächlinge innen\"", "\"gegen seitlich und für Schwächlinge innen\"");
        m.insert("spaltenNamen.Transzendentalien, Strukturalien, Universum n", "Transzendentalien, Strukturalien, Universum n");
        m.insert("spaltenNamen.Galaxie n", "Galaxie n");
        m.insert("spaltenNamen.Galaxie 1/n", "Galaxie 1/n");
        m.insert("spaltenNamen.Transzendentalien, Strukturalien, Universum 1/n", "Transzendentalien, Strukturalien, Universum 1/n");
        m.insert("spaltenNamen.Dagegen-Gegen-Transzendentalien, Gegen-Strukturalien, Universum n", "Dagegen-Gegen-Transzendentalien, Gegen-Strukturalien, Universum n");
        m.insert("spaltenNamen.neutrale Gegen-Transzendentalien, Gegen-Strukturalien, Universum n", "neutrale Gegen-Transzendentalien, Gegen-Strukturalien, Universum n");
        m.insert("spaltenNamen.Richtung-Richtung", "Richtung-Richtung");
        m.insert("primRicht.Primzahlwirkung (7, Richtung) ", "Primzahlwirkung (7, Richtung) ");
        m.insert("letztEnd.] * letztendlich: ", "] * letztendlich: ");
        m.insert("primVielGen.Primzahlvielfache, nicht generiert", "Primzahlvielfache, nicht generiert");
        m.insert("GalOrUniOrFehler.Fehler", "Fehler");
        m.insert("GalOrUniOrFehler.Universum", "Universum");
        m.insert("GalOrUniOrFehler.Galaxie", "Galaxie");
        m.insert("GalOrUniOrFehler.Emotion", "Emotion");
        m.insert("GalOrUniOrFehler.Strukturgroesse", "Strukturgroesse");
        m.insert("multipl.Multiplikationen", "Multiplikationen");
        m.insert("notGen.Nicht_generiert", "Nicht_generiert");
        m
    });
    
    pub fn get(key: &str) -> String {
        TRANSLATIONS.get(key).map(|&s| s.to_string()).unwrap_or_else(|| key.to_string())
    }
    
    // Convenience functions for specific keys
    pub fn polygon1(s: &str) -> &str { " der eigenen Strukturgröße (" }
    pub fn polygon2(s: &str) -> &str { ") auf dich bei gleichförmigen Polygonen" }
    pub fn gleichheit_freiheit_vergleich(s: &str) -> String { get(s) }
    pub fn energietopologie1(s: &str) -> String { get(s) }
    pub fn ausgabe_string(s: &str) -> String { get(s) }
    pub fn krea_zahl(s: &str) -> String { get(s) }
    pub fn mond_exp_log1(s: &str) -> String { get(s) }
    pub fn mond_exp_log2(s: &str) -> String { get(s) }
    pub fn modal_b(s: &str) -> String { get(s) }
    pub fn modal_c(s: &str) -> String { get(s) }
    pub fn modal_d(s: &str) -> String { get(s) }
    pub fn alles_nur_bezogen_auf_satz() -> &'static str { "Alles nur bezogen auf Satz: " }
    pub fn generiert_wort(s: &str) -> String { get(s) }
    pub fn headline1() -> &'static str { "Primzahlkreuz pro contra" }
    pub fn gegen(s: &str) -> String { get(s) }
    pub fn pro(s: &str) -> String { get(s) }
    pub fn hineinversetzen(s: &str) -> String { get(s) }
    pub fn pro_ist(s: &str) -> String { get(s) }
    pub fn contra_ist(s: &str) -> String { get(s) }
    pub fn hineinversetzen_satz() -> &'static str { " kann sich in diese Zahl hineinversetzen." }
    pub fn polygone(s: &str) -> String { get(s) }
    pub fn kombis_namen(s: &str) -> String { get(s) }
    pub fn faktorenbla(s: &str) -> String { get(s) }
    pub fn gen_mul(s: &str) -> String { get(s) }
    pub fn ausserdem(s: &str) -> String { get(s) }
    pub fn multiplikationen_(s: &str) -> String { get(s) }
    pub fn n_wichtigste(s: &str) -> String { get(s) }
    pub fn meta_or_what(s: &str) -> String { get(s) }
    pub fn thema_wort() -> &'static str { " Thema: " }
    pub fn meta_konkret(s: &str) -> String { get(s) }
    pub fn innen_aussen(s: &str) -> String { get(s) }
    pub fn spalten_namen(s: &str) -> String { get(s) }
    pub fn prim_richt(s: &str) -> String { get(s) }
    pub fn letzt_end(s: &str) -> String { get(s) }
    pub fn prim_viel_gen(s: &str) -> String { get(s) }
    pub fn gal_or_uni_or_fehler(s: &str) -> String { get(s) }
    pub fn multipl(s: &str) -> String { get(s) }
    pub fn not_gen(s: &str) -> String { get(s) }
}

pub fn info_log(msg: &str) {
    eprintln!("INFO: {}", msg);
}

pub fn output(msg: &str) {
    println!("{}", msg);
}

pub fn primfaktoren(zahl: i32) -> Vec<i32> {
    let mut n = zahl;
    let mut faktoren = Vec::new();
    let mut d = 2;
    
    while n > 1 {
        while n % d == 0 {
            faktoren.push(d);
            n /= d;
        }
        d += 1;
        if d * d > n {
            if n > 1 {
                faktoren.push(n);
            }
            break;
        }
    }
    
    faktoren
}

pub fn multiples(n: i32) -> Vec<(i32, i32)> {
    let mut result = Vec::new();
    for i in 1..=n {
        if n % i == 0 {
            result.push((i, n / i));
        }
    }
    result
}

pub fn unique_everseen<T, I, F>(iter: I, key: F) -> Vec<T>
where
    T: Clone + Eq + std::hash::Hash,
    I: IntoIterator<Item = T>,
    F: Fn(&T) -> T,
{
    let mut seen = HashSet::new();
    let mut result = Vec::new();
    
    for item in iter {
        let key_val = key(&item);
        if !seen.contains(&key_val) {
            seen.insert(key_val);
            result.push(item);
        }
    }
    
    result
}

pub fn x<T: std::fmt::Debug>(label: &str, value: T) {
    println!("{}: {:?}", label, value);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NPmEnum {
    UniN,
    Uni1pN,
    GalN,
    Gal1pN,
    EmoN,
    Emo1pN,
    GroeN,
    Groe1pN,
}

impl NPmEnum {
    pub fn uni() -> Vec<Self> {
        vec![Self::UniN, Self::Uni1pN]
    }
    
    pub fn gal() -> Vec<Self> {
        vec![Self::GalN, Self::Gal1pN]
    }
    
    pub fn emo() -> Vec<Self> {
        vec![Self::EmoN, Self::Emo1pN]
    }
    
    pub fn groe() -> Vec<Self> {
        vec![Self::GroeN, Self::Groe1pN]
    }
    
    pub fn n() -> Vec<Self> {
        vec![Self::UniN, Self::GalN, Self::EmoN, Self::GroeN]
    }
    
    pub fn eins_pn() -> Vec<Self> {
        vec![Self::Uni1pN, Self::Gal1pN, Self::Emo1pN, Self::Groe1pN]
    }
    
    pub fn value(&self) -> usize {
        match self {
            Self::UniN => 4,
            Self::Uni1pN => 5,
            Self::GalN => 2,
            Self::Gal1pN => 3,
            Self::EmoN => 6,
            Self::Emo1pN => 7,
            Self::GroeN => 8,
            Self::Groe1pN => 9,
        }
    }
}

pub fn n_pm_enum() -> NPmEnum {
    NPmEnum::UniN
}
