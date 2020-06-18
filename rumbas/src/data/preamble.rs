use crate::data::file_reference::FileString;
use crate::data::optional_overwrite::{Noneable, OptionalOverwrite};
use crate::data::to_numbas::{NumbasResult, ToNumbas};
use serde::{Deserialize, Serialize};

// TODO: maybe translatable? So different text's can be set for different languages?
optional_overwrite! {
    Preamble,
    js: FileString,
    css: FileString
}

impl ToNumbas for Preamble {
    type NumbasType = numbas::exam::Preamble;
    fn to_numbas(&self, _locale: &String) -> NumbasResult<numbas::exam::Preamble> {
        let empty_fields = self.empty_fields();
        if empty_fields.is_empty() {
            Ok(numbas::exam::Preamble::new(
                self.js.clone().unwrap().get_content(),
                self.css.clone().unwrap().get_content(),
            ))
        } else {
            Err(empty_fields)
        }
    }
}
