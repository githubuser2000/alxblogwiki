# lib4tables_concat - Rust Implementation

This is a Rust implementation of the Python `lib4tables_concat.py` module, which handles concatenation and transformation of tables with mathematical operations, CSV processing, and specialized formatting.

## Features

- **Table Concatenation**: Merge and transform tables with various operations
- **Mathematical Functions**: Prime number detection, factorization, fractions
- **CSV Processing**: Read and write CSV files with custom delimiters
- **Output Formats**: Support for HTML, BBCode, Markdown, CSV, and Emacs formats
- **Internationalization**: Multi-language support via i18n module
- **Error Handling**: Comprehensive error types and Result-based error handling

## Structure

- `src/concat.rs` - Main Concat struct and methods
- `src/center.rs` - Core functionality and i18n support
- `src/lib4tables.rs` - Table operations and mathematical functions
- `src/lib4tables_enum.rs` - Enum definitions
- `src/types.rs` - Type definitions and aliases
- `src/errors.rs` - Error types and handling

## Usage

```rust
use lib4tables_concat::*;
use std::collections::HashSet;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let tables = Tables::default();
    let mut concat = Concat::new(tables);
    
    let mut relitable = vec![vec!["Header".to_string()]];
    let mut rows_as_numbers = HashSet::new();
    
    // Perform concatenation operations
    concat.concat_love_polygon(&mut relitable, &mut rows_as_numbers)?;
    
    Ok(())
}
