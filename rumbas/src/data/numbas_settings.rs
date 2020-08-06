use crate::data::locale::SupportedLocale;
use crate::data::optional_overwrite::{Noneable, OptionalOverwrite};
use crate::data::template::Value;
use serde::{Deserialize, Serialize};

//TODO: is locale still being used?
optional_overwrite! {
    NumbasSettings,
    locale: SupportedLocale,
    theme: String //TODO: check if valid theme? Or is numbas error ok?
}
