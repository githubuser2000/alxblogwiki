use std::collections::HashMap;
use std::fs;
use std::io::{self, BufRead};
use regex::Regex;
use std::path::Path;

fn main() -> io::Result<()> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <directory>", args[0]);
        return Ok(());
    }

    let dir = &args[1];
    let token_regex = Regex::new(r"[A-Z][a-z]*").unwrap();

    let mut global_count: HashMap<String, usize> = HashMap::new();
    let mut csv_rows: Vec<(String, String, usize, usize)> = Vec::new();

    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() {
            if fs::metadata(&path)?.len() > 500 * 1024 { continue; }

            let file = fs::File::open(&path)?;
            let reader = io::BufReader::new(file);

            let mut file_count: HashMap<String, usize> = HashMap::new();
            for line in reader.lines() {
                let line = line?;
                for cap in token_regex.captures_iter(&line) {
                    let token = cap[0].to_string();
                    *file_count.entry(token.clone()).or_insert(0) += 1;
                    *global_count.entry(token.clone()).or_insert(0) += 1;
                }
            }

            for (token, count) in file_count {
                csv_rows.push((token, path.file_name().unwrap().to_string_lossy().to_string(), 0, count));
            }
        }
    }

    // Update global count
    for row in csv_rows.iter_mut() {
        row.2 = global_count[&row.0];
    }

    let mut wtr = csv::Writer::from_path("worttokens.csv")?;
    wtr.write_record(&["Token", "Dateiname", "HäufigkeitGesamt", "HäufigkeitDatei"])?;
    for row in csv_rows {
        wtr.write_record(&[row.0, row.1, row.2.to_string(), row.3.to_string()])?;
    }
    wtr.flush()?;

    println!("CSV erzeugt: worttokens.csv");
    Ok(())
}
