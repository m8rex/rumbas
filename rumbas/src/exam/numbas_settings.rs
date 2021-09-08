use rumbas_support::preamble::*;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Input, Overwrite, RumbasCheck)]
#[input(name = "NumbasSettingsInput")]
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct NumbasSettings {
    pub theme: String, //TODO: check if valid theme? Or is numbas error ok?
}
