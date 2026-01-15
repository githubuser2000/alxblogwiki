// lib4tables_concat.rs
use std::collections::{BTreeSet, HashMap, HashSet, BTreeMap};
use std::path::Path;
use std::fs::File;
use std::io::{BufReader, Read};
use std::cmp::Ordering;
use num_rational::Ratio; // For Fraction equivalent
use num_traits::{One, Zero};
use indexmap::{IndexSet, IndexMap};
use serde::{Deserialize, Serialize};
use anyhow::{Result, Context};

// Re-exports from other modules
mod center;
mod lib4tables;
mod lib4tables_enum;

use center::*;
use lib4tables::*;
use lib4tables_enum::*;

// Type aliases for clarity
type Row = Vec<String>;
type Table = Vec<Row>;
type OrderedSet<T> = IndexSet<T>;
type DefaultOrderedDict<K, V> = IndexMap<K, V>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Concat {
    tables: Tables, // Assuming Tables is defined elsewhere
    ones: OrderedSet<usize>,
    csvs_already_read: OrderedDict<usize, Table>,
    csvs_same: OrderedDict<usize, Vec<usize>>,
    brueche_uni: OrderedSet<Ratio<i32>>,
    brueche_gal: OrderedSet<Ratio<i32>>,
    gebr_rat_mul_stern_uni: OrderedSet<Ratio<i32>>,
    gebr_rat_div_stern_uni: OrderedSet<Ratio<i32>>,
    gebr_rat_mul_gleichf_uni: OrderedSet<Ratio<i32>>,
    gebr_rat_div_gleichf_uni: OrderedSet<Ratio<i32>>,
    gebr_rat_mul_stern_gal: OrderedSet<Ratio<i32>>,
    gebr_rat_div_stern_gal: OrderedSet<Ratio<i32>>,
    gebr_rat_mul_gleichf_gal: OrderedSet<Ratio<i32>>,
    gebr_rat_div_gleichf_gal: OrderedSet<Ratio<i32>>,
}

impl Concat {
    pub fn new(tables: Tables) -> Self {
        Self {
            tables,
            ones: OrderedSet::new(),
            csvs_already_read: OrderedDict::new(),
            csvs_same: OrderedDict::from([
                (1, vec![1]),
                (2, vec![2, 4]),
                (3, vec![3, 5]),
                (4, vec![2, 4]),
                (5, vec![3, 5]),
            ]),
            brueche_uni: OrderedSet::new(),
            brueche_gal: OrderedSet::new(),
            gebr_rat_mul_stern_uni: OrderedSet::new(),
            gebr_rat_div_stern_uni: OrderedSet::new(),
            gebr_rat_mul_gleichf_uni: OrderedSet::new(),
            gebr_rat_div_gleichf_uni: OrderedSet::new(),
            gebr_rat_mul_stern_gal: OrderedSet::new(),
            gebr_rat_div_stern_gal: OrderedSet::new(),
            gebr_rat_mul_gleichf_gal: OrderedSet::new(),
            gebr_rat_div_gleichf_gal: OrderedSet::new(),
        }
    }

    pub fn concat_love_polygon(&mut self, relitable: &mut Table, rows_as_numbers: &mut HashSet<usize>) -> Result<()> {
        if rows_as_numbers.contains(&9) {
            rows_as_numbers.insert(relitable[0].len());
            
            let tags = [ST::SternPolygon, ST::Galaxie, ST::GleichfoermigesPolygon];
            self.tables.generated_spalten_parameter_tags
                .insert(rows_as_numbers.len() - 1, tags.iter().cloned().collect());
            
            for i in 0..relitable.len() {
                if let Some(col) = relitable.get_mut(i) {
                    if col.len() > 8 && !col[8].trim().is_empty() {
                        let new_value = format!(
                            "{}{}{}{}",
                            col[8],
                            i18n::polygon1(" der eigenen Strukturgröße ("),
                            col.get(4).unwrap_or(&String::new()),
                            i18n::polygon2(") auf dich bei gleichförmigen Polygonen")
                        );
                        col.push(new_value);
                    } else {
                        col.push(String::new());
                    }
                }
            }
            
            let new_index = self.tables.generated_spalten_parameter.len() + self.tables.spalten_vanilla_amount;
            if self.tables.generated_spalten_parameter.contains_key(&new_index) {
                return Err(anyhow::anyhow!("Duplicate generated spalten parameter index"));
            }
            
            if let Some(row) = self.tables.data_dict.get(&0) {
                self.tables.generated_spalten_parameter.insert(new_index, row[9].clone());
            }
        }
        Ok(())
    }

