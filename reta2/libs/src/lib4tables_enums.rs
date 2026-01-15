//! lib4tables_enum module - equivalent to Python lib4tables_Enum module

use std::collections::HashSet;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ST {
    SternPolygon,
    Galaxie,
    GleichfoermigesPolygon,
    Universum,
    GebrRat,
}

impl ST {
    pub fn all() -> HashSet<Self> {
        vec![
            Self::SternPolygon,
            Self::Galaxie,
            Self::GleichfoermigesPolygon,
            Self::Universum,
            Self::GebrRat,
        ].into_iter().collect()
    }
}
