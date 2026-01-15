//! lib4tables module - equivalent to Python lib4tables module

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OutputSyntax {
    Html,
    BbCode,
    Csv,
    Markdown,
    Emacs,
}

pub struct BbCodeSyntax;
pub struct CsvSyntax;
pub struct EmacsSyntax;
pub struct HtmlSyntax;
pub struct MarkdownSyntax;

impl BbCodeSyntax {
    pub fn format(&self, text: &str) -> String {
        format!("[b]{}[/b]", text)
    }
}

impl CsvSyntax {
    pub fn format(&self, text: &str) -> String {
        text.replace(";", ",")
    }
}

impl HtmlSyntax {
    pub fn format(&self, text: &str) -> String {
        format!("<strong>{}</strong>", text)
    }
}

impl MarkdownSyntax {
    pub fn format(&self, text: &str) -> String {
        format!("**{}**", text)
    }
}

pub fn could_be_prime_number_primzahlkreuz(n: i32) -> bool {
    n > 1 && (2..=(n as f64).sqrt() as i32).all(|i| n % i != 0)
}

pub fn could_be_prime_number_primzahlkreuz_fuer_aussen(n: i32) -> bool {
    could_be_prime_number_primzahlkreuz(n) && n % 4 == 3
}

pub fn could_be_prime_number_primzahlkreuz_fuer_innen(n: i32) -> bool {
    could_be_prime_number_primzahlkreuz(n) && n % 4 == 1
}

pub fn divisor_generator(n: i32) -> Vec<i32> {
    let mut divisors = Vec::new();
    for i in 1..=n {
        if n % i == 0 {
            divisors.push(i);
        }
    }
    divisors
}

pub fn is_prim_multiple(n: i32) -> bool {
    n > 1 && (2..n).any(|i| n % i == 0 && could_be_prime_number_primzahlkreuz(i))
}

pub fn moon_number(n: i32) -> (Vec<i32>, Vec<i32>) {
    // Simplified implementation
    let bases = Vec::new();
    let exponents = Vec::new();
    (bases, exponents)
}

pub fn prim_creativity(n: i32) -> i32 {
    if n == 1 {
        0
    } else if could_be_prime_number_primzahlkreuz(n) {
        1
    } else if is_prim_multiple(n) {
        2
    } else {
        3
    }
}

pub fn prim_fak(n: i32) -> Vec<i32> {
    crate::center::primfaktoren(n)
}

pub fn prim_multiple(n: i32) -> Vec<(i32, i32)> {
    let mut result = Vec::new();
    for i in 1..=n {
        if n % i == 0 {
            result.push((i, n / i));
        }
    }
    result
}

pub fn prim_repeat(factors: Vec<i32>) -> Vec<(i32, i32)> {
    let mut result = Vec::new();
    let mut count_map = std::collections::HashMap::new();
    
    for &factor in &factors {
        *count_map.entry(factor).or_insert(0) += 1;
    }
    
    for (&factor, &count) in &count_map {
        result.push((factor, count));
    }
    
    result
}
