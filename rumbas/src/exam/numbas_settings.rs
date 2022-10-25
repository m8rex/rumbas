use comparable::Comparable;
use rumbas_support::preamble::*;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use structdoc::StructDoc;

#[derive(Input, Overwrite, RumbasCheck, Examples, StructDoc)]
#[input(name = "NumbasSettingsInput")]
#[derive(Serialize, Deserialize, Comparable, Debug, Clone, JsonSchema, PartialEq, Eq)]
pub struct NumbasSettings {
    /// The numbas theme to use
    pub theme: String, //TODO: check if valid theme? Or is numbas error ok?
}
