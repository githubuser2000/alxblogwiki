// Cargo.toml dependencies:
// [dependencies]
// strum = { version = "0.25", features = ["derive"] }
// strum_macros = "0.25"
// serde = { version = "1.0", features = ["derive"] }
// enumset = "1.1"
// itertools = "0.12"
// std::collections::{HashMap, HashSet, BTreeSet};

use std::collections::{HashMap, HashSet, BTreeSet};
use std::hash::{Hash, Hasher};
use std::ops::Deref;
use std::sync::Arc;

use enumset::{EnumSet, EnumSetType};
use itertools::Itertools;
use serde::{Deserialize, Serialize};
use strum::{Display, EnumIter, EnumString};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, EnumSetType, Display, EnumIter, EnumString)]
pub enum ST {
    /// Stern-Polygon (Star Polygon)
    SternPolygon = 0,
    /// Gleichförmiges Polygon (Uniform Polygon)
    GleichfoermigesPolygon = 1,
    /// Kein Polygon (No Polygon)
    KeinPolygon = 2,
    /// Galaxie (Galaxy)
    Galaxie = 3,
    /// Universum (Universe)
    Universum = 4,
    /// Kein Para oder Meta P (No Para or Meta P)
    KeinParaOdMetaP = 5,
    /// Gebrochene Rationalität (Fractional Rationality)
    GebrRat = 6,
}

impl ST {
    pub fn as_u8(&self) -> u8 {
        match self {
            ST::SternPolygon => 0,
            ST::GleichfoermigesPolygon => 1,
            ST::KeinPolygon => 2,
            ST::Galaxie => 3,
            ST::Universum => 4,
            ST::KeinParaOdMetaP => 5,
            ST::GebrRat => 6,
        }
    }
    
    pub fn from_u8(value: u8) -> Option<Self> {
        match value {
            0 => Some(ST::SternPolygon),
            1 => Some(ST::GleichfoermigesPolygon),
            2 => Some(ST::KeinPolygon),
            3 => Some(ST::Galaxie),
            4 => Some(ST::Universum),
            5 => Some(ST::KeinParaOdMetaP),
            6 => Some(ST::GebrRat),
            _ => None,
        }
    }
}

// Custom wrapper for EnumSet to use as HashMap key
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct STSet(EnumSet<ST>);

impl STSet {
    pub fn new(set: EnumSet<ST>) -> Self {
        STSet(set)
    }
    
    pub fn inner(&self) -> &EnumSet<ST> {
        &self.0
    }
    
    pub fn into_inner(self) -> EnumSet<ST> {
        self.0
    }
    
    pub fn from_vec(tags: Vec<ST>) -> Self {
        let mut set = EnumSet::new();
        for tag in tags {
            set.insert(tag);
        }
        STSet(set)
    }
}

impl PartialEq for STSet {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl Eq for STSet {}

impl Hash for STSet {
    fn hash<H: Hasher>(&self, state: &mut H) {
        // Convert to sorted vector for consistent hashing
        let mut tags: Vec<ST> = self.0.iter().collect();
        tags.sort_by_key(|st| st.as_u8());
        tags.hash(state);
    }
}

impl Deref for STSet {
    type Target = EnumSet<ST>;
    
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

// Table tags mapping
pub struct TableTags {
    tags: HashMap<STSet, BTreeSet<i32>>,
}

impl TableTags {
    pub fn new() -> Self {
        let mut tags = HashMap::new();
        
        // Populate the tags from Python code
        // Note: This is a subset; full implementation would include all mappings
        
        // frozenset({ST.keinParaOdMetaP, ST.sternPolygon, ST.galaxie})
        tags.insert(
            STSet::from_vec(vec![ST::KeinParaOdMetaP, ST::SternPolygon, ST::Galaxie]),
            BTreeSet::from([370, 411, 241, 394, 395, 424, 492, 493]),
        );
        
        // frozenset({ST.universum, ST.keinParaOdMetaP, ST.sternPolygon, ST.gleichfoermigesPolygon})
        tags.insert(
            STSet::from_vec(vec![
                ST::Universum,
                ST::KeinParaOdMetaP,
                ST::SternPolygon,
                ST::GleichfoermigesPolygon,
            ]),
            BTreeSet::from([14]),
        );
        
        // frozenset({ST.sternPolygon, ST.galaxie, ST.universum, ST.keinParaOdMetaP})
        tags.insert(
            STSet::from_vec(vec![
                ST::SternPolygon,
                ST::Galaxie,
                ST::Universum,
                ST::KeinParaOdMetaP,
            ]),
            BTreeSet::from([
                318, 4, 20, 15, 26, 140, 142, 143, 144, 141, 137, 120, 114, 115,
                116, 117, 17, 48, 123, 124, 125, 126, 127, 128, 129, 130, 100,
                101, 102, 103, 222, 36, 21, 4, 422, 495,
            ]),
        );
        
        // Add more mappings as needed...
        
        TableTags { tags }
    }
    
