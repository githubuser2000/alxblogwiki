// Cargo.toml dependencies:
// [dependencies]
// regex = "1.10"
// itertools = "0.12"
// colored = "2.1"
// serde = { version = "1.0", features = ["derive"] }
// serde_json = "1.0"
// lazy_static = "1.4"
// num = "0.4"
// anyhow = "1.0"
// thiserror = "1.0"
// console = "0.15"
// comfy-table = "7.1"

use std::collections::{BTreeSet, HashMap, HashSet, LinkedList};
use std::env;
use std::fmt::{self, Display, Formatter};
use std::fs;
use std::io::{self, Write};
use std::path::Path;
use std::sync::OnceLock;

use anyhow::{Context, Result};
use colored::*;
use itertools::Itertools;
use num::Integer;
use regex::Regex;
use serde::{Deserialize, Serialize};
use thiserror::Error;

// Configuration and internationalization
mod i18n {
    use super::*;
    
    pub struct I18n {
        pub befehle2: HashMap<String, String>,
        pub read_me_file_names: HashMap<String, String>,
        pub primzahlkreuz_pro_contra_strs_dict: HashMap<(String, String), String>,
        pub multiplikationen: Vec<String>,
        pub main_para_cmds: HashMap<String, String>,
    }
    
    impl I18n {
        pub fn new() -> Self {
            let mut befehle2 = HashMap::new();
            befehle2.insert("v".to_string(), "v".to_string());
            
            let mut read_me_file_names = HashMap::new();
            read_me_file_names.insert("reta_prompt".to_string(), "README_prompt.md".to_string());
            read_me_file_names.insert("reta".to_string(), "README.md".to_string());
            
            let mut primzahlkreuz_pro_contra_strs_dict = HashMap::new();
            primzahlkreuz_pro_contra_strs_dict.insert(
                ("Primzahlkreuz_pro_contra".to_string(), 
                 "nachvollziehen_emotional_oder_geistig_durch_Primzahl-Kreuz-Algorithmus_(15)".to_string()),
                "Some string here".to_string()
            );
            
            I18n {
                befehle2,
                read_me_file_names,
                primzahlkreuz_pro_contra_strs_dict,
                multiplikationen: vec![],
                main_para_cmds: {
                    let mut map = HashMap::new();
                    map.insert("debug".to_string(), "debug".to_string());
                    map
                },
            }
        }
        
