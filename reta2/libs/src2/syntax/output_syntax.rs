// src/syntax/output_syntax.rs
use crate::types::{OutputSyntaxType, SpaltenTag};
use std::collections::{HashMap, BTreeSet};

pub trait OutputSyntax: Send + Sync {
    fn syntax_type(&self) -> OutputSyntaxType;
    
    fn begin_table(&self) -> String;
    fn end_table(&self) -> String;
    
    fn generate_cell(
        &self,
        cell_index: i32,
        generated_spalten_parameter: &HashMap<i32, crate::types::ParameterValue>,
        zeile: i32,
        tables: &dyn std::any::Any,
    ) -> String;
    
    fn end_cell(&self) -> String;
    
    fn colored_begin_col(&self, zeile: i32) -> String;
    fn end_zeile(&self) -> String;
    
    fn format_cell_content(&self, content: &str, width: usize) -> String;
}

#[derive(Default)]
pub struct DefaultOutputSyntax;

impl OutputSyntax for DefaultOutputSyntax {
    fn syntax_type(&self) -> OutputSyntaxType {
        OutputSyntaxType::Default
    }
    
    fn begin_table(&self) -> String {
        String::new()
    }
    
    fn end_table(&self) -> String {
        String::new()
    }
    
    fn generate_cell(
        &self,
        _cell_index: i32,
        _generated_spalten_parameter: &HashMap<i32, crate::types::ParameterValue>,
        _zeile: i32,
        _tables: &dyn std::any::Any,
    ) -> String {
        String::new()
    }
    
    fn end_cell(&self) -> String {
        String::new()
    }
    
    fn colored_begin_col(&self, _zeile: i32) -> String {
        String::new()
    }
    
    fn end_zeile(&self) -> String {
        String::new()
    }
    
    fn format_cell_content(&self, content: &str, width: usize) -> String {
        format!("{:<width$}", content, width = width)
    }
}

pub struct NichtsSyntax;
impl OutputSyntax for NichtsSyntax {
    fn syntax_type(&self) -> OutputSyntaxType {
        OutputSyntaxType::Nichts
    }
    
    fn begin_table(&self) -> String { String::new() }
    fn end_table(&self) -> String { String::new() }
    fn generate_cell(&self, _: i32, _: &HashMap<i32, crate::types::ParameterValue>, _: i32, _: &dyn std::any::Any) -> String { String::new() }
    fn end_cell(&self) -> String { String::new() }
    fn colored_begin_col(&self, _: i32) -> String { String::new() }
    fn end_zeile(&self) -> String { String::new() }
    fn format_cell_content(&self, content: &str, width: usize) -> String { 
        format!("{:<width$}", content, width = width)
    }
}
