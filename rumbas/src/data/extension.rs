use crate::data::optional_overwrite::{Noneable, OptionalOverwrite};
use crate::data::template::{Value, ValueType};
use crate::data::to_numbas::{NumbasResult, ToNumbas};
use serde::{Deserialize, Serialize};
//TODO: add other extensions
optional_overwrite! {
    Extensions,
    jsx_graph: bool
}

impl ToNumbas for Extensions {
    type NumbasType = Vec<String>;
    fn to_numbas(&self, _locale: &String) -> NumbasResult<Vec<String>> {
        let empty_fields = self.empty_fields();
        if empty_fields.is_empty() {
            let mut extensions = Vec::new();
            if self.jsx_graph.unwrap() {
                extensions.push("jsx_graph".to_string());
            }
            Ok(extensions)
        } else {
            Err(empty_fields)
        }
    }
}

impl Extensions {
    pub fn new() -> Extensions {
        Extensions {
            jsx_graph: Value::Normal(false),
        }
    }

    pub fn combine(e: Extensions, f: Extensions) -> Extensions {
        Extensions {
            jsx_graph: Value::Normal(e.jsx_graph.unwrap() || f.jsx_graph.unwrap()),
        }
    }

    pub fn to_paths(&self) -> Vec<String> {
        let numbas_path = std::env::var("NUMBAS_FOLDER").expect("NUMBAS_FOLDER to be set"); //TODO: static str for NUMBAS_FOLDER
        let mut paths = Vec::new();
        if self.jsx_graph.unwrap() {
            paths.push("extensions/jsxgraph");
        }
        paths
            .into_iter()
            .map(|s| format!("{}/{}", numbas_path, s))
            .collect()
    }
}