        pub fn classify(&self, n: i32) -> &'static str {
            match n {
                0 => "zero",
                1 => "one",
                _ => "other",
            }
        }
    }
    
    lazy_static::lazy_static! {
        pub static ref INSTANCE: I18n = I18n::new();
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum NPmEnum {
    GalN = 2,
    Gal1pN = 3,
    UniN = 4,
    Uni1pN = 5,
    EmoN = 6,
    Emo1pN = 7,
    GroeN = 8,
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
}

#[derive(Error, Debug)]
pub enum PatternError {
    #[error("Invalid pattern")]
    InvalidPattern,
    #[error("Parse error: {0}")]
    ParseError(String),
}

pub struct PatternValidator {
    kpattern: Regex,
    zeilen_pattern: Regex,
    bruch_pattern: Regex,
}

impl PatternValidator {
    pub fn new() -> Result<Self> {
        let kpattern = Regex::new(r",(?![^]*[]})])")?;
        let zeilen_pattern_str = format!("^({}?-?\\d+)(-\\d+)?((\\+)(\\d+))*$", i18n::INSTANCE.befehle2["v"]);
        let zeilen_pattern = Regex::new(&zeilen_pattern_str)?;
        
        let bruch_pattern_str = format!("^({}?-?\\d+/\\d+)(-\\d+/\\d+)?((\\+)(\\d+/\\d+))*$", i18n::INSTANCE.befehle2["v"]);
        let bruch_pattern = Regex::new(&bruch_pattern_str)?;
        
        Ok(Self {
            kpattern,
            zeilen_pattern,
            bruch_pattern,
        })
    }
    
    pub fn is_zeilen_angabe(&self, text: &str) -> bool {
        let split_text: Vec<&str> = self.kpattern.split(text).collect();
        let any_at_all = split_text.iter().any(|s| !s.is_empty());
        
        split_text.iter().all(|g| {
            if g.is_empty() {
                any_at_all
            } else {
                self.is_zeilen_angabe_between_kommas(g)
            }
        })
    }
    
    pub fn is_zeilen_angabe_between_kommas(&self, g: &str) -> bool {
        if self.zeilen_pattern.is_match(g) {
            return true;
        }
        
        self.str_as_generator_to_list_of_num_strs(g).is_some() ||
        self.str_as_generator_to_list_of_num_strs(&g[1..]).is_some()
    }
    
    pub fn is_zeilen_bruch_angabe(&self, text: &str) -> bool {
        let split_text: Vec<&str> = text.split(',').collect();
        let any_at_all = split_text.iter().any(|s| !s.is_empty());
        
        split_text.iter().all(|g| {
            if g.is_empty() {
                any_at_all
            } else {
                self.is_zeilen_bruch_angabe_between_kommas(g)
            }
        })
    }
    
    pub fn is_zeilen_bruch_angabe_between_kommas(&self, g: &str) -> bool {
        self.bruch_pattern.is_match(g)
    }
    
    pub fn is_zeilen_bruch_or_ganz_zahl_angabe(&self, text: &str) -> bool {
        text.split(',')
            .all(|g| self.is_zeilen_bruch_angabe_between_kommas(g) || 
                     self.is_zeilen_angabe_between_kommas(g))
    }
    
    fn str_as_generator_to_list_of_num_strs(&self, text: &str) -> Option<HashSet<i32>> {
        if text.len() < 2 {
            return None;
        }
        
        let cleaned = if text.starts_with('(') && text.ends_with(')') {
            format!("[{}]", &text[1..text.len()-1])
        } else {
            text.to_string()
        };
        
        if (cleaned.starts_with('[') && cleaned.ends_with(']')) ||
           (cleaned.starts_with('{') && cleaned.ends_with('}')) {
            let inner = &cleaned[1..cleaned.len()-1];
            let numbers: Result<HashSet<i32>, _> = inner.split(',')
                .filter(|s| !s.trim().is_empty())
                .map(|s| s.trim().parse::<i32>())
                .collect();
            
            numbers.ok()
        } else {
            None
        }
    }
}

pub struct ConsoleHelper {
    pub shell_rows_amount: usize,
    pub shell_width: usize,
}

impl ConsoleHelper {
    pub fn new() -> Self {
        let shell_width = match terminal_size::terminal_size() {
            Some((terminal_size::Width(w), _)) => w as usize,
            None => 80,
        };
        
        let shell_rows_amount = match terminal_size::terminal_size() {
            Some((_, terminal_size::Height(h))) => h as usize,
            None => 24,
        };
        
        Self {
            shell_rows_amount,
            shell_width,
        }
    }
    
    pub fn get_text_wrap_things(&self, max_len: Option<usize>) -> (usize, usize) {
        let width = max_len.unwrap_or(self.shell_width);
        (width, self.shell_rows_amount)
    }
}

pub struct AppState {
    pub info_log: bool,
    pub output: bool,
    pub original_lines_range: std::ops::Range<i32>,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            info_log: false,
            output: true,
            original_lines_range: 0..1028,
        }
    }
}

pub fn reta_prompt_hilfe() -> Result<()> {
    let read_me = &i18n::INSTANCE.read_me_file_names["reta_prompt"];
    let place = Path::new(".")
        .join("doc")
        .join(read_me);
    
    let markdown_text = fs::read_to_string(&place)
        .with_context(|| format!("Failed to read {}", place.display()))?;
    
    if let Some(ab_da) = markdown_text.find("+++") {
        let cleaned_text = &markdown_text[ab_da + 3..];
        println!("{}", cleaned_text);
    } else {
        println!("{}", markdown_text);
    }
    
    Ok(())
}

pub fn reta_hilfe() -> Result<()> {
    let read_me = &i18n::INSTANCE.read_me_file_names["reta"];
    let place = Path::new(".")
        .join("doc")
        .join(read_me);
    
    let markdown_text = fs::read_to_string(&place)
        .with_context(|| format!("Failed to read {}", place.display()))?;
    
    println!("{}", markdown_text);
    Ok(())
}

pub fn x<T: Display>(state: &AppState, text1: &str, text: T) {
    if state.info_log && state.output {
        println!("{}: {}", text1, text);
    }
}

pub fn alxp<T: Display>(state: &AppState, text: T) {
    if state.info_log && state.output {
        println!("{}", text);
    }
}

pub fn chunks<T>(lst: &[T], n: usize) -> impl Iterator<Item = &[T]> {
    (0..lst.len()).step_by(n).map(move |i| {
        let end = std::cmp::min(i + n, lst.len());
        &lst[i..end]
    })
}

