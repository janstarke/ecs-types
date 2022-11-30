use std::{
    fs::{self, DirEntry, File},
    path::PathBuf, env, collections::HashMap
};

use anyhow::Result;

mod build_ecs;
use build_ecs::*;
use codegen::Scope;

const FILENAME_PREFIX: &str ="ecs_";

fn main() -> Result<()> {
    let out_dir = env::var("OUT_DIR").unwrap();
    let mut ecs_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    ecs_path.push("ecs");
    ecs_path.push("schemas");

    let entries = fs::read_dir(ecs_path)?;
    let mut mod_scope = Scope::new();

    let mut root_id = None;
    let mut ecs_types = HashMap::new();

    for entry in entries.flatten() {
        let filename = entry.file_name().into_string().unwrap();
        if filename.ends_with("yml") {
            println!("reading {}", entry.path().to_str().unwrap());
            let ext_index = filename.find(".yml").unwrap();
            let ecs_type = read_from_schema_file( entry)?;
            let module_name = &ecs_type.name;
            if module_name == "tracing" {
                continue;
            }
            
            mod_scope.raw(format!("mod {FILENAME_PREFIX}{module_name};"));
            mod_scope.import(&format!("{FILENAME_PREFIX}{module_name}"), "*").vis("pub");

            assert!(! ecs_types.contains_key(&filename[0..ext_index]));

            if ecs_type.is_root() {
                assert!(root_id.is_none());
                root_id.replace((&filename[0..ext_index]).to_owned());
            }
            ecs_types.insert((&filename[0..ext_index]).to_owned(), ecs_type);
        }
    }

    let root_id = root_id.unwrap();
    let mut root_type = ecs_types.remove(&root_id).unwrap();
    for ecs_type in ecs_types.values() {
        root_type.add_child(ecs_type);

    }
    ecs_types.insert(root_id, root_type);

    for (filetitle, ecs_type) in ecs_types.into_iter() {
        store_scope(&filetitle, ecs_type)?;
    }

    let mut rs_name = PathBuf::from(out_dir);
    rs_name.push("mod.rs");

    fs::write(rs_name, mod_scope.to_string())?;

    Ok(())
}


fn read_from_schema_file(entry: DirEntry) -> Result<EcsMetaEntry> {
    let ecs: Vec<EcsMetaEntry> = serde_yaml::from_reader(File::open(entry.path())?)?;
    assert_eq!(ecs.len(), 1);
    Ok(ecs.into_iter().next().unwrap())
}

fn store_scope(filetitle: &str, entry: EcsMetaEntry) -> Result<EcsMetaEntry> {
    let out_dir = env::var("OUT_DIR").unwrap();
    let mut rs_name = PathBuf::from(out_dir);

    rs_name.push(format!("{FILENAME_PREFIX}{filetitle}.rs"));

    let rs_scope = Scope::from(&entry);

    println!("write to {}", rs_name.to_str().unwrap());
    fs::write(rs_name, rs_scope.to_string())?;

    Ok(entry)
}