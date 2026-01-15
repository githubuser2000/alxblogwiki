// Cargo.toml dependencies:
// [dependencies]
// regex = "1.10"
// itertools = "0.12"
// num = "0.4"
// ordered-float = "3.9"
// enum-as-inner = "0.6"
// serde = { version = "1.0", features = ["derive"] }
// anyhow = "1.0"
// thiserror = "1.0"
// once_cell = "1.19"
// textwrap = "0.16"
// pyphen = "0.10"

use std::collections::{BTreeSet, HashMap, HashSet};
use std::fmt::{self, Display, Formatter};
use std::ops::Range;
use std::sync::{Arc, Mutex};
use std::sync::OnceLock;

use anyhow::{Context, Result};
use enum_as_inner::EnumAsInner;
use itertools::Itertools;
use num::Integer;
use ordered_float::OrderedFloat;
use regex::Regex;
use serde::{Deserialize, Serialize};
use textwrap::{fill, WordSeparator, WordSplitter};
use pyphen::{Config, Dictionary};

// Import from previously created modules
mod center {
    pub use super::*;
    pub use crate::lib4tables::*;
    pub use crate::lib4tables_enum::*;
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
        pub befehle2: HashMap<String, String>,
    }
    
    impl I18n {
        pub fn new() -> Self {
            let mut befehle2 = HashMap::new();
            befehle2.insert("v".to_string(), "v".to_string());
            
            I18n {
                befehle2,
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WrapType {
    Pyphen = 1,
    Pyhyphen = 2,
    Nohyphen = 3,
}

pub struct GlobalState {
    shell_rows_amount: usize,
    h_de: Option<Dictionary>,
    dic: Option<Dictionary>,
    fill: Option<bool>,
    wrapping_type: WrapType,
}

impl GlobalState {
    pub fn new() -> Self {
        let shell_rows_amount = 24; // Default
        let wrapping_type = WrapType::Pyhyphen;
        
        // Initialize hyphenation dictionaries
        let h_de = Dictionary::new(Config::lang("de_DE")).ok();
        let dic = h_de.clone();
        
        GlobalState {
            shell_rows_amount,
            h_de,
            dic,
            fill: Some(true),
            wrapping_type,
        }
    }
    
    pub fn get_text_wrap_things(&self, max_len: Option<usize>) -> (usize, Option<Dictionary>, Option<Dictionary>, Option<bool>) {
        let width = max_len.unwrap_or(self.shell_rows_amount);
        (width, self.h_de.clone(), self.dic.clone(), self.fill)
    }
    
    pub fn set_shell_rows_amount(&mut self, shell_rows_amount2: Option<usize>) {
        if let Some(amount) = shell_rows_amount2 {
            self.shell_rows_amount = amount;
        }
    }
}

static GLOBAL_STATE: OnceLock<Mutex<GlobalState>> = OnceLock::new();

fn get_global_state() -> &'static Mutex<GlobalState> {
    GLOBAL_STATE.get_or_init(|| Mutex::new(GlobalState::new()))
}

pub fn chunks<T>(lst: &[T], n: usize) -> impl Iterator<Item = &[T]> {
    (0..lst.len()).step_by(n).map(move |i| {
        let end = std::cmp::min(i + n, lst.len());
        &lst[i..end]
    })
}

pub fn split_more_if_not_small(text_list: &[String], len_to_be: usize) -> Vec<String> {
    let mut new_list = Vec::new();
    let mut needed_to_be_done_at_all = false;
    
    for text in text_list {
        if text.len() > len_to_be {
            needed_to_be_done_at_all = true;
            break;
        }
    }
    
    if needed_to_be_done_at_all {
        for text in text_list {
            if text.len() > len_to_be {
                let chunks: Vec<String> = text
                    .chars()
                    .collect::<Vec<char>>()
                    .chunks(len_to_be)
                    .map(|chunk| chunk.iter().collect())
                    .collect();
                new_list.extend(chunks);
            } else {
                new_list.push(text.clone());
            }
        }
    } else {
        new_list = text_list.to_vec();
    }
    
    new_list
}

pub fn alxwrap(text: &str, len: usize) -> Option<Vec<String>> {
    let state = get_global_state().lock().unwrap();
    
    if len == 0 {
        return Some(vec![text.to_string()]);
    }
    
    match state.wrapping_type {
        WrapType::Pyphen => {
            if let Some(ref dic) = state.dic {
                let hyphenated = dic.hyphenate(text, "-");
                Some(vec![hyphenated.to_string()])
            } else {
                Some(vec![text.to_string()])
            }
        }
        WrapType::Pyhyphen => {
            let wrapped = fill(text, len);
            let lines: Vec<String> = wrapped.lines().map(|s| s.to_string()).collect();
            Some(split_more_if_not_small(&lines, len))
        }
        WrapType::Nohyphen => {
            Some(vec![text.to_string()])
        }
    }
}

pub struct Prepare<'a> {
    tables: &'a Tables,
    hoechste_zeile: HashMap<i32, i32>,
    original_lines_range: Range<i32>,
    shell_rows_amount: usize,
    zaehlungen: (i32, HashMap<i32, i32>, HashMap<i32, i32>, HashMap<i32, i32>, HashMap<i32, (Vec<i32>, Vec<i32>)>),
    religion_numbers: Vec<i32>,
    gezaehlt: bool,
    if_zeilen_setted: bool,
    breiten: Vec<usize>,
    nummerierung: bool,
    textwidth: usize,
    rows_as_numbers: BTreeSet<i32>,
    headings_amount: usize,
    certaintextwidth: usize,
}

impl<'a> Prepare<'a> {
    pub fn new(tables: &'a Tables) -> Self {
        let state = get_global_state().lock().unwrap();
        let shell_rows_amount = state.shell_rows_amount;
        
        let hoechste_zeile = tables.hoechste_zeile.clone();
        let highest_line = hoechste_zeile.get(&1024).cloned().unwrap_or(0) + 4;
        
        Prepare {
            tables,
            hoechste_zeile: hoechste_zeile.clone(),
            original_lines_range: 0..highest_line,
            shell_rows_amount,
            zaehlungen: (0, HashMap::new(), HashMap::new(), HashMap::new(), HashMap::new()),
            religion_numbers: Vec::new(),
            gezaehlt: false,
            if_zeilen_setted: false,
            breiten: Vec::new(),
            nummerierung: false,
            textwidth: 80,
            rows_as_numbers: BTreeSet::new(),
            headings_amount: 0,
            certaintextwidth: 0,
        }
    }
    
    pub fn set_zaehlungen(&mut self, num: i32) {
        if self.gezaehlt {
            return;
        }
        
        self.gezaehlt = true;
        let num = self.original_lines_range.end;
        
        let mut was_moon = true;
        let mut is_moon = if self.zaehlungen.0 == 0 {
            true
        } else {
            !moon_number(self.zaehlungen.0).0.is_empty()
        };
        
        for i in (self.zaehlungen.0 + 1)..=num {
            was_moon = is_moon;
            let moon_type = moon_number(i);
            is_moon = !moon_type.0.is_empty();
            
            if was_moon && !is_moon {
                is_moon = false;
                let zaehlung_key = self.zaehlungen.1.len() as i32 + 1;
                self.zaehlungen.1.insert(zaehlung_key, i);
                self.zaehlungen.2.insert(i, self.zaehlungen.2.len() as i32 + 1);
            }
            
            self.zaehlungen.3.insert(i, self.zaehlungen.2.len() as i32);
            self.zaehlungen.4.insert(i, moon_type);
        }
        
        self.zaehlungen.0 = num;
    }
    
    pub fn set_breiten(&mut self, value: Vec<usize>) {
        self.breiten = value;
    }
    
    pub fn get_breiten(&self) -> &Vec<usize> {
        &self.breiten
    }
    
    pub fn set_nummerierung(&mut self, value: bool) {
        self.nummerierung = value;
    }
    
    pub fn get_nummerierung(&self) -> bool {
        self.nummerierung
    }
    
    pub fn set_text_width(&mut self, value: usize) {
        self.textwidth = value;
    }
    
    pub fn get_text_width(&self) -> usize {
        self.textwidth
    }
    
    pub fn wrapping(&self, text: &str, length: usize) -> Option<Vec<String>> {
        if text.len() > length && length != 0 {
            alxwrap(text, length)
        } else {
            None
        }
    }
    
    pub fn set_width(&self, row_to_display: usize, combi_rows1: usize) -> usize {
        if self.shell_rows_amount == 0 {
            return 0;
        }
        
        let combi_rows = if combi_rows1 != 0 { combi_rows1 } else { self.rows_as_numbers.len() };
        
        let breiten = if self.rows_as_numbers.len() - combi_rows < self.breiten.len() {
            &self.breiten[self.rows_as_numbers.len() - combi_rows..]
        } else {
            &[]
        };
        
        let delta = -1_i32;
        let adjusted_row = row_to_display as i32 + delta;
        
        if adjusted_row >= 0 && adjusted_row < breiten.len() as i32 {
            breiten[adjusted_row as usize]
        } else {
            self.textwidth
        }
    }
    
    pub fn parameters_cmd_with_some_bereich(
        &self,
        mehrere_bereiche: &str,
        symbol: &str,
        neg: &str,
        keine_neg_beruecksichtigung: bool,
    ) -> HashSet<String> {
        let mut results = HashSet::new();
        
        if keine_neg_beruecksichtigung {
            if is_zeilen_angabe(mehrere_bereiche) {
                results.insert(format!("_{}_{}", symbol, mehrere_bereiche));
            }
        } else {
            for ein_bereich in mehrere_bereiche.split(',') {
                if ((neg.is_empty() && !ein_bereich.is_empty() && !ein_bereich.starts_with('-'))
                    || (ein_bereich.starts_with(neg) && !neg.is_empty()))
                    && !ein_bereich.is_empty()
                {
                    let cleaned = if ein_bereich.starts_with(neg) {
                        &ein_bereich[neg.len()..]
                    } else {
                        ein_bereich
                    };
                    
                    if is_zeilen_angabe(cleaned) {
                        results.insert(format!("_{}_{}", symbol, cleaned));
                    }
                }
            }
        }
        
        results
    }
    
    pub fn delete_doubles_in_sets<T: Eq + std::hash::Hash + Clone>(
        &self,
        set1: &HashSet<T>,
        set2: &HashSet<T>,
    ) -> (HashSet<T>, HashSet<T>) {
        let intersection: HashSet<T> = set1.intersection(set2).cloned().collect();
        (
            set1.difference(&intersection).cloned().collect(),
            set2.difference(&intersection).cloned().collect(),
        )
    }
    
    pub fn from_until(&self, a: &[String]) -> (i32, i32) {
        if a.is_empty() {
            return (1, 1);
        }
        
        if let Ok(first) = a[0].parse::<i32>() {
            if a.len() == 2 {
                if let Ok(second) = a[1].parse::<i32>() {
                    (first, second)
                } else {
                    (1, first)
                }
            } else {
                (1, first)
            }
        } else {
            (1, 1)
        }
    }
    
    pub fn zeile_which_zaehlung(&self, zeile: i32) -> Option<i32> {
        self.zaehlungen.3.get(&zeile).cloned()
    }
    
    pub fn moonsun(
        &mut self,
        moon_not_sun: bool,
        num_range_yes_z: &mut HashSet<i32>,
        num_range: &HashSet<i32>,
        if_zaehlungen_at_all: bool,
    ) -> &mut HashSet<i32> {
        if !if_zaehlungen_at_all {
            self.set_zaehlungen(self.original_lines_range.end);
        }
        
        for &n in num_range {
            let is_moon = !self.zaehlungen.4.get(&n).unwrap_or(&(Vec::new(), Vec::new())).0.is_empty();
            if is_moon == moon_not_sun {
                num_range_yes_z.insert(n);
            }
        }
        
        num_range_yes_z
    }
    
    pub fn filter_original_lines(
        &mut self,
        num_range: HashSet<i32>,
        param_lines: &HashSet<String>,
    ) -> HashSet<i32> {
        let mut num_range = num_range;
        num_range.remove(&0);
        
        fn cutset(wether: bool, a: HashSet<i32>, b: HashSet<i32>) -> HashSet<i32> {
            if wether {
                a.intersection(&b).cloned().collect()
            } else {
                a
            }
        }
        
        let highest_1024 = self.hoechste_zeile.get(&1024).cloned().unwrap_or(0);
        let highest_114 = self.hoechste_zeile.get(&114).cloned().unwrap_or(0);
        
        if param_lines.contains("all")
            || param_lines.iter().all(|s| s == "ka" || s == "ka2")
            || !self.if_zeilen_setted
        {
            num_range = (1..=highest_1024).collect();
        } else {
            num_range.clear();
        }
        
        // Process _a_ parameters
        let mut if_a_at_all = false;
        let mut mehrere = Vec::new();
        let mut if_teiler = false;
        
        for condition in param_lines {
            if condition.starts_with("_a_") && condition.len() > 3 {
                if_a_at_all = true;
                mehrere.push(condition[3..].to_string());
            }
            if condition.starts_with("_w_") {
                if_teiler = true;
            }
        }
        
        if if_a_at_all {
            let bereich_numbers = bereich_to_numbers2(&mehrere.join(","), false, highest_1024 + 1);
            num_range.extend(bereich_numbers);
            
            if if_teiler {
                // Note: teiler function needs to be implemented
                let teiler_numbers = teiler(&num_range.iter().map(|n| n.to_string()).collect::<Vec<_>>().join(","));
                num_range.extend(teiler_numbers);
            }
            
            if !num_range.is_empty() {
                for eins in mehrere.iter() {
                    let ja1 = eins.starts_with('-');
                    let ja2 = eins.starts_with(&format!("{}-", i18n::INSTANCE.befehle2["v"]));
                    
                    if ja1 || ja2 {
                        let cleaned = if ja1 {
                            &eins[1..]
                        } else {
                            &eins[i18n::INSTANCE.befehle2["v"].len() + 1..]
                        };
                        
                        let minus_numbers = bereich_to_numbers2(cleaned, false, highest_1024 + 1);
                        num_range = num_range.difference(&minus_numbers).cloned().collect();
                    }
                }
            }
        }
        
        // Process _b_ parameters
        let mut if_b_at_all = false;
        let mut mehrere = Vec::new();
        let mut num_range_yes_z = HashSet::new();
        
        for condition in param_lines {
            if condition.starts_with("_b_") && condition.len() > 3 {
                if_b_at_all = true;
                mehrere.push(condition[3..].to_string());
            }
        }
        
        if if_b_at_all {
            if num_range.is_empty() && !if_a_at_all && !param_lines.contains("all") {
                num_range = (1..=highest_114).collect();
            }
            
            num_range_yes_z.extend(bereich_to_numbers2(&mehrere.join(","), true, highest_114 + 1));
            
            if !num_range_yes_z.is_empty() {
                num_range = num_range.intersection(&num_range_yes_z).cloned().collect();
            }
            
            if !num_range.is_empty() {
                for eins in mehrere.iter() {
                    let ja1 = eins.starts_with('-');
                    let ja2 = eins.starts_with(&format!("{}-", i18n::INSTANCE.befehle2["v"]));
                    
                    if ja1 || ja2 {
                        let cleaned = if ja1 {
                            &eins[1..]
                        } else {
                            &eins[i18n::INSTANCE.befehle2["v"].len() + 1..]
                        };
                        
                        let minus_numbers = bereich_to_numbers2(cleaned, true, highest_1024 + 1);
                        num_range = num_range.difference(&minus_numbers).cloned().collect();
                    }
                }
            }
        }
        
        // Process time parameters (=, <, >)
        let mut if_zeit_at_all = false;
        let mut num_range_yes_z = HashSet::new();
        
        for condition in param_lines {
            match condition.as_str() {
                "=" => {
                    if_zeit_at_all = true;
                    num_range_yes_z.insert(10);
                }
                "<" => {
                    if_zeit_at_all = true;
                    num_range_yes_z.extend(1..10);
                }
                ">" => {
                    if_zeit_at_all = true;
                    num_range_yes_z.extend(11..=highest_1024);
                }
                _ => {}
            }
        }
        
        if if_zeit_at_all {
            if num_range.is_empty() && !if_b_at_all && !if_a_at_all && !param_lines.contains("all") && num_range_yes_z.is_empty() {
                num_range = (1..=highest_1024).collect();
            }
            
            if if_a_at_all || param_lines.contains("all") || if_b_at_all {
                num_range = num_range.intersection(&num_range_yes_z).cloned().collect();
            } else {
                num_range.extend(num_range_yes_z);
            }
        }
        
        // Process _n_ parameters
        let mut if_zaehlungen_at_all = false;
        let mut mehrere = Vec::new();
        let mut num_range_yes_z = HashSet::new();
        
        for condition in param_lines {
            if condition.starts_with("_n_") && condition.len() > 3 {
                num_range_yes_z.extend(bereich_to_numbers2(&condition[3..], false, highest_1024 + 1));
                if_zaehlungen_at_all = true;
                mehrere.push(condition[3..].to_string());
            }
        }
        
        if if_zaehlungen_at_all {
            self.set_zaehlungen(self.original_lines_range.end);
            
            if num_range.is_empty() && !if_a_at_all && !if_b_at_all && !param_lines.contains("all") {
                num_range = (1..=highest_1024).collect();
            }
            
            let mut num_range_yes_z2 = HashSet::new();
            for &n in &num_range {
                for &z in &num_range_yes_z {
                    if let Some(zaehlung) = self.zaehlungen.3.get(&n) {
                        if *zaehlung == z {
                            num_range_yes_z2.insert(n);
                        }
                    }
                }
            }
            
            if if_zaehlungen_at_all {
                if !num_range_yes_z2.is_empty() && !num_range.is_empty() {
                    num_range = num_range.intersection(&num_range_yes_z2).cloned().collect();
                } else if num_range.is_empty() {
                    num_range = num_range_yes_z2;
                }
            }
            
            if !num_range.is_empty() {
                let mut minus_bereiche = HashSet::new();
                for eins in mehrere.iter() {
                    let ja1 = eins.starts_with('-');
                    let ja2 = eins.starts_with(&format!("{}-", i18n::INSTANCE.befehle2["v"]));
                    
                    if ja1 || ja2 {
                        let cleaned = if ja1 {
                            &eins[1..]
                        } else {
                            &eins[i18n::INSTANCE.befehle2["v"].len() + 1..]
                        };
                        
                        minus_bereiche.extend(bereich_to_numbers2(cleaned, false, highest_1024 + 1));
                    }
                }
                
                if !minus_bereiche.is_empty() {
                    num_range = num_range.difference(&minus_bereiche).cloned().collect();
                }
            }
        }
        
        // Process type parameters (aussenerste, innenerste, etc.)
        let type_params: HashSet<&str> = ["aussenerste", "innenerste", "aussenalle", "innenalle"]
            .iter()
            .cloned()
            .collect();
        
        if param_lines.iter().any(|s| type_params.contains(s.as_str())) {
            let mut prim_list = HashMap::new();
            let mut innen_aussen = HashMap::new();
            
            innen_aussen.insert(1, (true, false, true));
            
            let num_range_b: HashSet<i32> = num_range.difference(&[1, 2, 3].iter().cloned().collect()).cloned().collect();
            
            for &n in &num_range_b {
                prim_list.insert(n, prim_fak(n));
            }
            
            for (&anfangs_zahl, prim_zahlen) in &prim_list {
                let nur_eine_zahl = prim_zahlen.len() == 1;
                let ein_fach_vorkommen = nur_eine_zahl;
                let (mut innen, mut aussen) = (false, false);
                
                for &prim_zahl in prim_zahlen {
                    if !(4..).contains(&prim_zahl) {
                        let innen_or_aussen = prim_zahl % 6;
                        innen = innen || innen_or_aussen == 1;
                        aussen = aussen || innen_or_aussen == 5;
                    }
                }
                
                innen_aussen.insert(anfangs_zahl, (innen, aussen, ein_fach_vorkommen));
            }
            
            let mut num_range_yes_z = HashSet::new();
            
            if param_lines.contains("aussenerste") {
                for (&anfangs_zahl, &(innen, aussen, ein_fach)) in &innen_aussen {
                    if aussen && ein_fach {
                        num_range_yes_z.insert(anfangs_zahl);
                    }
                }
            }
            
            if param_lines.contains("innenerste") {
                for (&anfangs_zahl, &(innen, aussen, ein_fach)) in &innen_aussen {
                    if innen && ein_fach {
                        num_range_yes_z.insert(anfangs_zahl);
                    }
                }
            }
            
            if param_lines.contains("aussenalle") {
                for (&anfangs_zahl, &(innen, aussen, _)) in &innen_aussen {
                    if aussen {
                        num_range_yes_z.insert(anfangs_zahl);
                    }
                }
            }
            
            if param_lines.contains("innenalle") {
                for (&anfangs_zahl, &(innen, aussen, _)) in &innen_aussen {
                    if innen {
                        num_range_yes_z.insert(anfangs_zahl);
                    }
                }
            }
            
            let if_typ_at_all = !num_range_yes_z.is_empty();
            num_range = cutset(if_typ_at_all, num_range, num_range_yes_z);
        }
        
        // Process celestial parameters (mond, sonne, etc.)
        let mut if_typ_at_all = false;
        let mut num_range_yes_z = HashSet::new();
        
        if num_range.is_empty() && param_lines.iter().any(|s| s != "ka" && s != "ka2") {
            num_range = (1..=highest_1024).collect();
        }
        
        for condition in param_lines {
            match condition.as_str() {
                "mond" => {
                    self.moonsun(true, &mut num_range_yes_z, &num_range, if_zaehlungen_at_all);
                    if_typ_at_all = true;
                }
                "schwarzesonne" => {
                    if_typ_at_all = true;
                    for &n in &num_range {
                        if n % 3 == 0 {
                            num_range_yes_z.insert(n);
                        }
                    }
                }
                "sonne" => {
                    self.moonsun(false, &mut num_range_yes_z, &num_range, if_zaehlungen_at_all);
                    if_typ_at_all = true;
                }
                "planet" => {
                    if_typ_at_all = true;
                    for &n in &num_range {
                        if n % 2 == 0 {
                            num_range_yes_z.insert(n);
                        }
                    }
                }
                "SonneMitMondanteil" => {
                    if_typ_at_all = true;
                    for &n in &num_range {
                        let booleans: HashSet<bool> = prim_repeat2(&prim_fak(n))
                            .iter()
                            .map(|&(_, factor)| factor == 1)
                            .collect();
                        
                        if booleans.contains(&true) && booleans.contains(&false) {
                            num_range_yes_z.insert(n);
                        }
                    }
                }
                _ => {}
            }
        }
        
        num_range = cutset(if_typ_at_all, num_range, num_range_yes_z);
        
        // Process prime multiple parameters (xp)
        let mut if_prim_at_all = false;
        let mut prim_multiples = Vec::new();
        
        for condition in param_lines {
            if condition.len() > 1 && condition.ends_with('p') {
                if let Ok(num) = condition[..condition.len()-1].parse::<i32>() {
                    if_prim_at_all = true;
                    prim_multiples.push(num);
                }
            }
        }
        
        if if_prim_at_all {
            if num_range.is_empty() && !if_b_at_all && !if_a_at_all && !param_lines.contains("all") && !if_typ_at_all {
                num_range = (1..=highest_1024).collect();
            }
            
            let mut num_range_yes_z = HashSet::new();
            for &n in &num_range {
                if is_prim_multiple(n, &prim_multiples) {
                    num_range_yes_z.insert(n);
                }
            }
            
            num_range = cutset(if_prim_at_all, num_range, num_range_yes_z);
        }
        
        // Process power parameters (_^_)
        let mut if_power_at_all = false;
        let mut mehrere = Vec::new();
        
        for condition in param_lines {
            if condition.starts_with("_^_") && condition.len() > 3 {
                if_power_at_all = true;
                mehrere.push(condition[3..].to_string());
            }
        }
        
        let to_power_it: Vec<i32> = bereich_to_numbers2(&mehrere.join(",")).into_iter().collect();
        
        if if_power_at_all {
            let mut num_range_yes_z = HashSet::new();
            
            if num_range.is_empty() && param_lines.iter().any(|s| s != "ka" && s != "ka2") {
                num_range = (1..=highest_1024).collect();
            }
            
            if !num_range.is_empty() {
                let last_el = *num_range.iter().max().unwrap_or(&0);
                
                for &base in &to_power_it {
                    let mut n = 0;
                    loop {
                        let one_power = base.pow(n as u32);
                        if one_power <= last_el {
                            num_range_yes_z.insert(one_power);
                            n += 1;
                        } else {
                            break;
                        }
                    }
                }
                
                num_range = cutset(if_power_at_all, num_range, num_range_yes_z);
                num_range.remove(&1);
            }
        }
        
        // Process multiple parameters (xv)
        let mut if_multiples_from_any_at_all = false;
        let mut any_multiples = Vec::new();
        
        for condition in param_lines {
            if condition.len() > 1 && condition.ends_with('v') {
                if let Ok(num) = condition[..condition.len()-1].parse::<i32>() {
                    if_multiples_from_any_at_all = true;
                    any_multiples.push(num);
                }
            }
        }
        
        if if_multiples_from_any_at_all {
            let mut num_range_yes_z = HashSet::new();
            for &n in &num_range {
                for &divisor in &any_multiples {
                    if n % divisor == 0 {
                        num_range_yes_z.insert(n);
                        break;
                    }
                }
            }
            
            num_range = cutset(if_multiples_from_any_at_all, num_range, num_range_yes_z);
        }
        
        // Remove sun numbers above 114
        let highest_114 = self.hoechste_zeile.get(&114).cloned().unwrap_or(0);
        num_range.retain(|&n| {
            if n == 0 {
                return true;
            }
            
            let is_sun = self.zaehlungen.4.get(&n).map_or(true, |(bases, _)| bases.is_empty());
            !(is_sun && n > highest_114)
        });
        
        // Process inversion parameter (_i_)
        let mut invertieren = false;
        for condition in param_lines {
            if condition.starts_with("_i_") {
                invertieren = true;
                break;
            }
        }
        
        if invertieren {
            let mut num_range_list: Vec<i32> = num_range.into_iter().collect();
            num_range_list.sort();
            
            let h = highest_1024;
            let mut num_range2_set = HashSet::new();
            
            for i in 1..=h {
                let prev_in = num_range_list.contains(&(i-1));
                let next_in = num_range_list.contains(&(i+1));
                let current_in = num_range_list.contains(&i);
                
                if (prev_in || next_in) && !current_in {
                    num_range2_set.insert(i);
                }
            }
            
            num_range = num_range2_set;
        }
        
        // Process _z_ parameter
        let num_range_list: Vec<i32> = num_range.iter().cloned().collect();
        let num_range2_map: HashMap<usize, i32> = num_range_list.iter().enumerate()
            .map(|(i, &a)| (i + 1, a))
            .collect();
        
        let mut z_ja = false;
        let mut num_range_neu2 = HashSet::new();
        
        for condition in param_lines {
            if condition.starts_with("_z_") && condition.len() > 3 {
                z_ja = true;
                let num_range_keys: HashSet<usize> = num_range2_map.keys().cloned().collect();
                let num_range_neu = bereich_to_numbers2(&condition[3..], false, highest_1024 + 1)
                    .into_iter()
                    .filter(|&n| num_range_keys.contains(&(n as usize)))
                    .collect::<HashSet<i32>>();
                
                for a in num_range_neu {
                    if let Some(&value) = num_range2_map.get(&(a as usize)) {
                        num_range_neu2.insert(value);
                    }
                }
            }
        }
        
        if z_ja {
            num_range = num_range.intersection(&num_range_neu2).cloned().collect();
        }
        
        // Process _y_ parameter
        let mut y_ja = false;
        let mut num_range_neu2 = HashSet::new();
        
        for condition in param_lines {
            if condition.starts_with("_y_") && condition.len() > 3 {
                y_ja = true;
                let num_range_keys: HashSet<usize> = num_range2_map.keys().cloned().collect();
                let num_range_neu = bereich_to_numbers2(&condition[3..], true, highest_1024 + 1)
                    .into_iter()
                    .filter(|&n| num_range_keys.contains(&(n as usize)))
                    .collect::<HashSet<i32>>();
                
                for a in num_range_neu {
                    if let Some(&value) = num_range2_map.get(&(a as usize)) {
                        num_range_neu2.insert(value);
                    }
                }
            }
        }
        
        if y_ja {
            num_range = num_range.intersection(&num_range_neu2).cloned().collect();
        }
        
        num_range
    }
    
    pub fn prepare4out(
        &mut self,
        param_lines: &HashSet<String>,
        param_lines_not: &HashSet<String>,
        content_table: &[Vec<String>],
        rows_as_numbers: &BTreeSet<i32>,
        gebr_spalten: &HashMap<String, BTreeSet<i32>>,
        combi_rows: usize,
        reli_table_len_until_now: Option<usize>,
        prim_spalten: Option<&BTreeSet<i32>>,
        kombi_csv_number: i32,
    ) -> Result<(HashSet<i32>, Vec<Vec<Vec<String>>>, usize, Range<usize>, (HashMap<i32, i32>, HashMap<i32, i32>))> {
        let (finally_display_lines, headings_amount, newer_table, numlen, rows_range) =
            self.prepare4out_before_for_loop_spalten_zeilen_bestimmen(
                content_table, param_lines, param_lines_not
            )?;
        
        self.headings_amount = headings_amount;
        let mut old2_rows = (HashMap::new(), HashMap::new());
        let reli_numbers_bool = self.religion_numbers.is_empty();
        
        for (u, line) in content_table.iter().enumerate() {
            if finally_display_lines.contains(&(u as i32)) || combi_rows != 0 {
                let new2_lines = self.prepare4out_loop_body(
                    combi_rows,
                    gebr_spalten,
                    headings_amount,
                    line,
                    &mut old2_rows,
                    prim_spalten,
                    reli_numbers_bool,
                    reli_table_len_until_now,
                    rows_as_numbers,
                    u as i32,
                    kombi_csv_number,
                )?;
                
                if !new2_lines.is_empty() {
                    newer_table.push(new2_lines);
                }
            }
        }
        
        Ok((finally_display_lines, newer_table, numlen, rows_range, old2_rows))
    }
    
    fn prepare4out_before_for_loop_spalten
