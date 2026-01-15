use crate::{
    center::*,
    lib4tables::*,
    lib4tables_enum::ST,
    types::*,
    errors::*,
};
use std::path::{Path, PathBuf};
use std::fs::File;
use std::io::{BufReader, BufRead};
use num_rational::Ratio;
use indexmap::IndexSet;
use rayon::prelude::*;

/// Main Concat struct - equivalent to Python Concat class
#[derive(Debug, Clone)]
pub struct Concat {
    pub tables: Tables,
    pub ones: OrderedSet<usize>,
    pub csvs_already_read: OrderedDict<usize, Table>,
    pub csvs_same: OrderedDict<usize, Vec<usize>>,
    pub brueche_uni: OrderedSet<Ratio<i32>>,
    pub brueche_gal: OrderedSet<Ratio<i32>>,
    pub gebr_rat_mul_stern_uni: OrderedSet<Ratio<i32>>,
    pub gebr_rat_div_stern_uni: OrderedSet<Ratio<i32>>,
    pub gebr_rat_mul_gleichf_uni: OrderedSet<Ratio<i32>>,
    pub gebr_rat_div_gleichf_uni: OrderedSet<Ratio<i32>>,
    pub gebr_rat_mul_stern_gal: OrderedSet<Ratio<i32>>,
    pub gebr_rat_div_stern_gal: OrderedSet<Ratio<i32>>,
    pub gebr_rat_mul_gleichf_gal: OrderedSet<Ratio<i32>>,
    pub gebr_rat_div_gleichf_gal: OrderedSet<Ratio<i32>>,
    pub brueche_emo: OrderedSet<Ratio<i32>>,
    pub brueche_struk_groesse: OrderedSet<Ratio<i32>>,
    pub relitable: Option<Table>,
    pub rows_as_numbers: Option<HashSet<usize>>,
    pub transzendentalien: OrderedDict<String, Vec<String>>,
    pub motivation: OrderedDict<String, Vec<String>>,
    pub gebr_rat_etwa_schon_mal_dabei_gewesen: OrderedSet<Ratio<i32>>,
    pub strukt_and_invers_spalten: Option<(usize, usize)>,
    pub gebr_univ_table_4meta_konkret: Option<Table>,
}