pub fn cliout(text: &str, color: bool, stype: &str) {
    if color && !text.is_empty() {
        let text = text.split_whitespace().collect::<Vec<&str>>().join(" ");
        if stype == "html" {
            let text = text.replace("<tr", "\n <tr").replace("<td", "\n <td");
            println!("{}", text.green());
        } else {
            println!("{}", text.blue());
        }
    } else {
        println!("{}", text);
    }
}

pub fn unique_everseen<I, F, K>(iterable: I, key: F) -> impl Iterator<Item = I::Item>
where
    I: IntoIterator,
    F: Fn(&I::Item) -> K,
    K: Eq + std::hash::Hash,
{
    let mut seen = HashSet::new();
    iterable.into_iter().filter(move |item| seen.insert(key(item)))
}

pub struct RangeConverter {
    pattern_validator: PatternValidator,
}

impl RangeConverter {
    pub fn new() -> Result<Self> {
        let pattern_validator = PatternValidator::new()?;
        Ok(Self { pattern_validator })
    }
    
    pub fn bereich_to_numbers2(
        &self,
        mehrere_bereiche: &str,
        vielfache: bool,
        max_zahl: i32,
        allow_less_eq_zero: bool,
    ) -> Result<BTreeSet<i32>> {
        // Clean the input string
        let cleaned = self.clean_range_string(mehrere_bereiche)?;
        
        if !self.pattern_validator.is_zeilen_angabe(&cleaned) {
            return Ok(BTreeSet::new());
        }
        
        let max_limit = if !vielfache && max_zahl == 0 {
            i32::MAX
        } else {
            max_zahl
        };
        
        let bereiche: Vec<&str> = self.split_range_string(&cleaned);
        let mut dazu: BTreeSet<i32> = BTreeSet::new();
        let mut hinfort: BTreeSet<i32> = BTreeSet::new();
        
        for ein_bereich in bereiche {
            if let Some(numbers) = self.process_special_range(ein_bereich, &mut dazu, &mut hinfort) {
                continue;
            }
            
            let (range_str, is_vielfache) = self.extract_range_info(ein_bereich, vielfache);
            self.process_range(range_str, is_vielfache, max_limit, &mut dazu, &mut hinfort);
        }
        
        let result: BTreeSet<i32> = dazu.difference(&hinfort).cloned().collect();
        
        if allow_less_eq_zero {
            Ok(result)
        } else {
            Ok(result.into_iter().filter(|&x| x > 0).collect())
        }
    }
    
    fn clean_range_string(&self, input: &str) -> Result<String> {
        let parts: Vec<&str> = input.split(',').collect();
        let cleaned_parts: Vec<&str> = parts.iter()
            .filter(|s| !s.trim().is_empty())
            .cloned()
            .collect();
        Ok(cleaned_parts.join(","))
    }
    
    fn split_range_string(&self, input: &str) -> Vec<&str> {
        // Simplified splitting logic - in reality would need complex regex
        input.split(',').collect()
    }
    
    fn process_special_range(
        &self,
        ein_bereich: &str,
        dazu: &mut BTreeSet<i32>,
        hinfort: &mut BTreeSet<i32>,
    ) -> Option<()> {
        if ein_bereich.len() > 1 && ein_bereich.starts_with('-') {
            if let Some(numbers) = self.pattern_validator.str_as_generator_to_list_of_num_strs(&ein_bereich[1..]) {
                hinfort.extend(numbers);
                return Some(());
            }
        } else if !ein_bereich.is_empty() && !ein_bereich.starts_with('-') {
            if let Some(numbers) = self.pattern_validator.str_as_generator_to_list_of_num_strs(ein_bereich) {
                dazu.extend(numbers);
                return Some(());
            }
        }
        None
    }
    
    fn extract_range_info(&self, ein_bereich: &str, base_vielfache: bool) -> (String, bool) {
        let mut is_vielfache = base_vielfache;
        let mut range_str = ein_bereich.to_string();
        
        if ein_bereich.len() > 0 {
            if let Some(v_prefix) = i18n::INSTANCE.befehle2.get("v") {
                if ein_bereich.starts_with(v_prefix) {
                    is_vielfache = true;
                    range_str = ein_bereich[v_prefix.len()..].to_string();
                }
            }
        }
        
        (range_str, is_vielfache)
    }
    
