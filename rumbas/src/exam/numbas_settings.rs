use rumbas_support::preamble::*;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Input, Overwrite, RumbasCheck, Examples)]
#[input(name = "NumbasSettingsInput")]
#[input(test)]
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema, PartialEq)]
pub struct NumbasSettings {
    pub theme: String, //TODO: check if valid theme? Or is numbas error ok?
}
