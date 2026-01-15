// src/tables/combi.rs
use crate::types::*;
use std::collections::{BTreeSet, HashMap};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

#[derive(Debug)]
pub struct Combi {
    sum_of_all_combi_rows_amount: i32,
    religion_numbers: Vec<i32>,
    rows_of_combi: BTreeSet<i32>,
    tables: Option<*const Tables>, // Raw pointer for self-reference
}

impl Combi {
    pub fn new() -> Self {
        Self {
            sum_of_all_combi_rows_amount: 0,
            religion_numbers: Vec::new(),
            rows_of_combi: BTreeSet::new(),
            tables: None,
        }
    }
    
    pub fn set_tables(&mut self, tables: &Tables) {
        self.tables = Some(tables as *const Tables);
    }
    
    fn tables(&self) -> &Tables {
        unsafe { &*self.tables.unwrap() }
    }
    
    pub fn prepare_table_join(
        &self,
        chosen_kombi_lines: &HashMap<i32, BTreeSet<i32>>,
        new_table_kombi: &Table,
    ) -> Vec<HashMap<i32, Table>> {
        let mut kombi_tables = Vec::new();
        
        for (&key, value) in chosen_kombi_lines {
            let mut tables_map = HashMap::new();
            
            for &kombi_line_number in value {
                let into = self.tables().table_reduced_in_lines_by_type_set(
                    new_table_kombi,
                    &BTreeSet::from([kombi_line_number as usize]),
                );
                
                if !into.is_empty() {
                    tables_map
                        .entry(key)
                        .or_insert_with(Vec::new)
                        .push(into[0].clone());
                }
            }
            
            if !tables_map.is_empty() {
                kombi_tables.push(tables_map);
            }
        }
        
        kombi_tables
    }
    
    pub fn remove_one_number(&self, hinein: &[String], col_num: i32) -> Vec<String> {
        if hinein.is_empty() {
            return hinein.to_vec();
        }
        
        let mut hinein_str = hinein.join("");
        
        // Find parentheses content
        if let Some(start) = hinein_str.find('(') {
            if let Some(end) = hinein_str[start..].find(')') {
                let full_end = start + end;
                let substr = &hinein_str[start + 1..full_end];
                
                if !substr.is_empty() {
                    let substr_list_a: Vec<&str> = substr.split('|').collect();
                    let mut substr_list = Vec::new();
                    
                    for el in substr_list_a {
                        if el.starts_with('(') && el.ends_with(')') {
                            substr_list.push(&el[1..el.len() - 1]);
                        } else {
                            substr_list.push(el);
                        }
                    }
                    
                    let mut new_num_str_list = Vec::new();
                    for list_el in substr_list {
                        let parts: Vec<&str> = list_el.split('/').collect();
                        let mut num_list_part = Vec::new();
                        
                        for part in parts {
                            if part.trim().is_empty() {
                                continue;
                            }
                            if let Ok(num) = part.parse::<i32>() {
                                if num.abs() != col_num.abs() || parts.len() != 1 {
                                    num_list_part.push(part);
                                }
                            }
                        }
                        
                        if !num_list_part.is_empty() {
                            new_num_str_list.push(num_list_part.join("/"));
                        }
                    }
                    
                    if !new_num_str_list.is_empty() {
                        let new_num_list_str = new_num_str_list.join("|");
                        let result_str = format!("({}){}", new_num_list_str, &hinein_str[full_end..]);
                        
                        if self.tables().text_width() != 0 {
                            // Apply cell work
                            vec![result_str.replace("\n", "; ")]
                        } else {
                            vec![result_str]
                        }
                    } else {
                        vec![hinein_str[full_end + 1..].to_string()]
                    }
                } else {
                    hinein.to_vec()
                }
            } else {
                hinein.to_vec()
            }
        } else {
            hinein.to_vec()
        }
    }
    
