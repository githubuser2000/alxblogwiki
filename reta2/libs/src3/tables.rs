mod output;
mod combi;
mod prepare;
mod maintable;
mod concat;

use std::collections::{BTreeMap, BTreeSet, HashMap};
use std::sync::Arc;
use crate::types::*;
use crate::error::RetaError;
use crate::i18n;

pub use output::Output;
pub use combi::Combi;
pub use prepare::Prepare;
pub use maintable::MainTable;
pub use concat::Concat;

#[derive(Debug)]
pub struct Tables {
    config: TableConfig,
    color_config: ColorConfig,
    row_num_display_to_orig: OrderedDict<i32, i32>,
    generated_spalten_parameter: OrderedDict<i32, ParameterValue>,
    generated_spalten_parameter_tags: OrderedDict<i32, BTreeSet<SpaltenTag>>,
    prepare: Prepare,
    output: Output,
    combi: Combi,
    main_table: MainTable,
    concat: Concat,
    religion_numbers: Vec<i32>,
    gener_rows: OrderedSet<i32>,
    rows_of_combi: OrderedSet<i32>,
    spalten_vanilla_amount: i32,
    data_dict: Vec<OrderedDict<i32, serde_json::Value>>,
    syntax_type: SyntaxType,
    keine_leeren_inhalte: bool,
    keine_ueberschriften: bool,
    spalte_gestirn: bool,
    if_zeilen_setted: bool,
    if_prim_multis: bool,
    last_line_number: i32,
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
        
        let mut tables = Self {
            config: config.clone(),
            color_config: ColorConfig::default(),
            row_num_display_to_orig: OrderedDict::new(),
            generated_spalten_parameter: OrderedDict::new(),
            generated_spalten_parameter_tags: OrderedDict::new(),
            prepare: Prepare::new(),
            output: Output::new(txt),
            combi: Combi::new(),
            main_table: MainTable::new(),
            concat: Concat::new(),
            religion_numbers: Vec::new(),
            gener_rows: OrderedSet::new(),
            rows_of_combi: OrderedSet::new(),
            spalten_vanilla_amount: 0,
            data_dict: vec![OrderedDict::new(); 14],
            syntax_type: SyntaxType::Default,
            keine_leeren_inhalte: false,
            keine_ueberschriften: false,
            spalte_gestirn: false,
            if_zeilen_setted: false,
            if_prim_multis: false,
            last_line_number: 0,
        };
        
        // Set references
        tables.prepare.set_tables(&tables);
        tables.output.set_tables(&tables);
        tables.combi.set_tables(&tables);
        tables.main_table.set_tables(&tables);
        tables.concat.set_tables(&tables);
        
