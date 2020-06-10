use crate::data::locale::SupportedLocale;
use crate::data::optional_overwrite::{Noneable, OptionalOverwrite};
use serde::{Deserialize, Serialize};

optional_overwrite! {
    NumbasSettings,
    locale: SupportedLocale,
    theme: String //TODO: check if valid theme? Or is numbas error ok?
}
