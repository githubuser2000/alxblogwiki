// src/main.rs
use tables::Tables;
use tables::syntax::{MarkdownSyntax, HtmlSyntax, BBCodeSyntax};

fn main() {
    // Example usage
    let mut tables = Tables::new(Some(1024), "Example Table".to_string());
    
    // Set output type
    tables.set_out_type(Box::new(MarkdownSyntax));
    
    // Configure table
    tables.set_breitenn(vec![10, 20, 15]);
    tables.set_nummeriere(true);
    
    // Create a simple table
    let mut table = vec![
        vec![
            crate::types::Cell::new("Header 1"),
            crate::types::Cell::new("Header 2"),
            crate::types::Cell::new("Header 3"),
        ],
        vec![
            crate::types::Cell::new("Row 1 Col 1"),
            crate::types::Cell::new("Row 1 Col 2"),
            crate::types::Cell::new("Row 1 Col 3"),
        ],
        vec![
            crate::types::Cell::new("Row 2 Col 1"),
            crate::types::Cell::from_lines(vec![
                "Multi-line".to_string(),
                "content".to_string(),
            ]),
            crate::types::Cell::new("Row 2 Col 3"),
        ],
    ];
    
    // Output the table
    let display_lines = std::collections::BTreeSet::from([0, 1, 2]);
    let output = tables.get_out.cli_out(
        &display_lines,
        &table,
        3,
        0..2
    );
    
    for line in output {
        println!("{}", line);
    }
}