        tables
    }
    
    // Getters
    pub fn config(&self) -> &TableConfig {
        &self.config
    }
    
    pub fn config_mut(&mut self) -> &mut TableConfig {
        &mut self.config
    }
    
    pub fn prepare(&self) -> &Prepare {
        &self.prepare
    }
    
    pub fn prepare_mut(&mut self) -> &mut Prepare {
        &mut self.prepare
    }
    
    pub fn output(&self) -> &Output {
        &self.output
    }
    
    pub fn output_mut(&mut self) -> &mut Output {
        &mut self.output
    }
    
    pub fn combi(&self) -> &Combi {
        &self.combi
    }
    
    pub fn combi_mut(&mut self) -> &mut Combi {
        &mut self.combi
    }
    
    pub fn concat(&self) -> &Concat {
        &self.concat
    }
    
    pub fn concat_mut(&mut self) -> &mut Concat {
        &mut self.concat
    }
    
    pub fn hoechste_zeile(&self) -> (i32, i32) {
        self.config.hoechste_zeile
    }
    
    pub fn set_hoechste_zeile(&mut self, value: i32) {
        self.config.hoechste_zeile = (value, value);
    }
    
    pub fn text_width(&self) -> i32 {
        self.config.text_width
    }
    
    pub fn set_text_width(&mut self, value: i32) {
        self.config.text_width = value;
    }
    
    pub fn text_height(&self) -> i32 {
        self.config.text_height
    }
    
    pub fn set_text_height(&mut self, value: i32) {
        self.config.text_height = value;
    }
    
    pub fn nummeriere(&self) -> bool {
        self.config.nummeriere
    }
    
    pub fn set_nummeriere(&mut self, value: bool) {
        self.config.nummeriere = value;
    }
    
    pub fn keine_ueberschriften(&self) -> bool {
        self.keine_ueberschriften
    }
    
    pub fn set_keine_ueberschriften(&mut self, value: bool) {
        self.keine_ueberschriften = value;
    }
    
    pub fn keine_leeren_inhalte(&self) -> bool {
        self.keine_leeren_inhalte
    }
    
    pub fn set_keine_leeren_inhalte(&mut self, value: bool) {
        self.keine_leeren_inhalte = value;
    }
    
    pub fn spalte_gestirn(&self) -> bool {
        self.spalte_gestirn
    }
    
    pub fn set_spalte_gestirn(&mut self, value: bool) {
        self.spalte_gestirn = value;
    }
    
    pub fn breitenn(&self) -> &[i32] {
        &self.config.breiten
    }
    
    pub fn set_breitenn(&mut self, value: Vec<i32>) {
        self.config.breiten = value;
    }
    
    pub fn if_zeilen_setted(&self) -> bool {
        self.if_zeilen_setted
    }
    
    pub fn set_if_zeilen_setted(&mut self, value: bool) {
        self.if_zeilen_setted = value;
    }
    
    pub fn if_prim_multis(&self) -> bool {
        self.if_prim_multis
    }
    
    pub fn set_if_prim_multis(&mut self, value: bool) {
        self.if_prim_multis = value;
    }
    
    pub fn gener_rows(&self) -> &OrderedSet<i32> {
        &self.gener_rows
    }
    
    pub fn set_gener_rows(&mut self, value: OrderedSet<i32>) {
        self.gener_rows = value;
    }
    
    pub fn religion_numbers(&self) -> &[i32] {
        &self.religion_numbers
    }
    
    pub fn set_religion_numbers(&mut self, value: Vec<i32>) {
        self.religion_numbers = value;
    }
    
    pub fn rows_of_combi(&self) -> &OrderedSet<i32> {
        &self.rows_of_combi
    }
    
    pub fn set_rows_of_combi(&mut self, value: OrderedSet<i32>) {
        self.rows_of_combi = value;
    }
    
    pub fn spalten_vanilla_amount(&self) -> i32 {
        self.spalten_vanilla_amount
    }
    
    pub fn set_spalten_vanilla_amount(&mut self, value: i32) {
        self.spalten_vanilla_amount = value;
    }
    
    pub fn data_dict(&self) -> &[OrderedDict<i32, serde_json::Value>] {
        &self.data_dict
    }
    
    pub fn set_data_dict(&mut self, value: Vec<OrderedDict<i32, serde_json::Value>>) {
        self.data_dict = value;
    }
    
    pub fn syntax_type(&self) -> SyntaxType {
        self.syntax_type
    }
    
    pub fn set_syntax_type(&mut self, value: SyntaxType) {
        self.syntax_type = value;
    }
    
    pub fn last_line_number(&self) -> i32 {
        self.last_line_number
    }
    
    pub fn set_last_line_number(&mut self, value: i32) {
        self.last_line_number = value;
    }
    
    pub fn table_reduced_in_lines_by_type_set(
        &self,
        table: &Table,
        lines_allowed: &BTreeSet<usize>,
    ) -> Table {
        let mut new_table = Table::new();
        for (i, row) in table.iter().enumerate() {
            if lines_allowed.contains(&i) {
                new_table.push(row.clone());
            }
        }
        new_table
    }
    
    pub fn fill_both(mut liste1: Vec<String>, mut liste2: Vec<String>) -> (Vec<String>, Vec<String>) {
        while liste1.len() < liste2.len() {
            liste1.push(String::new());
        }
        while liste2.len() < liste1.len() {
            liste2.push(String::new());
        }
        (liste1, liste2)
    }
}

// Output submodule
pub mod output {
    use super::*;
    use crate::utils;
    
    #[derive(Debug)]
    pub struct Output {
        txt: String,
        resulting_table: Vec<String>,
        color: bool,
        one_table: bool,
        breiten: Vec<i32>,
        nummerierung: bool,
        textheight: i32,
        textwidth: i32,
        religion_numbers: Vec<i32>,
        rows_as_numbers: OrderedSet<i32>,
        tables: Option<*const Tables>, // Raw pointer for self-reference
    }
    
    impl Output {
        pub fn new(txt: String) -> Self {
            Self {
                txt,
                resulting_table: Vec::new(),
                color: true,
                one_table: false,
                breiten: Vec::new(),
                nummerierung: true,
                textheight: 0,
                textwidth: 21,
                religion_numbers: Vec::new(),
                rows_as_numbers: OrderedSet::new(),
                tables: None,
            }
        }
        
