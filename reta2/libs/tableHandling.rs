// table_handling.rs
use std::collections::{BTreeSet, HashMap};
use std::io;
use std::path::Path;

// Enums for output syntax types
#[derive(Debug, Clone, Copy)]
pub enum OutputSyntax {
    Nichts,
    Markdown,
    BBCode,
    Html,
    Csv,
    Emacs,
    Default,
}

// Main Tables struct
pub struct Tables {
    hoechste_zeile: (i32, i32), // (1024, 114) format
    keine_ueberschriften: bool,
    row_num_display_to_orig: HashMap<i32, i32>,
    generated_spalten_parameter: HashMap<i32, ParameterValue>,
    get_prepare: Prepare,
    get_out: Output,
    get_main_table: MainTable,
    text_height: i32,
    text_width: i32,
    nummeriere: bool,
    spalte_gestirn: bool,
    breitenn: Vec<i32>,
    religion_numbers: Vec<i32>,
    gener_rows: BTreeSet<i32>,
}

// Sub-structs
pub struct Output<'a> {
    tables: &'a Tables,
    one_table: bool,
    color: bool,
    out_type: OutputSyntax,
    txt: String,
    resulting_table: Vec<String>,
}

pub struct Prepare {
    ifprimmultis: bool,
    if_zeilen_setted: bool,
    breiten: Vec<i32>,
    nummerierung: bool,
    text_width: i32,
    religion_numbers: Vec<i32>,
}

pub struct MainTable {
    // Fields from Python class
}

// Combi struct - handles table combinations
pub struct Combi {
    sum_of_all_combi_rows_amount: i32,
    tables: Tables,
    parameter_name: String,
    rows_of_combi: BTreeSet<i32>,
}

impl Tables {
    pub fn new(hoechst_zeil: Option<i32>, txt: String) -> Self {
        let hoechste_zeile = match hoechst_zeil {
            Some(val) => (val, val),
            None => (1024, 163),
        };

        Self {
            hoechste_zeile,
            keine_ueberschriften: false,
            row_num_display_to_orig: HashMap::new(),
            generated_spalten_parameter: HashMap::new(),
            get_prepare: Prepare::new(),
            get_out: Output::new(&self, txt), // Note: self-reference issue here - needs Rc/Arc in real implementation
            get_main_table: MainTable::new(),
            text_height: 0,
            text_width: 21,
            nummeriere: true,
            spalte_gestirn: false,
            breitenn: Vec::new(),
            religion_numbers: Vec::new(),
            gener_rows: BTreeSet::new(),
        }
    }

    // Property getters/setters
    pub fn markdown_output_yes(&self) -> bool {
        matches!(self.get_out.out_type, OutputSyntax::Markdown)
    }

    pub fn bbcode_output_yes(&self) -> bool {
        matches!(self.get_out.out_type, OutputSyntax::BBCode)
    }

    pub fn html_output_yes(&self) -> bool {
        matches!(self.get_out.out_type, OutputSyntax::Html)
    }

    pub fn nichts_output_yes(&self) -> bool {
        matches!(self.get_out.out_type, OutputSyntax::Nichts)
    }

    pub fn set_out_type(&mut self, value: OutputSyntax) {
        self.get_out.out_type = value;
    }

    pub fn get_hoechste_zeile(&self) -> (i32, i32) {
        self.hoechste_zeile
    }

    pub fn set_hoechste_zeile(&mut self, value: i32) {
        self.hoechste_zeile = (value, value);
    }

    // Static method
    pub fn fill_both(liste1: &mut Vec<String>, liste2: &mut Vec<String>) -> (Vec<String>, Vec<String>) {
        while liste1.len() < liste2.len() {
            liste1.push(String::new());
        }
        while liste2.len() < liste1.len() {
            liste2.push(String::new());
        }
        (liste1.clone(), liste2.clone())
    }

    // Method to reduce table to specific lines
    pub fn table_reduced_in_lines_by_type_set(&self, table: &[Vec<String>], lines_allowed: &BTreeSet<usize>) -> Vec<Vec<String>> {
        let mut new_table = Vec::new();
        for (i, line) in table.iter().enumerate() {
            if lines_allowed.contains(&i) {
                new_table.push(line.clone());
            }
        }
        new_table
    }
}

