use rumbas_support::preamble::*;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use comparable::Comparable;

#[derive(Input, Overwrite, RumbasCheck, Examples)]
#[input(name = "NumbasSettingsInput")]
#[derive(Serialize, Deserialize, Comparable, Debug, Clone, JsonSchema, PartialEq)]
pub struct NumbasSettings {
    pub theme: String, //TODO: check if valid theme? Or is numbas error ok?
}
