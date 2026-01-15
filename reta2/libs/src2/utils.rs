// src/utils.rs
use std::env;

pub fn get_text_wrap_things() -> (i32, i32, i32, i32) {
    // Simplified - get terminal dimensions
    let rows = match env::var("ROWS") {
        Ok(val) => val.parse().unwrap_or(24),
        Err(_) => 24,
    };
    
    let cols = match env::var("COLUMNS") {
        Ok(val) => val.parse().unwrap_or(80),
        Err(_) => 80,
    };
    
    (rows, cols, 0, 0)
}

pub fn set_shell_rows_amount(amount: i32) {
    env::set_var("ROWS", amount.to_string());
}

pub fn shell_rows_amount() -> i32 {
    get_text_wrap_things().0
}
