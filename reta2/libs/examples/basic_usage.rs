use lib4tables_concat::*;
use std::collections::HashSet;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a Tables instance
    let tables = Tables {
        spalten_vanilla_amount: 10,
        last_line_number: 100,
        html_output_yes: false,
        bbcode_output_yes: false,
        ..Default::default()
    };
    
    // Create Concat instance
    let mut concat = Concat::new(tables);
    
    // Create sample data
    let mut relitable = vec![
        vec!["Number".to_string(), "Value".to_string()],
    ];
    
    for i in 1..=10 {
        relitable.push(vec![
            i.to_string(),
            format!("Value{}", i),
        ]);
    }
    
    // Test concat_love_polygon
    let mut rows_as_numbers = HashSet::new();
    rows_as_numbers.insert(9);
    
    concat.concat_love_polygon(&mut relitable, &mut rows_as_numbers)?;
    
    println!("Table after concat_love_polygon:");
    for row in &relitable {
        println!("{:?}", row);
    }
    
    // Test gleichheit_freiheit_vergleich
    for i in 1..=10 {
        let result = concat.gleichheit_freiheit_vergleich(i);
        println!("{}: {}", i, result);
    }
    
    // Test geist_emotion_energie_materie_topologie
    for i in 1..=10 {
        let result = concat.geist_emotion_energie_materie_topologie(i);
        println!("{}: {}", i, result);
    }
    
    Ok(())
}
