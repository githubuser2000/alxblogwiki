use std::collections::{HashMap, HashSet};
use indexmap::{IndexSet, IndexMap};
use num_rational::Ratio;
use serde::{Deserialize, Serialize};

// Type aliases
pub type Row = Vec<String>;
pub type Table = Vec<Row>;
pub type OrderedSet<T> = IndexSet<T>;
pub type OrderedDict<K, V> = IndexMap<K, V>;
pub type DefaultOrderedDict<K, V> = IndexMap<K, V>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Fraction(pub Ratio<i32>);

impl Fraction {
    pub fn new(numerator: i32, denominator: i32) -> Self {
        Fraction(Ratio::new(numerator, denominator))
    }
    
    pub fn from_float(value: f64) -> Option<Self> {
        // Simple float to fraction conversion
        // For better conversion, use a more sophisticated algorithm
        let tolerance = 1e-6;
        let mut n = 1;
        let mut m = 1;
        
        for i in 1..1000 {
            for j in 1..1000 {
                let approx = i as f64 / j as f64;
                if (approx - value).abs() < tolerance {
                    return Some(Fraction(Ratio::new(i as i32, j as i32)));
                }
            }
        }
        None
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tables {
    pub generated_spalten_parameter: OrderedDict<usize, String>,
    pub generated_spalten_parameter_tags: OrderedDict<usize, HashSet<ST>>,
    pub spalten_vanilla_amount: usize,
    pub last_line_number: usize,
    pub data_dict: OrderedDict<usize, Row>,
    pub html_output_yes: bool,
    pub bbcode_output_yes: bool,
    pub hoechste_zeile: OrderedDict<usize, usize>,
    pub spalten_vanilla_amount_backup: Option<usize>,
}

impl Default for Tables {
    fn default() -> Self {
        Self {
            generated_spalten_parameter: OrderedDict::new(),
            generated_spalten_parameter_tags: OrderedDict::new(),
            spalten_vanilla_amount: 0,
            last_line_number: 0,
            data_dict: OrderedDict::new(),
            html_output_yes: false,
            bbcode_output_yes: false,
            hoechste_zeile: OrderedDict::new(),
            spalten_vanilla_amount_backup: None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneratedBefehle {
    pub commands: OrderedSet<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParametersMain {
    pub bedeutung: (usize, String),
    pub procontra: (usize, String),
    pub grundstrukturen: (usize, String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HtmlTagParaClassWoerter {
    pub words: Vec<Vec<Vec<Vec<(String, String)>>>>,
}

pub struct Transpose;

impl Transpose {
    pub fn transpose<T: Clone>(matrix: &[Vec<T>]) -> Vec<Vec<T>> {
        if matrix.is_empty() {
            return vec![];
        }
        
        let rows = matrix.len();
        let cols = matrix[0].len();
        
        let mut result = vec![Vec::with_capacity(rows); cols];
        
        for row in matrix {
            for (col_idx, item) in row.iter().enumerate() {
                result[col_idx].push(item.clone());
            }
        }
        
        result
    }
}
