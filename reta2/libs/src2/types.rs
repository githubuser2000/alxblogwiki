// src/types.rs
use std::collections::{BTreeSet, HashMap, BTreeMap};
use std::fmt;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum OutputSyntaxType {
    Nichts,
    Markdown,
    BBCode,
    Html,
    Csv,
    Emacs,
    Default,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SpaltenTag {
    SternPolygon,
    Universum,
    Galaxie,
}

#[derive(Debug, Clone)]
pub enum ParameterValue {
    Single(Vec<(String, String)>),
    Double((Vec<(String, String)>, Vec<(String, String)>)),
}

#[derive(Debug, Clone)]
pub struct ColorConfig {
    pub enabled: bool,
    pub even_bg: String,
    pub odd_bg: String,
    pub prime_fg: String,
    pub moon_fg: String,
}

impl Default for ColorConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            even_bg: "\x1b[47m".to_string(),
            odd_bg: "\x1b[100m".to_string(),
            prime_fg: "\x1b[103m\x1b[30m\x1b[1m".to_string(),
            moon_fg: "\x1b[106m\x1b[30m".to_string(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct TableConfig {
    pub hoechste_zeile: (i32, i32),
    pub text_width: i32,
    pub text_height: i32,
    pub nummeriere: bool,
    pub keine_ueberschriften: bool,
    pub spalte_gestirn: bool,
    pub keine_leeren_inhalte: bool,
    pub breiten: Vec<i32>,
}

impl Default for TableConfig {
    fn default() -> Self {
        Self {
            hoechste_zeile: (1024, 163),
            text_width: 21,
            text_height: 0,
            nummeriere: true,
            keine_ueberschriften: false,
            spalte_gestirn: false,
            keine_leeren_inhalte: false,
            breiten: Vec::new(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Cell {
    pub lines: Vec<String>,
    pub metadata: HashMap<String, String>,
}

impl Cell {
    pub fn new(content: &str) -> Self {
        Self {
            lines: vec![content.to_string()],
            metadata: HashMap::new(),
        }
    }
    
    pub fn from_lines(lines: Vec<String>) -> Self {
        Self {
            lines,
            metadata: HashMap::new(),
        }
    }
}

pub type Row = Vec<Cell>;
pub type Table = Vec<Row>;

// Mathematical helpers (simplified - would need actual implementations)
pub fn is_prime(n: i32) -> bool {
    if n <= 1 {
        return false;
    }
    for i in 2..=(n as f64).sqrt() as i32 {
        if n % i == 0 {
            return false;
        }
    }
    true
}

pub fn prime_factors(n: i32) -> Vec<i32> {
    let mut factors = Vec::new();
    let mut num = n;
    let mut divisor = 2;
    
    while num > 1 {
        while num % divisor == 0 {
            factors.push(divisor);
            num /= divisor;
        }
        divisor += 1;
        if divisor * divisor > num {
            if num > 1 {
                factors.push(num);
            }
            break;
        }
    }
    factors
}

pub fn moon_number(n: i32) -> (i32, Vec<i32>) {
    // Simplified - implement actual moon number logic
    (n, Vec::new())
}
