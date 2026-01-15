use regex::Regex;

/// Prüft, ob ein einzelnes Element der Bruch-Syntax entspricht
/// z.B. "1/2-3/4+5/6", optional mit v-Präfix aus i18n.befehle2["v"]
fn is_zeilen_bruch_angabe_between_commas(g: &str, v_prefix: &str) -> bool {
    let pattern_str = format!(
        r"^({}?[-]?\d+/\\d+)(-\d+/\d+)?((\+)(\d+/\d+))*$",
        regex::escape(v_prefix)
    );
    let re = Regex::new(&pattern_str).unwrap();
    re.is_match(g)
}

/// Prüft, ob ein einzelnes Element Ganzzahlen-Syntax entspricht
/// z.B. "1-3+2"
fn is_zeilen_angabe_between_commas(g: &str, v_prefix: &str) -> bool {
    let pattern_str = format!(
        r"^({}?[-]?\d+)(-\d+)?((\+)(\d+))*$",
        regex::escape(v_prefix)
    );
    let re = Regex::new(&pattern_str).unwrap();

    re.is_match(g)
}

/// Prüft einen durch Kommas getrennten String für Brüche oder Ganzzahlen
fn is_zeilen_bruch_or_ganz_zahl_angabe(text: &str, v_prefix: &str) -> bool {
    text.split(',')
        .map(|g| {
            is_zeilen_bruch_angabe_between_commas(g, v_prefix)
                || is_zeilen_angabe_between_commas(g, v_prefix)
        })
        .all(|x| x)
}

/// Prüft einen durch Kommas getrennten String nur für Ganzzahlen
fn is_zeilen_angabe(text: &str, v_prefix: &str) -> bool {
    text.split(',')
        .map(|g| is_zeilen_angabe_between_commas(g, v_prefix))
        .all(|x| x)
}

fn main() {
    let v_prefix = ""; // entspricht i18n.befehle2["v"] in Python
    let test1 = "1-3+2,4-6+1";
    let test2 = "1/2-3/4+5/6,7/8";

    println!(
        "is_zeilen_angabe('{}') = {}",
        test1,
        is_zeilen_angabe(test1, v_prefix)
    );
    println!(
        "is_zeilen_bruch_or_ganz_zahl_angabe('{}') = {}",
        test2,
        is_zeilen_bruch_or_ganz_zahl_angabe(test2, v_prefix)
    );
}
