use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all="lowercase")]
pub enum EntryNormalize {
    Atomic,
    Array,
}

impl Default for EntryNormalize {
    fn default() -> Self {
        EntryNormalize::Atomic
    }
}