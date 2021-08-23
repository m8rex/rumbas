use schemars::JsonSchema;
use serde::Deserialize;
use serde::Serialize;
use serde_with::skip_serializing_none;
//TODO: remove Exam from front of all types?
//TODO: check what is optional etc
//TODO: advicethreshold?

impl std::convert::TryFrom<&str> for AnswerSimplificationType {
    type Error = &'static str;
    fn try_from(whole_item: &str) -> Result<Self, Self::Error> {
        let (item, value) = if whole_item.starts_with('!') {
            let mut chars = whole_item.chars();
            chars.next();
            (chars.as_str(), false)
        } else {
            (whole_item, true)
        };
        match item.to_lowercase().as_ref() {
            "all" => Ok(AnswerSimplificationType::All(value)),
            "basic" => Ok(AnswerSimplificationType::Basic(value)),
            "unitfactor" => Ok(AnswerSimplificationType::UnitFactor(value)),
            "unitpower" => Ok(AnswerSimplificationType::UnitPower(value)),
            "unitdenominator" => Ok(AnswerSimplificationType::UnitDenominator(value)),
            "zerofactor" => Ok(AnswerSimplificationType::ZeroFactor(value)),
            "zeroterm" => Ok(AnswerSimplificationType::ZeroTerm(value)),
            "zeropower" => Ok(AnswerSimplificationType::ZeroPower(value)),
            "collectnumbers" => Ok(AnswerSimplificationType::CollectNumbers(value)),
            "zerobase" => Ok(AnswerSimplificationType::ZeroBase(value)),
            "constantsfirst" => Ok(AnswerSimplificationType::ConstantsFirst(value)),
            "sqrtproduct" => Ok(AnswerSimplificationType::SqrtProduct(value)),
            "sqrtfivision" => Ok(AnswerSimplificationType::SqrtDivision(value)),
            "sqrtdquare" => Ok(AnswerSimplificationType::SqrtSquare(value)),
            "othernumbers" => Ok(AnswerSimplificationType::OtherNumbers(value)),
            "timesdot" => Ok(AnswerSimplificationType::TimesDot(value)),
            "expandbrackets" => Ok(AnswerSimplificationType::ExpandBrackets(value)),
            "noleadingminus" => Ok(AnswerSimplificationType::NoLeadingMinus(value)),
            "trig" => Ok(AnswerSimplificationType::Trigonometric(value)),
            "collectlikefractions" => Ok(AnswerSimplificationType::CollectLikeFractions(value)),
            "canonicalorder" => Ok(AnswerSimplificationType::CanonicalOrder(value)),
            "cancelfactors" => Ok(AnswerSimplificationType::CancelFactors(value)),
            "cancelterms" => Ok(AnswerSimplificationType::CancelTerms(value)),
            "simplifyfractions" => Ok(AnswerSimplificationType::Fractions(value)),
            _ => {
                /*       Err(serde::de::Error::custom(format!(
                    "unknown answer simplification type {}",
                    item
                )))*/
                Ok(AnswerSimplificationType::Unknown((item.to_string(), value)))
            }
        }
    }
}

