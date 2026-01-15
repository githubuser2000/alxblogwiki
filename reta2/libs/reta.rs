// reta.rs
use std::collections::{BTreeMap, BTreeSet, HashMap};
use std::env;
use std::fs::{self, File};
use std::io::{self, BufRead, BufReader, Write};
use std::path::{Path, PathBuf};
use std::str::FromStr;

use csv::{ReaderBuilder, WriterBuilder};
use serde_json::{json, Value};
use regex::Regex;

// Module imports
mod tables;
mod i18n;
mod utils;

use tables::{Tables, OutputSyntax, SyntaxType};
use i18n::{I18n, CsvFileNames, ParametersMain};
use utils::{get_text_wrap_things, set_shell_rows_amount, shell_rows_amount};

// Typ-Aliase für bessere Lesbarkeit
type OrderedSet<T> = BTreeSet<T>;
type OrderedDict<K, V> = BTreeMap<K, V>;

// Hauptprogramm-Struktur
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
    data_dict: Vec<OrderedDict<i32, Value>>,
    kombi_reverse_dict: OrderedDict<String, i32>,
    kombi_reverse_dict2: OrderedDict<String, i32>,
    spalten_arten_key_spaltennummern_value: OrderedDict<(i32, i32), OrderedSet<i32>>,
    spalten_type_naming: SpaltenTyp,
    rows_len: usize,
    relitable: Vec<Vec<String>>,
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

// Parameter-Datenstruktur
#[derive(Debug, Clone)]
struct ParameterData {
    spalten: Vec<OrderedSet<i32>>,
    lambda_func: Option<Box<dyn Fn(&str) -> OrderedSet<i32>>>,
}

