use std::fmt;
use std::io;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum RetaError {
    #[error("I/O error: {0}")]
    Io(#[from] io::Error),
    
    #[error("CSV error: {0}")]
    Csv(#[from] csv::Error),
    
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),
    
    #[error("Parse error: {0}")]
    Parse(String),
    
    #[error("Invalid parameter: {0}")]
    InvalidParameter(String),
    
    #[error("File not found: {0}")]
    FileNotFound(String),
    
    #[error("Configuration error: {0}")]
    Config(String),
    
    #[error("Table error: {0}")]
    Table(String),
    
    #[error("Unknown command: {0}")]
    UnknownCommand(String),
}

impl fmt::Display for RetaError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RetaError::Io(e) => write!(f, "I/O error: {}", e),
            RetaError::Csv(e) => write!(f, "CSV error: {}", e),
            RetaError::Json(e) => write!(f, "JSON error: {}", e),
            RetaError::Parse(s) => write!(f, "Parse error: {}", s),
            RetaError::InvalidParameter(s) => write!(f, "Invalid parameter: {}", s),
            RetaError::FileNotFound(s) => write!(f, "File not found: {}", s),
            RetaError::Config(s) => write!(f, "Configuration error: {}", s),
            RetaError::Table(s) => write!(f, "Table error: {}", s),
            RetaError::UnknownCommand(s) => write!(f, "Unknown command: {}", s),
        }
    }
}