    pub fn gleichheit_freiheit_vergleich(&self, zahl: i32) -> String {
        let mut ausgabe_string_list = Vec::new();
        
        match zahl % 4 {
            0 => ausgabe_string_list.push(i18n::gleichheit_freiheit_vergleich("Dominieren, Unterordnen")),
            1 => ausgabe_string_list.push(i18n::gleichheit_freiheit_vergleich("Freiheit")),
            3 => ausgabe_string_list.push(i18n::gleichheit_freiheit_vergleich("Einschränkung der Freiheit")),
            2 => {
                if (zahl - 2) % 8 == 0 {
                    ausgabe_string_list.push(i18n::gleichheit_freiheit_vergleich("Gleichheit"));
                }
                if (zahl - 6) % 16 == 0 {
                    ausgabe_string_list.push(i18n::gleichheit_freiheit_vergleich("den anderen überbieten wollen"));
                }
                if (zahl - 14) % 16 == 0 {
                    ausgabe_string_list.push(i18n::gleichheit_freiheit_vergleich("den anderen unterbieten wollen"));
                }
            }
            _ => {}
        }
        
        ausgabe_string_list.join("; ")
    }

    pub fn geist_emotion_energie_materie_topologie(&self, zahl: i32) -> String {
        let pr_fa = primfaktoren(zahl);
        let auss: Vec<bool> = pr_fa.iter().map(|&a| could_be_prime_number_primzahlkreuz_fuer_aussen(a)).collect();
        let innen: Vec<bool> = pr_fa.iter().map(|&a| could_be_prime_number_primzahlkreuz_fuer_innen(a)).collect();
        let zwei = pr_fa.iter().filter(|&&a| a == 2).count();
        let gefuehl = auss.iter().any(|&a| a);
        let denken = innen.iter().any(|&a| a);
        
        let total_topologie = zwei > 1 && gefuehl;
        let etwas_topologie = (zwei > 1 || (zwei > 0 && gefuehl)) && !total_topologie;
        let total_materie = zwei > 4;
        let etwas_materie = zwei == 4;
        let wenig_materie = zwei == 3;
        let kaum_materie = zwei == 2;
        
        let (x, y, z) = (denken, pr_fa.contains(&2), pr_fa.contains(&3));
        let total_energie = x && y && z;
        let einermassen_energie = ((x && y) || (y && z) || (x && z)) && !total_energie;
        let kaum_energie = !einermassen_energie && !total_energie && (x || y || z);
        
        let mut ausgabe_string_list = Vec::new();
        
        if denken {
            ausgabe_string_list.push(i18n::energietopologie1("eine Denkart"));
        }
        if gefuehl {
            ausgabe_string_list.push(i18n::energietopologie1("eine Gefühlsart"));
        }
        if total_materie {
            ausgabe_string_list.push(i18n::energietopologie1("total eine Art, etwas geistig zu erzeugen"));
        }
        if total_topologie {
            ausgabe_string_list.push(i18n::energietopologie1("total eine Art zu erleben"));
        }
        if total_energie {
            ausgabe_string_list.push(i18n::energietopologie1("total eine Energie-Art"));
        }
        if etwas_topologie {
            ausgabe_string_list.push(i18n::energietopologie1("etwas eine Art zu erleben"));
        }
        if etwas_materie {
            ausgabe_string_list.push(i18n::energietopologie1("etwas eine Art, etwas geistig zu erzeugen"));
        }
        if wenig_materie {
            ausgabe_string_list.push(i18n::energietopologie1("wenig eine Art, etwas geistig zu erzeugen"));
        }
        if einermassen_energie {
            ausgabe_string_list.push(i18n::energietopologie1("einigermaßen eine Energie-Art"));
        }
        if kaum_energie {
            ausgabe_string_list.push(i18n::energietopologie1("kaum eine Energie-Art"));
        }
        if kaum_materie {
            ausgabe_string_list.push(i18n::energietopologie1("kaum eine Art, etwas geistig zu erzeugen"));
        }
        
        ausgabe_string_list.join("; ")
    }

    pub fn concat_gleichheit_freiheit_dominieren(
        &mut self,
        relitable: &mut Table,
        rows_as_numbers: &mut HashSet<usize>
    ) -> Result<()> {
        if rows_as_numbers.contains(&132) {
            rows_as_numbers.insert(relitable[0].len());
            
            let tags = [ST::SternPolygon, ST::Universum];
            self.tables.generated_spalten_parameter_tags
                .insert(rows_as_numbers.len() - 1, tags.iter().cloned().collect());
            
            for i in 0..=self.tables.last_line_number {
                let ausgabe_string = if i == 0 {
                    i18n::gleichheit_freiheit_vergleich("Gleichheit, Freiheit, Dominieren (Ordnungen [12]) Generiert")
                } else {
                    self.gleichheit_freiheit_vergleich(i as i32)
                };
                
                if let Some(row) = relitable.get_mut(i) {
                    row.push(ausgabe_string);
                }
            }
            
            let new_index = self.tables.generated_spalten_parameter.len() + self.tables.spalten_vanilla_amount;
            if self.tables.generated_spalten_parameter.contains_key(&new_index) {
                return Err(anyhow::anyhow!("Duplicate generated spalten parameter index"));
            }
            
            if let Some(row) = self.tables.data_dict.get(&0) {
                self.tables.generated_spalten_parameter.insert(new_index, row[132].clone());
            }
        }
        Ok(())
    }