// Spalten-Typ-Benennung
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
    
    // Hauptarbeitsschritte
    fn workflow_everything(&mut self, argv: &[String]) -> Vec<String> {
        let (
            rows_len,
            param_lines,
            param_lines_not,
            relitable,
            rows_as_numbers,
            animals_professions_table,
            rows_of_combi,
            kombi_table_kombis,
            maintable2subtable_relation,
            spaltenreihenfolge_und_nur_diese,
            prim_spalten,
            gebr,
            animals_professions_table2,
            kombi_table_kombis2,
            maintable2subtable_relation2,
        ) = self.bring_all_important_begin_things(argv);
        
        let (
            finally_display_lines,
            new_table,
            numlen,
            rows_range,
            old2new_table,
        ) = self.tables.get_prepare().prepare4out(
            &param_lines,
            &param_lines_not,
            &relitable,
            &rows_as_numbers,
            &gebr,
            &prim_spalten,
        );
        
        let mut new_table = new_table;
        
        if !rows_of_combi.is_empty() {
            new_table = self.combi_table_workflow(
                animals_professions_table,
                &finally_display_lines,
                kombi_table_kombis,
                maintable2subtable_relation,
                new_table,
                &old2new_table,
                &param_lines,
                &CsvFileNames::kombi13,
            );
        }
        
        if !self.rows_of_combi2.is_empty() {
            new_table = self.combi_table_workflow(
                animals_professions_table2,
                &finally_display_lines,
                kombi_table_kombis2,
                maintable2subtable_relation2,
                new_table,
                &old2new_table,
                &param_lines,
                &CsvFileNames::kombi15,
            );
        }
        
        new_table = self.tables.get_out().only_that_columns(
            &new_table,
            &spaltenreihenfolge_und_nur_diese,
        );
        
        self.tables.get_out().cli_out(
            &finally_display_lines,
            &new_table,
            numlen,
            rows_range,
        )
    }
    
    fn combi_table_workflow(
        &self,
        animals_professions_table: Vec<Vec<String>>,
        finally_display_lines: &OrderedSet<i32>,
        kombi_table_kombis: Vec<Vec<i32>>,
        maintable2subtable_relation: (OrderedDict<i32, i32>, OrderedDict<i32, i32>),
        new_table: Vec<Vec<Vec<String>>>,
        old2new_table: &(Vec<i32>, Vec<i32>),
        param_lines: &OrderedSet<String>,
        csv_file_name: &CsvFileNames,
    ) -> Vec<Vec<Vec<String>>> {
        let chosen_kombi_lines = self.tables.get_combis().prepare_kombi(
            finally_display_lines,
            &animals_professions_table,
            param_lines,
            finally_display_lines,
            &kombi_table_kombis,
        );
        
        let komb_rows = match csv_file_name {
            CsvFileNames::kombi13 => &self.rows_of_combi,
            CsvFileNames::kombi15 => &self.rows_of_combi2,
            _ => &OrderedSet::new(),
        };
        
        let (
            finally_display_lines_kombi,
            new_table_kombi_1,
            _line_len_kombi_1,
            animals_professions_table_prepared,
            old2new_table_animals_professions,
        ) = self.tables.get_prepare().prepare4out(
            &OrderedSet::new(),
            &OrderedSet::new(),
            &animals_professions_table,
            komb_rows,
            &HashMap::new(),
            self.tables.get_combis().sum_of_all_combi_rows_amount(),
            Some(new_table[0].len() - komb_rows.len()),
            match csv_file_name {
                CsvFileNames::kombi13 => Some(0),
                CsvFileNames::kombi15 => Some(1),
                _ => None,
            },
        );
        
        let kombi_tables = self.tables.get_combis().prepare_table_join(
            &chosen_kombi_lines,
            &new_table_kombi_1,
        );
        
        self.tables.get_combis().table_join(
            new_table,
            &kombi_tables,
            &maintable2subtable_relation,
            old2new_table,
            komb_rows,
        )
    }
    
    // Parameter-Verarbeitung
    fn produce_all_spalten_numbers(&mut self, neg: &str) {
        fn resulting_spalten_from_tuple(
            tupl: &[OrderedSet<i32>],
            neg: &str,
            para_value: Option<&str>,
            befehl_name: Option<&str>,
            spalten_arten: &mut OrderedDict<(i32, i32), OrderedSet<i32>>,
            tables: &mut Tables,
            data_dict: &[OrderedDict<i32, Value>],
            i18n: &I18n,
        ) {
            for (i, eine_spalten_art_mit_spalten_nummern) in tupl.iter().enumerate() {
                if i == 2 && befehl_name.is_some() {
                    let befehl_name = befehl_name.unwrap();
                    let gebr_befehle_dict: OrderedDict<&str, i32> = [
                        (i18n.parameters_main.multiplikationen[0].as_str(), 2),
                        (i18n.parameters_main.gebrochenuniversum[0].as_str(), 5),
                        (i18n.parameters_main.gebrochenuniversum[1].as_str(), 5),
                        (i18n.parameters_main.gebrochengalaxie[0].as_str(), 6),
                        (i18n.parameters_main.gebrochengalaxie[1].as_str(), 6),
                        (i18n.parameters_main.gebrochenemotion[0].as_str(), 9),
                        (i18n.parameters_main.gebrochenemotion[1].as_str(), 9),
                        (i18n.parameters_main.gebrochengroesse[0].as_str(), 10),
                        (i18n.parameters_main.gebrochengroesse[1].as_str(), 10),
                    ].iter().cloned().collect();
                    
                    if let Some(&gebr_index) = gebr_befehle_dict.get(befehl_name) {
                        if let Some(para_val) = para_value {
                            let result = if befehl_name == i18n.parameters_main.multiplikationen[0] {
                                lambda_prim_galax(para_val)
                            } else {
                                lambda_gebr_univ_und_galax(para_val)
                            };
                            
                            let key = (neg.len() as i32, gebr_index);
                            if let Some(set) = spalten_arten.get_mut(&key) {
                                set.extend(result);
                            }
                        }
                    }
                } else {
                    let key = (neg.len() as i32, i as i32);
                    if let Some(set) = spalten_arten.get_mut(&key) {
                        set.extend(eine_spalten_art_mit_spalten_nummern);
                    }
                }
            }
        }
        
        fn lambda_gebr_univ_und_galax(para_values: &str) -> OrderedSet<i32> {
            para_values.split(',')
                .filter_map(|chosen| {
                    chosen.trim()
                        .parse::<i32>()
                        .ok()
                        .map(|num| num.abs())
                        .filter(|&num| num != 0 && num != 1)
                })
                .collect()
        }
        
        fn lambda_prim_galax(para_values: &str) -> OrderedSet<i32> {
            para_values.split(',')
                .filter_map(|chosen| {
                    chosen.trim()
                        .parse::<i32>()
                        .ok()
                        .map(|num| num.abs())
                        .filter(|&num| num != 0 && num != 1)
                        .filter(|&num| prim_creativity(num) == 1)
                })
                .collect()
        }
        
        fn prim_creativity(n: i32) -> i32 {
            // Implementierung der Primzahl-Kreativitäts-Funktion
            if n <= 1 {
                return 0;
            }
            for i in 2..=(n as f64).sqrt() as i32 {
                if n % i == 0 {
                    return 0;
                }
            }
            1
        }
        
        let main_para_cmds: OrderedDict<&str, i32> = [
            ("zeilen", 0),
            ("spalten", 1),
            ("kombination", 2),
            ("ausgabe", 3),
        ].iter().cloned().collect();
        
        let mut last_main_cmd = -1;
        
        for cmd in &self.argv[1..] {
            if cmd.starts_with('-') && !cmd.starts_with("--") {
                let cmd_body = &cmd[1..];
                if let Some(&cmd_num) = main_para_cmds.get(cmd_body) {
                    last_main_cmd = cmd_num;
                }
            } else if cmd.starts_with("--") {
                let cmd_body = &cmd[2..];
                
                match last_main_cmd {
                    1 => { // spalten
                        if self.breite_breiten_sys_argv_para(cmd_body, neg) {
                            continue;
                        }
                        
                        if cmd_body == i18n::RETA.keine_num_wort && neg.is_empty() {
                            self.tables.set_nummeriere(false);
                            continue;
                        }
                        
                        if let Some(eq_pos) = cmd_body.find('=') {
                            let cmd_name = &cmd_body[..eq_pos];
                            let cmd_values = &cmd_body[eq_pos + 1..];
                            
                            for one_of_things_after_eq_sign in cmd_values.split(',') {
                                let (value, yes1) = if one_of_things_after_eq_sign.starts_with('-') {
                                    (&one_of_things_after_eq_sign[1..], neg == "-")
                                } else {
                                    (one_of_things_after_eq_sign, neg.is_empty())
                                };
                                
                                if yes1 {
                                    if let Some(param_data) = self.para_dict.get(&(cmd_name.to_string(), value.to_string())) {
                                        resulting_spalten_from_tuple(
                                            &param_data.spalten,
                                            neg,
                                            Some(value),
                                            Some(cmd_name),
                                            &mut self.spalten_arten_key_spaltennummern_value,
                                            &mut self.tables,
                                            &self.data_dict,
                                            &i18n::I18N,
                                        );
                                    } else {
                                        // Fehlerbehandlung für unbekannte Parameter
                                        eprintln!("Unbekannter Parameter: {}={}", cmd_name, value);
                                    }
                                }
                            }
                        } else {
                            // Befehl ohne Wert
                            if let Some(param_data) = self.para_dict.get(&(cmd_body.to_string(), "".to_string())) {
                                resulting_spalten_from_tuple(
                                    &param_data.spalten,
                                    neg,
                                    None,
                                    Some(cmd_body),
                                    &mut self.spalten_arten_key_spaltennummern_value,
                                    &mut self.tables,
                                    &self.data_dict,
                                    &i18n::I18N,
                                );
                            }
                        }
                    }
                    2 => { // kombination
                        let gal_wort = format!("{}=", i18n::KOMBI_MAIN_PARAS.galaxie);
                        let uni_wort = format!("{}=", i18n::KOMBI_MAIN_PARAS.universum);
                        
                        if cmd_body.starts_with(&gal_wort) || cmd_body.starts_with(&uni_wort) {
                            let eq_pos = cmd_body.find('=').unwrap();
                            let cmd_type = &cmd_body[..eq_pos];
                            let values = &cmd_body[eq_pos + 1..];
                            
                            for one_kombi_spalte in values.split(',') {
                                let (value, yes1) = if one_kombi_spalte.starts_with('-') {
                                    (&one_kombi_spalte[1..], neg == "-")
                                } else {
                                    (one_kombi_spalte, neg.is_empty())
                                };
                                
                                if yes1 {
                                    let spalten_set = if cmd_type == i18n::KOMBI_MAIN_PARAS.galaxie {
                                        if let Some(&kombi_num) = self.kombi_reverse_dict.get(value) {
                                            OrderedSet::from([kombi_num])
                                        } else {
                                            OrderedSet::new()
                                        }
                                    } else {
                                        if let Some(&kombi_num) = self.kombi_reverse_dict2.get(value) {
                                            OrderedSet::from([kombi_num])
                                        } else {
                                            OrderedSet::new()
                                        }
                                    };
                                    
                                    let tupl = vec![
                                        OrderedSet::new(),
                                        OrderedSet::new(),
                                        OrderedSet::new(),
                                        spalten_set,
                                        OrderedSet::new(),
                                        OrderedSet::new(),
                                        OrderedSet::new(),
                                        OrderedSet::new(),
                                        OrderedSet::new(),
                                    ];
                                    
                                    resulting_spalten_from_tuple(
                                        &tupl,
                                        neg,
                                        Some(value),
                                        Some("kombinationen"),
                                        &mut self.spalten_arten_key_spaltennummern_value,
                                        &mut self.tables,
                                        &self.data_dict,
                                        &i18n::I18N,
                                    );
                                }
                            }
                        }
                    }
                    _ => {
                        // Unbekannter Hauptbefehl
                    }
                }
            }
        }
        
        // Entfernen von Dopplungen zwischen positiven und negativen Spalten
        if neg.is_empty() {
            self.produce_all_spalten_numbers("-");
            self.spalten_remove_doubles_n_then_remove_one_from_another();
        }
    }
    
    fn spalten_remove_doubles_n_then_remove_one_from_another(&mut self) {
        for el2_type in 0..12 {
            let pos_key = (0, el2_type);
            let neg_key = (1, el2_type);
            
            if let (Some(pos_set), Some(neg_set)) = (
                self.spalten_arten_key_spaltennummern_value.get(&pos_key),
                self.spalten_arten_key_spaltennummern_value.get(&neg_key),
            ) {
                let diff: OrderedSet<i32> = pos_set.difference(neg_set).cloned().collect();
                self.spalten_arten_key_spaltennummern_value.insert(pos_key, diff);
            }
        }
        
        // Entferne negative Spalten
        for el2_type in 0..12 {
            self.spalten_arten_key_spaltennummern_value.remove(&(1, el2_type));
        }
    }
    
    fn breite_breiten_sys_argv_para(&mut self, cmd: &str, neg: &str) -> bool {
        let para_breite = format!("{}=", i18n::AUSGABE_PARAS.breite);
        let para_breite_n = format!("{}=", i18n::AUSGABE_PARAS.breiten);
        
        if cmd.starts_with(&para_breite) && neg.is_empty() {
            let shell_rows_amount = shell_rows_amount();
            let value_str = &cmd[para_breite.len()..];
            
            if self.breite_has_been_once_zero {
                set_shell_rows_amount(0);
                self.tables.set_text_width(0);
                self.breite_or_breiten = true;
                return true;
            }
            
            if let Ok(breite) = value_str.parse::<i32>() {
                let breite = breite.abs();
                if breite == 0 {
                    self.breite_has_been_once_zero = true;
                    set_shell_rows_amount(0);
                } else if shell_rows_amount > 7 && breite > shell_rows_amount - 7 {
                    let breite = shell_rows_amount - 7;
                    self.tables.set_text_width(breite);
                } else {
                    let current_width = self.tables.text_width();
                    self.tables.set_text_width(breite.max(current_width));
                }
                self.breite_or_breiten = true;
            }
            return true;
        } else if cmd.starts_with(&para_breite_n) && neg.is_empty() {
            let values = &cmd[para_breite_n.len()..];
            let breiten: Vec<i32> = values.split(',')
                .filter_map(|s| s.trim().parse().ok())
                .collect();
            self.tables.set_breitenn(breiten);
            self.breite_or_breiten = true;
            return true;
        }
        
        false
    }
    
    // Parameter-Speicherung
    fn store_parameters_for_columns(&mut self) {
        let gebrochen_spalten_maximum_plus_1 = i18n::GEBROCHEN_SPALTEN_MAXIMUM_PLUS_1;
        
        // Lambda-Funktionen
        let lambda_gebr_univ_und_galax = Box::new(|para_values: &str| -> OrderedSet<i32> {
            para_values.split(',')
                .filter_map(|chosen| {
                    chosen.trim()
                        .parse::<i32>()
                        .ok()
                        .map(|num| num.abs())
                        .filter(|&num| num != 0 && num != 1)
                })
                .collect()
        });
        
        let lambda_prim_galax = Box::new(|para_values: &str| -> OrderedSet<i32> {
            para_values.split(',')
                .filter_map(|chosen| {
                    chosen.trim()
                        .parse::<i32>()
                        .ok()
                        .map(|num| num.abs())
                        .filter(|&num| num != 0 && num != 1)
                        .filter(|&num| {
                            // primCreativity Funktion
                            if num <= 1 {
                                return false;
                            }
                            for i in 2..=(num as f64).sqrt() as i32 {
                                if num % i == 0 {
                                    return false;
                                }
                            }
                            true
                        })
                })
                .collect()
        });
        
        // Erlaubte Primzahlen für Kommandos
        let allowed_prim_numbers_for_command: Vec<String> = (2..32)
            .filter(|&num| {
                // primCreativity Funktion
                if num <= 1 {
                    return false;
                }
                for i in 2..=(num as f64).sqrt() as i32 {
                    if num % i == 0 {
                        return false;
                    }
                }
                true
            })
            .map(|num| num.to_string())
            .collect();
        
        // Initialisiere Datenstrukturen
        let mut all_values = vec![
            OrderedSet::new(),  // 0: ordinary
            OrderedSet::new(),  // 1: generated1
            OrderedSet::new(),  // 2: concat1
            OrderedSet::new(),  // 3: kombi1
            OrderedSet::new(),  // 4: boolAndTupleSet1
            OrderedSet::new(),  // 5: gebroUni1
            OrderedSet::new(),  // 6: gebrGal1
            OrderedSet::new(),  // 7: generated2
            OrderedSet::new(),  // 8: kombi2
            OrderedSet::new(),  // 9: gebrEmo1
            OrderedSet::new(),  // 10: gebrGroe1
            OrderedSet::new(),  // 11: metakonkret
        ];
        
        // Fülle mit Basiswerten
        all_values[2] = allowed_prim_numbers_for_command
            .iter()
            .filter_map(|s| s.parse().ok())
            .collect();
        
        // Kombi-Parameter
        let kombi_para_data_matrix = &i18n::KOMBI_PARA_N_DATA_MATRIX;
        let kombi_para_data_matrix2 = &i18n::KOMBI_PARA_N_DATA_MATRIX2;
        
        all_values[3] = kombi_para_data_matrix.keys().cloned().collect();
        all_values[5] = (2..gebrochen_spalten_maximum_plus_1).collect();
        all_values[6] = (2..gebrochen_spalten_maximum_plus_1).collect();
        all_values[8] = kombi_para_data_matrix2.keys().cloned().collect();
        all_values[9] = (2..gebrochen_spalten_maximum_plus_1).collect();
        all_values[10] = (2..gebrochen_spalten_maximum_plus_1).collect();
        
        // Invertierung wenn nötig
        if self.invert_alles {
            for i in 1..=10 {
                all_values[i] = OrderedSet::new();
            }
        }
        
        // Parameter-Daten-Matrix
        let para_n_data_matrix = i18n::get_para_n_data_matrix();
        
        // Verarbeite jeden Parameter-Eintrag
        for parameter_entry in para_n_data_matrix {
            let (para_main_names, para_names, datas) = parameter_entry;
            
            // Erstelle Parameter-Daten
            let mut spalten_sets = Vec::new();
            for data in datas.iter().take(12) {
                spalten_sets.push(data.clone());
            }
            
            let param_data = ParameterData {
                spalten: spalten_sets,
                lambda_func: None,
            };
            
            // Füge zu para_dict hinzu
            if para_names.is_empty() {
                self.para_dict.insert((para_main_names[0].clone(), "".to_string()), param_data);
            } else {
                for para_name in para_names {
                    self.para_dict.insert((para_main_names[0].clone(), para_name.clone()), param_data.clone());
                }
            }
            
            // Füge zu para_main_dict hinzu
            for para_main_name in para_main_names {
                self.para_main_dict.insert(para_main_name.clone(), para_names.clone());
            }
        }
        
        // Speichere Kombi-Daten
        self.data_dict[3] = kombi_para_data_matrix.clone();
        self.data_dict[8] = kombi_para_data_matrix2.clone();
        
        // Erstelle Reverse-Dicts für Kombi
        for (&key, values) in kombi_para_data_matrix {
            if let Value::Array(arr) = values {
                for value in arr {
                    if let Value::String(s) = value {
                        self.kombi_reverse_dict.insert(s.clone(), key);
                    }
                }
            }
        }
        
        for (&key, values) in kombi_para_data_matrix2 {
            if let Value::Array(arr) = values {
                for value in arr {
                    if let Value::String(s) = value {
                        self.kombi_reverse_dict2.insert(s.clone(), key);
                    }
                }
            }
        }
        
        self.tables.set_data_dict(self.data_dict.clone());
    }
    
    // Parameter zu Kommandos und Zahlen
    fn parameters_to_commands_and_numbers(
        &mut self,
        argv: &[String],
        neg: &str,
    ) -> (OrderedSet<String>, OrderedSet<i32>, OrderedSet<i32>, Vec<i32>, OrderedSet<i32>, OrderedSet<i32>) {
        let mut param_lines = OrderedSet::new();
        let mut rows_as_numbers = OrderedSet::new();
        let mut rows_of_combi = OrderedSet::new();
        let mut spaltenreihenfolge_und_nur_diese = Vec::new();
        let mut puniverseprims_only = OrderedSet::new();
        let mut gener_rows = OrderedSet::new();
        
        let mut in_zeilen_section = false;
        let mut in_ausgabe_section = false;
        
        for arg in &argv[1..] {
            if arg.starts_with('-') && !arg.starts_with("--") {
                // Hauptparameter
                match arg.as_str() {
                    "-zeilen" => {
                        in_zeilen_section = true;
                        in_ausgabe_section = false;
                        self.big_parameter.push("zeilen".to_string());
                    }
                    "-spalten" => {
                        in_zeilen_section = false;
                        in_ausgabe_section = false;
                        self.big_parameter.push("spalten".to_string());
                    }
                    "-ausgabe" => {
                        in_zeilen_section = false;
                        in_ausgabe_section = true;
                        self.big_parameter.push("ausgabe".to_string());
                    }
                    "-kombination" => {
                        in_zeilen_section = false;
                        in_ausgabe_section = false;
                        self.big_parameter.push("kombination".to_string());
                    }
                    "-h" | "-help" => {
                        if neg.is_empty() {
                            self.help_page();
                        }
                    }
                    _ => {}
                }
            } else if arg.starts_with("--") {
                let cmd = &arg[2..];
                
                if in_zeilen_section {
                    // Zeilen-Parameter
                    if cmd == i18n::ZEILEN_PARAS.alles && neg.is_empty() {
                        param_lines.insert("all".to_string());
                        self.ob_zeilen_bereiche_angegeben = true;
                    } else if let Some(eq_pos) = cmd.find('=') {
                        let cmd_name = &cmd[..eq_pos];
                        let cmd_value = &cmd[eq_pos + 1..];
                        
                        match cmd_name {
                            "zeit" => {
                                self.ob_zeilen_bereiche_angegeben = true;
                                for subpara in cmd_value.split(',') {
                                    match subpara {
                                        s if s == neg.to_string() + i18n::ZEILEN_PARAS.heute => {
                                            param_lines.insert("=".to_string());
                                        }
                                        s if s == neg.to_string() + i18n::ZEILEN_PARAS.gestern => {
                                            param_lines.insert("<".to_string());
                                        }
                                        s if s == neg.to_string() + i18n::ZEILEN_PARAS.morgen => {
                                            param_lines.insert(">".to_string());
                                        }
                                        _ => {}
                                    }
                                }
                            }
                            "zaehlung" => {
                                self.ob_zeilen_bereiche_angegeben = true;
                                if neg.is_empty() {
                                    let lines = self.tables.get_prepare().parameters_cmd_with_some_bereich(
                                        cmd_value,
                                        "n",
                                        "",
                                        true,
                                    );
                                    param_lines.extend(lines);
                                }
                            }
                            "hoehemaximal" => {
                                if let Ok(height) = cmd_value.parse::<i32>() {
                                    self.tables.set_text_height(height.abs());
                                }
                            }
                            "typ" => {
                                self.ob_zeilen_bereiche_angegeben = true;
                                for word in cmd_value.split(',') {
                                    match word {
                                        w if w == neg.to_string() + i18n::ZEILEN_PARAS.sonne => {
                                            param_lines.insert("sonne".to_string());
                                        }
                                        w if w == neg.to_string() + i18n::ZEILEN_PARAS.schwarzesonne => {
                                            param_lines.insert("schwarzesonne".to_string());
                                        }
                                        w if w == neg.to_string() + i18n::ZEILEN_PARAS.planet => {
                                            param_lines.insert("planet".to_string());
                                        }
                                        w if w == neg.to_string() + i18n::ZEILEN_PARAS.mond => {
                                            param_lines.insert("mond".to_string());
                                        }
                                        _ => {}
                                    }
                                }
                            }
                            "primzahlen" => {
                                self.ob_zeilen_bereiche_angegeben = true;
                                for word in cmd_value.split(',') {
                                    match word {
                                        w if w == neg.to_string() + i18n::ZEILEN_PARAS.aussenerste => {
                                            param_lines.insert("aussenerste".to_string());
                                        }
                                        w if w == neg.to_string() + i18n::ZEILEN_PARAS.innenerste => {
                                            param_lines.insert("innenerste".to_string());
                                        }
                                        w if w == neg.to_string() + i18n::ZEILEN_PARAS.aussenalle => {
                                            param_lines.insert("aussenalle".to_string());
                                        }
                                        w if w == neg.to_string() + i18n::ZEILEN_PARAS.innenalle => {
                                            param_lines.insert("innenalle".to_string());
                                        }
                                        _ => {}
                                    }
                                }
                            }
                            "potenzenvonzahlen" => {
                                self.ob_zeilen_bereiche_angegeben = true;
                                if neg.is_empty() {
                                    let lines = self.tables.get_prepare().parameters_cmd_with_some_bereich(
                                        cmd_value,
                                        "^",
                                        neg,
                                        false,
                                    );
                                    param_lines.extend(lines);
                                }
                            }
                            "vielfachevonzahlen" => {
                                self.ob_zeilen_bereiche_angegeben = true;
                                if neg.is_empty() {
                                    let lines = self.tables.get_prepare().parameters_cmd_with_some_bereich(
                                        cmd_value,
                                        "b",
                                        neg,
                                        true,
                                    );
                                    param_lines.extend(lines);
                                }
                            }
                            "primzahlvielfache" => {
                                self.ob_zeilen_bereiche_angegeben = true;
                                if neg.is_empty() {
                                    let numbers = bereich_to_numbers2(cmd_value);
                                    for zahl in numbers {
                                        param_lines.insert(format!("{}p", zahl));
                                    }
                                }
                            }
                            _ => {}
                        }
                    } else {
                        // Boolesche Parameter ohne Wert
                        match cmd {
                            "invertieren" => {
                                self.ob_zeilen_bereiche_angegeben = true;
                                if neg.is_empty() {
                                    let lines = self.tables.get_prepare().parameters_cmd_with_some_bereich(
                                        "1",
                                        "i",
                                        neg,
                                        true,
                                    );
                                    param_lines.extend(lines);
                                }
                            }
                            _ => {}
                        }
                    }
                } else if in_ausgabe_section {
                    // Ausgabe-Parameter
                    if self.breite_breiten_sys_argv_para(cmd, neg) {
                        continue;
                    }
                    
                    match cmd {
                        "keineueberschriften" => {
                            self.tables.set_keine_ueberschriften(true);
                        }
                        "keinenummerierung" => {
                            self.tables.set_nummeriere(false);
                        }
                        "keineleereninhalte" => {
                            self.keine_leeren_inhalte = true;
                            self.tables.set_keine_leeren_inhalte(true);
                        }
                        _ => {
                            if let Some(eq_pos) = cmd.find('=') {
                                let cmd_name = &cmd[..eq_pos];
                                let cmd_value = &cmd[eq_pos + 1..];
                                
                                match cmd_name {
                                    "spaltenreihenfolgeundnurdiese" => {
                                        spaltenreihenfolge_und_nur_diese = bereich_to_numbers2(cmd_value);
                                    }
                                    "art" => {
                                        let breite_ist_null = format!("--{}=0", i18n::AUSGABE_PARAS.breite);
                                        match cmd_value {
                                            "shell" => {
                                                self.tables.set_out_type(SyntaxType::Default);
                                            }
                                            "nichts" => {
                                                self.tables.set_out_type(SyntaxType::Nichts);
                                            }
                                            "csv" => {
                                                self.tables.set_out_type(SyntaxType::Csv);
                                                self.tables.get_out_mut().set_one_table(true);
                                                self.breite_breiten_sys_argv_para(&breite_ist_null[2..], "");
                                            }
                                            "bbcode" => {
                                                self.html_or_bbcode = true;
                                                self.tables.set_out_type(SyntaxType::BBCode);
                                            }
                                            "html" => {
                                                self.tables.set_out_type(SyntaxType::Html);
                                                self.html_or_bbcode = true;
                                            }
                                            "emacs" => {
                                                self.tables.get_out_mut().set_one_table(true);
                                                self.tables.set_out_type(SyntaxType::Emacs);
                                                self.breite_breiten_sys_argv_para(&breite_ist_null[2..], "");
                                            }
                                            "markdown" => {
                                                self.tables.set_out_type(SyntaxType::Markdown);
                                                self.tables.get_out_mut().set_one_table(true);
                                                self.breite_breiten_sys_argv_para(&breite_ist_null[2..], "");
                                            }
                                            _ => {}
                                        }
                                    }
                                    _ => {}
                                }
                            } else {
                                // Boolesche Ausgabe-Parameter
                                match cmd {
                                    "nocolor" | "justtext" => {
                                        if neg.is_empty() {
                                            self.tables.get_out_mut().set_color(false);
                                        }
                                    }
                                    "endlessscreen" | "endless" | "dontwrap" | "onetable" => {
                                        if neg.is_empty() {
                                            self.tables.get_out_mut().set_one_table(true);
                                        }
                                    }
                                    _ => {}
                                }
                            }
                        }
                    }
                }
            }
        }
        
        if !self.tables.get_out().one_table() {
            let shell_rows_amount = shell_rows_amount();
            let text_width = self.tables.text_width();
            self.tables.set_text_width(
                if shell_rows_amount > text_width + 7 || shell_rows_amount <= 0 {
                    text_width
                } else {
                    shell_rows_amount - 7
                }
            );
        }
        
        self.tables.set_if_zeilen_setted(self.ob_zeilen_bereiche_angegeben);
        
        (
            param_lines,
            rows_as_numbers,
            rows_of_combi,
            spaltenreihenfolge_und_nur_diese,
            puniverseprims_only,
            gener_rows,
        )
    }
    
    fn help_page(&self) {
        // Hilfeseite anzeigen
        println!("{}", i18n::RETA_HILFE);
    }
    
    fn bring_all_important_begin_things(
        &mut self,
        argv: &[String],
    ) -> (
        usize,
        OrderedSet<String>,
        OrderedSet<String>,
        Vec<Vec<String>>,
        OrderedSet<i32>,
        Vec<Vec<String>>,
        OrderedSet<i32>,
        Vec<Vec<i32>>,
        (OrderedDict<i32, i32>, OrderedDict<i32, i32>),
        Vec<i32>,
        OrderedDict<i32, Vec<String>>,
        HashMap<String, OrderedDict<i32, Vec<String>>>,
        Vec<Vec<String>>,
        Vec<Vec<i32>>,
        (OrderedDict<i32, i32>, OrderedDict<i32, i32>),
    ) {
        // Lese CSV-Datei
        let csv_path = PathBuf::from("csv").join(CsvFileNames::religion.filename());
        let mut relitable = Vec::new();
        
        if let Ok(file) = File::open(&csv_path) {
            let reader = BufReader::new(file);
            
            for (i, line) in reader.lines().enumerate() {
                if let Ok(line) = line {
                    let columns: Vec<String> = line.split(';')
                        .map(|s| s.trim().to_string())
                        .collect();
                    
                    // Verarbeite JSON-in-CSV falls nötig
                    let processed_columns: Vec<String> = columns.iter()
                        .map(|col| {
                            if col.starts_with("|{") && col.ends_with("}|") {
                                if let Ok(json_val) = serde_json::from_str::<Value>(&col[1..col.len()-1]) {
                                    if self.html_or_bbcode {
                                        if self.tables.out_type() == SyntaxType::BBCode {
                                            json_val.get("bbcode")
                                                .and_then(|v| v.as_str())
                                                .unwrap_or(col)
                                                .to_string()
                                        } else if self.tables.out_type() == SyntaxType::Html {
                                            html_escape(json_val.get("html")
                                                .and_then(|v| v.as_str())
                                                .unwrap_or(col))
                                        } else {
                                            json_val.get("")
                                                .and_then(|v| v.as_str())
                                                .unwrap_or(col)
                                                .to_string()
                                        }
                                    } else {
                                        json_val.get("")
                                            .and_then(|v| v.as_str())
                                            .unwrap_or(col)
                                            .to_string()
                                    }
                                } else {
                                    col.clone()
                                }
                            } else {
                                col.clone()
                            }
                        })
                        .collect();
                    
                    relitable.push(processed_columns);
                    
                    if i == 0 {
                        self.rows_len = relitable[0].len();
                    }
                }
            }
        }
        
        // Fülle mit leeren Zeilen auf
        let target_height = self.tables.hoechste_zeile().0 as usize + 2;
        let row_width = if !relitable.is_empty() { relitable[0].len() } else { 0 };
        
        while relitable.len() < target_height {
            relitable.push(vec!["".to_string(); row_width]);
        }
        
        self.relitable = relitable.clone();
        
        // Initialisiere Flags
        self.html_or_bbcode = false;
        self.breite_or_breiten = false;
        self.keine_leeren_inhalte = false;
        self.tables.set_keine_leeren_inhalte(false);
        
        // Verarbeite Parameter
        let (param_lines, rows_as_numbers, rows_of_combi, spaltenreihenfolge_und_nur_diese, puniverseprims, gener_rows) =
            self.parameters_to_commands_and_numbers(argv, "");
        
        let (param_lines_not, rows_as_numbers_not, rows_of_combi_not, _, puniverseprims_not, gener_rows_not) =
            self.parameters_to_commands_and_numbers(argv, "-");
        
        // Initialisiere Datenstrukturen
        self.data_dict = vec![OrderedDict::new(); 14];
        
        // Store parameters
        self.store_parameters_for_columns();
        
        // Produce spalten numbers
        self.produce_all_spalten_numbers("");
        
        // Setze HTML/BBcode-Breite
        if self.html_or_bbcode && !self.breite_or_breiten {
            set_shell_rows_amount(0);
            self.tables.set_text_width(0);
        }
        
        // Entferne Dopplungen
        let (param_lines, param_lines_not) = self.tables.get_prepare().delete_doubles_in_sets(
            param_lines,
            param_lines_not,
        );
        
        // Setze Spalten-Mengen
        self.rows_as_numbers = self.spalten_arten_key_spaltennummern_value[&self.spalten_type_naming.ordinary].clone();
        self.gener_rows = self.spalten_arten_key_spaltennummern_value[&self.spalten_type_naming.generated1].clone();
        self.puniverseprims = self.spalten_arten_key_spaltennummern_value[&self.spalten_type_naming.concat1].clone();
        self.rows_of_combi = self.spalten_arten_key_spaltennummern_value[&self.spalten_type_naming.kombi1].clone();
        self.rows_of_combi2 = self.spalten_arten_key_spaltennummern_value[&self.spalten_type_naming.kombi2].clone();
        
        // Setze weitere Tabellen-Strukturen
        self.tables.set_gener_rows(self.gener_rows.clone());
        self.tables.get_prepare_mut().set_rows_as_numbers(self.rows_as_numbers.clone());
        self.tables.get_out_mut().set_rows_as_numbers(self.rows_as_numbers.clone());
        
        self.spalten_vanilla_amount = self.rows_as_numbers.len() as i32;
        
        // Lese Konkatenations-CSV-Dateien
        let mut csv_theirs_spalten: HashMap<i32, OrderedDict<i32, Vec<String>>> = HashMap::new();
        
        let inputs = vec![
            (self.puniverseprims.clone(), 1),
            (self.spalten_arten_key_spaltennummern_value[&self.spalten_type_naming.gebr_gal1].clone(), 2),
            (self.spalten_arten_key_spaltennummern_value[&self.spalten_type_naming.gebr_gal1].clone(), 3),
            (self.spalten_arten_key_spaltennummern_value[&self.spalten_type_naming.gebro_uni1].clone(), 4),
            (self.spalten_arten_key_spaltennummern_value[&self.spalten_type_naming.gebro_uni1].clone(), 5),
            (self.spalten_arten_key_spaltennummern_value[&self.spalten_type_naming.gebr_emo1].clone(), 6),
            (self.spalten_arten_key_spaltennummern_value[&self.spalten_type_naming.gebr_emo1].clone(), 7),
            (self.spalten_arten_key_spaltennummern_value[&self.spalten_type_naming.gebr_groe1].clone(), 8),
            (self.spalten_arten_key_spaltennummern_value[&self.spalten_type_naming.gebr_groe1].clone(), 9),
        ];
        
        for (input, i) in inputs {
            let (new_relitable, new_rows_as_numbers, csv_spalten) = 
                self.tables.get_concat().read_concat_csv(
                    &self.relitable,
                    &self.rows_as_numbers,
                    &input,
                    i,
                );
            
            self.relitable = new_relitable;
            self.rows_as_numbers = new_rows_as_numbers;
            csv_theirs_spalten.insert(i, csv_spalten);
        }
        
        let prim_spalten = csv_theirs_spalten.get(&1).cloned().unwrap_or_default();
        
        let mut gebr = HashMap::new();
        gebr.insert("Gal".to_string(), csv_theirs_spalten.get(&2).cloned().unwrap_or_default());
        gebr.insert("Gal2".to_string(), csv_theirs_spalten.get(&3).cloned().unwrap_or_default());
        gebr.insert("Uni".to_string(), csv_theirs_spalten.get(&4).cloned().unwrap_or_default());
        gebr.insert("Uni2".to_string(), csv_theirs_spalten.get(&5).cloned().unwrap_or_default());
        gebr.insert("Emo".to_string(), csv_theirs_spalten.get(&6).cloned().unwrap_or_default());
        gebr.insert("Emo2".to_string(), csv_theirs_spalten.get(&7).cloned().unwrap_or_default());
        gebr.insert("Groe".to_string(), csv_theirs_spalten.get(&8).cloned().unwrap_or_default());
        gebr.insert("Groe2".to_string(), csv_theirs_spalten.get(&9).cloned().unwrap_or_default());
        
        // Vorbereitung für Ausgabe
        let (finally_display_lines_early, headings_amount_early, newer_table_early, numlen_early, rows_range_early) =
            self.tables.get_prepare().prepare4out_before_for_loop_spalten_zeilen_bestimmen(
                &self.relitable,
                &param_lines,
                &param_lines_not,
            );
        
        // Setze letzte Zeilennummer
        if let Some(&last_line) = finally_display_lines_early.iter().max() {
            self.last_line_number = last_line;
        }
        
        // Führe Konkatenationen durch
        self.relitable = self.tables.get_concat().concat_vervielfache_zeile(
            self.relitable.clone(),
            self.rows_as_numbers.clone(),
        ).0;
        
        self.relitable = self.tables.get_concat().concat_modallogik(
            self.relitable.clone(),
            self.gener_rows.clone(),
            self.rows_as_numbers.clone(),
        ).0;
        
        self.relitable = self.tables.get_concat().concat_prim_creativity_type(
            self.relitable.clone(),
            self.rows_as_numbers.clone(),
        ).0;
        
        self.relitable = self.tables.get_concat().concat_gleichheit_freiheit_dominieren(
            self.relitable.clone(),
            self.rows_as_numbers.clone(),
        ).0;
        
        self.relitable = self.tables.get_concat().concat_geist_emotion_energie_materie_topologie(
            self.relitable.clone(),
            self.rows_as_numbers.clone(),
        ).0;
        
        self.relitable = self.tables.get_concat().concat_mond_exponzieren_logarithmus_typ(
            self.relitable.clone(),
            self.rows_as_numbers.clone(),
        ).0;
        
        // Lese Kombi-CSV-Dateien
        let animals_professions_table;
        let kombi_table_kombis;
        let maintable2subtable_relation;
        
        if !self.rows_of_combi.is_empty() {
            (animals_professions_table, self.relitable, kombi_table_kombis, maintable2subtable_relation) =
                self.tables.get_combis().read_kombi_csv(
                    &self.relitable,
                    &self.rows_as_numbers,
                    &self.rows_of_combi,
                    &CsvFileNames::kombi13,
                );
        } else {
            animals_professions_table = Vec::new();
            kombi_table_kombis = Vec::new();
            maintable2subtable_relation = (OrderedDict::new(), OrderedDict::new());
        }
        
        let animals_professions_table2;
        let kombi_table_kombis2;
        let maintable2subtable_relation2;
        
        if !self.rows_of_combi2.is_empty() {
            (animals_professions_table2, self.relitable, kombi_table_kombis2, maintable2subtable_relation2) =
                self.tables.get_combis().read_kombi_csv(
                    &self.relitable,
                    &self.rows_as_numbers,
                    &self.rows_of_combi2,
                    &CsvFileNames::kombi15,
                );
        } else {
            animals_professions_table2 = Vec::new();
            kombi_table_kombis2 = Vec::new();
            maintable2subtable_relation2 = (OrderedDict::new(), OrderedDict::new());
        }
        
        (
            self.rows_len,
            param_lines,
            param_lines_not,
            self.relitable.clone(),
            self.rows_as_numbers.clone(),
            animals_professions_table,
            self.rows_of_combi.clone(),
            kombi_table_kombis,
            maintable2subtable_relation,
            spaltenreihenfolge_und_nur_diese,
            prim_spalten,
            gebr,
            animals_professions_table2,
            kombi_table_kombis2,
            maintable2subtable_relation2,
        )
    }
    
    fn oberes_maximum_arg(&self, arg: &str) -> (Vec<i32>, bool) {
        let mut werte = Vec::new();
        
        let oberesmaximum_str = format!("{}=", i18n::ZEILEN_PARAS.oberesmaximum);
        let vorhervonausschnitt_str = format!("{}=", i18n::ZEILEN_PARAS.vorhervonausschnitt);
        
        if arg.starts_with(&oberesmaximum_str) {
            let value_str = &arg[oberesmaximum_str.len()..];
            if let Ok(value) = value_str.parse::<i32>() {
                werte.push(value);
                return (werte, true);
            }
        } else if arg.starts_with(&vorhervonausschnitt_str) {
            let value_str = &arg[vorhervonausschnitt_str.len()..];
            let werte_list = bereich_to_numbers2(value_str, false, 0)
                .iter()
                .map(|&a| a + 1)
                .collect::<Vec<i32>>();
            werte = werte_list.iter().map(|&w| w.max(1024)).collect();
            return (werte, false);
        }
        
        (werte, false)
    }
    
    fn oberes_maximum2(&self, argv2: &[String]) -> Option<i32> {
        let mut werte = vec![self.tables.hoechste_zeile().0];
        
        for arg in argv2 {
            let (new_werte, _) = self.oberes_maximum_arg(arg);
            werte.extend(new_werte);
        }
        
        if werte.is_empty() {
            None
        } else {
            Some(*werte.iter().max().unwrap())
        }
    }
    
    fn oberes_maximum(&mut self, arg: &str) -> bool {
        let (liste, wahrheitswert) = self.oberes_maximum_arg(arg);
        if liste.is_empty() || !wahrheitswert {
            return false;
        }
        let max_ = *liste.iter().chain(&[self.tables.hoechste_zeile().0]).max().unwrap();
        self.tables.set_hoechste_zeile((max_, max_));
        true
    }
}

