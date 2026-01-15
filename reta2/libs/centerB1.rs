use std::collections::{BTreeSet, HashSet};

/// Parst Strings wie "(1,2,3)" oder "[1,2,3]" in eine Menge von Zahlen
fn str_as_generator_to_list_of_num_strs(text: &str) -> Option<HashSet<i32>> {
    let mut t = text.trim();
    if t.starts_with('(') && t.ends_with(')') {
        t = &format!("[{}]", &t[1..t.len()-1]);
    }

    if (t.starts_with('[') && t.ends_with(']')) || (t.starts_with('{') && t.ends_with('}')) {
        let inner = &t[1..t.len() - 1];
        let mut result = HashSet::new();
        for part in inner.split(',') {
            match part.trim().parse::<i32>() {
                Ok(num) => { result.insert(num); },
                Err(_) => { return None; }
            }
        }
        return Some(result);
    }
    None
}

/// Hilfsfunktion für "1-3+2" oder "-2+1" Bereich
fn bereich_to_numbers_einbereich(einbereich: &str, max_zahl: i32, vielfache: bool, menge: &mut BTreeSet<i32>) {
    if einbereich.is_empty() { return; }

    let mut parts = einbereich.split('-').collect::<Vec<_>>();
    let first = parts[0];
    let rest = if parts.len() > 1 { parts[1] } else { first };

    let mut start = first.parse::<i32>().unwrap_or(0);
    let mut end = rest.parse::<i32>().unwrap_or(start);

    let plus_parts = einbereich.split('+').skip(1).filter_map(|x| x.parse::<i32>().ok()).collect::<Vec<_>>();

    if vielfache {
        let mut i = 1;
        while start * i <= max_zahl {
            menge.insert(start * i);
            for p in &plus_parts {
                let val_plus = start * i + p;
                if val_plus <= max_zahl { menge.insert(val_plus); }
                let val_minus = start * i - p;
                if val_minus > 0 && val_minus <= max_zahl { menge.insert(val_minus); }
            }
            i += 1;
        }
    } else {
        for number in start..=end {
            menge.insert(number);
            for p in &plus_parts {
                let val_plus = number + p;
                if val_plus <= max_zahl { menge.insert(val_plus); }
                let val_minus = number - p;
                if val_minus > 0 && val_minus <= max_zahl { menge.insert(val_minus); }
            }
        }
    }
}

/// Komplettes `BereichToNumbers2` in Rust
fn bereich_to_numbers2(bereiche: &str, vielfache: bool, max_zahl: i32, allow_zero: bool) -> BTreeSet<i32> {
    let mut dazu: BTreeSet<i32> = BTreeSet::new();
    let mut hinfort: BTreeSet<i32> = BTreeSet::new();

    for einbereich in bereiche.split(',') {
        let einbereich = einbereich.trim();
        if einbereich.is_empty() { continue; }

        if einbereich.starts_with('-') {
            if let Some(gen) = str_as_generator_to_list_of_num_strs(&einbereich[1..]) {
                hinfort.extend(gen);
            } else {
                bereich_to_numbers_einbereich(&einbereich[1..], max_zahl, vielfache, &mut hinfort);
            }
        } else {
            if let Some(gen) = str_as_generator_to_list_of_num_strs(einbereich) {
                dazu.extend(gen);
            } else {
                bereich_to_numbers_einbereich(einbereich, max_zahl, vielfache, &mut dazu);
            }
        }
    }

    // Entferne ausgeschlossene Zahlen
    for val in hinfort {
        dazu.remove(&val);
    }

    // Filter nach allow_zero
    if !allow_zero {
        dazu = dazu.into_iter().filter(|&x| x > 0).collect();
    }

    dazu
}

/// Test / Beispiel
fn main() {
    let numbers = bereich_to_numbers2("1-3+2,-2", false, 100, false);
    println!("Parsed numbers: {:?}", numbers);

    let numbers2 = bereich_to_numbers2("2-4+1", true, 20, false);
    println!("Vielfache & Plus-Zusatz: {:?}", numbers2);
}
