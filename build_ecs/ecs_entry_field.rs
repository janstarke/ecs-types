use codegen::{Field, Function};
use serde::{Deserialize, Serialize};

use super::{EntryLevel, EntryNormalize, EntryType, format_docs};
const FIELD_PREFIX: &str = "ecs_";

#[derive(Serialize, Deserialize)]
pub struct EcsEntryField {
    pub name: String,
    pub level: EntryLevel,
    #[serde(rename = "type")]
    pub entry_type: EntryType,
    pub description: String,

    pub example: Option<String>,

    #[serde(default = "Vec::default")]
    pub normalize: Vec<EntryNormalize>,

    #[serde(default = "bool::default")]
    pub required: bool,
}

impl EcsEntryField {
    pub fn fieldname(&self) -> String {
        format!("{FIELD_PREFIX}{}", self.atomic_fieldname())
    }

    pub fn fieldtype(&self, as_reference: bool) -> String {
        let prefix = if as_reference { "&" } else { "" };

        match self.normalize.first() {
            Some(EntryNormalize::Array) => format!("{}Vec<{}>", prefix, self.entry_type),
            _ => {
                if self.required {
                    format!("{}{}", prefix, self.entry_type)
                } else {
                    format!("Option<{}{}>", prefix, self.entry_type)
                }
            }
        }
    }

    pub fn atomic_fieldname(&self) -> String {
        self.name.replace('.', "_").replace('@', "")
    }

    pub fn atomic_arg_name(&self) -> String {
        match self.normalize.first() {
            Some(EntryNormalize::Array) => {
                if self.atomic_fieldname().ends_with('s') {
                    let fieldname = self.atomic_fieldname();
                    let mut ch = fieldname.chars();
                    ch.next_back();
                    ch.as_str().into()
                } else {
                    self.atomic_fieldname()
                }
            }
            _ => self.atomic_fieldname(),
        }
    }

    pub fn arg_name(&self) -> String {
        format!("{}_arg", self.atomic_arg_name())
    }

    pub fn getter(&self) -> Function {
        let mut function = Function::new(format!("get_{}", self.atomic_fieldname()));

        let function_line = match self.normalize.first() {
            Some(EntryNormalize::Array) => format!("&self.{}", self.fieldname()),
            _ => {
                if self.required {
                    format!("&self.{}", self.fieldname())
                } else {
                    format!("self.{}.as_ref()", self.fieldname())
                }
            }
        };

        function
            .vis("pub")
            .arg_ref_self()
            .ret(self.fieldtype(true))
            .line(function_line);
        function
    }

    pub fn setter(&self) -> Function {
        let arg_name = self.atomic_arg_name();
        let function_verb = match self.normalize.first() {
            Some(EntryNormalize::Array) => "add",
            _ => "set",
        };

        let mut function = Function::new(format!("{function_verb}_{arg_name}"));
        let arg_name = self.arg_name();

        let function_line = match self.normalize.first() {
            Some(EntryNormalize::Array) => format!("self.{}.push({arg_name});", self.fieldname()),
            _ => if self.required {
                format!("self.{} = {arg_name};", self.fieldname())
            } else {
                format!("self.{} = Some({arg_name});", self.fieldname())
            }
        };

        function
            .vis("pub")
            .arg_mut_self()
            .arg(&arg_name, self.entry_type.to_string())
            .line(function_line);
        function
    }
}

impl From<&EcsEntryField> for Field {
    fn from(entry: &EcsEntryField) -> Self {
        let fieldname = entry.fieldname();

        let mut field = Field::new(&fieldname, entry.fieldtype(false));

        field
            .doc(format_docs(&entry.description))
            .annotation(format!("#[serde(rename=\"{}\")]", entry.name))
            //.vis("pub")
            ;
        field
    }
}
