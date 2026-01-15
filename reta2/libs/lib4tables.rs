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

use std::collections::{HashMap, HashSet, BTreeMap};
use std::fmt::{self, Display, Formatter};
use std::ops::Range;
use std::sync::OnceLock;

use anyhow::{Context, Result};
use enum_as_inner::EnumAsInner;
use itertools::Itertools;
use num::Integer;
use num_rational::Rational64;
use ordered_float::OrderedFloat;
use regex::Regex;
use serde::{Deserialize, Serialize};

// Import from the previously created center module
mod center {
    pub use super::*;
}

// I18n module
mod i18n {
    use super::*;
    use once_cell::sync::Lazy;
    
    pub static INSTANCE: Lazy<I18n> = Lazy::new(|| I18n::new());
    
    #[derive(Debug, Clone)]
    pub struct I18n {
        pub zaehlung: HashMap<String, String>,
        pub nummerier: HashMap<String, String>,
        pub alles: HashMap<String, String>,
    }
    
    impl I18n {
        pub fn new() -> Self {
            let mut zaehlung = HashMap::new();
            zaehlung.insert("Zählung".to_string(), "Zählung".to_string());
            
            let mut nummerier = HashMap::new();
            nummerier.insert("Nummerierung".to_string(), "Nummerierung".to_string());
            
            let mut alles = HashMap::new();
            alles.insert("alles".to_string(), "alles".to_string());
            
            I18n {
                zaehlung,
                nummerier,
                alles,
            }
        }
    }
}

// Syntax traits and implementations
pub trait OutputSyntax {
    fn colored_begin_col(&self, num: i32, rest: bool) -> String;
    fn generate_cell(
        &self,
        num: i32,
        data_dict: &HashMap<i32, Vec<(String, String)>>,
        content: Option<&str>,
        zeile: Option<i32>,
        tables: Option<&Tables>,
    ) -> String;
    
    fn begin_table(&self) -> &str;
    fn end_table(&self) -> &str;
    fn begin_cell(&self) -> &str;
    fn end_cell(&self) -> &str;
    fn begin_zeile(&self) -> &str;
    fn end_zeile(&self) -> &str;
}

#[derive(Debug, Clone)]
pub struct NichtsSyntax;

impl OutputSyntax for NichtsSyntax {
    fn colored_begin_col(&self, _num: i32, _rest: bool) -> String {
        String::new()
    }
    
    fn generate_cell(
        &self,
        _num: i32,
        _data_dict: &HashMap<i32, Vec<(String, String)>>,
        _content: Option<&str>,
        _zeile: Option<i32>,
        _tables: Option<&Tables>,
    ) -> String {
        String::new()
    }
    
    fn begin_table(&self) -> &str { "" }
    fn end_table(&self) -> &str { "" }
    fn begin_cell(&self) -> &str { "" }
    fn end_cell(&self) -> &str { "" }
    fn begin_zeile(&self) -> &str { "" }
    fn end_zeile(&self) -> &str { "" }
}

#[derive(Debug, Clone)]
pub struct CsvSyntax;

impl OutputSyntax for CsvSyntax {
    fn colored_begin_col(&self, _num: i32, _rest: bool) -> String {
        String::new()
    }
    
    fn generate_cell(
        &self,
        _num: i32,
        _data_dict: &HashMap<i32, Vec<(String, String)>>,
        _content: Option<&str>,
        _zeile: Option<i32>,
        _tables: Option<&Tables>,
    ) -> String {
        String::new()
    }
    
    fn begin_table(&self) -> &str { "" }
    fn end_table(&self) -> &str { "" }
    fn begin_cell(&self) -> &str { "" }
    fn end_cell(&self) -> &str { "" }
    fn begin_zeile(&self) -> &str { "" }
    fn end_zeile(&self) -> &str { "" }
}

#[derive(Debug, Clone)]
pub struct EmacsSyntax;

impl OutputSyntax for EmacsSyntax {
    fn colored_begin_col(&self, _num: i32, _rest: bool) -> String {
        String::new()
    }
    
    fn generate_cell(
        &self,
        _num: i32,
        _data_dict: &HashMap<i32, Vec<(String, String)>>,
        _content: Option<&str>,
        _zeile: Option<i32>,
        _tables: Option<&Tables>,
    ) -> String {
        "|".to_string()
    }
    
