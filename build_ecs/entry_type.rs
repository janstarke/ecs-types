use codegen::Type;
use serde::{Serialize, Deserialize};


#[derive(Serialize, Deserialize)]
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

    Source
}

impl From<EntryType> for Type {
    fn from(me: EntryType) -> Self {
        Type::new(match me {
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
        })
    }
}