    fn process_range(
        &self,
        range_str: String,
        vielfache: bool,
        max_zahl: i32,
        menge: &mut BTreeSet<i32>,
    ) {
        if range_str.is_empty() {
            return;
        }
        
        let parts: Vec<&str> = range_str.split('-').collect();
        if parts.len() != 2 {
            return;
        }
        
        let start = parts[0].parse::<i32>().unwrap_or(0);
        let rest_parts: Vec<&str> = parts[1].split('+').collect();
        let end = rest_parts[0].parse::<i32>().unwrap_or(0);
        let around: Vec<i32> = rest_parts[1..]
            .iter()
            .filter_map(|s| s.parse::<i32>().ok())
            .collect();
        
        if vielfache {
            self.process_vielfache_range(start, end, &around, max_zahl, menge);
        } else {
            self.process_normal_range(start, end, &around, max_zahl, menge);
        }
    }
    
    fn process_normal_range(
        &self,
        start: i32,
        end: i32,
        around: &[i32],
        max_zahl: i32,
        menge: &mut BTreeSet<i32>,
    ) {
        for number in start..=end {
            for &a in around {
                let c = number + a;
                if c < max_zahl && c > 0 {
                    menge.insert(c);
                }
                
                let d = number - a;
                if d > 0 && d < max_zahl {
                    menge.insert(d);
                }
            }
            
            if around.is_empty() && number > 0 && number < max_zahl {
                menge.insert(number);
            }
        }
    }
    
    fn process_vielfache_range(
        &self,
        start: i32,
        end: i32,
        around: &[i32],
        max_zahl: i32,
        menge: &mut BTreeSet<i32>,
    ) {
        if around.is_empty() || around.iter().all(|&x| x == 0) {
            let mut i = 1;
            while start * i <= max_zahl {
                for number in start..=end {
                    let c = number * i;
                    if c <= max_zahl && c > 0 {
                        menge.insert(c);
                    }
                }
                i += 1;
            }
        } else {
            let mut i = 1;
            while start * i <= max_zahl {
                for number in start..=end {
                    for &a in around {
                        let c = (number * i) + a;
                        if c <= max_zahl && c > 0 {
                            menge.insert(c);
                        }
                        
                        let d = (number * i) - a;
                        if d > 0 && d <= max_zahl {
                            menge.insert(d);
                        }
                    }
                }
                i += 1;
            }
        }
    }
}

pub fn multiples(a: i32, mul1: bool) -> Vec<(i32, i32)> {
    let mut menge = HashSet::new();
    let sqrt_a = (a as f64).sqrt().floor() as i32;
    
    for b in 2..=sqrt_a {
        let c_f64 = a as f64 / b as f64;
        let c = (c_f64 * 1000.0).round() / 1000.0;
        
        if (c - c.round()).abs() < 0.0001 {
            menge.insert((c as i32, b));
        }
    }
    
    let mut result: Vec<(i32, i32)> = menge.into_iter().collect();
    
    if mul1 {
        result.push((a, 1));
    }
    
    result.sort();
    result
}

pub fn teiler(zahlen_bereichs_angabe: &str) -> Result<(Vec<String>, BTreeSet<i32>)> {
    let converter = RangeConverter::new()?;
    let zahlen_bereich_menge = converter.bereich_to_numbers2(zahlen_bereichs_angabe, false, 0, false)?;
    
    let mut zahlen_w_bereich_menge = BTreeSet::new();
    
    for &each1 in &zahlen_bereich_menge {
        for each2 in multiples(each1 as i32, true) {
            zahlen_w_bereich_menge.insert(each2.0);
            zahlen_w_bereich_menge.insert(each2.1);
        }
    }
    
    if zahlen_w_bereich_menge != BTreeSet::from([1]) {
        zahlen_w_bereich_menge.remove(&1);
    }
    
    let zahlen_w_bereich_string_liste: Vec<String> = zahlen_w_bereich_menge
        .iter()
        .map(|n| n.to_string())
        .collect();
    
    Ok((zahlen_w_bereich_string_liste, zahlen_w_bereich_menge))
}

pub fn invert_dict_b(d: &HashMap<String, Vec<String>>) -> HashMap<i32, Vec<String>> {
    let mut new_dict: HashMap<i32, Vec<String>> = HashMap::new();
    
    for (key, value_list) in d {
        for value in value_list {
            if let Ok(int_val) = value.parse::<i32>() {
                let entry = new_dict.entry(int_val).or_insert_with(Vec::new);
                if !entry.contains(key) {
                    entry.push(key.clone());
                }
            }
        }
    }
    
    new_dict
}

pub fn text_hat_ziffer(text: &str) -> bool {
    text.chars().any(|c| c.is_ascii_digit())
}

