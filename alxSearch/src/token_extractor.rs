use regex::Regex;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub struct TokenExtractor {
    pub global_count: HashMap<String, usize>,
    pub rows: Vec<(String,String,usize,usize)>,
    token_regex: Regex,
}

impl TokenExtractor {
    pub fn new() -> Self {
        TokenExtractor { 
            global_count: HashMap::new(),
            rows: Vec::new(),
            token_regex: Regex::new(r"[A-Z][a-z]*").unwrap(),
        }
    }

    pub fn process_file(&mut self, path: &Path) -> io::Result<()> {
        let file = File::open(path)?;
        let reader = io::BufReader::new(file);
        let mut file_count = HashMap::new();

        for line in reader.lines() {
            let line = line?;
            for cap in self.token_regex.captures_iter(&line) {
                let token = cap[0].to_string();
                *file_count.entry(token.clone()).or_insert(0) +=1;
                *self.global_count.entry(token.clone()).or_insert(0) +=1;
            }
        }

        for (token,count) in file_count {
            self.rows.push((token, path.file_name().unwrap().to_string_lossy().to_string(), 0, count));
        }
        Ok(())
    }

    pub fn write_csv(&mut self, csv_file: &str) -> csv::Result<()> {
        let mut wtr = csv::Writer::from_path(csv_file)?;
        wtr.write_record(&["Token","Dateiname","HäufigkeitGesamt","HäufigkeitDatei"])?;
        for row in self.rows.iter_mut() {
            row.2 = self.global_count[&row.0];
            wtr.write_record(&[&row.0, &row.1, &row.2.to_string(), &row.3.to_string()])?;
        }
        wtr.flush()?;
        Ok(())
    }
}