    pub fn table_join(
        &self,
        main_table: &mut Table,
        many_sub_tables: &[HashMap<i32, Table>],
        maintable2subtable_relation: &(HashMap<i32, i32>, HashMap<i32, i32>),
        old2new_rows: &(Vec<i32>, Vec<i32>),
        rows_of_combi: &BTreeSet<i32>,
    ) {
        let rows_of_combi_list: Vec<i32> = rows_of_combi.iter().copied().collect();
        let (relation_map, _) = maintable2subtable_relation;
        
        let one_line_per_line = matches!(
            self.tables().out_type().syntax_type(),
            OutputSyntaxType::Html | OutputSyntaxType::BBCode
        );
        
        for (col_num, col) in main_table.iter_mut().enumerate() {
            let reli_num = if col_num < self.religion_numbers.len() {
                self.religion_numbers[col_num]
            } else {
                continue;
            };
            
            for sub_table in many_sub_tables {
                if let Some(sub_table_cells) = sub_table.get(&reli_num) {
                    for (row_idx, big_cell) in col.iter_mut().enumerate() {
                        let old_row_num = if row_idx < old2new_rows.1.len() {
                            old2new_rows.1[row_idx]
                        } else {
                            continue;
                        };
                        
                        if let Some(&sub_row_num) = relation_map.get(&old_row_num) {
                            for sub_table_cell in sub_table_cells {
                                if let Some(cell_idx) = rows_of_combi_list.iter().position(|&x| x == sub_row_num + 1) {
                                    if cell_idx < sub_table_cell.len() && !sub_table_cell[cell_idx].lines.is_empty() {
                                        let mut hinein = sub_table_cell[cell_idx].lines.clone();
                                        hinein = self.remove_one_number(&hinein, reli_num);
                                        
                                        if one_line_per_line {
                                            if !hinein.is_empty() && hinein[0].trim().len() > 2 {
                                                let formatted = match self.tables().out_type().syntax_type() {
                                                    OutputSyntaxType::Html => {
                                                        format!("<li>{}</li>", hinein[0])
                                                    }
                                                    OutputSyntaxType::BBCode => {
                                                        format!("[*]{}", hinein[0])
                                                    }
                                                    _ => {
                                                        format!("{} |", hinein[0])
                                                    }
                                                };
                                                hinein[0] = formatted;
                                            }
                                            
                                            if big_cell.lines.len() == 1 && big_cell.lines[0].is_empty() {
                                                big_cell.lines = hinein;
                                            } else if !hinein.is_empty() {
                                                let last_idx = big_cell.lines.len() - 1;
                                                big_cell.lines[last_idx].push_str(&hinein[0]);
                                            }
                                        } else {
                                            if big_cell.lines.len() == 1 && big_cell.lines[0].is_empty() {
                                                big_cell.lines = hinein;
                                            } else {
                                                big_cell.lines.extend(hinein);
                                            }
                                        }
                                    }
                                }
                            }
                            
                            // Post-process for HTML/BBcode
                            if one_line_per_line {
                                match self.tables().out_type().syntax_type() {
                                    OutputSyntaxType::Html => {
                                        for line in &mut big_cell.lines {
                                            *line = format!("<ul>{}</ul>", line);
                                        }
                                    }
                                    OutputSyntaxType::BBCode => {
                                        for line in &mut big_cell.lines {
                                            *line = format!("[list]{}[/list]", line);
                                        }
                                    }
                                    _ => {}
                                }
                            } else if self.tables().text_width() == 0 {
                                big_cell.lines = vec![big_cell.lines.join(" | ")];
                            }
                        }
                    }
                }
            }
        }
    }
    