    fn begin_table(&self) -> &str { "" }
    fn end_table(&self) -> &str { "" }
    fn begin_cell(&self) -> &str { "|" }
    fn end_cell(&self) -> &str { "" }
    fn begin_zeile(&self) -> &str { "" }
    fn end_zeile(&self) -> &str { "|" }
}

#[derive(Debug, Clone)]
pub struct MarkdownSyntax;

impl OutputSyntax for MarkdownSyntax {
    fn colored_begin_col(&self, _num: i32, _rest: bool) -> String {
        String::new()
    }
    
    fn generate_cell(
        &self,
        _num: i32,
        _data_dict: &HashMap<i32, Vec<(String, String)>>,
        _content: Option<&str>,
        _zeile: Option<i32>,
        _tables: Option<&Tables>,
    ) -> String {
        "|".to_string()
    }
    
    fn begin_table(&self) -> &str { "" }
    fn end_table(&self) -> &str { "" }
    fn begin_cell(&self) -> &str { "|" }
    fn end_cell(&self) -> &str { "" }
    fn begin_zeile(&self) -> &str { "" }
    fn end_zeile(&self) -> &str { "|" }
}

#[derive(Debug, Clone)]
pub struct BbCodeSyntax;

impl OutputSyntax for BbCodeSyntax {
    fn colored_begin_col(&self, num: i32, rest: bool) -> String {
        let num = num;
        let number_type = prim_creativity(num);
        
        if rest {
            "[tr]".to_string()
        } else if number_type == 1 {
            if num % 2 == 0 {
                r#"[tr="background-color:#66ff66;color:#000000;"]"#.to_string()
            } else {
                r#"[tr="background-color:#009900;color:#ffffff;"]"#.to_string()
            }
        } else if number_type == 2 || num == 1 {
            if num % 2 == 0 {
                r#"[tr="background-color:#ffff66;color:#000099;"]"#.to_string()
            } else {
                r#"[tr="background-color:#555500;color:#aaaaff;"]"#.to_string()
            }
        } else if number_type == 3 {
            if num % 2 == 0 {
                r#"[tr="background-color:#9999ff;color:#202000;"]"#.to_string()
            } else {
                r#"[tr="background-color:#000099;color:#ffff66;"]"#.to_string()
            }
        } else if num == 0 {
            r#"[tr="background-color:#ff2222;color:#002222;"]"#.to_string()
        } else {
            "[tr]".to_string()
        }
    }
    
    fn generate_cell(
        &self,
        spalte: i32,
        _data_dict: &HashMap<i32, Vec<(String, String)>>,
        content: Option<&str>,
        _zeile: Option<i32>,
        _tables: Option<&Tables>,
    ) -> String {
        let spalte = spalte + 2;
        
        if let Some(content_str) = content {
            if let Ok(content_num) = content_str.parse::<i32>() {
                if content_num % 2 == 0 {
                    r#"[td="background-color:#000000;color:#ffffff"]"#.to_string()
                } else {
                    r#"[td="background-color:#ffffff;color:#000000"]"#.to_string()
                }
            } else {
                "[td]".to_string()
            }
        } else {
            "[td]".to_string()
        }
    }
    
    fn begin_table(&self) -> &str { "[table]" }
    fn end_table(&self) -> &str { "[/table]" }
    fn begin_cell(&self) -> &str { "[td]" }
    fn end_cell(&self) -> &str { "[/td]" }
    fn begin_zeile(&self) -> &str { "[tr]" }
    fn end_zeile(&self) -> &str { "[/tr]" }
}

#[derive(Debug, Clone)]
pub struct HtmlSyntax {
    zeile: i32,
}

impl HtmlSyntax {
    pub fn new() -> Self {
        HtmlSyntax { zeile: 0 }
    }
}

