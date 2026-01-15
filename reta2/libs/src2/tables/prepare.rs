// src/tables/prepare.rs
use crate::types::*;

#[derive(Debug)]
pub struct Prepare {
    pub ifprimmultis: bool,
    pub if_zeilen_setted: bool,
    pub breiten: Vec<i32>,
    pub nummerierung: bool,
    pub text_width: i32,
    pub religion_numbers: Vec<i32>,
}

impl Prepare {
    pub fn new() -> Self {
        Self {
            ifprimmultis: false,
            if_zeilen_setted: false,
            breiten: Vec::new(),
            nummerierung: true,
            text_width: 0,
            religion_numbers: Vec::new(),
        }
    }
    
    pub fn zeile_which_zaehlung(&self, zeile: i32) -> i32 {
        // Simplified - implement actual logic
        zeile
    }
    
    pub fn cell_work(&self, content: &str, certain_text_width: i32) -> Vec<String> {
        // Simplified text wrapping
        let width = certain_text_width as usize;
        let mut result = Vec::new();
        let mut current_line = String::new();
        
        for word in content.split_whitespace() {
            if current_line.len() + word.len() + 1 > width && !current_line.is_empty() {
                result.push(current_line.clone());
                current_line.clear();
            }
            
            if !current_line.is_empty() {
                current_line.push(' ');
            }
            current_line.push_str(word);
        }
        
        if !current_line.is_empty() {
            result.push(current_line);
        }
        
        result
    }
}
