use codegen::{Field, Type};
use serde::{Deserialize, Serialize};

use super::{EntryLevel, EntryNormalize, EntryType};
const FIELD_PREFIX: &str = "ecs_";

#[derive(Serialize, Deserialize)]
pub struct EcsEntryField {
    pub name: String,
    pub level: EntryLevel,
    #[serde(rename = "type")]
    pub entry_type: EntryType,
    pub description: String,

    pub example: Option<String>,

    #[serde(default="Vec::default")]
    pub normalize: Vec<EntryNormalize>,

    #[serde(default="bool::default")]
    required: bool,
}

impl From<EcsEntryField> for Field {
    fn from(entry: EcsEntryField) -> Self {
        let fieldname = format!(
            "{FIELD_PREFIX}{}",
            entry.name.replace('.', "_").replace('@', "")
        );

        let mut field = match entry.normalize.first() {
            Some(EntryNormalize::Array) => {
                let mut field_type = Type::new("Vec");
                field_type.generic(entry.entry_type);
                Field::new(&fieldname, field_type)
            }
            _ => if entry.required {
                Field::new(&fieldname, entry.entry_type)
            } else {
                let mut field_type = Type::new("Option");
                field_type.generic(entry.entry_type);
                Field::new(&fieldname, field_type)
            }
        };

        field
            .doc(entry.description)
            .annotation(format!("#[serde(rename=\"{}\")]", entry.name))
            .vis("pub");
        field
    }
}