impl<'a> Output<'a> {
    pub fn new(tables: &'a Tables, txt: String) -> Self {
        Self {
            tables,
            one_table: false,
            color: true,
            out_type: OutputSyntax::Default,
            txt,
            resulting_table: Vec::new(),
        }
    }

    pub fn cli_out(
        &mut self,
        finally_display_lines_set: &BTreeSet<i32>,
        new_table: &[Vec<Vec<String>>],
        numlen: usize,
        rows_range: std::ops::Range<usize>,
    ) -> Vec<String> {
        // Implementation would go here
        // This is complex and would need significant work to translate
        Vec::new()
    }

    fn colorize(&self, text: &str, num: i32, rest: bool) -> String {
        if num == 0 {
            format!("\x1b[41m\x1b[30m\x1b[4m{}\x1b[0m", text)
        } else if rest {
            if num % 2 == 0 {
                format!("\x1b[47m\x1b[30m{}\x1b[0m\x1b[0m", text)
            } else {
                format!("\x1b[40m\x1b[37m{}\x1b[0m\x1b[0m", text)
            }
        } else {
            // Simplified version - in real code you'd need moonNumber and primFak equivalents
            if num % 2 == 0 {
                format!("\x1b[47m\x1b[30m{}\x1b[0m\x1b[0m", text)
            } else {
                format!("\x1b[100m\x1b[37m{}\x1b[0m\x1b[0m", text)
            }
        }
    }
}

impl Combi {
    pub fn new(tables: Tables) -> Self {
        Self {
            sum_of_all_combi_rows_amount: 0,
            tables,
            parameter_name: "kombination".to_string(),
            rows_of_combi: BTreeSet::new(),
        }
    }

    pub fn read_kombi_csv(
        &mut self,
        relitable: &[Vec<String>],
        rows_as_numbers: &mut BTreeSet<i32>,
        rows_of_combi: &BTreeSet<i32>,
        csv_file_name: &str,
    ) -> Result<(Vec<Vec<String>>, Vec<Vec<String>>, Vec<Vec<i32>>, (HashMap<i32, i32>, HashMap<i32, i32>)), Box<dyn std::error::Error>> {
        let path = Path::new(csv_file_name);
        
        if rows_of_combi.is_empty() {
            return Ok((vec![vec![]], relitable.to_vec(), vec![vec![]], (HashMap::new(), HashMap::new())));
        }

        // Read CSV file
        let mut reader = csv::ReaderBuilder::new()
            .delimiter(b';')
            .from_path(path)?;

        let mut kombi_table = Vec::new();
        let mut kombi_table_kombis = Vec::new();

        for result in reader.records() {
            let record = result?;
            let mut row: Vec<String> = record.iter().map(|s| s.to_string()).collect();
            kombi_table.push(row.clone());
            
            // Process first column for combinations
            if !row.is_empty() {
                let mut kombis = Vec::new();
                if let Some(first_col) = row.get(0) {
                    self.process_kombi_numbers(first_col, &mut kombis)?;
                }
                kombi_table_kombis.push(kombis);
            }
        }

        // Merge with relitable
        let mut new_relitable = relitable.to_vec();
        // ... rest of merging logic

        Ok((kombi_table, new_relitable, kombi_table_kombis, (HashMap::new(), HashMap::new())))
    }

    fn process_kombi_numbers(&self, num_str: &str, result: &mut Vec<i32>) -> Result<(), Box<dyn std::error::Error>> {
        let trimmed = num_str.trim();
        
        if trimmed.starts_with('(') && trimmed.ends_with(')') {
            self.process_kombi_numbers(&trimmed[1..trimmed.len()-1], result)?;
            return Ok(());
        }
        
        if let Ok(num) = trimmed.parse::<i32>() {
            result.push(num.abs());
        } else if trimmed.contains('/') {
            let parts: Vec<&str> = trimmed.split('/').collect();
            for part in parts {
                self.process_kombi_numbers(part, result)?;
            }
        } else {
            return Err(format!("Invalid kombi number: {}", trimmed).into());
        }
        
        Ok(())
    }
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
}

// Parameter value type
#[derive(Clone)]
pub enum ParameterValue {
    Single(Vec<(String, String)>),
    Double((Vec<(String, String)>, Vec<(String, String)>)),
}
