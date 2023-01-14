use crate::support::file_reference::FileString;
use crate::support::to_numbas::ToNumbas;
use crate::support::to_rumbas::ToRumbas;
use comparable::Comparable;
use rumbas_support::preamble::*;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use structdoc::StructDoc;

// TODO: maybe translatable? So different text's can be set for different languages?
#[derive(Input, Overwrite, RumbasCheck, Examples, StructDoc)]
#[input(name = "PreambleInput")]
#[derive(Serialize, Deserialize, Comparable, Debug, Clone, JsonSchema, PartialEq, Eq)]
pub struct Preamble {
    /// The JavaScript to add to the outputfiles
    pub js: FileString,
    /// The CSS to add to the outputfiles
    pub css: FileString,
}

impl ToNumbas<numbas::question::preamble::Preamble> for Preamble {
    type ToNumbasHelper = ();
    fn to_numbas(&self, locale: &str, _data: &Self::ToNumbasHelper) -> numbas::question::preamble::Preamble {
        numbas::question::preamble::Preamble {
            js: self.js.to_numbas(locale, &()),
            css: self.css.to_numbas(locale, &()),
        }
    }
}

impl ToRumbas<Preamble> for numbas::question::preamble::Preamble {
    fn to_rumbas(&self) -> Preamble {
        Preamble {
            js: self.js.to_rumbas(),
            css: self.css.to_rumbas(),
        }
    }
}
