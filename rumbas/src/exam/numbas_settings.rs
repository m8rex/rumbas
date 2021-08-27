use crate::support::optional_overwrite::*;
use crate::support::rumbas_types::*;
use crate::support::template::Value;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

optional_overwrite! {
    pub struct NumbasSettings {
        theme: RumbasString //TODO: check if valid theme? Or is numbas error ok?
    }
}