    pub fn get_tags(&self) -> &HashMap<STSet, BTreeSet<i32>> {
        &self.tags
    }
    
    pub fn get_by_number(&self, number: i32) -> Option<STSet> {
        for (tag_set, numbers) in &self.tags {
            if numbers.contains(&number) {
                return Some(tag_set.clone());
            }
        }
        None
    }
}

// Kombi table tags
pub struct TableTagsKombi {
    tags: HashMap<STSet, BTreeSet<i32>>,
}

impl TableTagsKombi {
    pub fn new() -> Self {
        let mut tags = HashMap::new();
        
        // tableTags_kombiTable
        tags.insert(
            STSet::from_vec(vec![ST::Galaxie, ST::SternPolygon, ST::GleichfoermigesPolygon]),
            BTreeSet::from([1, 2, 3, 7, 8, 9, 10, 12, 13, 16, 17]),
        );
        
        tags.insert(
            STSet::from_vec(vec![
                ST::Universum,
                ST::Galaxie,
                ST::SternPolygon,
                ST::GleichfoermigesPolygon,
            ]),
            BTreeSet::from([5, 6, 11, 15]),
        );
        
        TableTagsKombi { tags }
    }
    
    pub fn get_tags(&self) -> &HashMap<STSet, BTreeSet<i32>> {
        &self.tags
    }
}

pub struct TableTagsKombi2 {
    tags: HashMap<STSet, BTreeSet<i32>>,
}

impl TableTagsKombi2 {
    pub fn new() -> Self {
        let mut tags = HashMap::new();
        
        // tableTags_kombiTable2
        tags.insert(
            STSet::from_vec(vec![
                ST::Universum,
                ST::Galaxie,
                ST::SternPolygon,
                ST::GleichfoermigesPolygon,
            ]),
            BTreeSet::from([5]),
        );
        
        tags.insert(
            STSet::from_vec(vec![
                ST::Universum,
                ST::GleichfoermigesPolygon,
                ST::SternPolygon,
            ]),
            BTreeSet::from([1, 2, 3, 4, 6, 7, 8, 9, 10, 11, 12, 13, 15, 16, 17, 18]),
        );
        
        TableTagsKombi2 { tags }
    }
    
    pub fn get_tags(&self) -> &HashMap<STSet, BTreeSet<i32>> {
        &self.tags
    }
}

// Inverted dictionaries
pub struct TableTags2 {
    tags: HashMap<i32, STSet>,
}

impl TableTags2 {
    pub fn new() -> Self {
        let table_tags = TableTags::new();
        let mut inverted = HashMap::new();
        
        for (tag_set, numbers) in table_tags.get_tags() {
            for &number in numbers {
                inverted.insert(number, tag_set.clone());
            }
        }
        
        TableTags2 { tags: inverted }
    }
    
    pub fn get(&self, number: i32) -> Option<&STSet> {
        self.tags.get(&number)
    }
    
    pub fn contains(&self, number: i32) -> bool {
        self.tags.contains_key(&number)
    }
    
    pub fn get_all(&self) -> &HashMap<i32, STSet> {
        &self.tags
    }
}

pub struct TableTags2Kombi {
    tags: HashMap<i32, STSet>,
}

impl TableTags2Kombi {
    pub fn new() -> Self {
        let table_tags_kombi = TableTagsKombi::new();
        let mut inverted = HashMap::new();
        
        for (tag_set, numbers) in table_tags_kombi.get_tags() {
            for &number in numbers {
                inverted.insert(number, tag_set.clone());
            }
        }
        
        TableTags2Kombi { tags: inverted }
    }
    
    pub fn get(&self, number: i32) -> Option<&STSet> {
        self.tags.get(&number)
    }
}

pub struct TableTags2Kombi2 {
    tags: HashMap<i32, STSet>,
}

impl TableTags2Kombi2 {
    pub fn new() -> Self {
        let table_tags_kombi2 = TableTagsKombi2::new();
        let mut inverted = HashMap::new();
        
        for (tag_set, numbers) in table_tags_kombi2.get_tags() {
            for &number in numbers {
                inverted.insert(number, tag_set.clone());
            }
        }
        
        TableTags2Kombi2 { tags: inverted }
    }
    