impl Concat {
    /// Create a new Concat instance
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
            brueche_emo: OrderedSet::new(),
            brueche_struk_groesse: OrderedSet::new(),
            relitable: None,
            rows_as_numbers: None,
            transzendentalien: OrderedDict::new(),
            motivation: OrderedDict::new(),
            gebr_rat_etwa_schon_mal_dabei_gewesen: OrderedSet::new(),
            strukt_and_invers_spalten: None,
            gebr_univ_table_4meta_konkret: None,
        }
    }

    /// Equivalent to Python's concatLovePolygon method
    pub fn concat_love_polygon(
        &mut self,
        relitable: &mut Table,
        rows_as_numbers: &mut HashSet<usize>,
    ) -> Result<()> {
        self.relitable = Some(relitable.clone());
        
        if rows_as_numbers.contains(&9) {
            if let Some(first_row_len) = relitable.get(0).map(|r| r.len()) {
                rows_as_numbers.insert(first_row_len);
                
                let tags = vec![ST::SternPolygon, ST::Galaxie, ST::GleichfoermigesPolygon];
                self.tables.generated_spalten_parameter_tags.insert(
                    rows_as_numbers.len() - 1,
                    tags.into_iter().collect(),
                );
                
                for i in 0..relitable.len() {
                    if let Some(row) = relitable.get_mut(i) {
                        if row.len() > 8 && !row[8].trim().is_empty() {
                            let new_value = format!(
                                "{}{}{}{}",
                                row[8],
                                i18n::polygon1(" der eigenen Strukturgröße ("),
                                row.get(4).unwrap_or(&String::new()),
                                i18n::polygon2(") auf dich bei gleichförmigen Polygonen")
                            );
                            row.push(new_value);
                        } else {
                            row.push(String::new());
                        }
                    }
                }
                
                let new_index = self.tables.generated_spalten_parameter.len() 
                    + self.tables.spalten_vanilla_amount;
                
                if self.tables.generated_spalten_parameter.contains_key(&new_index) {
                    return Err(ConcatError::DuplicateIndex);
                }
                
                if let Some(row) = self.tables.data_dict.get(&0) {
                    if row.len() > 9 {
                        self.tables.generated_spalten_parameter.insert(
                            new_index,
                            row[9].clone(),
                        );
                    }
                }
            }
        }
        
        self.rows_as_numbers = Some(rows_as_numbers.clone());
        Ok(())
    }

    /// Equivalent to Python's gleichheitFreiheitVergleich method
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

    /// Equivalent to Python's geistEmotionEnergieMaterieTopologie method
    pub fn geist_emotion_energie_materie_topologie(&self, zahl: i32) -> String {
        let pr_fa = primfaktoren(zahl);
        let auss: Vec<bool> = pr_fa.iter()
            .map(|&a| could_be_prime_number_primzahlkreuz_fuer_aussen(a))
            .collect();
        let innen: Vec<bool> = pr_fa.iter()
            .map(|&a| could_be_prime_number_primzahlkreuz_fuer_innen(a))
            .collect();
        
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

    /// Equivalent to Python's concatGleichheitFreiheitDominieren method
    pub fn concat_gleichheit_freiheit_dominieren(
        &mut self,
        relitable: &mut Table,
        rows_as_numbers: &mut HashSet<usize>,
    ) -> Result<()> {
        self.relitable = Some(relitable.clone());
        
        if rows_as_numbers.contains(&132) {
            if let Some(first_row_len) = relitable.get(0).map(|r| r.len()) {
                rows_as_numbers.insert(first_row_len);
                
                let tags = vec![ST::SternPolygon, ST::Universum];
                self.tables.generated_spalten_parameter_tags.insert(
                    rows_as_numbers.len() - 1,
                    tags.into_iter().collect(),
                );
                
                for i in 0..=self.tables.last_line_number {
                    let ausgabe_string = if i == 0 {
                        i18n::gleichheit_freiheit_vergleich(
                            "Gleichheit, Freiheit, Dominieren (Ordnungen [12]) Generiert"
                        )
                    } else {
                        self.gleichheit_freiheit_vergleich(i as i32)
                    };
                    
                    if let Some(row) = relitable.get_mut(i) {
                        row.push(ausgabe_string);
                    }
                }
                
                let new_index = self.tables.generated_spalten_parameter.len() 
                    + self.tables.spalten_vanilla_amount;
                
                if self.tables.generated_spalten_parameter.contains_key(&new_index) {
                    return Err(ConcatError::DuplicateIndex);
                }
                
                if let Some(row) = self.tables.data_dict.get(&0) {
                    if row.len() > 132 {
                        self.tables.generated_spalten_parameter.insert(
                            new_index,
                            row[132].clone(),
                        );
                    }
                }
            }
        }
        
        self.rows_as_numbers = Some(rows_as_numbers.clone());
        Ok(())
    }

    /// Equivalent to Python's concatGeistEmotionEnergieMaterieTopologie method
    pub fn concat_geist_emotion_energie_materie_topologie(
        &mut self,
        relitable: &mut Table,
        rows_as_numbers: &mut HashSet<usize>,
    ) -> Result<()> {
        self.relitable = Some(relitable.clone());
        
        if rows_as_numbers.contains(&242) {
            if let Some(first_row_len) = relitable.get(0).map(|r| r.len()) {
                rows_as_numbers.insert(first_row_len);
                
                let tags = vec![ST::SternPolygon, ST::Universum];
                self.tables.generated_spalten_parameter_tags.insert(
                    rows_as_numbers.len() - 1,
                    tags.into_iter().collect(),
                );
                
                for i in 0..=self.tables.last_line_number {
                    let ausgabe_string = if i == 0 {
                        i18n::ausgabe_string(
                            "Energie oder Denkart oder Gefühlsart oder Materie-Art oder Topologie-Art"
                        )
                    } else {
                        self.geist_emotion_energie_materie_topologie(i as i32)
                    };
                    
                    if let Some(row) = relitable.get_mut(i) {
                        row.push(ausgabe_string);
                    }
                }
                
                let new_index = self.tables.generated_spalten_parameter.len() 
                    + self.tables.spalten_vanilla_amount;
                
                if self.tables.generated_spalten_parameter.contains_key(&new_index) {
                    return Err(ConcatError::DuplicateIndex);
                }
                
                if let Some(row) = self.tables.data_dict.get(&0) {
                    if row.len() > 242 {
                        self.tables.generated_spalten_parameter.insert(
                            new_index,
                            row[242].clone(),
                        );
                    }
                }
            }
        }
        
        self.rows_as_numbers = Some(rows_as_numbers.clone());
        Ok(())
    }

    /// Equivalent to Python's concatPrimCreativityType method
    pub fn concat_prim_creativity_type(
        &mut self,
        relitable: &mut Table,
        rows_as_numbers: &mut HashSet<usize>,
    ) -> Result<()> {
        self.relitable = Some(relitable.clone());
        
        if rows_as_numbers.contains(&64) {
            if let Some(first_row_len) = relitable.get(0).map(|r| r.len()) {
                rows_as_numbers.insert(first_row_len);
                
                let tags = vec![ST::SternPolygon, ST::Galaxie];
                self.tables.generated_spalten_parameter_tags.insert(
                    rows_as_numbers.len() - 1,
                    tags.into_iter().collect(),
                );
                
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
                
                let new_index = self.tables.generated_spalten_parameter.len() 
                    + self.tables.spalten_vanilla_amount;
                
                if self.tables.generated_spalten_parameter.contains_key(&new_index) {
                    return Err(ConcatError::DuplicateIndex);
                }
                
                if let Some(row) = self.tables.data_dict.get(&0) {
                    if row.len() > 64 {
                        self.tables.generated_spalten_parameter.insert(
                            new_index,
                            row[64].clone(),
                        );
                    }
                }
            }
        }
        
        self.rows_as_numbers = Some(rows_as_numbers.clone());
        Ok(())
    }

    /// Equivalent to Python's concatMondExponzierenLogarithmusTyp method
    pub fn concat_mond_exponzieren_logarithmus_typ(
        &mut self,
        relitable: &mut Table,
        rows_as_numbers: &mut HashSet<usize>,
    ) -> Result<()> {
        self.relitable = Some(relitable.clone());
        
        if rows_as_numbers.contains(&64) {
            let hardcoded_couple = (44, 56);
            let row_headings = [
                i18n::mond_exp_log1("Mond-Typ eines Sternpolygons"),
                i18n::mond_exp_log1("Mond-Typ eines gleichförmigen Polygons"),
            ];
            
            for (rownum, rowheading) in hardcoded_couple.into_iter().zip(row_headings) {
                if let Some(first_row_len) = relitable.get(0).map(|r| r.len()) {
                    rows_as_numbers.insert(first_row_len);
                    
                    let tags = if rownum == 44 {
                        vec![ST::SternPolygon, ST::Universum, ST::Galaxie]
                    } else {
                        vec![ST::GleichfoermigesPolygon, ST::Universum, ST::Galaxie]
                    };
                    
                    self.tables.generated_spalten_parameter_tags.insert(
                        rows_as_numbers.len() - 1,
                        tags.into_iter().collect(),
                    );
                    
                    for i in 0..=self.tables.last_line_number {
                        let moon_types_of_1_num = moon_number(i as i32);
                        let into = if i == 0 {
                            vec![rowheading.to_string()]
                        } else {
                            let mut parts = Vec::new();
                            
                            parts.push(if self.tables.bbcode_output_yes {
                                "[list]".to_string()
                            } else if self.tables.html_output_yes {
                                "<ul>".to_string()
                            } else {
                                String::new()
                            });
                            
                            if moon_types_of_1_num.0.is_empty() {
                                parts.push(i18n::mond_exp_log2("kein Mond").to_string());
                            }
                            
                            for (k, (basis, exponent_minus2)) in 
                                moon_types_of_1_num.0.iter()
                                .zip(moon_types_of_1_num.1.iter())
                                .enumerate() 
                            {
                                if k > 0 {
                                    parts.push(" | ".to_string());
                                }
                                
                                if self.tables.html_output_yes {
                                    parts.push("<li>".to_string());
                                } else if self.tables.bbcode_output_yes {
                                    parts.push("[*]".to_string());
                                }
                                
                                // Handle SG replacement
                                let insert = if let Some(row_i) = relitable.get(i) {
                                    if row_i.len() > 4 {
                                        let sg = row_i[4].trim();
                                        self.relitable.as_ref()
                                            .and_then(|r| r.get(*basis as usize))
                                            .and_then(|row_basis| {
                                                if row_basis.len() > rownum as usize {
                                                    let mut text = row_basis[rownum].clone();
                                                    text = text.replace("<SG>", sg);
                                                    text = text.replace("&lt;SG&gt;", sg);
                                                    Some(text)
                                                } else {
                                                    None
                                                }
                                            })
                                            .unwrap_or_default()
                                    } else {
                                        String::new()
                                    }
                                } else {
                                    String::new()
                                };
                                
                                parts.push(insert);
                                parts.push(" - ".to_string());
                                
                                if let Some(row_exp) = relitable.get((exponent_minus2 + 2) as usize) {
                                    if row_exp.len() > 10 {
                                        parts.push(row_exp[10].clone());
                                    }
                                }
                                
                                parts.push(" | ".to_string());
                                
                                if self.tables.html_output_yes {
                                    parts.push("</li>".to_string());
                                }
                                
                                if let Some(row_i) = relitable.get(i) {
                                    if row_i.len() > 10 {
                                        parts.push(row_i[10].clone());
                                    }
                                    if row_i.len() > 11 {
                                        parts.push(" + ".to_string());
                                        parts.push(row_i[11].clone());
                                    }
                                }
                                
                                parts.push(", ".to_string());
                                
                                if let Some(row_exp) = relitable.get((exponent_minus2 + 2) as usize) {
                                    if row_exp.len() > 85 {
                                        parts.push(row_exp[85].clone());
                                    }
                                }
                            }
                            
                            if self.tables.html_output_yes {
                                parts.push("</ul>".to_string());
                            }
                            
                            parts
                        };
                        
                        if let Some(row) = relitable.get_mut(i) {
                            row.push(into.join(""));
                        }
                    }
                    
                    let new_index = self.tables.generated_spalten_parameter.len() 
                        + self.tables.spalten_vanilla_amount;
                    
                    if self.tables.generated_spalten_parameter.contains_key(&new_index) {
                        return Err(ConcatError::DuplicateIndex);
                    }
                    
                    if let Some(row) = self.tables.data_dict.get(&0) {
                        if row.len() > 64 {
                            self.tables.generated_spalten_parameter.insert(
                                new_index,
                                row[64].clone(),
                            );
                        }
                    }
                }
            }
        }
        
        self.rows_as_numbers = Some(rows_as_numbers.clone());
        Ok(())
    }

    /// Equivalent to Python's concatVervielfacheZeile method
    pub fn concat_vervielfache_zeile(
        &mut self,
        relitable: &mut Table,
        rows_as_numbers: &mut HashSet<usize>,
    ) -> Result<()> {
        self.relitable = Some(relitable.clone());
        
        let spalten_to_vervielfache: HashSet<usize> = 
            rows_as_numbers.intersection(&[90, 19].iter().cloned().collect())
            .cloned()
            .collect();
        
        for &s in &spalten_to_vervielfache {
            let mut store: OrderedDict<(usize, usize), String> = OrderedDict::new();
            
            for z in 2..=self.tables.last_line_number {
                if let Some(row) = relitable.get(z) {
                    if row.len() > s {
                        let content = row[s].trim();
                        if !content.is_empty() {
                            store.insert((z, s), content.to_string());
                        }
                    }
                }
            }
            
            let mut multis: OrderedDict<usize, Vec<usize>> = OrderedDict::new();
            
            for (&(orig_z, _), content) in &store {
                let mut vielfacher = 1;
                let mut ergebnis = vielfacher * orig_z;
                
                multis.entry(ergebnis)
                    .or_insert_with(Vec::new)
                    .push(orig_z);
                
                while ergebnis < relitable.len() {
                    vielfacher += 1;
                    ergebnis = vielfacher * orig_z;
                    
                    multis.entry(ergebnis)
                        .or_insert_with(Vec::new)
                        .push(orig_z);
                }
            }
            
            for z in 2..=self.tables.last_line_number {
                if let Some(row) = relitable.get_mut(z) {
                    if row.len() > s {
                        let mut new_value = Vec::new();
                        
                        if !row[s].trim().is_empty() {
                            if self.tables.html_output_yes {
                                new_value.push("<li>".to_string());
                                new_value.push(row[s].clone());
                                new_value.push("</li>".to_string());
                            } else if self.tables.bbcode_output_yes {
                                new_value.push("[*]".to_string());
                                new_value.push(row[s].clone());
                            } else {
                                new_value.push(row[s].clone());
                                new_value.push(" | ".to_string());
                            }
                        } else {
                            new_value.push(row[s].clone());
                        }
                        
                        if let Some(ur_zeilen) = multis.get(&z) {
                            for &ur_zeile in ur_zeilen {
                                if ur_zeile != z {
                                    let original_content = store.get(&(ur_zeile, s))
                                        .cloned()
                                        .unwrap_or_default();
                                    
                                    let current_content = new_value.join("");
                                    let should_add = !current_content.contains(&original_content);
                                    
                                    if should_add && !original_content.is_empty() {
                                        if self.tables.html_output_yes {
                                            new_value.push("<li>".to_string());
                                            new_value.push(original_content);
                                            new_value.push("</li>".to_string());
                                        } else if self.tables.bbcode_output_yes {
                                            new_value.push("[*]".to_string());
                                            new_value.push(original_content);
                                        } else {
                                            new_value.push(original_content);
                                            new_value.push(" | ".to_string());
                                        }
                                    }
                                }
                            }
                        }
                        
                        if self.tables.html_output_yes && !new_value.is_empty() {
                            row[s] = format!("<ul>{}</ul>", new_value.join(""));
                        } else if self.tables.bbcode_output_yes && !new_value.is_empty() {
                            row[s] = format!("[list]{}[/list]", new_value.join(""));
                        } else if !self.tables.html_output_yes && !self.tables.bbcode_output_yes {
                            // Remove trailing " | "
                            let joined = new_value.join("");
                            if joined.ends_with(" | ") {
                                row[s] = joined[..joined.len() - 3].to_string();
                            } else {
                                row[s] = joined;
                            }
                        }
                    }
                }
            }
        }
        
        self.rows_as_numbers = Some(rows_as_numbers.clone());
        Ok(())
    }

    /// Equivalent to Python's convertSetOfPaarenToDictOfNumToPaareDiv method
    pub fn convert_set_of_paaren_to_dict_of_num_to_paare_div(
        &self,
        paare_set: &OrderedSet<(i32, i32)>,
        gleichf: bool,
    ) -> DefaultOrderedDict<i32, OrderedSet<(i32, i32)>> {
        let mut result: DefaultOrderedDict<i32, OrderedSet<(i32, i32)>> = 
            DefaultOrderedDict::new();
        
        for &paar in paare_set {
            let div = if !gleichf {
                paar.0 as f64 / paar.1 as f64
            } else {
                paar.1 as f64 / paar.0 as f64
            };
            
            let div_int = (div * 1000.0).round() / 1000.0;
            let div_rounded = div_int.round() as i32;
            
            result.entry(div_rounded)
                .or_insert_with(OrderedSet::new)
                .insert(paar);
        }
        
        result
    }

    /// Equivalent to Python's convertSetOfPaarenToDictOfNumToPaareMul method
    pub fn convert_set_of_paaren_to_dict_of_num_to_paare_mul(
        &self,
        paare_set: &OrderedSet<(i32, i32)>,
        gleichf: bool,
    ) -> DefaultOrderedDict<i32, OrderedSet<(i32, i32)>> {
        let mut result: DefaultOrderedDict<i32, OrderedSet<(i32, i32)>> = 
            DefaultOrderedDict::new();
        
        for &paar in paare_set {
            let mul = if !gleichf {
                paar.0 * paar.1
            } else {
                ((1.0 / (paar.0 as f64 * paar.1 as f64)) * 1000.0).round() / 1000.0
            };
            
            let mul_rounded = mul.round() as i32;
            
            result.entry(mul_rounded)
                .or_insert_with(OrderedSet::new)
                .insert(paar);
        }
        
        result
    }

    /// Equivalent to Python's convertFractionsToDictOfNumToPaareOfMulOfIntAndFraction method
    pub fn convert_fractions_to_dict_of_num_to_paare_of_mul_of_int_and_fraction(
        &self,
        fracs: &OrderedSet<Ratio<i32>>,
        fracs2: &OrderedSet<Ratio<i32>>,
        gleichf: bool,
    ) -> DefaultOrderedDict<i32, OrderedSet<(Ratio<i32>, Ratio<i32>)>> {
        let mut result: DefaultOrderedDict<i32, OrderedSet<(Ratio<i32>, Ratio<i32>)>> = 
            DefaultOrderedDict::new();
        
        if !gleichf {
            for &frac in fracs {
                let highest_line = *self.tables.hoechste_zeile.get(&1024).unwrap_or(&1024);
                
                for zusatz_mul in 1..=highest_line {
                    let paar = (frac, Ratio::from_integer(zusatz_mul as i32) * 
                        Ratio::new(*frac.denom(), 1));
                    let mul = paar.0 * paar.1;
                    let mul_float = *mul.numer() as f64 / *mul.denom() as f64;
                    
                    if mul_float > highest_line as f64 {
                        break;
                    }
                    
                    let mul_rounded = mul_float.round() as i32;
                    result.entry(mul_rounded)
                        .or_insert_with(OrderedSet::new)
                        .insert(paar);
                }
                
                for zusatz_mul in (1..=highest_line).rev() {
                    let faktor = Ratio::new(*frac.denom(), 1) / 
                        Ratio::from_integer(zusatz_mul as i32);
                    
                    if fracs2.contains(&faktor) || *faktor.numer() == 1 {
                        let paar = (frac, faktor);
                        let mul = paar.0 * paar.1;
                        let mul_float = *mul.numer() as f64 / *mul.denom() as f64;
                        
                        if mul_float > highest_line as f64 {
                            break;
                        }
                        
                        let mul_rounded = mul_float.round() as i32;
                        result.entry(mul_rounded)
                            .or_insert_with(OrderedSet::new)
                            .insert(paar);
                    }
                }
            }
        } else {
            for &frac in fracs {
                let highest_line = *self.tables.hoechste_zeile.get(&1024).unwrap_or(&1024);
                
                for zusatz_div in 1..=highest_line {
                    let paar = (
                        frac,
                        Ratio::new(1, 1) / 
                        (Ratio::new(1, *frac.numer()) * Ratio::from_integer(zusatz_div as i32))
                    );
                    
                    let div = Ratio::new(1, 1) / (paar.1 * paar.0);
                    let div_float = *div.numer() as f64 / *div.denom() as f64;
                    
                    if div_float > highest_line as f64 {
                        break;
                    }
                    
                    let div_rounded = div_float.round() as i32;
                    result.entry(div_rounded)
                        .or_insert_with(OrderedSet::new)
                        .insert(paar);
                }
                
                for zusatz_div in 1..=highest_line {
                    let faktor = (Ratio::new(1, 1) / frac) / 
                        Ratio::from_integer(zusatz_div as i32);
                    
                    if fracs2.contains(&faktor) || *faktor.numer() == 1 {
                        let paar = (frac, faktor);
                        let mul = Ratio::new(1, 1) / (paar.1 * paar.0);
                        let mul_float = *mul.numer() as f64 / *mul.denom() as f64;
                        
                        if (1.0 / mul_float) > highest_line as f64 {
                            break;
                        }
                        
                        let mul_rounded = mul_float.round() as i32;
                        result.entry(mul_rounded)
                            .or_insert_with(OrderedSet::new)
                            .insert(paar);
                    }
                }
            }
        }
        
        result
    }

    /// Equivalent to Python's combineDicts method
    pub fn combine_dicts<K, V>(
        &self,
        a: &DefaultOrderedDict<K, V>,
        b: &DefaultOrderedDict<K, V>,
    ) -> DefaultOrderedDict<K, V>
    where
        K: Eq + std::hash::Hash + Clone,
        V: Clone + std::ops::BitOr<Output = V> + Default,
    {
        let mut e: DefaultOrderedDict<K, V> = a.clone();
        
        for (key, value) in b {
            e.entry(key.clone())
                .and_modify(|v| *v = v.clone() | value.clone())
                .or_insert_with(|| value.clone());
        }
        
        e
    }

    /// Read a CSV file and return its contents
    pub fn read_one_csv_and_return(&mut self, wahl: usize) -> Result<&Table> {
        let place = self.read_concat_csv_chose_csv_file(wahl);
        
        if !self.csvs_already_read.contains_key(&wahl) {
            let file = File::open(&place)
                .map_err(|e| ConcatError::FileNotFound(place.clone()))?;
            
            let mut reader = csv::ReaderBuilder::new()
                .delimiter(b';')
                .from_reader(BufReader::new(file));
            
            let mut table = Table::new();
            
            for result in reader.records() {
                let record = result.map_err(ConcatError::Csv)?;
                let row: Vec<String> = record.iter()
                    .map(|field| field.to_string())
                    .collect();
                table.push(row);
            }
            
            self.csvs_already_read.insert(wahl, table);
            
            // Update brueche sets
            let table_ref = &self.csvs_already_read[&wahl];
            let brueche = self.get_all_brueche(table_ref);
            
            match wahl {
                2 | 3 => {
                    self.brueche_gal = brueche;
                }
                4 | 5 => {
                    self.brueche_uni = brueche;
                }
                6 | 7 => {
                    self.brueche_emo = brueche;
                }
                8 | 9 => {
                    self.brueche_struk_groesse = brueche;
                }
                _ => {}
            }
        }
        
        Ok(&self.csvs_already_read[&wahl])
    }

    /// Get all fractions from a table
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

    /// Choose CSV file based on type
    fn read_concat_csv_chose_csv_file(&self, concat_table: usize) -> PathBuf {
        let filename = match concat_table {
            1 => "prim.csv",
            2 | 3 => "bruch13.csv",
            4 | 5 => "bruch15.csv",
            6 | 7 => "bruch7.csv",
            8 | 9 => "bruchStrukGroesse.csv",
            _ => "unknown.csv",
        };
        
        let mut path = std::env::current_dir().unwrap_or_default();
        path.push("csv");
        path.push(filename);
        
        path
    }

    /// Find all fractions and their combinations
    pub fn find_all_brueche_and_their_combinations(&mut self) -> Result<OrderedDict<String, OrderedDict<String, OrderedDict<String, OrderedSet<(Ratio<i32>, Ratio<i32>)>>>> {
        self.read_one_csv_and_return(2)?; // galN
        self.read_one_csv_and_return(4)?; // uniN
        
        let mut kombis2 = OrderedDict::new();
        kombis2.insert("mul".to_string(), OrderedSet::new());
        kombis2.insert("div".to_string(), OrderedSet::new());
        
        let mut kombis1 = OrderedDict::new();
        kombis1.insert("stern".to_string(), kombis2.clone());
        kombis1.insert("gleichf".to_string(), kombis2);
        
        let mut gebr_rat_all_combis = OrderedDict::new();
        gebr_rat_all_combis.insert("UniUni".to_string(), kombis1.clone());
        gebr_rat_all_combis.insert("UniGal".to_string(), kombis1.clone());
        gebr_rat_all_combis.insert("GalUni".to_string(), kombis1.clone());
        gebr_rat_all_combis.insert("GalGal".to_string(), kombis1);
        
        let combinations = [
            (&self.brueche_gal, &self.brueche_gal, "Gal", "Gal"),
            (&self.brueche_gal, &self.brueche_uni, "Gal", "Uni"),
            (&self.brueche_uni, &self.brueche_gal, "Uni", "Gal"),
            (&self.brueche_uni, &self.brueche_uni, "Uni", "Uni"),
        ];
        
        for (brueche1, brueche2, gal_or_uni1, gal_or_uni2) in &combinations {
            let key = format!("{}{}", gal_or_uni1, gal_or_uni2);
            
            for &brueche_un in brueche1.iter() {
                for &brueche_un2 in brueche2.iter() {
                    if brueche_un != brueche_un2 {
                        let couple = (brueche_un, brueche_un2);
                        
                        // Check multiplication for stern
                        let mul_result = brueche_un * brueche_un2;
                        if (mul_result.numer() as f64 / mul_result.denom() as f64)
                            .round() == (mul_result.numer() as f64 / mul_result.denom() as f64)
                        {
                            gebr_rat_all_combis
                                .get_mut(&key).unwrap()
                                .get_mut("stern").unwrap()
                                .get_mut("mul").unwrap()
                                .insert(couple);
                        }
                        
                        // Check division for stern
                        let div_result = brueche_un / brueche_un2;
                        if (div_result.numer() as f64 / div_result.denom() as f64)
                            .round() == (div_result.numer() as f64 / div_result.denom() as f64)
                        {
                            gebr_rat_all_combis
                                .get_mut(&key).unwrap()
                                .get_mut("stern").unwrap()
                                .get_mut("div").unwrap()
                                .insert(couple);
                        }
                        
                        // Check multiplication for gleichf
                        let mul_inv = Ratio::new(1, 1) / (brueche_un * brueche_un2);
                        if (mul_inv.numer() as f64 / mul_inv.denom() as f64)
                            .round() == (mul_inv.numer() as f64 / mul_inv.denom() as f64)
                        {
                            gebr_rat_all_combis
                                .get_mut(&key).unwrap()
                                .get_mut("gleichf").unwrap()
                                .get_mut("mul").unwrap()
                                .insert(couple);
                        }
                        
                        // Check division for gleichf
                        let div_inv = Ratio::new(1, 1) / (brueche_un / brueche_un2);
                        if (div_inv.numer() as f64 / div_inv.denom() as f64)
                            .round() == (div_inv.numer() as f64 / div_inv.denom() as f64)
                        {
                            gebr_rat_all_combis
                                .get_mut(&key).unwrap()
                                .get_mut("gleichf").unwrap()
                                .get_mut("div").unwrap()
                                .insert(couple);
                        }
                    }
                }
            }
        }
        
        Ok(gebr_rat_all_combis)
    }

    /// Read and concatenate CSV data
    pub fn read_concat_csv(
        &mut self,
        relitable: &mut Table,
        rows_as_numbers: &mut HashSet<usize>,
        concat_table_selection: &HashSet<usize>,
        concat_table: usize,
    ) -> Result<HashSet<usize>> {
        self.relitable = Some(relitable.clone());
        
        let mut concat_csv_spalten = HashSet::new();
        
        if !concat_table_selection.is_empty() && (1..=9).contains(&concat_table) {
            let table_to_add = self.read_one_csv_and_return(concat_table)?;
            let mut table_to_add = self.read_concat_csv_change_table_to_add_to_table(
                concat_table,
                table_to_add,
            )?;
            
            if concat_table == 1 {
                let mut table_to_add2 = vec![
                    vec![i18n::prim_viel_gen("Primzahlvielfache, nicht generiert")],
                ];
                
                for (t, zeile) in table_to_add.iter().enumerate().skip(1) {
                    let mut zeile_neu = Vec::new();
                    
                    for zelle in zeile {
                        if zelle.trim().len() > 3 {
                            let mut new_cell = String::new();
                            
                            if self.tables.html_output_yes {
                                new_cell.push_str("<li>");
                            } else if self.tables.bbcode_output_yes {
                                new_cell.push_str("[*]");
                            }
                            
                            new_cell.push_str(zelle);
                            
                            if self.tables.html_output_yes {
                                new_cell.push_str("</li>");
                            }
                            
                            zeile_neu.push(new_cell);
                        }
                    }
                    
                    let joined = if self.tables.html_output_yes {
                        format!("<ul>{}</ul>", zeile_neu.join(""))
                    } else if self.tables.bbcode_output_yes {
                        format!("[list]{}[/list]", zeile_neu.join(""))
                    } else {
                        zeile_neu.join(" | ")
                    };
                    
                    table_to_add2.push(vec![joined]);
                }
                
                table_to_add = table_to_add2;
            }
            
            // Fill both tables to same length
            let max_len = relitable.len().max(table_to_add.len());
            while relitable.len() < max_len {
                relitable.push(vec![]);
            }
            while table_to_add.len() < max_len {
                table_to_add.push(vec![]);
            }
            
            let last_len = relitable[0].len();
            let mut max_len_cols = 0;
            
            for i in 0..relitable.len() {
                let dazu_len = table_to_add[i].len();
                if dazu_len > max_len_cols {
                    max_len_cols = dazu_len;
                }
                
                let dazu = if i != 0 && (2..=9).contains(&concat_table) {
                    self.read_concat_csv_tabelle_dazu_colchange(
                        i,
                        &table_to_add[i],
                        concat_table,
                    )?
                } else {
                    table_to_add[i].clone()
                };
                
                let padding = max_len_cols - dazu.len();
                let mut extended = dazu;
                extended.extend(vec![String::new(); padding]);
                relitable[i].extend(extended);
                
                if i == 0 {
                    for (u, heading) in relitable[i][last_len..].iter().enumerate() {
                        if ((u + 2) as usize) < concat_table_selection.len() 
                            && (2..=9).contains(&concat_table)
                            || (concat_table == 1 && !heading.trim().is_empty())
                        {
                            let delta = if (2..=9).contains(&concat_table) { 1 } else { 0 };
                            let selected_spalten = last_len + u - (max_len_cols - dazu_len) + delta;
                            
                            rows_as_numbers.insert(selected_spalten);
                            concat_csv_spalten.insert(selected_spalten);
                            
                            let new_index = self.tables.generated_spalten_parameter.len() 
                                + self.tables.spalten_vanilla_amount;
                            
                            if self.tables.generated_spalten_parameter.contains_key(&new_index) {
                                return Err(ConcatError::DuplicateIndex);
                            }
                            
                            self.read_concat_csv_set_html_parameters(concat_table, heading, u)?;
                        }
                    }
                }
            }
        }
        
        self.rows_as_numbers = Some(rows_as_numbers.clone());
        Ok(concat_csv_spalten)
    }

    /// Helper method for read_concat_csv
    fn read_concat_csv_change_table_to_add_to_table(
        &self,
        concat_table: usize,
        table_to_add: &Table,
    ) -> Result<Table> {
        let mut result = table_to_add.clone();
        
        if (2..=9).contains(&concat_table) {
            // Add header row
            let header: Vec<String> = (0..table_to_add[0].len())
                .map(|n| {
                    let prefix = if concat_table == 2 || concat_table == 4 || concat_table == 6 || concat_table == 8 {
                        format!("n/{} ", n + 1)
                    } else {
                        format!("{}/n ", n + 1)
                    };
                    
                    let suffix = match concat_table {
                        2 | 3 => "Universum",
                        4 | 5 => "Galaxie",
                        6 | 7 => "Emotion",
                        8 | 9 => "Strukturgroesse",
                        _ => "Fehler",
                    };
                    
                    format!("{}{}", prefix, suffix)
                })
                .collect();
            
            result.insert(0, header);
        }
        
        Ok(result)
    }

    /// Helper method for read_concat_csv
    fn read_concat_csv_tabelle_dazu_colchange(
        &self,
        zeilen_nr: usize,
        tabelle_dazu_col: &[String],
        concat_table: usize,
    ) -> Result<Vec<String>> {
        let mut tabelle_dazu_col_neu = Vec::new();
        
        for (i, cell) in tabelle_dazu_col.iter().enumerate() {
            let gebr_rat_zahl = if concat_table == 2 || concat_table == 4 || concat_table == 6 || concat_table == 8 {
                Ratio::new(zeilen_nr as i32, (i + 1) as i32)
            } else {
                Ratio::new((i + 1) as i32, zeilen_nr as i32)
            };
            
            let cell_neu = self.spalte_meta_konkret_theorie_abstrakt_get_gebr_rat_univ_strukturalie(
                gebr_rat_zahl,
                self.strukt_and_invers_spalten.unwrap_or((5, 131)),
                self.gebr_univ_table_4meta_konkret.as_ref(),
                !matches!(concat_table, 2 | 3),
            )?;
            
            tabelle_dazu_col_neu.push(cell_neu.unwrap_or_default());
        }
        
        Ok(tabelle_dazu_col_neu)
    }

    /// Helper method for read_concat_csv
    fn read_concat_csv_set_html_parameters(
        &mut self,
        concat_table: usize,
        heading: &str,
        u: usize,
    ) -> Result<()> {
        if (2..=9).contains(&concat_table) {
            let range_to_data_dict = vec![
                (2, 6), (3, 6), (4, 5), (5, 5), (6, 9), (7, 9), (8, 10), (9, 10)
            ];
            
            let data_dict_key = range_to_data_dict
                .iter()
                .find(|&&(ct, _)| ct == concat_table)
                .map(|&(_, dk)| dk)
                .ok_or_else(|| ConcatError::UnknownCsvType(concat_table))?;
            
            if let Some(data_vec) = self.tables.data_dict.get(&data_dict_key) {
                if u + 2 < data_vec.len() {
                    let new_index = self.tables.generated_spalten_parameter.len() 
                        + self.tables.spalten_vanilla_amount;
                    
                    self.tables.generated_spalten_parameter.insert(
                        new_index,
                        data_vec[u + 2].clone(),
                    );
                }
            }
        }
        
        if concat_table == 1 {
            let into_html_para = vec![
                vec![
                    (
                        i18n::multipl("Multiplikationen").to_string(),
                        i18n::not_gen("Nicht_generiert").to_string(),
                    )
                ]
            ];
            
            let new_index = self.tables.generated_spalten_parameter.len() 
                + self.tables.spalten_vanilla_amount;
            
            // Store as JSON string for simplicity
            let json_string = serde_json::to_string(&into_html_para)
                .map_err(|e| ConcatError::Config(e.to_string()))?;
            
            self.tables.generated_spalten_parameter.insert(
                new_index,
                json_string,
            );
        }
        
        Ok(())
    }

    /// Get gebr rat univ strukturalie
    fn spalte_meta_konkret_theorie_abstrakt_get_gebr_rat_univ_strukturalie(
        &self,
        koord: Ratio<i32>,
        n_and_invers_spalten: (usize, usize),
        gebr_table_4meta_konkret_and_more: Option<&Table>,
        is_not_universe: bool,
    ) -> Result<Option<String>> {
        if *koord.denom() == 0 || *koord.numer() == 0 {
            return Ok(Some(String::new()));
        }
        
        if *koord.denom() > 100 || *koord.numer() > 100 {
            return Ok(None);
        }
        
        if *koord.numer() == 1 {
            let denom = *koord.denom() as usize;
            
            if let Some(relitable) = &self.relitable {
                if let Some(row) = relitable.get(denom) {
                    if row.len() > n_and_invers_spalten.1 {
                        let strukname = if is_not_universe {
                            row[n_and_invers_spalten.1].clone()
                        } else {
                            format!(
                                "{} (1/{}); {}",
                                row[n_and_invers_spalten.1],
                                denom,
                                row.get(201).unwrap_or(&String::new())
                            )
                        };
                        return Ok(Some(strukname));
                    }
                }
            }
            return Ok(Some(String::new()));
        }
        
        if *koord.denom() == 1 {
            let numer = *koord.numer() as usize;
            
            if let Some(relitable) = &self.relitable {
                if let Some(row) = relitable.get(numer) {
                    if row.len() > n_and_invers_spalten.0 {
                        let strukname = if is_not_universe {
                            row[n_and_invers_spalten.0].clone()
                        } else {
                            format!(
                                "{} ({}); {}",
                                row[n_and_invers_spalten.0],
                                numer,
                                row.get(198).unwrap_or(&String::new())
                            )
                        };
                        return Ok(Some(strukname));
                    }
                }
            }
            return Ok(Some(String::new()));
        }
        
        if let Some(table) = gebr_table_4meta_konkret_and_more {
            let numer_idx = *koord.numer() as usize - 1;
            let denom_idx = *koord.denom() as usize - 1;
            
            if numer_idx < table.len() && denom_idx < table[numer_idx].len() {
                return Ok(Some(table[numer_idx][denom_idx].clone()));
            }
        }
        
        Ok(Some(String::new()))
    }
}
