// src/tables/mod.rs
mod output;
mod combi;
mod prepare;
mod maintable;

pub use output::Output;
pub use combi::Combi;
pub use prepare::Prepare;
pub use maintable::MainTable;

use crate::syntax::{OutputSyntax, NichtsSyntax, MarkdownSyntax, HtmlSyntax, BBCodeSyntax};
use crate::types::*;
use std::collections::{BTreeSet, HashMap, BTreeMap};
use std::sync::Arc;

#[derive(Debug)]
pub struct Tables {
    config: TableConfig,
    color_config: ColorConfig,
    row_num_display_to_orig: BTreeMap<i32, i32>,
    generated_spalten_parameter: HashMap<i32, ParameterValue>,
    generated_spalten_parameter_tags: HashMap<i32, BTreeSet<SpaltenTag>>,
    pub get_prepare: Prepare,
    pub get_out: Output,
    pub get_combi: Combi,
    pub get_main_table: MainTable,
    religion_numbers: Vec<i32>,
    gener_rows: BTreeSet<i32>,
    rows_of_combi: BTreeSet<i32>,
    spalten_vanilla_amount: i32,
    data_dict: HashMap<i32, HashMap<i32, ParameterValue>>,
    out_type: Box<dyn OutputSyntax>,
    pub keine_leeren_inhalte: bool,
}

impl Tables {
    pub fn new(hoechst_zeil: Option<i32>, txt: String) -> Self {
        let config = if let Some(hz) = hoechst_zeil {
            TableConfig {
                hoechste_zeile: (hz, hz),
                ..Default::default()
            }
        } else {
            TableConfig::default()
        };
        
        let out_type: Box<dyn OutputSyntax> = Box::new(NichtsSyntax);
        
        let mut tables = Self {
            config,
            color_config: ColorConfig::default(),
            row_num_display_to_orig: BTreeMap::new(),
            generated_spalten_parameter: HashMap::new(),
            generated_spalten_parameter_tags: HashMap::new(),
            get_prepare: Prepare::new(),
            get_out: Output::new(txt),
            get_combi: Combi::new(),
            get_main_table: MainTable::new(),
            religion_numbers: Vec::new(),
            gener_rows: BTreeSet::new(),
            rows_of_combi: BTreeSet::new(),
            spalten_vanilla_amount: 0,
            data_dict: HashMap::new(),
            out_type,
            keine_leeren_inhalte: false,
        };
        
        // Set self references
        tables.get_out.set_tables(&tables);
        tables.get_combi.set_tables(&tables);
        tables.get_main_table.set_tables(&tables);
        
        tables
    }
    
    // Property getters
    pub fn nichts_output_yes(&self) -> bool {
        self.out_type.syntax_type() == OutputSyntaxType::Nichts
    }
    
    pub fn markdown_output_yes(&self) -> bool {
        self.out_type.syntax_type() == OutputSyntaxType::Markdown
    }
    
    pub fn bbcode_output_yes(&self) -> bool {
        self.out_type.syntax_type() == OutputSyntaxType::BBCode
    }
    
    pub fn html_output_yes(&self) -> bool {
        self.out_type.syntax_type() == OutputSyntaxType::Html
    }
    
    pub fn out_type(&self) -> &dyn OutputSyntax {
        &*self.out_type
    }
    
    pub fn set_out_type(&mut self, syntax: Box<dyn OutputSyntax>) {
        self.out_type = syntax;
    }
    
    pub fn hoechste_zeile(&self) -> (i32, i32) {
        self.config.hoechste_zeile
    }
    
    pub fn set_hoechste_zeile(&mut self, value: i32) {
        self.config.hoechste_zeile = (value, value);
    }
    
    pub fn gener_rows(&self) -> &BTreeSet<i32> {
        &self.gener_rows
    }
    
    pub fn set_gener_rows(&mut self, value: BTreeSet<i32>) {
        self.gener_rows = value;
    }
    
    pub fn if_prim_multis(&self) -> bool {
        self.get_prepare.ifprimmultis
    }
    
    pub fn set_if_prim_multis(&mut self, value: bool) {
        self.get_prepare.ifprimmultis = value;
    }
    
