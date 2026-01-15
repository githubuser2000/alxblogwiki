use std::env;

pub fn get_text_wrap_things() -> (i32, i32, i32, i32) {
    // Try to get terminal size from environment
    let rows = env::var("ROWS")
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(24);
    
    let cols = env::var("COLUMNS")
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(80);
    
    // Return default values for other parameters
    (rows, cols, 0, 0)
}

pub fn set_shell_rows_amount(amount: i32) {
    env::set_var("ROWS", amount.to_string());
}

pub fn shell_rows_amount() -> i32 {
    get_text_wrap_things().0
}

pub fn bereich_to_numbers(s: &str) -> Vec<i32> {
    let mut result = Vec::new();
    
    for part in s.split(',') {
        let part = part.trim();
        if part.is_empty() {
            continue;
        }
        
        if let Some(dash_pos) = part.find('-') {
            let start_str = &part[..dash_pos];
            let end_str = &part[dash_pos + 1..];
            
            if let (Ok(start), Ok(end)) = (start_str.parse::<i32>(), end_str.parse::<i32>()) {
                for i in start..=end {
                    result.push(i);
                }
            }
        } else if let Ok(num) = part.parse::<i32>() {
            result.push(num);
        }
    }
    
    result
}

pub fn html_escape(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#39;")
}

pub fn parse_range(s: &str) -> Result<Vec<i32>, String> {
    let mut numbers = Vec::new();
    
    for part in s.split(',') {
        let part = part.trim();
        if part.is_empty() {
            continue;
        }
        
        if part.contains('-') {
            let parts: Vec<&str> = part.split('-').collect();
            if parts.len() != 2 {
                return Err(format!("Invalid range: {}", part));
            }
            
            let start = parts[0].parse::<i32>()
                .map_err(|e| format!("Invalid start in range '{}': {}", part, e))?;
            let end = parts[1].parse::<i32>()
                .map_err(|e| format!("Invalid end in range '{}': {}", part, e))?;
            
            if start <= end {
                for i in start..=end {
                    numbers.push(i);
                }
            } else {
                for i in (end..=start).rev() {
                    numbers.push(i);
                }
            }
        } else {
            let num = part.parse::<i32>()
                .map_err(|e| format!("Invalid number '{}': {}", part, e))?;
            numbers.push(num);
        }
    }
    
    Ok(numbers)
}

pub fn format_table(rows: &[Vec<String>], separator: &str) -> String {
    if rows.is_empty() {
        return String::new();
    }
    
    // Find maximum width for each column
    let num_cols = rows[0].len();
    let mut col_widths = vec![0; num_cols];
    
    for row in rows {
        for (col_idx, cell) in row.iter().enumerate() {
            if col_idx < num_cols {
                col_widths[col_idx] = col_widths[col_idx].max(cell.len());
            }
        }
    }
    
    // Format each row
    let mut result = String::new();
    for row in rows {
        let formatted_cells: Vec<String> = row.iter()
            .enumerate()
            .map(|(col_idx, cell)| {
                if col_idx < col_widths.len() {
                    format!("{:width$}", cell, width = col_widths[col_idx])
                } else {
                    cell.clone()
                }
            })
            .collect();
        
        result.push_str(&formatted_cells.join(separator));
        result.push('\n');
    }
    
    result
}
