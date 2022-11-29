use serde::{Serialize, Deserialize};


#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EntryLevel {
    Core,
    Extended,
}