use rumbas_support::preamble::*;
use schemars::JsonSchema;
use serde::Deserialize;
use serde::Serialize;
use comparable::Comparable;

#[derive(Input, Overwrite, RumbasCheck, Examples)]
#[input(name = "LocaleInput")]
#[derive(Serialize, Deserialize, Comparable, Debug, Clone, JsonSchema, PartialEq)]
pub struct Locale {
    pub name: String, //TODO: document names best used for shareability?
    /// The locale to use in the Numbas interface
    pub numbas_locale: SupportedLocale,
}

macro_rules! create_support_locale {
    ($($name: ident => $key: literal),*) => {
        #[derive(Input, Overwrite, RumbasCheck, Examples)]
        #[input(name = "SupportedLocaleInput")]
        /// Locales supported by Numbas
        #[derive(Serialize, Deserialize, Comparable, JsonSchema, Debug, Clone, Copy, PartialEq)]
        pub enum SupportedLocale {
            $(
                #[serde(rename = $key)]
                $name
            ),*
        }

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
