use crate::support::optional_overwrite::*;
use crate::support::rumbas_types::*;
use crate::support::template::{Value, ValueType};
use schemars::JsonSchema;
use serde::Deserialize;
use serde::Serialize;

optional_overwrite! {
    pub struct Locale {
        name: RumbasString, //TODO: document names best used for shareability?
        /// The locale to use in the Numbas interface
        numbas_locale: SupportedLocale
    }
}

pub type LocalesInput = Vec<Value<Locale>>;
pub type Locales = Vec<Locale>;

macro_rules! create_support_locale {
    ($($name: ident => $key: literal),*) => {
        /// Locales supported by Numbas
        #[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, Copy, PartialEq)]
        pub enum SupportedLocale {
            $(
                #[serde(rename = $key)]
                $name
            ),*
        }
        impl_optional_overwrite!(SupportedLocale);

        //TODO? macro to reduce duplication?
        impl SupportedLocale {
            pub fn to_str(self) -> &'static str {
                match self {
                    $(SupportedLocale::$name => $key),*
                }
            }
        }

    }
}

create_support_locale! {
    ArSA => "ar-SA",
    DeDE => "de-DE",
    EnGB => "en-GB",
    EsES => "es-ES",
    FrFR => "fr-FR",
    HeIL => "he-IL",
    ItIT => "it-IT",
    JaJP => "ja-JP",
    KoKR => "ko-KR",
    NbNO => "nb-NO",
    NlNL => "nl-NL",
    PlPL => "pl-PL",
    PtBR => "pt-BR",
    SqAL => "sq-AL",
    SvSE => "sv-SE",
    TrTR => "tr-TR",
    ViVN => "vi-VN",
    ZhCN => "zg-CN"
}