pub fn primfaktoren(n: i32, modulo: bool) -> Vec<i32> {
    let mut faktoren = Vec::new();
    let mut z = n;
    
    while z > 1 {
        let mut gefunden = false;
        let mut i = 2;
        let mut p = z;
        
        while i * i <= n && !gefunden {
            if z % i == 0 {
                gefunden = true;
                p = i;
            } else {
                i += 1;
            }
        }
        
        let factor = if modulo { p % 24 } else { p };
        faktoren.push(factor);
        z /= p;
    }
    
    faktoren
}

pub fn prim_repeat(n: &[i32]) -> Vec<String> {
    let mut reversed = n.to_vec();
    reversed.reverse();
    
    let mut c = 1;
    let mut b = None;
    let mut d = Vec::new();
    
    for &a in &reversed {
        if Some(a) == b {
            c += 1;
        } else {
            c = 1;
        }
        d.push((a, c));
        b = Some(a);
    }
    
    d.reverse();
    let mut b = None;
    let mut f = Vec::new();
    
    for &(e, g) in &d {
        if Some(e) != b {
            if g == 1 {
                f.push(e.to_string());
            } else {
                f.push(format!("{}^{}", e, g));
            }
        }
        b = Some(e);
    }
    
    f
}

pub fn prim_repeat2(n: &[i32]) -> Vec<(i32, i32)> {
    let mut reversed = n.to_vec();
    reversed.reverse();
    
    let mut c = 1;
    let mut b = None;
    let mut d = Vec::new();
    
    for &a in &reversed {
        if Some(a) == b {
            c += 1;
        } else {
            c = 1;
        }
        d.push((a, c));
        b = Some(a);
    }
    
    d.reverse();
    let mut b = None;
    let mut f = Vec::new();
    
    for &(e, g) in &d {
        if Some(e) != b {
            f.push((e, g));
        }
        b = Some(e);
    }
    
    f
}

pub fn modulo_a(zahlen: &[i32]) {
    for &arg in zahlen {
        for var in 2..26 {
            print!("{} % {} = ", arg, var);
            let modulo = arg % var;
            print!("{} {}, ", modulo, i18n::INSTANCE.classify(modulo));
            let mod2 = var - modulo;
            println!("{} {}", mod2, i18n::INSTANCE.classify(mod2));
        }
    }
}

pub struct App {
    state: AppState,
    console_helper: ConsoleHelper,
    pattern_validator: PatternValidator,
    range_converter: RangeConverter,
}

impl App {
    pub fn new() -> Result<Self> {
        let state = AppState::default();
        let console_helper = ConsoleHelper::new();
        let pattern_validator = PatternValidator::new()?;
        let range_converter = RangeConverter::new()?;
        
        Ok(Self {
            state,
            console_helper,
            pattern_validator,
            range_converter,
        })
    }
    
    pub fn process_args(&mut self) -> Result<()> {
        let args: Vec<String> = env::args().collect();
        
        for arg in &args {
            if arg == format!("-{}", i18n::INSTANCE.main_para_cmds["debug"]) {
                self.state.info_log = true;
            }
        }
        
        Ok(())
    }
    
    pub fn run(&self) -> Result<()> {
        // Example usage
        println!("App initialized with shell width: {}", self.console_helper.shell_width);
        
        // Test pattern validation
        let test_pattern = "1-10,20-30";
        println!("Is '{}' a valid pattern? {}", test_pattern, 
                 self.pattern_validator.is_zeilen_angabe(test_pattern));
        
        // Test range conversion
        let numbers = self.range_converter.bereich_to_numbers2("1-10", false, 100, false)?;
        println!("Numbers in range 1-10: {:?}", numbers);
        
        // Test prime factors
        let factors = primfaktoren(24, false);
        println!("Prime factors of 24: {:?}", factors);
        
        Ok(())
    }
}

fn main() -> Result<()> {
    let mut app = App::new()?;
    app.process_args()?;
    app.run()?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_text_hat_ziffer() {
        assert!(text_hat_ziffer("abc123"));
        assert!(!text_hat_ziffer("abc"));
        assert!(text_hat_ziffer("1"));
    }
    
    #[test]
    fn test_multiples() {
        let result = multiples(12, true);
        assert!(result.contains(&(12, 1)));
        assert!(result.contains(&(6, 2)));
        assert!(result.contains(&(4, 3)));
    }
    
    #[test]
    fn test_primfaktoren() {
        let result = primfaktoren(24, false);
        assert_eq!(result, vec![2, 2, 2, 3]);
    }
    
    #[test]
    fn test_prim_repeat() {
        let factors = vec![2, 2, 2, 3];
        let result = prim_repeat(&factors);
        assert_eq!(result, vec!["2^3", "3"]);
    }
}