impl std::fmt::Display for AnswerSimplificationType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                AnswerSimplificationType::All(true) => "all".to_string(),
                AnswerSimplificationType::All(false) => "!all".to_string(),
                AnswerSimplificationType::Basic(true) => "basic".to_string(),
                AnswerSimplificationType::Basic(false) => "!basic".to_string(),
                AnswerSimplificationType::UnitFactor(true) => "unitFactor".to_string(),
                AnswerSimplificationType::UnitFactor(false) => "!unitFactor".to_string(),
                AnswerSimplificationType::UnitPower(true) => "unitPower".to_string(),
                AnswerSimplificationType::UnitPower(false) => "!unitPower".to_string(),
                AnswerSimplificationType::UnitDenominator(true) => "unitDenominator".to_string(),
                AnswerSimplificationType::UnitDenominator(false) => "!unitDenominator".to_string(),
                AnswerSimplificationType::ZeroFactor(true) => "zeroFactor".to_string(),
                AnswerSimplificationType::ZeroFactor(false) => "!zeroFactor".to_string(),
                AnswerSimplificationType::ZeroTerm(true) => "zeroTerm".to_string(),
                AnswerSimplificationType::ZeroTerm(false) => "!zeroTerm".to_string(),
                AnswerSimplificationType::ZeroPower(true) => "zeroPower".to_string(),
                AnswerSimplificationType::ZeroPower(false) => "!zeroPower".to_string(),
                AnswerSimplificationType::CollectNumbers(true) => "collectNumbers".to_string(),
                AnswerSimplificationType::CollectNumbers(false) => "!collectNumbers".to_string(),
                AnswerSimplificationType::ZeroBase(true) => "zeroBase".to_string(),
                AnswerSimplificationType::ZeroBase(false) => "!zeroBase".to_string(),
                AnswerSimplificationType::ConstantsFirst(true) => "constantsFirst".to_string(),
                AnswerSimplificationType::ConstantsFirst(false) => "!constantsFirst".to_string(),
                AnswerSimplificationType::SqrtProduct(true) => "sqrtProduct".to_string(),
                AnswerSimplificationType::SqrtProduct(false) => "!sqrtProduct".to_string(),
                AnswerSimplificationType::SqrtDivision(true) => "sqrtDivision".to_string(),
                AnswerSimplificationType::SqrtDivision(false) => "!sqrtDivision".to_string(),
                AnswerSimplificationType::SqrtSquare(true) => "sqrtSquare".to_string(),
                AnswerSimplificationType::SqrtSquare(false) => "!sqrtSquare".to_string(),
                AnswerSimplificationType::OtherNumbers(true) => "otherNumbers".to_string(),
                AnswerSimplificationType::OtherNumbers(false) => "!otherNumbers".to_string(),
                AnswerSimplificationType::TimesDot(true) => "timesDot".to_string(),
                AnswerSimplificationType::TimesDot(false) => "!timesDot".to_string(),
                AnswerSimplificationType::ExpandBrackets(true) => "expandBrackets".to_string(),
                AnswerSimplificationType::ExpandBrackets(false) => "!expandBrackets".to_string(),
                AnswerSimplificationType::NoLeadingMinus(true) => "noLeadingMinus".to_string(),
                AnswerSimplificationType::NoLeadingMinus(false) => "!noLeadingMinus".to_string(),
                AnswerSimplificationType::Trigonometric(true) => "trig".to_string(),
                AnswerSimplificationType::Trigonometric(false) => "!trig".to_string(),
                AnswerSimplificationType::CollectLikeFractions(true) => {
                    "collectLikeFractions".to_string()
                }
                AnswerSimplificationType::CollectLikeFractions(false) => {
                    "!collectLikeFractions".to_string()
                }
                AnswerSimplificationType::CanonicalOrder(true) => "canonicalOrder".to_string(),
                AnswerSimplificationType::CanonicalOrder(false) => "!canonicalOrder".to_string(),
                AnswerSimplificationType::CancelFactors(true) => "cancelFactors".to_string(),
                AnswerSimplificationType::CancelFactors(false) => "!cancelFactors".to_string(),
                AnswerSimplificationType::CancelTerms(true) => "cancelTerms".to_string(),
                AnswerSimplificationType::CancelTerms(false) => "!cancelTerms".to_string(),
                AnswerSimplificationType::Fractions(true) => "simplifyFractions".to_string(),
                AnswerSimplificationType::Fractions(false) => "!simplifyFractions".to_string(),
                AnswerSimplificationType::Unknown((n, true)) => n.to_string(),
                AnswerSimplificationType::Unknown((n, false)) => format!("!{}", n),
            }
        )
    }
}

impl std::convert::From<AnswerSimplificationType> for String {
    fn from(a: AnswerSimplificationType) -> String {
        format!("{}", a)
    }
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq)]
#[serde(try_from = "&str")]
#[serde(into = "String")]
pub enum AnswerSimplificationType {
    //TODO casing?
    All(bool),
    Basic(bool),
    UnitFactor(bool),
    UnitPower(bool),
    UnitDenominator(bool),
    ZeroFactor(bool),
    ZeroTerm(bool),
    ZeroPower(bool),
    CollectNumbers(bool),
    ZeroBase(bool),
    ConstantsFirst(bool),
    SqrtProduct(bool),
    SqrtDivision(bool),
    SqrtSquare(bool),
    OtherNumbers(bool),
    TimesDot(bool),
    ExpandBrackets(bool),
    CollectLikeFractions(bool),
    CanonicalOrder(bool),
    NoLeadingMinus(bool),
    Fractions(bool),
    Trigonometric(bool),
    CancelTerms(bool),
    CancelFactors(bool),
    Unknown((String, bool)),
}
