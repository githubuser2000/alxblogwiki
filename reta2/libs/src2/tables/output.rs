// src/tables/output.rs
use crate::syntax::OutputSyntax;
use crate::types::*;
use std::collections::{BTreeSet, HashMap};

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
            tables: None,
        }
    }
    
    pub fn set_tables(&mut self, tables: &Tables) {
        self.tables = Some(tables as *const Tables);
    }
    
    fn tables(&self) -> &Tables {
        unsafe { &*self.tables.unwrap() }
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
        finally_display_lines_set: &BTreeSet<i32>,
        new_table: &Table,
        numlen: usize,
        rows_range: std::ops::Range<usize>,
    ) -> Vec<String> {
        if finally_display_lines_set.is_empty() || 
           (finally_display_lines_set.len() == 1 && finally_display_lines_set.contains(&0)) {
            return Vec::new();
        }
        
        let mut finally_display_lines: Vec<i32> = finally_display_lines_set.iter().copied().collect();
        finally_display_lines.sort();
        
        // Find max cell text lengths
        let max_cell_text_len = self.find_max_cell_text_len(&finally_display_lines, new_table, &rows_range);
        
        let mut result = Vec::new();
        let out_type = self.tables().out_type();
        
        // Begin table if needed
        if matches!(out_type.syntax_type(), OutputSyntaxType::Html | OutputSyntaxType::BBCode) {
            result.push(out_type.begin_table());
        }
        
        // Process rows
        for (big_cell_line_number, &filtered_line_number) in finally_display_lines.iter().enumerate() {
            if big_cell_line_number == 0 && self.tables().config.keine_ueberschriften {
                continue;
            }
            
            for iter_whole_line in rows_range.clone() {
                // Build line with row number if needed
                let mut line_parts = Vec::new();
                
                if self.nummerierung && filtered_line_number > 0 {
                    let cell_content = if filtered_line_number % 2 == 0 {
                        "●"
                    } else {
                        " "
                    };
                    
                    line_parts.push(format!(
                        "{}{}{}",
                        out_type.generate_cell(-2, &HashMap::new(), filtered_line_number, self.tables()),
                        cell_content,
                        out_type.end_cell()
                    ));
                }
                
                // Add content for each cell
                for (cell_index, cell) in new_table[big_cell_line_number].iter().enumerate() {
                    let cell_width = self.determine_row_width(cell_index, &max_cell_text_len);
                    
                    if let Some(cell_line) = cell.lines.get(iter_whole_line) {
                        let formatted = if self.color && matches!(out_type.syntax_type(), OutputSyntaxType::Default) {
                            self.colorize(cell_line, filtered_line_number, false)
                        } else {
                            out_type.format_cell_content(cell_line, cell_width)
                        };
                        
                        line_parts.push(format!(
                            "{}{}{}",
                            out_type.generate_cell(cell_index as i32, &HashMap::new(), filtered_line_number, self.tables()),
                            formatted,
                            out_type.end_cell()
                        ));
                    }
                }
                
                // Join line parts
                let line = if matches!(out_type.syntax_type(), OutputSyntaxType::Csv) {
                    line_parts.join(";")
                } else {
                    format!(
                        "{}{}{}",
                        out_type.colored_begin_col(filtered_line_number),
                        line_parts.join(" "),
                        out_type.end_zeile()
                    )
                };
                
                result.push(line);
            }
            
            // Add separator for markdown
            if matches!(out_type.syntax_type(), OutputSyntaxType::Markdown) && big_cell_line_number == 0 {
                let sep_count = new_table[0].len() + if self.nummerierung { 2 } else { 0 };
                result.push(format!("|{}|", ":--:|".repeat(sep_count)));
            }
        }
        
        // End table if needed
        if matches!(out_type.syntax_type(), OutputSyntaxType::Html | OutputSyntaxType::BBCode) {
            result.push(out_type.end_table());
        }
        
        self.resulting_table = result.clone();
        result
    }
    
    fn find_max_cell_text_len(
        &self,
        finally_display_lines: &[i32],
        table: &Table,
        rows_range: &std::ops::Range<usize>,
    ) -> HashMap<usize, usize> {
        let mut max_len = HashMap::new();
        
        for (line_idx, &_line_num) in finally_display_lines.iter().enumerate() {
            for cell_idx in 0..table[line_idx].len() {
                for line_in_cell in rows_range.clone() {
                    if let Some(cell_line) = table[line_idx][cell_idx].lines.get(line_in_cell) {
                        let entry = max_len.entry(cell_idx).or_insert(0);
                        *entry = (*entry).max(cell_line.len());
                    }
                }
            }
        }
        
        max_len
    }
    
    fn determine_row_width(&self, cell_index: usize, max_cell_text_len: &HashMap<usize, usize>) -> usize {
        let default_width = self.textwidth as usize;
        
        if cell_index < self.breiten.len() {
            let certain_width = self.breiten[cell_index] as usize;
            let max_len = max_cell_text_len.get(&cell_index).copied().unwrap_or(0);
            
            if certain_width > max_len || (certain_width == 0 && !self.tables().bbcode_output_yes() 
                && !self.tables().html_output_yes()) {
                max_len
            } else {
                certain_width
            }
        } else {
            default_width
        }
    }
    
    pub fn colorize(&self, text: &str, num: i32, rest: bool) -> String {
        let num = if num < 0 { 0 } else { num };
        
        if num == 0 {
            format!("\x1b[41m\x1b[30m\x1b[4m{}\x1b[0m", text)
        } else if rest {
            if num % 2 == 0 {
                format!("\x1b[47m\x1b[30m{}\x1b[0m\x1b[0m", text)
            } else {
                format!("\x1b[40m\x1b[37m{}\x1b[0m\x1b[0m", text)
            }
        } else {
            let (_, moon_factors) = moon_number(num);
            let prime_factors = prime_factors(num);
            
            if !moon_factors.is_empty() {
                if num % 2 == 0 {
                    format!("\x1b[106m\x1b[30m{}\x1b[0m\x1b[0m", text)
                } else {
                    format!("\x1b[46m\x1b[30m{}\x1b[0m\x1b[0m", text)
                }
            } else if prime_factors.len() == 1 {
                if num % 2 == 0 {
                    format!("\x1b[103m\x1b[30m\x1b[1m{}\x1b[0m", text)
                } else {
                    format!("\x1b[43m\x1b[30m{}\x1b[0m\x1b[0m", text)
                }
            } else if num % 2 == 0 {
                format!("\x1b[47m\x1b[30m{}\x1b[0m\x1b[0m", text)
            } else {
                format!("\x1b[100m\x1b[37m{}\x1b[0m\x1b[0m", text)
            }
        }
    }
    
    pub fn cliout2(&mut self, text: String) {
        self.resulting_table.push(text.clone());
        
        if !self.tables().nichts_output_yes() {
            // Actual output would go here
            println!("{}", text);
        }
    }
}
