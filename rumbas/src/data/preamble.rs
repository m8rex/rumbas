use crate::data::optional_overwrite::{Noneable, OptionalOverwrite};
use crate::data::to_numbas::{NumbasResult, ToNumbas};
use serde::{Deserialize, Serialize};

optional_overwrite! {
    Preamble,
    js: String,
    css: String
}

impl ToNumbas for Preamble {
    type NumbasType = numbas::exam::Preamble;
    fn to_numbas(&self, _locale: &String) -> NumbasResult<numbas::exam::Preamble> {
        let empty_fields = self.empty_fields();
        if empty_fields.is_empty() {
            Ok(numbas::exam::Preamble::new(
                self.js.clone().unwrap(),
                self.css.clone().unwrap(),
            ))
        } else {
            Err(empty_fields)
        }
    }
}