        pub fn set_tables(&mut self, tables: &Tables) {
            self.tables = Some(tables as *const Tables);
        }
        
        fn tables(&self) -> &Tables {
            unsafe { &*self.tables.unwrap() }
        }
        
        pub fn color(&self) -> bool {
            self.color
        }
        
        pub fn set_color(&mut self, value: bool) {
            self.color = value;
        }
        
        pub fn one_table(&self) -> bool {
            self.one_table
        }
        
        pub fn set_one_table(&mut self, value: bool) {
            self.one_table = value;
        }
        
        pub fn rows_as_numbers(&self) -> &OrderedSet<i32> {
            &self.rows_as_numbers
        }
        
        pub fn set_rows_as_numbers(&mut self, value: OrderedSet<i32>) {
            self.rows_as_numbers = value;
        }
        
        pub fn only_that_columns(&self, table: &Table, only_that_columns: &[usize]) -> Table {
            if only_that_columns.is_empty() {
                return table.clone();
            }
            
            let mut new_table = Table::new();
            for row in table {
                let mut new_row = Row::new();
                for &col_idx in only_that_columns {
                    if col_idx > 0 && col_idx <= row.len() {
                        new_row.push(row[col_idx - 1].clone());
                    }
                }
                if !new_row.is_empty() {
                    new_table.push(new_row);
                }
            }
            
            if new_table.is_empty() {
                table.clone()
            } else {
                new_table
            }
        }
        
        pub fn cli_out(
            &mut self,
            finally_display_lines: &BTreeSet<i32>,
            new_table: &Table,
            numlen: usize,
            rows_range: std::ops::Range<usize>,
        ) -> Vec<String> {
            // Implementation of complex output logic
            let mut result = Vec::new();
            
            // Simple implementation for now
            for (row_idx, row) in new_table.iter().enumerate() {
                let mut line_parts = Vec::new();
                
                if self.nummerierung && row_idx > 0 {
                    line_parts.push(format!("{:>numlen$}", row_idx));
                }
                
                for (col_idx, cell) in row.iter().enumerate() {
                    let content = if !cell.lines.is_empty() {
                        cell.lines[0].clone()
                    } else {
                        "".to_string()
                    };
                    line_parts.push(content);
                }
                
                result.push(line_parts.join(" | "));
            }
            
            self.resulting_table = result.clone();
            result
        }
    }
}

// Prepare submodule
pub mod prepare {
    use super::*;
    
