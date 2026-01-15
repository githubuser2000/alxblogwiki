use std::collections::{BTreeMap, BTreeSet, HashMap};
use std::fmt;
use serde::{Deserialize, Serialize};

pub type OrderedSet<T> = BTreeSet<T>;
pub type OrderedDict<K, V> = BTreeMap<K, V>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SyntaxType {
    Default,
    Nichts,
    Markdown,
    BBCode,
    Html,
    Csv,
    Emacs,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SpaltenTag {
    SternPolygon,
    Universum,
    Galaxie,
    Emotion,
    Groesse,
}

#[derive(Debug, Clone)]
pub enum ParameterValue {
    Single(Vec<(String, String)>),
    Double(Vec<(String, String)>, Vec<(String, String)>),
    Lambda(Box<dyn Fn(&str) -> OrderedSet<i32>>),
}

impl fmt::Display for ParameterValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParameterValue::Single(v) => write!(f, "Single({:?})", v),
            ParameterValue::Double(v1, v2) => write!(f, "Double({:?}, {:?})", v1, v2),
            ParameterValue::Lambda(_) => write!(f, "Lambda(...)"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct ColorConfig {
    pub enabled: bool,
    pub even_bg: String,
    pub odd_bg: String,
    pub prime_fg: String,
    pub moon_fg: String,
    pub header_bg: String,
}

impl Default for ColorConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            even_bg: "\x1b[47m".to_string(),
            odd_bg: "\x1b[100m".to_string(),
            prime_fg: "\x1b[103m\x1b[30m\x1b[1m".to_string(),
            moon_fg: "\x1b[106m\x1b[30m".to_string(),
            header_bg: "\x1b[41m\x1b[30m\x1b[4m".to_string(),
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
    pub shell_rows_amount: i32,
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
            shell_rows_amount: 24,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Cell {
    pub lines: Vec<String>,
    pub metadata: HashMap<String, String>,
    pub cell_type: CellType,
}

impl Cell {
    pub fn new(content: &str) -> Self {
        Self {
            lines: vec![content.to_string()],
            metadata: HashMap::new(),
            cell_type: CellType::Text,
        }
    }
    
    pub fn from_lines(lines: Vec<String>) -> Self {
        Self {
            lines,
            metadata: HashMap::new(),
            cell_type: CellType::Text,
        }
    }
    
    pub fn empty() -> Self {
        Self {
            lines: vec!["".to_string()],
            metadata: HashMap::new(),
            cell_type: CellType::Empty,
        }
    }
    
    pub fn is_empty(&self) -> bool {
        self.lines.is_empty() || (self.lines.len() == 1 && self.lines[0].trim().is_empty())
    }
    
    pub fn join_lines(&self, separator: &str) -> String {
        self.lines.join(separator)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum CellType {
    Text,
    Number,
    Header,
    Generated,
    Empty,
}

pub type Row = Vec<Cell>;
pub type Table = Vec<Row>;

#[derive(Debug, Clone)]
pub struct TableData {
    pub table: Table,
    pub headers: Row,
    pub column_types: Vec<CellType>,
    pub column_widths: Vec<usize>,
}

impl TableData {
    pub fn new() -> Self {
        Self {
            table: Vec::new(),
            headers: Vec::new(),
            column_types: Vec::new(),
            column_widths: Vec::new(),
        }
    }
    
    pub fn num_rows(&self) -> usize {
        self.table.len()
    }
    
    pub fn num_cols(&self) -> usize {
        if !self.table.is_empty() {
            self.table[0].len()
        } else {
            0
        }
    }
}

// Mathematical functions
pub fn is_prime(n: i32) -> bool {
    if n <= 1 {
        return false;
    }
    if n <= 3 {
        return true;
    }
    if n % 2 == 0 || n % 3 == 0 {
        return false;
    }
    
    let mut i = 5;
    while i * i <= n {
        if n % i == 0 || n % (i + 2) == 0 {
            return false;
        }
        i += 6;
    }
    true
}

pub fn prime_factors(mut n: i32) -> Vec<i32> {
    let mut factors = Vec::new();
    
    // Handle 2 separately
    while n % 2 == 0 {
        factors.push(2);
        n /= 2;
    }
    
    // Check odd numbers
    let mut i = 3;
    while i * i <= n {
        while n % i == 0 {
            factors.push(i);
            n /= i;
        }
        i += 2;
    }
    
    // If n is still greater than 1, it's a prime
    if n > 1 {
        factors.push(n);
    }
    
    factors
}

pub fn prim_creativity(n: i32) -> i32 {
    if is_prime(n) {
        // For prime numbers, creativity is based on position
        let small_primes = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31];
        if small_primes.contains(&n) {
            1
        } else {
            2
        }
    } else {
        // For composite numbers, creativity is based on prime factors
        let factors = prime_factors(n);
        if factors.len() == 1 {
            0
        } else {
            factors.len() as i32
        }
    }
}

pub fn moon_number(n: i32) -> (i32, Vec<i32>) {
    let mut result = Vec::new();
    
    // Check if n is a "moon number" (has special properties)
    if n % 4 == 0 {
        result.push(4);
    }
    if n % 7 == 0 {
        result.push(7);
    }
    if n % 13 == 0 {
        result.push(13);
    }
    if n % 28 == 0 {
        result.push(28);
    }
    
    (n, result)
}

#[derive(Debug, Clone)]
pub struct RangeSpec {
    pub start: Option<i32>,
    pub end: Option<i32>,
    pub step: Option<i32>,
    pub invert: bool,
}

impl RangeSpec {
    pub fn parse(s: &str) -> Result<Self, String> {
        let mut spec = Self {
            start: None,
            end: None,
            step: None,
            invert: s.starts_with('!'),
        };
        
        let s = if spec.invert { &s[1..] } else { s };
        
        if let Some(dash_pos) = s.find('-') {
            let start_str = &s[..dash_pos];
            let rest = &s[dash_pos + 1..];
            
            if !start_str.is_empty() {
                spec.start = Some(start_str.parse().map_err(|e| format!("Invalid start: {}", e))?);
            }
            
            if let Some(colon_pos) = rest.find(':') {
                let end_str = &rest[..colon_pos];
                let step_str = &rest[colon_pos + 1..];
                
                if !end_str.is_empty() {
                    spec.end = Some(end_str.parse().map_err(|e| format!("Invalid end: {}", e))?);
                }
                
                if !step_str.is_empty() {
                    spec.step = Some(step_str.parse().map_err(|e| format!("Invalid step: {}", e))?);
                }
            } else if !rest.is_empty() {
                spec.end = Some(rest.parse().map_err(|e| format!("Invalid end: {}", e))?);
            }
        } else {
            // Single number
            spec.start = Some(s.parse().map_err(|e| format!("Invalid number: {}", e))?);
            spec.end = spec.start;
        }
        
        Ok(spec)
    }
    
    pub fn to_numbers(&self, max_value: i32) -> OrderedSet<i32> {
        let mut numbers = OrderedSet::new();
        
        let start = self.start.unwrap_or(1);
        let end = self.end.unwrap_or(max_value);
        let step = self.step.unwrap_or(1);
        
        if step > 0 {
            let mut current = start;
            while current <= end {
                numbers.insert(current);
                current += step;
            }
        } else if step < 0 {
            let mut current = start;
            while current >= end {
                numbers.insert(current);
                current += step;
            }
        }
        
        if self.invert {
            let all_numbers: OrderedSet<i32> = (1..=max_value).collect();
            all_numbers.difference(&numbers).cloned().collect()
        } else {
            numbers
        }
    }
}
