use std::{path::PathBuf, fs::{File, self, DirEntry}, collections::{HashMap, BTreeMap}};

use anyhow::Result;
use jtd::Schema;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
enum EntryLevel {
    Core,
    Extended,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
enum EntryType {
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
    Wildcard
}

#[derive(Serialize, Deserialize)]
struct EcsMetaEntry {
    name: String,
    title: String,
    group: u32,
    short: String,
    description: String,
    footnote: String,

    #[serde(rename="type")]
    entry_type: EntryType,

    fields: Vec<EcsEntryField>
}

#[derive(Serialize, Deserialize)]
struct EcsEntryField {
    name: String,
    level: EntryLevel,
    entry_type: EntryType,
    description: String,
    example: String
}

impl From<EcsEntryField> for Schema {
    fn from(entry: EcsEntryField) -> Self {
        let properties = BTreeMap::new();
        
        Schema::Properties {
            definitions: BTreeMap::default(),
            metadata: BTreeMap::default(),
            nullable: false,
            properties,
            optional_properties: BTreeMap::default(),
            properties_is_present: true,
            additional_properties: false,
        }
    }
}

impl From<EcsMetaEntry> for Schema {
    fn from(entry: EcsMetaEntry) -> Self {
        let mut properties = BTreeMap::new();
        for field in entry.fields.into_iter() {
            let name = field.name.clone();
            properties.insert(name, field.into());
        }
        
        Schema::Properties {
            definitions: BTreeMap::default(),
            metadata: BTreeMap::default(),
            nullable: false,
            properties,
            optional_properties: BTreeMap::default(),
            properties_is_present: true,
            additional_properties: false,
        }
    }
}

fn main() -> Result<()> {
    let mut ecs_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    ecs_path.push("build");
    ecs_path.push("ecs");
    ecs_path.push("schemas");

    let entries = fs::read_dir(ecs_path)?;
    for entry in entries.flatten() {
        let filename = entry.file_name().into_string().unwrap();
        if filename.ends_with("yml") {
            println!("reading {filename}");
            handle_schema_file(entry)?;
        }
    }
    Ok(())
}

fn handle_schema_file(entry: DirEntry) -> Result<()> {
    let ecs: HashMap<String, EcsMetaEntry> = serde_yaml::from_reader(File::open(entry.path())?)?;
    Ok(())
}