use crate::support::optional_overwrite::*;
use crate::support::template::{Value, ValueType};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

optional_overwrite! {
    pub struct NumbasSettings {
        theme: String //TODO: check if valid theme? Or is numbas error ok?
    }
}