impl OutputSyntax for HtmlSyntax {
    fn colored_begin_col(&self, num: i32, rest: bool) -> String {
        let num = num;
        let number_type = prim_creativity(num);
        
        if rest {
            "<tr>\n".to_string()
        } else if number_type == 1 {
            if num % 2 == 0 {
                r#"<tr style="background-color:#66ff66;color:#000000;">"#.to_string()
            } else {
                r#"<tr style="background-color:#009900;color:#ffffff;">"#.to_string()
            }
        } else if number_type == 2 || num == 1 {
            if num % 2 == 0 {
                r#"<tr style="background-color:#ffff66;color:#000099;">"#.to_string()
            } else {
                r#"<tr style="background-color:#555500;color:#aaaaff;">"#.to_string()
            }
        } else if number_type == 3 {
            if num % 2 == 0 {
                r#"<tr style="background-color:#9999ff;color:#202000;">"#.to_string()
            } else {
                r#"<tr style="background-color:#000099;color:#ffff66;">"#.to_string()
            }
        } else if num == 0 {
            r#"<tr style="background-color:#ff2222;color:#002222;">"#.to_string()
        } else {
            "<tr>\n".to_string()
        }
    }
    
    fn generate_cell(
        &self,
        spalte: i32,
        data_dict: &HashMap<i32, Vec<(String, String)>>,
        content: Option<&str>,
        zeile: Option<i32>,
        tables: Option<&Tables>,
    ) -> String {
        let zeile_val = zeile.unwrap_or(0);
        
        if spalte == -2 {
            let tuple_of_lists_of_couples = vec![
                vec![(i18n::INSTANCE.zaehlung["Zählung"].clone(), "".to_string())]
            ];
            self.format_cell(spalte, &tuple_of_lists_of_couples, content, zeile_val, tables)
        } else if spalte == -1 {
            let tuple_of_lists_of_couples = vec![
                vec![(i18n::INSTANCE.nummerier["Nummerierung"].clone(), "".to_string())]
            ];
            self.format_cell(spalte, &tuple_of_lists_of_couples, content, zeile_val, tables)
        } else {
            if let Some(couples) = data_dict.get(&spalte) {
                let tuple_of_lists_of_couples = vec![couples.clone()];
                self.format_cell(spalte, &tuple_of_lists_of_couples, content, zeile_val, tables)
            } else {
                String::new()
            }
        }
    }
    
