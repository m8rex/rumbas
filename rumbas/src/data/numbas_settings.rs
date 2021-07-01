use crate::data::locale::SupportedLocale;
use crate::data::optional_overwrite::{Noneable, OptionalOverwrite};
use crate::data::template::{Value, ValueType};
use serde::{Deserialize, Serialize};

//TODO: is locale still being used?
optional_overwrite! {
    pub struct NumbasSettings {
        locale: SupportedLocale,
        theme: String, //TODO: check if valid theme? Or is numbas error ok?
        /// Whether the student is allowed to print the exam
        allow_printing: bool
    }
}