    pub fn get(&self, number: i32) -> Option<&STSet> {
        self.tags.get(&number)
    }
}

// Global instances (similar to Python module-level variables)
lazy_static::lazy_static! {
    pub static ref TABLE_TAGS: TableTags = TableTags::new();
    pub static ref TABLE_TAGS_KOMBI: TableTagsKombi = TableTagsKombi::new();
    pub static ref TABLE_TAGS_KOMBI2: TableTagsKombi2 = TableTagsKombi2::new();
    pub static ref TABLE_TAGS2: TableTags2 = TableTags2::new();
    pub static ref TABLE_TAGS2_KOMBI: TableTags2Kombi = TableTags2Kombi::new();
    pub static ref TABLE_TAGS2_KOMBI2: TableTags2Kombi2 = TableTags2Kombi2::new();
}

// Column type naming system
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SpaltenTypKey {
    Ordinary = 0,
    Generated1 = 1,
    Concat1 = 2,
    Kombi1 = 3,
    BoolAndTupleSet1 = 4,
    OrdinaryNot = 5,
    Generated1dNot = 6,
    Concat1Not = 7,
    Kombi1Not = 8,
    BoolAndTupleSet1Not = 9,
}

impl SpaltenTypKey {
    pub fn as_tuple(self) -> (u8, u8) {
        match self {
            SpaltenTypKey::Ordinary => (0, 0),
            SpaltenTypKey::Generated1 => (0, 1),
            SpaltenTypKey::Concat1 => (0, 2),
            SpaltenTypKey::Kombi1 => (0, 3),
            SpaltenTypKey::BoolAndTupleSet1 => (0, 4),
            SpaltenTypKey::OrdinaryNot => (1, 0),
            SpaltenTypKey::Generated1dNot => (1, 1),
            SpaltenTypKey::Concat1Not => (1, 2),
            SpaltenTypKey::Kombi1Not => (1, 3),
            SpaltenTypKey::BoolAndTupleSet1Not => (1, 4),
        }
    }
    
    pub fn from_tuple(tuple: (u8, u8)) -> Option<Self> {
        match tuple {
            (0, 0) => Some(SpaltenTypKey::Ordinary),
            (0, 1) => Some(SpaltenTypKey::Generated1),
            (0, 2) => Some(SpaltenTypKey::Concat1),
            (0, 3) => Some(SpaltenTypKey::Kombi1),
            (0, 4) => Some(SpaltenTypKey::BoolAndTupleSet1),
            (1, 0) => Some(SpaltenTypKey::OrdinaryNot),
            (1, 1) => Some(SpaltenTypKey::Generated1dNot),
            (1, 2) => Some(SpaltenTypKey::Concat1Not),
            (1, 3) => Some(SpaltenTypKey::Kombi1Not),
            (1, 4) => Some(SpaltenTypKey::BoolAndTupleSet1Not),
            _ => None,
        }
    }
}

pub struct SpaltenArten {
    pub spalten_arten_key_spaltennummern_value: HashMap<(u8, u8), BTreeSet<i32>>,
}

impl SpaltenArten {
    pub fn new() -> Self {
        let mut map = HashMap::new();
        
        // Initialize empty sets for all keys
        for i in 0..=1 {
            for j in 0..=4 {
                map.insert((i, j), BTreeSet::new());
            }
        }
        
        SpaltenArten {
            spalten_arten_key_spaltennummern_value: map,
        }
    }
    
    pub fn get_ordinary(&self) -> &BTreeSet<i32> {
        &self.spalten_arten_key_spaltennummern_value[&(0, 0)]
    }
    
    pub fn get_generated1(&self) -> &BTreeSet<i32> {
        &self.spalten_arten_key_spaltennummern_value[&(0, 1)]
    }
    
    pub fn get_concat1(&self) -> &BTreeSet<i32> {
        &self.spalten_arten_key_spaltennummern_value[&(0, 2)]
    }
    
    pub fn get_kombi1(&self) -> &BTreeSet<i32> {
        &self.spalten_arten_key_spaltennummern_value[&(0, 3)]
    }
    
    pub fn get_bool_and_tuple_set1(&self) -> &BTreeSet<i32> {
        &self.spalten_arten_key_spaltennummern_value[&(0, 4)]
    }
    
    pub fn get_ordinary_not(&self) -> &BTreeSet<i32> {
        &self.spalten_arten_key_spaltennummern_value[&(1, 0)]
    }
    
    pub fn get_generated1d_not(&self) -> &BTreeSet<i32> {
        &self.spalten_arten_key_spaltennummern_value[&(1, 1)]
    }
    