    fn begin_table(&self) -> &str { r#"<table border=0 id="bigtable">"# }
    fn end_table(&self) -> &str { "</table>\n" }
    fn begin_cell(&self) -> &str { "<td>\n" }
    fn end_cell(&self) -> &str { "\n</td>\n" }
    fn begin_zeile(&self) -> &str { "" }
    fn end_zeile(&self) -> &str { "</tr>\n" }
}

impl HtmlSyntax {
    fn format_cell(
        &self,
        spalte: i32,
        tuple_of_lists_of_couples: &[Vec<(String, String)>],
        content: Option<&str>,
        zeile: i32,
        tables: Option<&Tables>,
    ) -> String {
        let mut things1: BTreeMap<usize, Vec<String>> = BTreeMap::new();
        
        for (c, couples) in tuple_of_lists_of_couples.iter().enumerate() {
            for para_num in 0..2 {
                if let Some(couple) = couples.get(para_num) {
                    let para_name = if para_num == 0 { &couple.0 } else { &couple.1 };
                    if !para_name.trim().is_empty() {
                        let formatted_name = if para_num == 1 {
                            format!("p3_{}_{}", c, para_name)
                        } else {
                            para_name.clone()
                        };
                        
                        things1.entry(para_num)
                            .or_insert_with(Vec::new)
                            .push(formatted_name);
                    }
                }
            }
        }
        
        let mut things: BTreeMap<usize, String> = BTreeMap::new();
        
        for (key, values) in things1 {
            let mut parts = Vec::new();
            for el in values {
                if el != i18n::INSTANCE.alles["alles"] {
                    if key == 0 {
                        parts.push("✗".to_string());
                    }
                    parts.push(el);
                    parts.push(",".to_string());
                }
            }
            things.insert(key, parts.concat());
        }
        
        let spalte_adj = spalte + 2;
        
        if things.len() < 2 {
            String::new()
        } else {
            let p4 = if let Some(tables_ref) = tables {
                if let Some(tags) = tables_ref.generated_spalten_parameter_tags.get(&(spalte_adj - 2)) {
                    let mut tag_strings: Vec<String> = tags.iter()
                        .map(|tag| tag.value().to_string())
                        .collect();
                    tag_strings.join(",")
                } else {
                    String::new()
                }
            } else {
                String::new()
            };
            
            let mut parts = Vec::new();
            parts.push("<td".to_string());
            
            if zeile == 0 {
                parts.push(format!(
                    r#" class="z_{} r_{} p1_{} p2_{} p4_{}""#,
                    zeile,
                    spalte_adj,
                    things.get(&0).unwrap_or(&String::new()),
                    things.get(&1).unwrap_or(&String::new()),
                    p4
                ));
            }
            
            if spalte_adj == 0 || spalte_adj == 1 {
                if let Some(content_str) = content {
                    if let Ok(content_num) = content_str.parse::<i32>() {
                        if content_num % 2 == 0 {
                            parts.push(r#" style="background-color:#000000;color:#ffffff;""#.to_string());
                        } else {
                            parts.push(r#" style="background-color:#ffffff;color:#000000;""#.to_string());
                        }
                    }
                }
            } else if zeile == 0 {
                // Hidden style commented out
                // parts.push(r#" style="display:none""#.to_string());
            } else if let Some(values) = things1.get(&0) {
                if values.iter().any(|v| v.contains("Symbole")) {
                    parts.push(r#" class="tdSymbole" style="background-image: url();background-size: cover;background-repeat: no-repeat;background-position: right; ""#.to_string());
                }
            }
            
            parts.push(">\n".to_string());
            parts.concat()
        }
    }
}

// Tag system for tables
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Tag {
    String(String),
    Number(i32),
    Boolean(bool),
}

impl Tag {
    pub fn value(&self) -> String {
        match self {
            Tag::String(s) => s.clone(),
            Tag::Number(n) => n.to_string(),
            Tag::Boolean(b) => b.to_string(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Tables {
    pub generated_spalten_parameter_tags: HashMap<i32, Vec<Tag>>,
}

impl Tables {
    pub fn new() -> Self {
        Tables {
            generated_spalten_parameter_tags: HashMap::new(),
        }
    }
}

// Mathematical functions
pub fn moon_number(num: i32) -> (Vec<i32>, Vec<i32>) {
    let mut results = Vec::new();
    let mut exponents = Vec::new();
    
    for i in 2..num {
        let one_result = (num as f64).powf(1.0 / i as f64);
        if (one_result.round() * 100000.0).round() == (one_result * 100000.0).round() {
            results.push(one_result.round() as i32);
            exponents.push(i - 2);
        }
    }
    
    (results, exponents)
}

pub fn prim_fak(n: i32) -> Vec<i32> {
    let mut faktoren = Vec::new();
    let mut z = n;
    
    while z > 1 {
        let mut i = 2;
        let mut gefunden = false;
        let mut p = z;
        
        while i * i <= n && !gefunden {
            if z % i == 0 {
                gefunden = true;
                p = i;
            } else {
                i += 1;
            }
        }
        
        if !gefunden {
            p = z;
        }
        
        faktoren.push(p);
        z = z / p;
    }
    
    faktoren
}

pub fn divisor_generator(n: i32) -> impl Iterator<Item = i32> {
    let sqrt_n = (n as f64).sqrt() as i32;
    let mut large_divisors = Vec::new();
    
    (1..=sqrt_n).filter_map(move |i| {
        if n % i == 0 {
            if i * i != n {
                large_divisors.push(n / i);
            }
            Some(i)
        } else {
            None
        }
    }).chain(large_divisors.into_iter().rev())
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PrimCreativity {
    Zero = 0,
    Prime = 1,
    Composite = 2,
    Power = 3,
}

pub fn prim_creativity(num: i32) -> i32 {
    if num == 0 {
        return 0;
    }
    
    let fak = prim_repeat(&prim_fak(num));
    
    if fak.len() == 1 && fak[0].1 == 1 {
        return 1;
    }
    
    if fak.len() == 1 {
        return 3;
    }
    
    if fak.is_empty() {
        return 0;
    }
    
    let prim_amounts: Vec<i32> = fak.iter().map(|(_, amount)| *amount).collect();
    let mut schnittmenge: Option<HashSet<i32>> = None;
    
    for prim_amount in prim_amounts {
        let divisors: HashSet<i32> = divisor_generator(prim_amount)
            .filter(|&d| d != 1)
            .collect();
        
        if divisors.is_empty() {
            schnittmenge = None;
            break;
        }
        
        schnittmenge = Some(match schnittmenge.take() {
            Some(mut set) => {
                set.retain(|d| divisors.contains(d));
                set
            }
            None => divisors,
        });
    }
    
    if let Some(set) = schnittmenge {
        if !set.is_empty() {
            3
        } else {
            2
        }
    } else {
        2
    }
}

pub fn prim_repeat(n: &[i32]) -> Vec<(i32, i32)> {
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

pub fn prim_multiple(n: i32) -> Vec<(i32, i32)> {
    let mut multiples = vec![(1, n)];
    let prim_factors = prim_repeat(&prim_fak(n));
    
    for (prim, _) in prim_factors {
        multiples.push((prim, n / prim));
    }
    
    multiples
}

pub fn is_prim_multiple(is_it: i32, multiples1: &[i32], dont_return_list: bool) -> Result<bool, Vec<bool>> {
    let multiples2 = prim_multiple(is_it);
    let mut are_they = Vec::new();
    
    for &multiple1 in multiples1 {
        let mut found = false;
        for &(_, multiple2) in &multiples2 {
            if multiple1 == multiple2 {
                found = true;
                break;
            }
        }
        are_they.push(found);
        
        if dont_return_list && found {
            return Ok(true);
        }
    }
    
    if dont_return_list {
        Ok(false)
    } else {
        Err(are_they)
    }
}

pub fn could_be_prime_number_primzahlkreuz(num: i32) -> bool {
    let under_24 = [1, 5, 7, 11, 13, 17, 19, 23];
    under_24.contains(&(num % 24))
}

pub fn could_be_prime_number_primzahlkreuz_fuer_innen(num: i32) -> bool {
    let under_24 = [5, 11, 17, 23];
    under_24.contains(&(num % 24))
}

pub fn could_be_prime_number_primzahlkreuz_fuer_aussen(num: i32) -> bool {
    let under_24 = [1, 7, 13, 19];
    under_24.contains(&(num % 24))
}

// Utility functions
pub fn get_logarithm_only_as_pure_int(potenz: i32, basis: i32) -> Option<i32> {
    let exponent = (potenz as f64).ln() / (basis as f64).ln();
    if (exponent - exponent.round()).abs() < 1e-10 {
        Some(exponent.round() as i32)
    } else {
        None
    }
}

// Table formatting utilities
pub struct TableFormatter<'a> {
    syntax: &'a dyn OutputSyntax,
    tables: Option<&'a Tables>,
}

impl<'a> TableFormatter<'a> {
    pub fn new(syntax: &'a dyn OutputSyntax, tables: Option<&'a Tables>) -> Self {
        TableFormatter { syntax, tables }
    }
    
    pub fn format_table(
        &self,
        data: &HashMap<i32, HashMap<i32, String>>,
        columns: &HashMap<i32, Vec<(String, String)>>,
    ) -> String {
        let mut output = String::new();
        output.push_str(self.syntax.begin_table());
        
        // Sort rows
        let mut row_keys: Vec<&i32> = data.keys().collect();
        row_keys.sort();
        
        for &row_key in row_keys {
            if let Some(row_data) = data.get(&row_key) {
                output.push_str(&self.syntax.colored_begin_col(row_key, false));
                
                // Sort columns
                let mut col_keys: Vec<&i32> = row_data.keys().collect();
                col_keys.sort();
                
                for &col_key in col_keys {
                    let cell_content = row_data.get(&col_key).unwrap();
                    let cell_html = self.syntax.generate_cell(
                        col_key,
                        columns,
                        Some(cell_content),
                        Some(row_key),
                        self.tables,
                    );
                    
                    output.push_str(&cell_html);
                    output.push_str(cell_content);
                    output.push_str(self.syntax.end_cell());
                }
                
                output.push_str(self.syntax.end_zeile());
            }
        }
        
        output.push_str(self.syntax.end_table());
        output
    }
}

// Main application
pub struct Lib4Tables {
    tables: Tables,
}

impl Lib4Tables {
    pub fn new() -> Self {
        Lib4Tables {
            tables: Tables::new(),
        }
    }
    
    pub fn run_demo(&self) -> Result<()> {
        println!("=== Lib4Tables Demo ===");
        
        // Test prime factorization
        let test_num = 24;
        let factors = prim_fak(test_num);
        println!("Prime factors of {}: {:?}", test_num, factors);
        
        let repeated = prim_repeat(&factors);
        println!("Repeated factors: {:?}", repeated);
        
        // Test prim creativity
        let creativity = prim_creativity(test_num);
        println!("Prim creativity of {}: {}", test_num, creativity);
        
        // Test moon number
        let (results, exponents) = moon_number(8);
        println!("Moon number 8: bases={:?}, exponents={:?}", results, exponents);
        
        // Test prime multiples
        let multiples = prim_multiple(test_num);
        println!("Prime multiples of {}: {:?}", test_num, multiples);
        
        // Test prime number checks
        println!("Could 23 be prime (Primzahlkreuz)? {}", could_be_prime_number_primzahlkreuz(23));
        println!("Could 23 be inner prime? {}", could_be_prime_number_primzahlkreuz_fuer_innen(23));
        println!("Could 23 be outer prime? {}", could_be_prime_number_primzahlkreuz_fuer_aussen(23));
        
        // Test table formatting
        self.test_table_formatting()?;
        
        Ok(())
    }
    
    fn test_table_formatting(&self) -> Result<()> {
        println!("\n=== Table Formatting Test ===");
        
        // Create test data
        let mut data = HashMap::new();
        let mut row1 = HashMap::new();
        row1.insert(0, "A1".to_string());
        row1.insert(1, "B1".to_string());
        data.insert(0, row1);
        
        let mut row2 = HashMap::new();
        row2.insert(0, "A2".to_string());
        row2.insert(1, "B2".to_string());
        data.insert(1, row2);
        
        let mut columns = HashMap::new();
        columns.insert(0, vec![("Header1".to_string(), "Sub1".to_string())]);
        columns.insert(1, vec![("Header2".to_string(), "Sub2".to_string())]);
        
        // Test different syntaxes
        let syntaxes: Vec<Box<dyn OutputSyntax>> = vec![
            Box::new(NichtsSyntax),
            Box::new(CsvSyntax),
            Box::new(EmacsSyntax),
            Box::new(MarkdownSyntax),
            Box::new(BbCodeSyntax),
            Box::new(HtmlSyntax::new()),
        ];
        
        for (i, syntax) in syntaxes.iter().enumerate() {
            let formatter = TableFormatter::new(syntax.as_ref(), Some(&self.tables));
            let result = formatter.format_table(&data, &columns);
            
            println!("\nSyntax {}: {}", i, std::any::type_name::<dyn OutputSyntax>());
            println!("{}", result);
        }
        
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_prim_fak() {
        assert_eq!(prim_fak(24), vec![2, 2, 2, 3]);
        assert_eq!(prim_fak(17), vec![17]);
        assert_eq!(prim_fak(1), vec![]);
    }
    
    #[test]
    fn test_prim_repeat() {
        let factors = vec![2, 2, 2, 3];
        let repeated = prim_repeat(&factors);
        assert_eq!(repeated, vec![(2, 3), (3, 1)]);
    }
    
    #[test]
    fn test_prim_creativity() {
        assert_eq!(prim_creativity(17), 1); // Prime
        assert_eq!(prim_creativity(24), 2); // Composite
        assert_eq!(prim_creativity(8), 3);  // Power of prime
        assert_eq!(prim_creativity(0), 0);  // Zero
    }
    
    #[test]
    fn test_could_be_prime_number_primzahlkreuz() {
        assert!(could_be_prime_number_primzahlkreuz(23));
        assert!(!could_be_prime_number_primzahlkreuz(24));
        assert!(could_be_prime_number_primzahlkreuz(5));
    }
    
    #[test]
    fn test_divisor_generator() {
        let divisors: Vec<i32> = divisor_generator(12).collect();
        assert_eq!(divisors, vec![1, 2, 3, 4, 6, 12]);
    }
}

fn main() -> Result<()> {
    let lib = Lib4Tables::new();
    lib.run_demo()?;
    
    Ok(())
}