    pub fn concat_geist_emotion_energie_materie_topologie(
        &mut self,
        relitable: &mut Table,
        rows_as_numbers: &mut HashSet<usize>
    ) -> Result<()> {
        if rows_as_numbers.contains(&242) {
            rows_as_numbers.insert(relitable[0].len());
            
            let tags = [ST::SternPolygon, ST::Universum];
            self.tables.generated_spalten_parameter_tags
                .insert(rows_as_numbers.len() - 1, tags.iter().cloned().collect());
            
            for i in 0..=self.tables.last_line_number {
                let ausgabe_string = if i == 0 {
                    i18n::ausgabe_string("Energie oder Denkart oder Gefühlsart oder Materie-Art oder Topologie-Art")
                } else {
                    self.geist_emotion_energie_materie_topologie(i as i32)
                };
                
                if let Some(row) = relitable.get_mut(i) {
                    row.push(ausgabe_string);
                }
            }
            
            let new_index = self.tables.generated_spalten_parameter.len() + self.tables.spalten_vanilla_amount;
            if self.tables.generated_spalten_parameter.contains_key(&new_index) {
                return Err(anyhow::anyhow!("Duplicate generated spalten parameter index"));
            }
            
            if let Some(row) = self.tables.data_dict.get(&0) {
                self.tables.generated_spalten_parameter.insert(new_index, row[242].clone());
            }
        }
        Ok(())
    }

    pub fn concat_prim_creativity_type(
        &mut self,
        relitable: &mut Table,
        rows_as_numbers: &mut HashSet<usize>
    ) -> Result<()> {
        if rows_as_numbers.contains(&64) {
            rows_as_numbers.insert(relitable[0].len());
            
            let tags = [ST::SternPolygon, ST::Galaxie];
            self.tables.generated_spalten_parameter_tags
                .insert(rows_as_numbers.len() - 1, tags.iter().cloned().collect());
            
            for i in 0..=self.tables.last_line_number {
                let prim_creativity_type = prim_creativity(i as i32);
                let value = if i == 0 {
                    i18n::krea_zahl("Evolutions-Züchtungs-Kreativität")
                } else {
                    match prim_creativity_type {
                        0 => i18n::krea_zahl("0. Primzahl 1"),
                        1 => i18n::krea_zahl("1. Primzahl und Sonnenzahl"),
                        2 => i18n::krea_zahl("2. Sonnenzahl, aber keine Primzahl"),
                        3 => i18n::krea_zahl("3. Mondzahl"),
                        _ => String::new(),
                    }
                };
                
                if let Some(row) = relitable.get_mut(i) {
                    row.push(value);
                }
            }
            
            let new_index = self.tables.generated_spalten_parameter.len() + self.tables.spalten_vanilla_amount;
            if self.tables.generated_spalten_parameter.contains_key(&new_index) {
                return Err(anyhow::anyhow!("Duplicate generated spalten parameter index"));
            }
            
            if let Some(row) = self.tables.data_dict.get(&0) {
                self.tables.generated_spalten_parameter.insert(new_index, row[64].clone());
            }
        }
        Ok(())
    }

    pub fn convert_set_of_paaren_to_dict_of_num_to_paare_div(
        &self,
        paare_set: &OrderedSet<(i32, i32)>,
        gleichf: bool
    ) -> DefaultOrderedDict<i32, OrderedSet<(i32, i32)>> {
        let mut result: DefaultOrderedDict<i32, OrderedSet<(i32, i32)>> = DefaultOrderedDict::new();
        
        for paar in paare_set {
            let (a, b) = *paar;
            let div = if !gleichf {
                a as f64 / b as f64
            } else {
                b as f64 / a as f64
            };
            let div_int = div.round() as i32;
            
            result.entry(div_int).or_insert_with(OrderedSet::new).insert((a, b));
        }
        
        result
    }

    pub fn convert_set_of_paaren_to_dict_of_num_to_paare_mul(
        &self,
        paare_set: &OrderedSet<(i32, i32)>,
        gleichf: bool
    ) -> DefaultOrderedDict<i32, OrderedSet<(i32, i32)>> {
        let mut result: DefaultOrderedDict<i32, OrderedSet<(i32, i32)>> = DefaultOrderedDict::new();
        
        for paar in paare_set {
            let (a, b) = *paar;
            let mul = if !gleichf {
                a * b
            } else {
                (1.0 / (a as f64 * b as f64)).round() as i32
            };
            
            result.entry(mul).or_insert_with(OrderedSet::new).insert((a, b));
        }
        
        result
    }