    pub fn read_kombi_csv(
        &mut self,
        relitable: &Table,
        rows_as_numbers: &mut BTreeSet<i32>,
        rows_of_combi: &BTreeSet<i32>,
        csv_file_name: &str,
    ) -> Result<(Table, Table, Vec<Vec<i32>>, (HashMap<i32, i32>, HashMap<i32, i32>)), Box<dyn std::error::Error>> {
        let path = Path::new(csv_file_name);
        
        if rows_of_combi.is_empty() {
            return Ok((
                vec![vec![]],
                relitable.clone(),
                vec![vec![]],
                (HashMap::new(), HashMap::new())
            ));
        }
        
        self.sum_of_all_combi_rows_amount += rows_of_combi.len() as i32;
        
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        
        let mut kombi_table = Table::new();
        let mut kombi_table_kombis = Vec::new();
        
        for (z, line) in reader.lines().enumerate() {
            let line = line?;
            let mut row_cells: Vec<Cell> = line.split(';')
                .map(|s| Cell::new(s.trim()))
                .collect();
            
            // Process content as in Python code
            if row_cells.len() > 1 && !row_cells[0].lines[0].is_empty() {
                for i in 1..row_cells.len() {
                    if !row_cells[i].lines[0].is_empty() {
                        let content = format!("({}) {} ({})", 
                            row_cells[0].lines[0], 
                            row_cells[i].lines[0],
                            row_cells[0].lines[0]
                        );
                        row_cells[i] = Cell::new(&content);
                    }
                }
            }
            
            kombi_table.push(row_cells.clone());
            
            // Process combinations from first column
            let mut kombis = Vec::new();
            if !row_cells.is_empty() && z > 0 {
                let first_col = &row_cells[0].lines[0];
                self.kombi_numbers_correct_test_and_set(first_col, &mut kombis)?;
            }
            kombi_table_kombis.push(kombis);
        }
        
        // Merge with relitable
        let mut new_relitable = relitable.clone();
        let headings_amount = if !new_relitable.is_empty() { new_relitable[0].len() } else { 0 };
        
        let mut maintable2subtable_relation = (HashMap::new(), HashMap::new());
        
        if !kombi_table.is_empty() {
            for i in 0..kombi_table.len().max(new_relitable.len()) {
                if i == 0 && !new_relitable.is_empty() {
                    // Add headers from kombi table
                    if i < kombi_table.len() && kombi_table[i].len() > 1 {
                        let kombi_headers: Vec<Cell> = kombi_table[i][1..].to_vec();
                        
                        for (t, _) in kombi_headers.iter().enumerate() {
                            maintable2subtable_relation.0.insert(
                                (new_relitable[0].len() + t) as i32,
                                t as i32
                            );
                            maintable2subtable_relation.1.insert(
                                t as i32,
                                (new_relitable[0].len() + t) as i32
                            );
                        }
                        
                        new_relitable[0].extend(kombi_headers);
                    }
                } else if i < new_relitable.len() {
                    // Add empty cells for kombi columns
                    let kombi_cols = if i < kombi_table.len() && kombi_table[i].len() > 1 {
                        kombi_table[i][1..].len()
                    } else {
                        0
                    };
                    new_relitable[i].extend(vec![Cell::new(""); kombi_cols]);
                }
            }
        }
        
        Ok((kombi_table, new_relitable, kombi_table_kombis, maintable2subtable_relation))
    }
    
    fn kombi_numbers_correct_test_and_set(
        &self,
        num_str: &str,
        result: &mut Vec<i32>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let trimmed = num_str.trim();
        
        if trimmed.starts_with('(') && trimmed.ends_with(')') {
            self.kombi_numbers_correct_test_and_set(&trimmed[1..trimmed.len()-1], result)?;
            return Ok(());
        }
        
        if let Ok(num) = trimmed.parse::<i32>() {
            result.push(num.abs());
        } else if trimmed.starts_with('+') || trimmed.starts_with('-') {
            if let Ok(num) = trimmed.parse::<i32>() {
                result.push(num.abs());
            }
        } else if trimmed.contains('/') {
            let parts: Vec<&str> = trimmed.split('/').collect();
            for part in parts {
                self.kombi_numbers_correct_test_and_set(part, result)?;
            }
        } else {
            return Err(format!("Invalid kombi number: {}", trimmed).into());
        }
        
        Ok(())
    }
}
