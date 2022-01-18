use rumbas_support::preamble::*;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_diff::{Apply, Diff, SerdeDiff};

#[derive(Input, Overwrite, RumbasCheck, Examples)]
#[input(name = "NumbasSettingsInput")]
#[derive(Serialize, Deserialize, SerdeDiff, Debug, Clone, JsonSchema, PartialEq)]
pub struct NumbasSettings {
    pub theme: String, //TODO: check if valid theme? Or is numbas error ok?
}
