// src/syntax/bbcode.rs
use super::OutputSyntax;
use crate::types::{OutputSyntaxType, ParameterValue};
use std::collections::HashMap;

pub struct BBCodeSyntax;

impl OutputSyntax for BBCodeSyntax {
    fn syntax_type(&self) -> OutputSyntaxType {
        OutputSyntaxType::BBCode
    }
    
    fn begin_table(&self) -> String {
        "[table]\n".to_string()
    }
    
    fn end_table(&self) -> String {
        "[/table]\n".to_string()
    }
    
    fn generate_cell(
        &self,
        cell_index: i32,
        _generated_spalten_parameter: &HashMap<i32, ParameterValue>,
        zeile: i32,
        _tables: &dyn std::any::Any,
    ) -> String {
        if cell_index == -2 {
            format!("[th class=\"zeile-{}\"]", zeile)
        } else if cell_index == -1 {
            format!("[td class=\"nummer-{}\"]", zeile)
        } else {
            format!("[td class=\"spalte-{} zeile-{}\"]", cell_index, zeile)
        }
    }
    
    fn end_cell(&self) -> String {
        if self.syntax_type() == OutputSyntaxType::BBCode && self.generate_cell(0, &HashMap::new(), 0, &()) == "[th" {
            "[/th]".to_string()
        } else {
            "[/td]".to_string()
        }
    }
    
    fn colored_begin_col(&self, zeile: i32) -> String {
        format!("[tr class=\"zeile-{}\"]", zeile)
    }
    
    fn end_zeile(&self) -> String {
        "[/tr]\n".to_string()
    }
    
    fn format_cell_content(&self, content: &str, _width: usize) -> String {
        content.to_string()
    }
}
