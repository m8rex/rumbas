use std::collections::HashMap;

pub trait RumbasCheck {
    /// Check the read rumbas data
    fn check(&self, locale: &str) -> RumbasCheckResult;
}

impl<O: RumbasCheck> RumbasCheck for Vec<O> {
    fn check(&self, locale: &str) -> RumbasCheckResult {
        let mut result = RumbasCheckResult::empty();
        for (i, item) in self.iter().enumerate() {
            let mut previous_result = item.check(locale);
            previous_result.extend_path(i.to_string());
            result.union(&previous_result)
        }
        result
    }
}

impl<T: RumbasCheck> RumbasCheck for HashMap<String, T> {
    fn check(&self, locale: &str) -> RumbasCheckResult {
        let mut result = RumbasCheckResult::empty();
        // Key is not displayable, so show an index, just to differentiate
        for (i, (_key, item)) in self.iter().enumerate() {
            let mut previous_result = item.check(locale);
            previous_result.extend_path(i.to_string());
            result.union(&previous_result)
        }
        result
    }
}

impl<O: RumbasCheck> RumbasCheck for Box<O> {
    fn check(&self, locale: &str) -> RumbasCheckResult {
        (**self).check(locale)
    }
}

impl<A: RumbasCheck, B: RumbasCheck> RumbasCheck for (A, B) {
    fn check(&self, locale: &str) -> RumbasCheckResult {
        let mut result = RumbasCheckResult::empty();
        let i = 0;
        let mut previous_result = self.0.check(locale);
        previous_result.extend_path(i.to_string());
        result.union(&previous_result);
        let i = 1;
        let mut previous_result = self.1.check(locale);
        previous_result.extend_path(i.to_string());
        result.union(&previous_result);
        result
    }
}

macro_rules! impl_rumbas_check {
    ($($t: ty),*) => {
        $(
        impl RumbasCheck for $t {
            fn check(&self, _locale: &str) -> RumbasCheckResult {
                RumbasCheckResult::empty()
            }
        }
        )*
    };
}

impl_rumbas_check!(String);
impl_rumbas_check!(f64, f32, [f64; 2]);
impl_rumbas_check!(u128, u64, u32, u16, u8, usize);
impl_rumbas_check!(i128, i64, i32, i16, i8, isize);
impl_rumbas_check!(bool);

impl_rumbas_check!(std::path::PathBuf);

impl_rumbas_check!(numbas::jme::ContentAreaString);
impl_rumbas_check!(numbas::jme::EmbracedJMEString);
impl_rumbas_check!(numbas::jme::JMEString);
impl_rumbas_check!(numbas::question::part::match_answers::MatchAnswersWithChoicesLayout);
impl_rumbas_check!(numbas::question::part::match_answers::MatchAnswersWithChoicesDisplayType);
impl_rumbas_check!(numbas::question::part::match_answers::MultipleChoiceWarningType);
impl_rumbas_check!(numbas::question::part::pattern_match::PatternMatchMode);
impl_rumbas_check!(numbas::support::answer_style::AnswerStyle);
impl_rumbas_check!(numbas::support::primitive::Number);
impl_rumbas_check!(numbas::question::function::FunctionType);

#[derive(Debug, Clone, PartialEq)]
pub struct RumbasCheckResult {
    // When adding a field, do also add it to is_empty
    missing_translations: Vec<RumbasCheckMissingData>,
    invalid_jme_strings: Vec<RumbasCheckInvalidJMEStringData>,
}

impl RumbasCheckResult {
    pub fn from_missing_translation(os: Option<String>) -> RumbasCheckResult {
        RumbasCheckResult {
            missing_translations: vec![RumbasCheckMissingData {
                path: RumbasCheckPath::with_last(os),
            }],
            invalid_jme_strings: vec![],
        }
    }

    pub fn from_invalid_jme(e: &numbas::jme::parser::ConsumeError) -> RumbasCheckResult {
        RumbasCheckResult {
            missing_translations: vec![],
            invalid_jme_strings: vec![RumbasCheckInvalidJMEStringData {
                path: RumbasCheckPath::without_last(),
                error: e.clone(),
            }],
        }
    }
    pub fn empty() -> RumbasCheckResult {
        RumbasCheckResult {
            missing_translations: vec![],
            invalid_jme_strings: vec![],
        }
    }
    pub fn is_empty(&self) -> bool {
        self.missing_translations.len() == 0 && self.invalid_jme_strings.len() == 0
    }
    pub fn extend_path(&mut self, s: String) {
        for missing_value in self.missing_translations.iter_mut() {
            missing_value.path.add(s.clone());
        }
        for invalid_value in self.invalid_jme_strings.iter_mut() {
            invalid_value.path.add(s.clone());
        }
    }
    pub fn union(&mut self, other: &Self) {
        self.missing_translations
            .extend(other.missing_translations.clone());
        self.invalid_jme_strings
            .extend(other.invalid_jme_strings.clone());
    }
    pub fn missing_translations(&self) -> Vec<RumbasCheckMissingData> {
        self.missing_translations.clone()
    }
    pub fn invalid_jme_fields(&self) -> Vec<RumbasCheckInvalidJMEStringData> {
        self.invalid_jme_strings.clone()
    }
}

impl RumbasCheckResult {
    pub fn log(&self) {
        let missing_translations = self.missing_translations();
        let invalid_jme_fields = self.invalid_jme_fields();
        if !missing_translations.is_empty() {
            log::error!("Found {} missing translations:", missing_translations.len());
            for (idx, error) in missing_translations.iter().enumerate() {
                log::error!("{}\t{}", idx + 1, error.to_string());
            }
        }
        if !invalid_jme_fields.is_empty() {
            log::error!(
                "Found {} invalid jme expressions:",
                invalid_jme_fields.len()
            );
            for (idx, error) in invalid_jme_fields.iter().enumerate() {
                log::error!("{}\t{}", idx + 1, error.to_string());
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct RumbasCheckPath {
    parts: Vec<String>,
    last_part: Option<String>,
}

impl RumbasCheckPath {
    pub fn with_last(os: Option<String>) -> Self {
        RumbasCheckPath {
            parts: vec![],
            last_part: os,
        }
    }
    pub fn without_last() -> Self {
        Self::with_last(None)
    }
    pub fn add(&mut self, s: String) {
        self.parts.insert(0, s)
    }
}

impl std::fmt::Display for RumbasCheckPath {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let base = self.parts.join(".");
        write!(
            f,
            "{}",
            if let Some(ref e) = self.last_part {
                format!("{}.{}", base, e)
            } else {
                base
            }
        )
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct RumbasCheckMissingData {
    path: RumbasCheckPath,
}

impl std::fmt::Display for RumbasCheckMissingData {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.path)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct RumbasCheckInvalidJMEStringData {
    path: RumbasCheckPath,
    error: numbas::jme::parser::ConsumeError,
}

impl std::fmt::Display for RumbasCheckInvalidJMEStringData {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let p = self.path.to_string();
        write!(f, "{}\n With error:\n{}", p, self.error)
    }
}