    pub fn get_concat1_not(&self) -> &BTreeSet<i32> {
        &self.spalten_arten_key_spaltennummern_value[&(1, 2)]
    }
    
    pub fn get_kombi1_not(&self) -> &BTreeSet<i32> {
        &self.spalten_arten_key_spaltennummern_value[&(1, 3)]
    }
    
    pub fn get_bool_and_tuple_set1_not(&self) -> &BTreeSet<i32> {
        &self.spalten_arten_key_spaltennummern_value[&(1, 4)]
    }
    
    pub fn insert(&mut self, key: SpaltenTypKey, value: i32) {
        let tuple_key = key.as_tuple();
        if let Some(set) = self.spalten_arten_key_spaltennummern_value.get_mut(&tuple_key) {
            set.insert(value);
        }
    }
    
    pub fn remove(&mut self, key: SpaltenTypKey, value: i32) {
        let tuple_key = key.as_tuple();
        if let Some(set) = self.spalten_arten_key_spaltennummern_value.get_mut(&tuple_key) {
            set.remove(&value);
        }
    }
    
    pub fn get_rows_as_numbers(&self) -> &BTreeSet<i32> {
        self.get_ordinary()
    }
    
    pub fn get_gener_rows(&self) -> &BTreeSet<i32> {
        self.get_generated1()
    }
    
    pub fn get_puniverseprims(&self) -> &BTreeSet<i32> {
        self.get_concat1()
    }
    
    pub fn get_rows_of_combi(&self) -> &BTreeSet<i32> {
        self.get_kombi1()
    }
    
    pub fn get_only_generated(&self) -> &BTreeSet<i32> {
        self.get_bool_and_tuple_set1()
    }
}

// Utility functions
pub fn dict_vice_versa(dic: &HashMap<STSet, BTreeSet<i32>>) -> HashMap<i32, STSet> {
    let mut new_dict = HashMap::new();
    
    for (key, value) in dic {
        for &number in value {
            new_dict.insert(number, key.clone());
        }
    }
    
    new_dict
}

// Example usage and tests
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_st_enum() {
        assert_eq!(ST::SternPolygon.as_u8(), 0);
        assert_eq!(ST::from_u8(0), Some(ST::SternPolygon));
        assert_eq!(ST::from_u8(99), None);
    }
    
    #[test]
    fn test_st_set() {
        let set1 = STSet::from_vec(vec![ST::SternPolygon, ST::Galaxie]);
        let set2 = STSet::from_vec(vec![ST::Galaxie, ST::SternPolygon]);
        
        assert_eq!(set1, set2); // Should be equal regardless of insertion order
        
        let mut map = HashMap::new();
        map.insert(set1.clone(), "test");
        
        assert_eq!(map.get(&set2), Some(&"test"));
    }
    
    #[test]
    fn test_table_tags() {
        let tags = TABLE_TAGS2.get(14);
        assert!(tags.is_some());
        
        let tag_set = tags.unwrap();
        assert!(tag_set.contains(ST::Universum));
        assert!(tag_set.contains(ST::KeinParaOdMetaP));
        assert!(tag_set.contains(ST::SternPolygon));
        assert!(tag_set.contains(ST::GleichfoermigesPolygon));
    }
    
    #[test]
    fn test_spalten_arten() {
        let mut spalten = SpaltenArten::new();
        
        spalten.insert(SpaltenTypKey::Ordinary, 1);
        spalten.insert(SpaltenTypKey::Ordinary, 2);
        spalten.insert(SpaltenTypKey::Generated1, 3);
        
        assert_eq!(spalten.get_ordinary().len(), 2);
        assert!(spalten.get_ordinary().contains(&1));
        assert!(spalten.get_ordinary().contains(&2));
        assert!(spalten.get_generated1().contains(&3));
        
        assert_eq!(spalten.get_rows_as_numbers().len(), 2);
        assert_eq!(spalten.get_gener_rows().len(), 1);
    }
}

// Main module exports
pub use self::{
    ST, STSet, TableTags, TableTagsKombi, TableTagsKombi2,
    TableTags2, TableTags2Kombi, TableTags2Kombi2,
    SpaltenTypKey, SpaltenArten,
};

// Convenience functions for accessing global instances
pub fn table_tags() -> &'static TableTags {
    &TABLE_TAGS
}

pub fn table_tags2() -> &'static TableTags2 {
    &TABLE_TAGS2
}

pub fn table_tags2_kombi() -> &'static TableTags2Kombi {
    &TABLE_TAGS2_KOMBI
}

pub fn table_tags2_kombi2() -> &'static TableTags2Kombi2 {
    &TABLE_TAGS2_KOMBI2
}
