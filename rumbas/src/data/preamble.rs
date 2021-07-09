use crate::data::file_reference::FileString;
use crate::data::optional_overwrite::*;
use crate::data::template::{Value, ValueType};
use crate::data::to_numbas::{NumbasResult, ToNumbas};
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

impl ToNumbas for Preamble {
    type NumbasType = numbas::exam::Preamble;
    fn to_numbas(&self, locale: &String) -> NumbasResult<numbas::exam::Preamble> {
        let check = self.check();
        if check.is_empty() {
            Ok(numbas::exam::Preamble::new(
                self.js.clone().unwrap().get_content(&locale),
                self.css.clone().unwrap().get_content(&locale),
            ))
        } else {
            Err(check)
        }
    }
}
