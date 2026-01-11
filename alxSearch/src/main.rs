mod file_utils;
mod token_extractor;

use file_utils::list_text_files;
use token_extractor::TokenExtractor;
use std::path::Path;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len()<2 { 
        eprintln!("Usage: {} <directory>", args[0]);
        return;
    }
    let dir = Path::new(&args[1]);
    let files = list_text_files(dir, 500*1024);
    let mut extractor = TokenExtractor::new();

    for f in files {
        extractor.process_file(Path::new(&f)).unwrap();
    }
    extractor.write_csv("worttokens.csv").unwrap();
    println!("CSV erzeugt: worttokens.csv");
}
