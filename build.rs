use std::{
    fs::{self, DirEntry, File},
    path::PathBuf, env
};

use anyhow::Result;

mod build_ecs;
use build_ecs::*;
use codegen::Scope;

const FILENAME_PREFIX: &str ="ecs_";

fn main() -> Result<()> {
    let mut ecs_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    ecs_path.push("ecs");
    ecs_path.push("schemas");

    let entries = fs::read_dir(ecs_path)?;
    let mut mod_scope = Scope::new();
    for entry in entries.flatten() {
        let filename = entry.file_name().into_string().unwrap();
        if filename.ends_with("yml") {
            println!("reading {}", entry.path().to_str().unwrap());
            let ext_index = filename.find(".yml").unwrap();
            let module = handle_schema_file(&filename[0..ext_index], entry)?;
            mod_scope.raw(format!("mod {FILENAME_PREFIX}{module};"));
            mod_scope.import(&format!("{FILENAME_PREFIX}{module}"), "*").vis("pub");
        }
    }


    let mut rs_name = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    rs_name.push("src");
    rs_name.push("generated");
    rs_name.push("mod.rs");

    fs::write(rs_name, mod_scope.to_string())?;

    Ok(())
}

fn handle_schema_file(filetitle: &str, entry: DirEntry) -> Result<String> {
    let ecs: Vec<EcsMetaEntry> = serde_yaml::from_reader(File::open(entry.path())?)?;
    assert_eq!(ecs.len(), 1);
    store_scope(filetitle, ecs.into_iter().next().unwrap())
}

fn store_scope(filetitle: &str, entry: EcsMetaEntry) -> Result<String> {
    let mut rs_name = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    rs_name.push("src");
    rs_name.push("generated");

    if ! rs_name.exists() {
        fs::create_dir_all(&rs_name)?;
    }

    rs_name.push(format!("{FILENAME_PREFIX}{filetitle}.rs"));
    let type_name = entry.name.clone();

    let rs_scope = Scope::from(entry);

    println!("write to {}", rs_name.to_str().unwrap());
    fs::write(rs_name, rs_scope.to_string())?;

    Ok(type_name)
}