    pub fn if_zeilen_setted(&self) -> bool {
        self.get_prepare.if_zeilen_setted
    }
    
    pub fn set_if_zeilen_setted(&mut self, value: bool) {
        self.get_prepare.if_zeilen_setted = value;
    }
    
    pub fn gebr_univ_set(&self) -> BTreeSet<i32> {
        // Implement based on actual logic
        BTreeSet::new()
    }
    
    pub fn breitenn(&self) -> &[i32] {
        &self.config.breiten
    }
    
    pub fn set_breitenn(&mut self, mut value: Vec<i32>) {
        // Adjust based on shell rows amount
        // This is simplified - actual implementation would need shell info
        for v in &mut value {
            if *v > 80 { // Simplified shell width
                *v = 80 - 7;
            }
        }
        self.config.breiten = value;
    }
    
    pub fn nummeriere(&self) -> bool {
        self.config.nummeriere
    }
    
    pub fn set_nummeriere(&mut self, value: bool) {
        self.config.nummeriere = value;
        self.get_prepare.nummerierung = value;
    }
    
    pub fn text_height(&self) -> i32 {
        self.config.text_height
    }
    
    pub fn set_text_height(&mut self, value: i32) {
        self.config.text_height = value;
    }
    
    pub fn text_width(&self) -> i32 {
        self.config.text_width
    }
    
    pub fn set_text_width(&mut self, value: i32) {
        // Simplified adjustment
        let adjusted = if value > 80 - 7 && value != 0 {
            80 - 7
        } else {
            value
        };
        self.config.text_width = adjusted;
        self.get_prepare.text_width = adjusted;
    }
    
    pub fn spalte_gestirn(&self) -> bool {
        self.config.spalte_gestirn
    }
    
    pub fn set_spalte_gestirn(&mut self, value: bool) {
        self.config.spalte_gestirn = value;
    }
    
    pub fn religion_numbers(&self) -> &[i32] {
        &self.religion_numbers
    }
    
    pub fn set_religion_numbers(&mut self, value: Vec<i32>) {
        self.religion_numbers = value;
        self.get_out.religion_numbers = value.clone();
        self.get_prepare.religion_numbers = value.clone();
        self.get_combi.religion_numbers = value.clone();
    }
    
    // Static method
    pub fn fill_both(mut liste1: Vec<String>, mut liste2: Vec<String>) -> (Vec<String>, Vec<String>) {
        while liste1.len() < liste2.len() {
            liste1.push(String::new());
        }
        while liste2.len() < liste1.len() {
            liste2.push(String::new());
        }
        (liste1, liste2)
    }
    
    // Main table operations
    pub fn table_reduced_in_lines_by_type_set(&self, table: &Table, lines_allowed: &BTreeSet<usize>) -> Table {
        let mut new_table = Table::new();
        for (i, row) in table.iter().enumerate() {
            if lines_allowed.contains(&i) {
                new_table.push(row.clone());
            }
        }
        new_table
    }
    
    pub fn create_spalte_gestirn(&mut self, relitable: &mut Table, rows_as_numbers: &mut BTreeSet<i32>) {
        if rows_as_numbers.contains(&64) {
            // Implementation based on Python logic
            // This would add a "Gestirn" column
            if !relitable.is_empty() {
                // Add column metadata
                let new_index = relitable[0].len() as i32;
                rows_as_numbers.insert(new_index);
                
                // Add header
                relitable[0].push(Cell::new("Gestirn"));
                relitable[1].push(Cell::new("Sonne (keine Potenzen)"));
                
                // Add content for each row
                for (i, row) in relitable.iter_mut().enumerate().skip(2) {
                    let mut content = Vec::new();
                    
                    if i % 3 == 1 {
                        content.push("wäre eine schwarze Sonne (-3*n), wenn ins Negative durch eine Typ 13 verdreht".to_string());
                    }
                    
                    if moon_number(i as i32).1.is_empty() {
                        content.push("Sonne (keine Potenzen)".to_string());
                    } else {
                        content.push("Mond (Potenzen)".to_string());
                    }
                    
                    if i % 2 == 0 {
                        content.push("Planet (2*n)".to_string());
                    }
                    
                    row.push(Cell::from_lines(content));
                }
            }
        }
    }
}
