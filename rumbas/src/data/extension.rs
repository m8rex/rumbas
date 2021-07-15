use crate::data::optional_overwrite::*;
use crate::data::template::{Value, ValueType};
use crate::data::to_numbas::{NumbasResult, ToNumbas};
use serde::{Deserialize, Serialize};
//TODO: add other extensions
optional_overwrite! {
    /// Specify which extensions should be enabled
    pub struct Extensions {
        /// Whether the jsx_graph extension is enabled
        jsx_graph: bool,
        /// Whether the stats extension is enabled
        stats: bool,
        /// Whether the eukleides extension is enabled
        eukleides: bool
    }
}

impl ToNumbas for Extensions {
    // TODO: create macro
    type NumbasType = Vec<String>;
    fn to_numbas(&self, _locale: &String) -> NumbasResult<Vec<String>> {
        let check = self.check();
        if check.is_empty() {
            let mut extensions = Vec::new();
            if self.jsx_graph.unwrap() {
                extensions.push("jsx_graph".to_string());
            }
            if self.stats.unwrap() {
                extensions.push("stats".to_string());
            }
            if self.eukleides.unwrap() {
                extensions.push("eukleides".to_string());
            }
            Ok(extensions)
        } else {
            Err(check)
        }
    }
}

impl Extensions {
    pub fn new() -> Extensions {
        Extensions {
            jsx_graph: Value::Normal(false),
            stats: Value::Normal(false),
            eukleides: Value::Normal(false),
        }
    }

    pub fn combine(e: Extensions, f: Extensions) -> Extensions {
        Extensions {
            jsx_graph: Value::Normal(e.jsx_graph.unwrap() || f.jsx_graph.unwrap()),
            stats: Value::Normal(e.stats.unwrap() || f.stats.unwrap()),
            eukleides: Value::Normal(e.eukleides.unwrap() || f.eukleides.unwrap()),
        }
    }

    pub fn to_paths(&self) -> Vec<String> {
        let numbas_path = std::env::var("NUMBAS_FOLDER").expect("NUMBAS_FOLDER to be set"); //TODO: static str for NUMBAS_FOLDER
        let mut paths = Vec::new();
        if self.jsx_graph.unwrap() {
            paths.push("jsxgraph"); // TODO: add a toString?
        }
        if self.stats.unwrap() {
            paths.push("stats");
        }
        if self.eukleides.unwrap() {
            paths.push("eukleides");
        }
        paths
            .into_iter()
            .map(|s| format!("{}/extensions/{}", numbas_path, s))
            .collect()
    }
}
