mod tables;
mod i18n;
mod utils;
mod types;
mod error;

use std::env;
use std::process;
use error::RetaError;
use tables::Program;

fn main() -> Result<(), RetaError> {
    // Parse command line arguments
    let args: Vec<String> = env::args().collect();
    
    // Check for help request
    if args.len() == 1 || args.contains(&"-h".to_string()) || args.contains(&"--help".to_string()) {
        print_help();
        return Ok(());
    }
    
    // Create and run program
    let mut program = Program::new(args, None, true);
    
    // Print resulting table
    for line in program.resulting_table() {
        println!("{}", line);
    }
    
    Ok(())
}

fn print_help() {
    println!("RETA - Tabellenverarbeitungsprogramm");
    println!();
    println!("Verwendung: reta [OPTIONEN]");
    println!();
    println!("Hauptoptionen:");
    println!("  -zeilen          Zeilenparameter festlegen");
    println!("  -spalten         Spaltenparameter festlegen");
    println!("  -ausgabe         Ausgabeparameter festlegen");
    println!("  -kombination     Tabellenkombinationen");
    println!("  -h, --help       Diese Hilfe anzeigen");
    println!();
    println!("Beispiele:");
    println!("  reta -zeilen --alles --typ=sonne,mond");
    println!("  reta -spalten --multiplikationen=2,3,5");
    println!("  reta -ausgabe --art=html --breite=80");
    println!();
    println!("Ausgabeformate:");
    println!("  --art=shell      Standard-Shell-Ausgabe (default)");
    println!("  --art=html       HTML-Ausgabe");
    println!("  --art=bbcode     BBCode-Ausgabe");
    println!("  --art=markdown   Markdown-Ausgabe");
    println!("  --art=csv        CSV-Ausgabe");
    println!("  --art=emacs      Emacs-org-mode Ausgabe");
}
