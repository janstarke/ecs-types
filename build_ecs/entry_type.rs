use std::fmt::Display;

use codegen::Type;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "snake_case")]
pub enum EntryType {
    Boolean,
    ConstantKeyword,
    Date,
    Flattened,
    Float,
    GeoPoint,
    Group,
    Ip,
    Keyword,
    Long,
    MatchOnlyText,
    Nested,
    Object,
    ScaledFloat,
    Wildcard,

    Source,
}

impl Display for EntryType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                EntryType::Boolean => "bool",
                EntryType::ConstantKeyword => "String",
                EntryType::Date => "crate::types::Timestamp",
                EntryType::Flattened => "String",
                EntryType::Float => "f64",
                EntryType::GeoPoint => "crate::types::GeoPoint",
                EntryType::Group => "String",
                EntryType::Ip => "String",
                EntryType::Keyword => "String",
                EntryType::Long => "u64",
                EntryType::MatchOnlyText => "String",
                EntryType::Nested => "String",
                EntryType::Object => "serde_json::Value",
                EntryType::ScaledFloat => "String",
                EntryType::Wildcard => "String",
                EntryType::Source => "String",
            }
        )
    }
}

impl From<EntryType> for Type {
    fn from(me: EntryType) -> Self {
        Type::new(me.to_string())
    }
}
