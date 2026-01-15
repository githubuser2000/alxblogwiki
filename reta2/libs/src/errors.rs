use thiserror::Error;
use std::path::PathBuf;

#[derive(Error, Debug)]
pub enum ConcatError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    
    #[error("CSV error: {0}")]
    Csv(#[from] csv::Error),
    
    #[error("Parse error: {0}")]
    Parse(String),
    
    #[error("Duplicate generated spalten parameter index")]
    DuplicateIndex,
    
    #[error("Invalid row index: {0}")]
    InvalidRowIndex(usize),
    
    #[error("Invalid column index: {0}")]
    InvalidColumnIndex(usize),
    
    #[error("File not found: {0}")]
    FileNotFound(PathBuf),
    
    #[error("Configuration error: {0}")]
    Config(String),
    
    #[error("Unknown CSV type: {0}")]
    UnknownCsvType(usize),
}

pub type Result<T> = std::result::Result<T, ConcatError>;
