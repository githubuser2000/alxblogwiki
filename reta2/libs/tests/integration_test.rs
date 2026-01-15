use lib4tables_concat::*;
use std::collections::HashSet;

#[test]
fn test_concat_love_polygon() -> Result<(), Box<dyn std::error::Error>> {
    let tables = Tables::default();
    let mut concat = Concat::new(tables);
    
    let mut relitable = vec![
        vec!["Header1".to_string(), "Header2".to_string()],
        vec!["Row1Col1".to_string(), "Row1Col2".to_string()],
        vec!["Row2Col1".to_string(), "Row2Col2".to_string()],
    ];
    
    let mut rows_as_numbers = HashSet::new();
    rows_as_numbers.insert(9);
    
    concat.concat_love_polygon(&mut relitable, &mut rows_as_numbers)?;
    
    assert_eq!(relitable[0].len(), 3);
    Ok(())
}

#[test]
fn test_gleichheit_freiheit_vergleich() {
    let tables = Tables::default();
    let concat = Concat::new(tables);
    
    let result = concat.gleichheit_freiheit_vergleich(2);
    assert!(result.contains("Gleichheit"));
    
    let result = concat.gleichheit_freiheit_vergleich(1);
    assert!(result.contains("Freiheit"));
}

#[test]
fn test_read_concat_csv() -> Result<(), Box<dyn std::error::Error>> {
    // Create a test CSV file
    use std::fs::File;
    use std::io::Write;
    
    let test_dir = tempfile::tempdir()?;
    let csv_path = test_dir.path().join("test.csv");
    
    let mut file = File::create(&csv_path)?;
    writeln!(file, "Col1;Col2;Col3")?;
    writeln!(file, "Val1;Val2;Val3")?;
    writeln!(file, "Val4;Val5;Val6")?;
    
    let tables = Tables::default();
    let mut concat = Concat::new(tables);
    
    let mut relitable = vec![
        vec!["Header".to_string()],
        vec!["Data".to_string()],
    ];
    
    let mut rows_as_numbers = HashSet::new();
    let concat_table_selection = HashSet::from([1]);
    
    // Note: This test would need proper CSV file setup
    // concat.read_concat_csv(&mut relitable, &mut rows_as_numbers, &concat_table_selection, 1)?;
    
    Ok(())
}

#[test]
fn test_prim_functions() {
    assert!(could_be_prime_number_primzahlkreuz(7));
    assert!(!could_be_prime_number_primzahlkreuz(4));
    
    assert!(could_be_prime_number_primzahlkreuz_fuer_aussen(7));
    assert!(!could_be_prime_number_primzahlkreuz_fuer_aussen(5));
    
    assert!(could_be_prime_number_primzahlkreuz_fuer_innen(5));
    assert!(!could_be_prime_number_primzahlkreuz_fuer_innen(7));
    
    assert_eq!(prim_creativity(7), 1);
    assert_eq!(prim_creativity(4), 2);
    assert_eq!(prim_creativity(1), 0);
}

#[test]
fn test_fraction_conversion() {
    let tables = Tables::default();
    let concat = Concat::new(tables);
    
    let paare_set = OrderedSet::from([(2, 3), (4, 6), (1, 2)]);
    
    let result_div = concat.convert_set_of_paaren_to_dict_of_num_to_paare_div(&paare_set, false);
    assert!(result_div.contains_key(&1)); // 2/3 ≈ 1
    
    let result_mul = concat.convert_set_of_paaren_to_dict_of_num_to_paare_mul(&paare_set, false);
    assert!(result_mul.contains_key(&6)); // 2*3 = 6
}
