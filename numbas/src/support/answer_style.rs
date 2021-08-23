use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

// See https://github.com/numbas/Numbas/blob/26e5c25be75f5bb1a7d6b625bc8ed0c6a59224e5/runtime/scripts/util.js#L1259
#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq)]
pub enum AnswerStyle {
    /// English style - commas separate thousands, dot for decimal point
    #[serde(rename = "en")]
    English,
    /// Plain English style - no thousands separator, dot for decimal point
    #[serde(rename = "plain")]
    EnglishPlain,
    /// English SI style - spaces separate thousands, dot for decimal point
    #[serde(rename = "si-en")]
    EnglishSI,
    /// Continental European style - dots separate thousands, comma for decimal poin
    #[serde(rename = "eu")]
    European,
    /// Plain French style - no thousands separator, comma for decimal point
    #[serde(rename = "plain-eu")]
    EuropeanPlain,
    /// French SI style - spaces separate thousands, comma for decimal point
    #[serde(rename = "si-fr")]
    FrenchSI,
    /// Indian style - commas separate groups, dot for decimal point. The rightmost group is three digits, other groups are two digits.
    #[serde(rename = "in")]
    Indian,
    /// Significand-exponent ("scientific") style
    #[serde(rename = "scientific")]
    Scientific,
    /// Swiss style - apostrophes separate thousands, dot for decimal point
    #[serde(rename = "ch")]
    Swiss,
}
