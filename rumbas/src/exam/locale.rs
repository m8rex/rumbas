use comparable::Comparable;
use rumbas_support::preamble::*;
use schemars::JsonSchema;
use serde::Deserialize;
use serde::Serialize;
use structdoc::StructDoc;

#[derive(Input, Overwrite, RumbasCheck, Examples, StructDoc)]
#[input(name = "LocaleInput")]
#[derive(Serialize, Deserialize, Comparable, Debug, Clone, JsonSchema, PartialEq, Eq)]
pub struct Locale {
    /// The internal name used for the locale. It is best to use en for English, nl for dutch etc
    pub name: String, //TODO: document names best used for shareability?
    /// The locale to use in the Numbas interface
    pub numbas_locale: SupportedLocale,
}

macro_rules! create_support_locale {
    ($(#[$meta:meta] $name: ident => $key: literal),*) => {
        #[derive(Input, Overwrite, RumbasCheck, Examples, StructDoc)]
        #[input(name = "SupportedLocaleInput")]
        /// Locales supported by Numbas
        /// See http://www.lingoes.net/en/translator/langcode.htm
        #[derive(Serialize, Deserialize, Comparable, JsonSchema, Debug, Clone, Copy, PartialEq, Eq)]
        pub enum SupportedLocale {
            $(
                #[serde(rename = $key)]
                #[$meta]
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
    /// Arabic (Saudi Arabia)
    ArSA => "ar-SA",
    /// German (Germany)
    DeDE => "de-DE",
    /// English (United Kingdom)
    EnGB => "en-GB",
    /// Spanish (Spain)
    EsES => "es-ES",
    /// French (France)
    FrFR => "fr-FR",
    /// Hebrew (Israel)
    HeIL => "he-IL",
    /// Indonesian (Indonesia)
    InId => "in-ID",
    /// Italian (Italy)
    ItIT => "it-IT",
    /// Japanese (Japan)
    JaJP => "ja-JP",
    /// Korean (Korea)
    KoKR => "ko-KR",
    /// Norwegian (Norway)
    NbNO => "nb-NO",
    /// Dutch (Netherlands)
    NlNL => "nl-NL",
    /// Polish (Poland)
    PlPL => "pl-PL",
    /// Portuguese (Brazil)
    PtBR => "pt-BR",
    /// Albanian (Albania)
    SqAL => "sq-AL",
    /// Swedish (Sweden)
    SvSE => "sv-SE",
    /// Turkish (Turkey)
    TrTR => "tr-TR",
    /// Vietnamese (Viet Nam)
    ViVN => "vi-VN",
    /// Chinese (S)
    ZhCN => "zg-CN"
}