    pub fn combine_dicts<K, V>(
        &self,
        a: &DefaultOrderedDict<K, V>,
        b: &DefaultOrderedDict<K, V>
    ) -> DefaultOrderedDict<K, V>
    where
        K: Eq + std::hash::Hash + Clone,
        V: Clone + std::ops::BitOr<Output = V>,
    {
        let mut result = a.clone();
        
        for (key, value) in b {
            result.entry(key.clone())
                .and_modify(|v| *v = v.clone() | value.clone())
                .or_insert_with(|| value.clone());
        }
        
        result
    }

    pub fn read_one_csv_and_return(&mut self, wahl: usize) -> Result<&Table> {
        let place = self.read_concat_csv_chose_csv_file(wahl);
        
        if !self.csvs_already_read.contains_key(&wahl) {
            let file = File::open(&place)
                .with_context(|| format!("Failed to open CSV file: {}", place))?;
            let mut rdr = csv::ReaderBuilder::new()
                .delimiter(b';')
                .from_reader(BufReader::new(file));
            
            let mut table = Table::new();
            for result in rdr.records() {
                let record = result?;
                let row: Vec<String> = record.iter().map(|s| s.to_string()).collect();
                table.push(row);
            }
            
            self.csvs_already_read.insert(wahl, table);
            
            // Update brueche sets based on wahl
            let table_ref = &self.csvs_already_read[&wahl];
            let brueche = self.get_all_brueche(table_ref);
            
            match wahl {
                2 | 3 => self.brueche_gal = brueche,
                4 | 5 => self.brueche_uni = brueche,
                _ => {}
            }
        }
        
        Ok(&self.csvs_already_read[&wahl])
    }

    fn get_all_brueche(&self, table: &Table) -> OrderedSet<Ratio<i32>> {
        let mut menge = OrderedSet::new();
        
        for (i, row) in table.iter().enumerate().skip(1) {
            for (k, cell) in row.iter().enumerate().skip(1) {
                if cell.trim().len() > 3 {
                    let frac = Ratio::new((i + 2) as i32, (k + 2) as i32);
                    if *frac.denom() != 1 && *frac.numer() != 1 {
                        menge.insert(frac);
                    }
                }
            }
        }
        
        menge
    }

    fn read_concat_csv_chose_csv_file(&self, concat_table: usize) -> String {
        let filename = match concat_table {
            1 => "prim.csv",
            2 | 3 => "bruch13.csv",
            4 | 5 => "bruch15.csv",
            6 | 7 => "bruch7.csv",
            8 | 9 => "bruchStrukGroesse.csv",
            _ => "unknown.csv",
        };
        
        let mut path = std::env::current_dir().unwrap();
        path.push("csv");
        path.push(filename);
        
        path.to_string_lossy().into_owned()
    }
}

// Helper functions that need to be implemented elsewhere
mod i18n {
    pub fn polygon1(s: &str) -> &str { s }
    pub fn polygon2(s: &str) -> &str { s }
    pub fn gleichheit_freiheit_vergleich(s: &str) -> String { s.to_string() }
    pub fn energietopologie1(s: &str) -> String { s.to_string() }
    pub fn ausgabe_string(s: &str) -> String { s.to_string() }
    pub fn krea_zahl(s: &str) -> String { s.to_string() }
    // ... other i18n functions
}

// Main tables struct (simplified for now)
#[derive(Debug, Clone)]
pub struct Tables {
    pub generated_spalten_parameter: HashMap<usize, String>,
    pub generated_spalten_parameter_tags: HashMap<usize, HashSet<ST>>,
    pub spalten_vanilla_amount: usize,
    pub last_line_number: usize,
    pub data_dict: HashMap<usize, Vec<String>>,
    pub html_output_yes: bool,
    pub bbcode_output_yes: bool,
    pub hoechste_zeile: HashMap<usize, usize>,
}

// Enum for ST (SternType)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ST {
    SternPolygon,
    Galaxie,
    GleichfoermigesPolygon,
    Universum,
    GebrRat,
    // ... other variants
}

// Functions from imported modules (simplified for now)
fn primfaktoren(zahl: i32) -> Vec<i32> {
    // Implementation needed
    vec![]
}

fn could_be_prime_number_primzahlkreuz_fuer_aussen(zahl: i32) -> bool {
    // Implementation needed
    false
}

fn could_be_prime_number_primzahlkreuz_fuer_innen(zahl: i32) -> bool {
    // Implementation needed
    false
}

fn prim_creativity(zahl: i32) -> i32 {
    // Implementation needed
    0
}
