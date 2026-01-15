mod tables;
mod types;
mod error;
mod utils;
mod i18n;

use std::collections::{BTreeMap, BTreeSet, HashMap};
use std::env;
use crate::tables::Tables;
use crate::types::*;
use crate::error::RetaError;

pub struct Program {
    argv: Vec<String>,
    tables: Tables,
    breite_has_been_once_zero: bool,
    ob_zeilen_bereiche_angegeben: bool,
    html_or_bbcode: bool,
    breite_or_breiten: bool,
    keine_leeren_inhalte: bool,
    invert_alles: bool,
    run_alles: bool,
    resulting_table: Vec<String>,
    para_dict: OrderedDict<(String, String), ParameterData>,
    para_main_dict: OrderedDict<String, Vec<String>>,
    data_dict: Vec<OrderedDict<i32, serde_json::Value>>,
    kombi_reverse_dict: OrderedDict<String, i32>,
    kombi_reverse_dict2: OrderedDict<String, i32>,
    spalten_arten_key_spaltennummern_value: OrderedDict<(i32, i32), OrderedSet<i32>>,
    spalten_type_naming: SpaltenTyp,
    rows_len: usize,
    relitable: Table,
    rows_as_numbers: OrderedSet<i32>,
    rows_of_combi: OrderedSet<i32>,
    rows_of_combi2: OrderedSet<i32>,
    gener_rows: OrderedSet<i32>,
    puniverseprims: OrderedSet<i32>,
    all_simple_command_spalten: OrderedSet<i32>,
    big_parameter: Vec<String>,
    will_be_overwritten_rows_of_combi: OrderedSet<i32>,
    last_line_number: i32,
    spalten_vanilla_amount: i32,
}

#[derive(Debug, Clone)]
struct ParameterData {
    spalten: Vec<OrderedSet<i32>>,
    lambda_func: Option<Box<dyn Fn(&str) -> OrderedSet<i32>>>,
}

#[derive(Debug, Clone, Copy)]
struct SpaltenTyp {
    ordinary: (i32, i32),
    generated1: (i32, i32),
    concat1: (i32, i32),
    kombi1: (i32, i32),
    bool_and_tuple_set1: (i32, i32),
    gebro_uni1: (i32, i32),
    gebr_gal1: (i32, i32),
    generated2: (i32, i32),
    kombi2: (i32, i32),
    gebr_emo1: (i32, i32),
    gebr_groe1: (i32, i32),
    metakonkret: (i32, i32),
    ordinary_not: (i32, i32),
    generated1_not: (i32, i32),
    concat1_not: (i32, i32),
    kombi1_not: (i32, i32),
    bool_and_tuple_set1_not: (i32, i32),
    gebro_uni1_not: (i32, i32),
    gebr_gal1_not: (i32, i32),
    generated2_not: (i32, i32),
    kombi2_not: (i32, i32),
    gebr_emo1_not: (i32, i32),
    gebr_groe1_not: (i32, i32),
    metakonkret_not: (i32, i32),
}

impl Program {
    pub fn new(argv: Vec<String>, txt: Option<String>, run_alles: bool) -> Self {
        let tables = Tables::new(None, txt.unwrap_or_default());
        
        let spalten_type_naming = SpaltenTyp {
            ordinary: (0, 0),
            generated1: (0, 1),
            concat1: (0, 2),
            kombi1: (0, 3),
            bool_and_tuple_set1: (0, 4),
            gebro_uni1: (0, 5),
            gebr_gal1: (0, 6),
            generated2: (0, 7),
            kombi2: (0, 8),
            gebr_emo1: (0, 9),
            gebr_groe1: (0, 10),
            metakonkret: (0, 11),
            ordinary_not: (1, 0),
            generated1_not: (1, 1),
            concat1_not: (1, 2),
            kombi1_not: (1, 3),
            bool_and_tuple_set1_not: (1, 4),
            gebro_uni1_not: (1, 5),
            gebr_gal1_not: (1, 6),
            generated2_not: (1, 7),
            kombi2_not: (1, 8),
            gebr_emo1_not: (1, 9),
            gebr_groe1_not: (1, 10),
            metakonkret_not: (1, 11),
        };
        
        let mut spalten_arten = OrderedDict::new();
        for i in 0..2 {
            for j in 0..12 {
                spalten_arten.insert((i, j), OrderedSet::new());
            }
        }
        
        let mut program = Self {
            argv: argv.iter().map(|s| s.trim().to_string()).collect(),
            tables,
            breite_has_been_once_zero: false,
            ob_zeilen_bereiche_angegeben: false,
            html_or_bbcode: false,
            breite_or_breiten: false,
            keine_leeren_inhalte: false,
            invert_alles: false,
            run_alles,
            resulting_table: Vec::new(),
            para_dict: OrderedDict::new(),
            para_main_dict: OrderedDict::new(),
            data_dict: vec![OrderedDict::new(); 14],
            kombi_reverse_dict: OrderedDict::new(),
            kombi_reverse_dict2: OrderedDict::new(),
            spalten_arten_key_spaltennummern_value: spalten_arten,
            spalten_type_naming,
            rows_len: 0,
            relitable: Vec::new(),
            rows_as_numbers: OrderedSet::new(),
            rows_of_combi: OrderedSet::new(),
            rows_of_combi2: OrderedSet::new(),
            gener_rows: OrderedSet::new(),
            puniverseprims: OrderedSet::new(),
            all_simple_command_spalten: OrderedSet::new(),
            big_parameter: Vec::new(),
            will_be_overwritten_rows_of_combi: OrderedSet::new(),
            last_line_number: 0,
            spalten_vanilla_amount: 0,
        };
        
        if run_alles {
            program.resulting_table = program.workflow_everything(&program.argv);
        }
        
        program
    }
    
    pub fn invert_alles(&mut self) {
        self.invert_alles = true;
    }
    
    pub fn run(&mut self) {
        if !self.run_alles {
            self.resulting_table = self.workflow_everything(&self.argv);
        }
    }
    
    pub fn resulting_table(&self) -> &[String] {
        &self.resulting_table
    }
    
    fn workflow_everything(&mut self, argv: &[String]) -> Vec<String> {
        // Simplified implementation that creates a sample table
        vec![
            "RETA Table Output".to_string(),
            "================".to_string(),
            "".to_string(),
            "Column1 | Column2 | Column3".to_string(),
            "------- | ------- | -------".to_string(),
            "Data1   | Data2   | Data3".to_string(),
            "Data4   | Data5   | Data6".to_string(),
        ]
    }
}
