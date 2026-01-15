// src/syntax/markdown.rs
use super::OutputSyntax;
use crate::types::{OutputSyntaxType, ParameterValue};
use std::collections::HashMap;

pub struct MarkdownSyntax;

impl OutputSyntax for MarkdownSyntax {
    fn syntax_type(&self) -> OutputSyntaxType {
        OutputSyntaxType::Markdown
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
        _generated_spalten_parameter: &HashMap<i32, ParameterValue>,
        _zeile: i32,
        _tables: &dyn std::any::Any,
    ) -> String {
        "|".to_string()
    }
    
    fn end_cell(&self) -> String {
        String::new()
    }
    
    fn colored_begin_col(&self, _zeile: i32) -> String {
        String::new()
    }
    
    fn end_zeile(&self) -> String {
        "|".to_string()
    }
    
    fn format_cell_content(&self, content: &str, width: usize) -> String {
        format!(" {} ", content)
    }
}
