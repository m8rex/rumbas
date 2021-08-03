use crate::data::optional_overwrite::*;
use crate::data::template::{Value, ValueType};
use schemars::JsonSchema;
use serde::Deserialize;
use serde::Serialize;

optional_overwrite! {
    pub struct Locale {
        name: String, //TODO: document names best used for shareability?
        /// The locale to use in the Numbas interface
        numbas_locale: SupportedLocale
    }
}

/// Locales supported by Numbas
#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, Copy, PartialEq)]
pub enum SupportedLocale {
    #[serde(rename = "ar-SA")]
    ArSA,
    #[serde(rename = "de-DE")]
    DeDE,
    #[serde(rename = "en-GB")]
    EnGB,
    #[serde(rename = "es-ES")]
    EsES,
    #[serde(rename = "fr-FR")]
    FrFR,
    #[serde(rename = "he-IL")]
    HeIL,
    #[serde(rename = "it-IT")]
    ItIT,
    #[serde(rename = "ja-JP")]
    JaJP,
    #[serde(rename = "ko-KR")]
    KoKR,
    #[serde(rename = "nb-NO")]
    NbNO,
    #[serde(rename = "nl-NL")]
    NlNL,
    #[serde(rename = "pl-PL")]
    PlPL,
    #[serde(rename = "pt-BR")]
    PtBR,
    #[serde(rename = "sq-AL")]
    SqAL,
    #[serde(rename = "sv-SE")]
    SvSE,
    #[serde(rename = "tr-TR")]
    TrTR,
    #[serde(rename = "vi-VN")]
    ViVN,
    #[serde(rename = "zh-CN")]
    ZhCN,
}
impl_optional_overwrite!(SupportedLocale);

//TODO? macro to reduce duplication?
impl SupportedLocale {
    pub fn to_str(self) -> &'static str {
        match self {
            SupportedLocale::ArSA => "ar-SA",
            SupportedLocale::DeDE => "de-DE",
            SupportedLocale::EnGB => "en-GB",
            SupportedLocale::EsES => "es-ES",
            SupportedLocale::FrFR => "fr-FR",
            SupportedLocale::HeIL => "he-IL",
            SupportedLocale::ItIT => "it-IT",
            SupportedLocale::JaJP => "ja-JP",
            SupportedLocale::KoKR => "ko-KR",
            SupportedLocale::NbNO => "nb-NO",
            SupportedLocale::NlNL => "nl-NL",
            SupportedLocale::PlPL => "pl-PL",
            SupportedLocale::PtBR => "pt-BR",
            SupportedLocale::SqAL => "sq-AL",
            SupportedLocale::SvSE => "sv-SE",
            SupportedLocale::TrTR => "tr-TR",
            SupportedLocale::ViVN => "vi-VN",
            SupportedLocale::ZhCN => "zg-CN",
        }
    }
}
