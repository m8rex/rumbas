use crate::data::optional_overwrite::{Noneable, OptionalOverwrite};
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
