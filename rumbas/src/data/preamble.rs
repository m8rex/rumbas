use crate::data::file_reference::FileString;
use crate::data::optional_overwrite::*;
use crate::data::template::{Value, ValueType};
use crate::data::to_numbas::ToNumbas;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

// TODO: maybe translatable? So different text's can be set for different languages?
optional_overwrite! {
    pub struct Preamble {
        /// The JavaScript to add to the outputfiles
        js: FileString,
        /// The CSS to add to the outputfiles
        css: FileString
    }
}

impl ToNumbas<numbas::exam::Preamble> for Preamble {
    fn to_numbas(&self, locale: &str) -> numbas::exam::Preamble {
        numbas::exam::Preamble {
            js: self.js.to_numbas(locale),
            css: self.css.to_numbas(locale),
        }
    }
}
