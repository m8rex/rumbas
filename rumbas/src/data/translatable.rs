use crate::data::optional_overwrite::{Noneable, OptionalOverwrite};
use serde::Deserialize;
use serde::Serialize;
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(untagged)]
pub enum TranslatableString {
    Translation(HashMap<String, String>),
    NotTranslated(String),
}

impl OptionalOverwrite for TranslatableString {
    type Item = TranslatableString;
    fn empty_fields(&self) -> Vec<String> {
        let empty = Vec::new();
        empty
    }
    fn overwrite(&mut self, _other: &Self::Item) {
        //TODO: Maybe add languages of other that are missing in self?
        // These default values should be read before language is interpreted
    }
}
impl_optional_overwrite_option!(TranslatableString);

impl TranslatableString {
    pub fn to_string(&self, locale: &String) -> Option<String> {
        match self {
            TranslatableString::NotTranslated(s) => Some(s.clone()),
            TranslatableString::Translation(m) => m.get(locale).map(|s| s.clone()),
        }
    }
}