    #[derive(Debug)]
    pub struct Prepare {
        ifprimmultis: bool,
        if_zeilen_setted: bool,
        breiten: Vec<i32>,
        nummerierung: bool,
        text_width: i32,
        religion_numbers: Vec<i32>,
        rows_as_numbers: OrderedSet<i32>,
        tables: Option<*const Tables>,
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
                rows_as_numbers: OrderedSet::new(),
                tables: None,
            }
        }
        
        pub fn set_tables(&mut self, tables: &Tables) {
            self.tables = Some(tables as *const Tables);
        }
        
        pub fn ifprimmultis(&self) -> bool {
            self.ifprimmultis
        }
        
        pub fn set_ifprimmultis(&mut self, value: bool) {
            self.ifprimmultis = value;
        }
        
        pub fn if_zeilen_setted(&self) -> bool {
            self.if_zeilen_setted
        }
        
        pub fn set_if_zeilen_setted(&mut self, value: bool) {
            self.if_zeilen_setted = value;
        }
        
        pub fn rows_as_numbers(&self) -> &OrderedSet<i32> {
            &self.rows_as_numbers
        }
        
        pub fn set_rows_as_numbers(&mut self, value: OrderedSet<i32>) {
            self.rows_as_numbers = value;
        }
        
        pub fn parameters_cmd_with_some_bereich(
            &self,
            s: &str,
            cmd_type: &str,
            neg: &str,
            keine_neg_beruecksichtigung: bool,
        ) -> OrderedSet<String> {
            let mut result = OrderedSet::new();
            
            // Parse range specifications
            for part in s.split(',') {
                let part = part.trim();
                if part.is_empty() {
                    continue;
                }
                
                match cmd_type {
                    "n" => {
                        // Number range
                        if let Ok(range) = RangeSpec::parse(part) {
                            let numbers = range.to_numbers(1000); // Default max
                            for num in numbers {
                                result.insert(num.to_string());
                            }
                        }
                    }
                    "^" => {
                        // Powers
                        if let Ok(num) = part.parse::<i32>() {
                            result.insert(format!("{}^{}", num, 2)); // Default power 2
                        }
                    }
                    "b" => {
                        // Multiples
                        if let Ok(num) = part.parse::<i32>() {
                            result.insert(format!("{}b", num));
                        }
                    }
                    _ => {
                        result.insert(part.to_string());
                    }
                }
            }
            
            result
        }
        
        pub fn delete_doubles_in_sets(
            &self,
            set1: OrderedSet<String>,
            set2: OrderedSet<String>,
        ) -> (OrderedSet<String>, OrderedSet<String>) {
            let diff1: OrderedSet<String> = set1.difference(&set2).cloned().collect();
            let diff2: OrderedSet<String> = set2.difference(&set1).cloned().collect();
            (diff1, diff2)
        }
        
        pub fn prepare4out_before_for_loop_spalten_zeilen_bestimmen(
            &self,
            relitable: &Table,
            param_lines: &OrderedSet<String>,
            param_lines_not: &OrderedSet<String>,
        ) -> (OrderedSet<i32>, usize, Table, usize, std::ops::Range<usize>) {
            // Simplified implementation
            let finally_display_lines: OrderedSet<i32> = (0..relitable.len() as i32).collect();
            let headings_amount = if !relitable.is_empty() { relitable[0].len() } else { 0 };
            let newer_table = relitable.clone();
            let numlen = 3; // Default
            let rows_range = 0..10; // Default range
            
            (finally_display_lines, headings_amount, newer_table, numlen, rows_range)
        }
        
        pub fn prepare4out(
            &self,
            param_lines: &OrderedSet<String>,
            param_lines_not: &OrderedSet<String>,
            relitable: &Table,
            rows_as_numbers: &OrderedSet<i32>,
            gebr_spalten: &HashMap<String, OrderedDict<i32, Vec<String>>>,
            prim_spalten: &OrderedDict<i32, Vec<String>>,
        ) -> (
            OrderedSet<i32>,
            Table,
            usize,
            std::ops::Range<usize>,
            (Vec<i32>, Vec<i32>),
        ) {
            // Simplified implementation
            let finally_display_lines: OrderedSet<i32> = (0..relitable.len() as i32).collect();
            let table = relitable.clone();
            let numlen = 3;
            let rows_range = 0..10;
            let old2new_table = (Vec::new(), Vec::new());
            
            (finally_display_lines, table, numlen, rows_range, old2new_table)
        }
        
        pub fn zeile_which_zaehlung(&self, zeile: i32) -> i32 {
            // Simple implementation - just return the line number
            zeile
        }
        
        pub fn cell_work(&self, content: &str, certain_text_width: i32) -> Vec<String> {
            let width = certain_text_width as usize;
            let mut result = Vec::new();
            
            if width == 0 {
                return vec![content.to_string()];
            }
            
            let mut current_line = String::new();
            for word in content.split_whitespace() {
                if current_line.len() + word.len() + 1 > width && !current_line.is_empty() {
                    result.push(current_line);
                    current_line = String::new();
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
}

// Combi submodule
pub mod combi {
    use super::*;
    use std::fs::File;
    use std::io::{BufRead, BufReader};
    
    #[derive(Debug)]
    pub struct Combi {
        sum_of_all_combi_rows_amount: i32,
        religion_numbers: Vec<i32>,
        rows_of_combi: OrderedSet<i32>,
        tables: Option<*const Tables>,
    }
    
    impl Combi {
        pub fn new() -> Self {
            Self {
                sum_of_all_combi_rows_amount: 0,
                religion_numbers: Vec::new(),
                rows_of_combi: OrderedSet::new(),
                tables: None,
            }
        }
        
        pub fn set_tables(&mut self, tables: &Tables) {
            self.tables = Some(tables as *const Tables);
        }
        
        pub fn sum_of_all_combi_rows_amount(&self) -> i32 {
            self.sum_of_all_combi_rows_amount
        }
        
        pub fn prepare_kombi(
            &self,
            finally_display_lines: &BTreeSet<i32>,
            kombi_table: &Table,
            param_lines: &BTreeSet<String>,
            displaying_zeilen: &BTreeSet<i32>,
            kombi_table_kombis: &[Vec<i32>],
        ) -> OrderedDict<i32, OrderedSet<i32>> {
            let mut chosen_kombi_lines = OrderedDict::
