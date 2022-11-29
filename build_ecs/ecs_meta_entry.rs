use std::collections::HashSet;

use build_helper::warning;
use codegen::Scope;
use convert_case::{Casing, Case};
use serde::{Deserialize, Serialize};

use super::{EcsEntryField, EntryType};

const TYPENAME_PREFIX: &str ="";

#[derive(Serialize, Deserialize)]
pub struct EcsMetaEntry {
    pub name: String,
    pub title: String,

    #[serde(default="last_group")]
    pub group: u32,
    pub short: Option<String>,
    pub description: String,
    pub footnote: Option<String>,

    #[serde(rename = "type")]
    pub entry_type: Option<EntryType>,

    pub fields: Vec<EcsEntryField>,
}

fn last_group() -> u32 { u32::MAX }

impl From<EcsMetaEntry> for Scope {
    fn from(entry: EcsMetaEntry) -> Self {
        let mut scope = Scope::new();
        scope.import("serde", "Serialize");

        let typename = format!("{TYPENAME_PREFIX}{}", entry.name);

        let my_struct = scope.new_struct(&typename.to_case(Case::UpperCamel))
            .derive("Serialize")
            .doc(&entry.description)
            .vis("pub");
        
        let mut fields = HashSet::new();
        
        for field in entry.fields.into_iter() {
            let fieldname = field.name.clone();
            if fields.contains(&fieldname) {
                warning!("field {fieldname} in {typename} defined more than once, ignoring the second occurance");
            } else {
                my_struct.push_field(field.into());
                fields.insert(fieldname);
            }
        }
        scope
    }
}
/*
impl From<EcsMetaEntry> for Schema {
    fn from(entry: EcsMetaEntry) -> Self {
        let mut metadata = BTreeMap::new();
        let mut properties = BTreeMap::new();

        metadata.insert("rustType".into(), json!(entry.name));
        metadata.insert("description".into(), json!(entry.description));

        for field in entry.fields.into_iter() {
            let name = if field.name == "type" {
                "ecs_type".into()
            } else {
                field.name.clone()
            };

            properties.insert(name, field.into());
        }

        Schema::Properties {
            definitions: BTreeMap::default(),
            metadata,
            nullable: false,
            properties,
            optional_properties: BTreeMap::default(),
            properties_is_present: true,
            additional_properties: false,
        }
    }
}
 */