// Hilfsfunktionen
fn html_escape(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#39;")
}

fn bereich_to_numbers2(s: &str, include_end: bool, default: i32) -> Vec<i32> {
    let mut result = Vec::new();
    
    for part in s.split(',') {
        let part = part.trim();
        if part.contains('-') {
            let parts: Vec<&str> = part.split('-').collect();
            if parts.len() == 2 {
                if let (Ok(start), Ok(end)) = (parts[0].parse::<i32>(), parts[1].parse::<i32>()) {
                    if include_end {
                        for i in start..=end {
                            result.push(i);
                        }
                    } else {
                        for i in start..end {
                            result.push(i);
                        }
                    }
                }
            }
        } else if let Ok(num) = part.parse::<i32>() {
            result.push(num);
        }
    }
    
    if result.is_empty() {
        vec![default]
    } else {
        result
    }
}

// i18n-Modul (vereinfacht)
mod i18n {
    use lazy_static::lazy_static;
    use std::collections::HashMap;
    use serde_json::Value;
    
    pub struct CsvFileNames;
    impl CsvFileNames {
        pub const religion: &'static str = "religion.csv";
        pub const kombi13: &'static str = "kombi13.csv";
        pub const kombi15: &'static str = "kombi15.csv";
        
        pub fn filename(&self) -> &'static str {
            match self {
                CsvFileNames::religion => "religion.csv",
                CsvFileNames::kombi13 => "kombi13.csv",
                CsvFileNames::kombi15 => "kombi15.csv",
            }
        }
    }
    
    pub struct ParametersMain {
        pub multiplikationen: [String; 1],
        pub gebrochenuniversum: [String; 2],
        pub gebrochengalaxie: [String; 2],
        pub gebrochenemotion: [String; 2],
        pub gebrochengroesse: [String; 2],
        pub primvielfache: Vec<String>,
        pub alles: String,
    }
    
    lazy_static! {
        pub static ref I18N: I18n = I18n::new();
        pub static ref PARAMETERS_MAIN: ParametersMain = ParametersMain {
            multiplikationen: ["multiplikationen".to_string()],
            gebrochenuniversum: ["gebrochenuniversum".to_string(), "gebrochenuniversum".to_string()],
            gebrochengalaxie: ["gebrochengalaxie".to_string(), "gebrochengalaxie".to_string()],
            gebrochenemotion: ["gebrochenemotion".to_string(), "gebrochenemotion".to_string()],
            gebrochengroesse: ["gebrochengroesse".to_string(), "gebrochengroesse".to_string()],
            primvielfache: vec!["primvielfache".to_string()],
            alles: "alles".to_string(),
        };
        
        pub static ref RETA: RetaStrings = RetaStrings::new();
        pub static ref ZEILEN_PARAS: ZeilenParas = ZeilenParas::new();
        pub static ref AUSGABE_PARAS: AusgabeParas = AusgabeParas::new();
        pub static ref KOMBI_MAIN_PARAS: KombiMainParas = KombiMainParas::new();
        pub static ref KOMBI_PARA_N_DATA_MATRIX: HashMap<i32, Value> = HashMap::new();
        pub static ref KOMBI_PARA_N_DATA_MATRIX2: HashMap<i32, Value> = HashMap::new();
    }
    
    pub const GEBROCHEN_SPALTEN_MAXIMUM_PLUS_1: i32 = 100;
    pub const RETA_HILFE: &str = "Hilfetext für RETA";
    
    pub fn get_para_n_data_matrix() -> Vec<(Vec<String>, Vec<String>, Vec<Vec<i32>>)> {
        vec![
            (
                vec!["multiplikationen".to_string()],
                vec![],
                vec![vec![], vec![], vec![], vec![], vec![], vec![], vec![], vec![], vec![], vec![], vec![], vec![]],
            ),
            // Weitere Einträge hier hinzufügen
        ]
    }
    
    pub struct I18n {
        pub sprachen_wahl: String,
        pub sprachen: HashMap<String, String>,
    }
    
    impl I18n {
        pub fn new() -> Self {
            let mut sprachen = HashMap::new();
            sprachen.insert("de".to_string(), "de".to_string());
            sprachen.insert("en".to_string(), "en".to_string());
            
            Self {
                sprachen_wahl: "de".to_string(),
                sprachen,
            }
        }
    }
    
    pub struct RetaStrings {
        pub keine_num_wort: String,
        pub cliout_saetze: [String; 10],
    }
    
    impl RetaStrings {
        pub fn new() -> Self {
            Self {
                keine_num_wort: "keinenummerierung".to_string(),
                cliout_saetze: [
                    "Fehler: ".to_string(),
                    " ist kein gültiger Parameter für ".to_string(),
                    "Mögliche Parameter: ".to_string(),
                    // Weitere Strings hier hinzufügen
                ].map(|s| s.to_string()),
            }
        }
    }
    
    pub struct ZeilenParas {
        pub alles: String,
        pub heute: String,
        pub gestern: String,
        pub morgen: String,
        pub oberesmaximum: String,
        pub vorhervonausschnitt: String,
        pub sonne: String,
        pub schwarzesonne: String,
        pub planet: String,
        pub mond: String,
        pub aussenerste: String,
        pub innenerste: String,
        pub aussenalle: String,
        pub innenalle: String,
        pub invertieren: String,
    }
    
    impl ZeilenParas {
        pub fn new() -> Self {
            Self {
                alles: "alles".to_string(),
                heute: "heute".to_string(),
                gestern: "gestern".to_string(),
                morgen: "morgen".to_string(),
                oberesmaximum: "oberesmaximum".to_string(),
                vorhervonausschnitt: "vorhervonausschnitt".to_string(),
                sonne: "sonne".to_string(),
                schwarzesonne: "schwarzesonne".to_string(),
                planet: "planet".to_string(),
                mond: "mond".to_string(),
                aussenerste: "aussenerste".to_string(),
                innenerste: "innenerste".to_string(),
                aussenalle: "aussenalle".to_string(),
                innenalle: "innenalle".to_string(),
                invertieren: "invertieren".to_string(),
            }
        }
    }
    
    pub struct AusgabeParas {
        pub breite: String,
        pub breiten: String,
        pub keineueberschriften: String,
        pub keinenummerierung: String,
        pub keineleereninhalte: String,
        pub spaltenreihenfolgeundnurdiese: String,
        pub art: String,
        pub nocolor: String,
        pub justtext: String,
        pub endlessscreen: String,
        pub endless: String,
        pub dontwrap: String,
        pub onetable: String,
    }
    
    impl AusgabeParas {
        pub fn new() -> Self {
            Self {
                breite: "breite".to_string(),
                breiten: "breiten".to_string(),
                keineueberschriften: "keineueberschriften".to_string(),
                keinenummerierung: "keinenummerierung".to_string(),
                keineleereninhalte: "keineleereninhalte".to_string(),
                spaltenreihenfolgeundnurdiese: "spaltenreihenfolgeundnurdiese".to_string(),
                art: "art".to_string(),
                nocolor: "nocolor".to_string(),
                justtext: "justtext".to_string(),
                endlessscreen: "endlessscreen".to_string(),
                endless: "endless".to_string(),
                dontwrap: "dontwrap".to_string(),
                onetable: "onetable".to_string(),
            }
        }
    }
    
    pub struct KombiMainParas {
        pub galaxie: String,
        pub universum: String,
    }
    
    impl KombiMainParas {
        pub fn new() -> Self {
            Self {
                galaxie: "galaxie".to_string(),
                universum: "universum".to_string(),
            }
        }
    }
}

// Hauptfunktion
fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 {
        println!("Verwendung: reta [OPTIONEN]");
        println!("Für Hilfe: reta -h oder reta --help");
        return;
    }
    
    let mut program = Program::new(args, None, true);
    
    // Gib die resultierende Tabelle aus
    for line in program.resulting_table() {
        println!("{}", line);
    }
}
