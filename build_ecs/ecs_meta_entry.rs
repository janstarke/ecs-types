use std::collections::HashSet;

use build_helper::warning;
use codegen::{Scope, Impl, Function};
use convert_case::{Casing, Case};
use serde::{Deserialize, Serialize};

use super::{EcsEntryField, EntryType, format_docs};

const TYPENAME_PREFIX: &str ="";

#[derive(Serialize, Deserialize, Clone)]
pub struct EcsMetaEntry {
    pub name: String,
    pub title: String,

    #[serde(default="last_group")]
    pub group: u32,
    pub short: Option<String>,
    pub description: String,
    pub footnote: Option<String>,

    #[serde(default="Default::default")]
    root: bool,

    #[serde(rename = "type")]
    pub entry_type: Option<EntryType>,

    pub fields: Vec<EcsEntryField>,

    #[serde(skip)]
    children: Vec<Self>
}

fn last_group() -> u32 { u32::MAX }

impl EcsMetaEntry {
    pub fn is_root(&self) -> bool {
        self.root
    }

    pub fn add_child(&mut self, child: &Self) {
        self.children.push(child.clone());
    }
}

impl From<&EcsMetaEntry> for Scope {
    fn from(entry: &EcsMetaEntry) -> Self {
        let mut scope = Scope::new();
        scope.import("serde", "Serialize");

        let typename = format!("{TYPENAME_PREFIX}{}", entry.name);

        let my_struct = scope.new_struct(&typename.to_case(Case::UpperCamel))
            .derive("Serialize")
            .derive("Clone")
            .doc(&format_docs(&entry.description))
            .vis("pub");
        
        let mut my_impl = Impl::new(&typename.to_case(Case::UpperCamel));
        
        let mut fields = HashSet::new();
        
        for field in entry.fields.iter() {
            let fieldname = field.name.clone();
            if fields.contains(&fieldname) {
                warning!("field {fieldname} in {typename} defined more than once, ignoring the second occurance");
            } else {
                my_impl.push_fn(field.getter());
                my_impl.push_fn(field.setter());
                my_struct.push_field(field.into());
                fields.insert(fieldname);
            }
        }

        for child in entry.children.iter() {
            let child_type = format!("crate::{}", child.name.to_case(Case::UpperCamel));
             let field = my_struct.new_field(format!("child_{}", child.name), format!("Option<{}>", child_type));
             field.annotation(format!("#[serde(rename=\"{}\", skip_serializing_if = \"Option::is_none\")]", child.name));

             let method = my_impl.new_fn(&format!("with_{}", child.name));
             method.vis("pub")
                .arg_mut_self()
                .arg(&format!("{}_arg", child.name), &child_type)
                .ret("&mut Self")
                .line(format!("self.child_{} = Some({}_arg);", child.name, child.name))
                .line("self")
                .doc(format!("add child of type [{}]", child_type));
        }


        let required_fields: Vec<_> = entry.fields.iter().filter(|f|f.required).collect();
        if required_fields.is_empty() {
            my_struct.derive("Default");
        } else {
            let mut method = Function::new("new");
            method.vis("pub")
                .ret("Self");
            for field in required_fields {
                method.arg(&field.arg_name(), field.fieldtype(false));
            }
            method.line("Self {");
            for field in entry.fields.iter() {
                if field.required {
                    method.line(format!("{}: {},", field.fieldname(), field.arg_name()));
                } else {
                    method.line(format!("{}: Default::default(),", field.fieldname()));
                }
            }
            for child in entry.children.iter() {
                method.line(format!("child_{}: Default::default(),", child.name));
            }
            method.line("}");
            my_impl.push_fn(method);
        }

        scope.push_impl(my_impl);
        scope
    }
}
