use crate::value::Value;
use crate::value::ValueType;
use std::collections::HashMap;

pub trait Input: Clone {
    type Normal;
    /// This method assumes that it is called by a function that is initially called from `to_normal_safe`
    fn to_normal(&self) -> Self::Normal;

    /// Method that safely convets the input type to the normal type
    fn to_normal_safe(&self) -> Result<Self::Normal, InputCheckResult> {
        let check = self.find_missing();
        if check.is_empty() {
            Ok(self.to_normal())
        } else {
            Err(check)
        }
    }

    /// Check the optional data
    fn find_missing(&self) -> InputCheckResult;

    fn from_normal(normal: Self::Normal) -> Self;

    fn insert_template_value(&mut self, key: &str, val: &serde_yaml::Value);
}

pub trait InputInverse {
    type Input;
    type EnumInput;
}

impl<O: InputInverse> InputInverse for Vec<O> {
    type Input = Vec<ValueType<<O as InputInverse>::Input>>;
    type EnumInput = Self::Input;
}
impl<O: Input> Input for Vec<O> {
    type Normal = Vec<<O as Input>::Normal>;

    fn to_normal(&self) -> <Self as Input>::Normal {
        self.iter().map(|a| a.to_normal()).collect()
    }
    fn from_normal(normal: <Self as Input>::Normal) -> Self {
        normal.into_iter().map(<O as Input>::from_normal).collect()
    }

    fn find_missing(&self) -> InputCheckResult {
        let mut result = InputCheckResult::empty();
        for (i, item) in self.iter().enumerate() {
            let mut previous_result = item.find_missing();
            previous_result.extend_path(i.to_string());
            result.union(&previous_result)
        }
        result
    }

    fn insert_template_value(&mut self, key: &str, val: &serde_yaml::Value) {
        for (_i, item) in self.iter_mut().enumerate() {
            item.insert_template_value(key, val);
        }
    }
}

impl<O: InputInverse> InputInverse for HashMap<String, O> {
    type Input = HashMap<String, ValueType<<O as InputInverse>::Input>>;
    type EnumInput = Self::Input;
}
impl<O: Input> Input for HashMap<String, O> {
    type Normal = HashMap<String, <O as Input>::Normal>;

    fn to_normal(&self) -> <Self as Input>::Normal {
        self.iter()
            .map(|(s, a)| (s.to_owned(), a.to_normal()))
            .collect()
    }
    fn from_normal(normal: <Self as Input>::Normal) -> Self {
        normal
            .into_iter()
            .map(|(s, a)| (s, <O as Input>::from_normal(a)))
            .collect()
    }

    fn find_missing(&self) -> InputCheckResult {
        let mut result = InputCheckResult::empty();
        // Key is not displayable, so show an index, just to differentiate
        for (i, (_key, item)) in self.iter().enumerate() {
            let mut previous_result = item.find_missing();
            previous_result.extend_path(i.to_string());
            result.union(&previous_result)
        }
        result
    }

    fn insert_template_value(&mut self, key: &str, val: &serde_yaml::Value) {
        for (_i, (_key, item)) in self.iter_mut().enumerate() {
            item.insert_template_value(key, val);
        }
    }
}

impl<O: InputInverse> InputInverse for Box<O> {
    type Input = Box<<O as InputInverse>::Input>;
    type EnumInput = Self::Input;
}
impl<O: Input> Input for Box<O> {
    type Normal = Box<<O as Input>::Normal>;

    fn to_normal(&self) -> <Self as Input>::Normal {
        Box::new((**self).to_normal())
    }

    fn from_normal(normal: <Self as Input>::Normal) -> Self {
        Box::new(Input::from_normal(*normal))
    }

    fn find_missing(&self) -> InputCheckResult {
        (**self).find_missing()
    }

    fn insert_template_value(&mut self, key: &str, val: &serde_yaml::Value) {
        (**self).insert_template_value(key, val)
    }
}

impl<A: InputInverse, B: InputInverse> InputInverse for (A, B) {
    type Input = (
        Value<<A as InputInverse>::Input>,
        Value<<B as InputInverse>::Input>,
    );
    type EnumInput = Self::Input; // TODO?
}
impl<A: Input, B: Input> Input for (A, B) {
    type Normal = (<A as Input>::Normal, <B as Input>::Normal);

    fn to_normal(&self) -> <Self as Input>::Normal {
        (self.0.to_normal(), self.1.to_normal())
    }
    fn from_normal(normal: <Self as Input>::Normal) -> Self {
        (
            <A as Input>::from_normal(normal.0),
            <B as Input>::from_normal(normal.1),
        )
    }

    fn find_missing(&self) -> InputCheckResult {
        let mut result = InputCheckResult::empty();
        let i = 0;
        let mut previous_result = self.0.find_missing();
        previous_result.extend_path(i.to_string());
        result.union(&previous_result);
        let i = 1;
        let mut previous_result = self.1.find_missing();
        previous_result.extend_path(i.to_string());
        result.union(&previous_result);
        result
    }

    fn insert_template_value(&mut self, key: &str, val: &serde_yaml::Value) {
        self.0.insert_template_value(key, val);
        self.1.insert_template_value(key, val);
    }
}

macro_rules! impl_input {
    ($($t: ty),*) => {
        $(
        impl InputInverse for $t {
            type Input = Self;
            type EnumInput = Self;
        }
        impl Input for $t {
            type Normal = Self;

            fn to_normal(&self) -> <Self as Input>::Normal {
                self.to_owned()
            }

            fn from_normal(normal: <Self as Input>::Normal) -> Self {
                normal
            }

            fn find_missing(&self) -> InputCheckResult {
                InputCheckResult::empty()
            }

            fn insert_template_value(&mut self, _key: &str, _val: &serde_yaml::Value) {

            }
        }
        )*
    };
}

impl_input!(String);
impl_input!(f64, f32, [f64; 2]);
impl_input!(u128, u64, u32, u16, u8, usize);
impl_input!(i128, i64, i32, i16, i8, isize);
impl_input!(bool);

impl_input!(std::path::PathBuf);

impl_input!(numbas::jme::ContentAreaString);
impl_input!(numbas::jme::EmbracedJMEString);
impl_input!(numbas::jme::JMEString);
impl_input!(numbas::question::part::match_answers::MatchAnswersWithChoicesLayout);
impl_input!(numbas::question::part::match_answers::MatchAnswersWithChoicesDisplayType);
impl_input!(numbas::question::part::match_answers::MultipleChoiceWarningType);
impl_input!(numbas::question::part::pattern_match::PatternMatchMode);
impl_input!(numbas::support::answer_style::AnswerStyle);
impl_input!(numbas::question::function::FunctionType);
impl_input!(numbas::question::custom_part_type::CustomPartTypeSetting);
impl_input!(numbas::support::primitive::Number);

#[derive(Debug, Clone, PartialEq)]
pub struct InputCheckResult {
    // When adding a field, do also add it to is_empty
    missing_values: Vec<InputCheckMissingData>,
    invalid_yaml_values: Vec<InputCheckInvalidYamlData>,
}

impl InputCheckResult {
    pub fn from_missing(os: Option<String>) -> InputCheckResult {
        InputCheckResult {
            missing_values: vec![InputCheckMissingData {
                path: InputCheckPath::with_last(os),
            }],
            invalid_yaml_values: vec![],
        }
    }
    pub fn from_invalid(v: &serde_yaml::Value) -> InputCheckResult {
        InputCheckResult {
            missing_values: vec![],
            invalid_yaml_values: vec![InputCheckInvalidYamlData {
                path: InputCheckPath::without_last(),
                data: v.clone(),
            }],
        }
    }
    pub fn empty() -> InputCheckResult {
        InputCheckResult {
            missing_values: vec![],
            invalid_yaml_values: vec![],
        }
    }
    pub fn is_empty(&self) -> bool {
        self.missing_values.len() == 0 && self.invalid_yaml_values.len() == 0
    }
    pub fn extend_path(&mut self, s: String) {
        for missing_value in self.missing_values.iter_mut() {
            missing_value.path.add(s.clone());
        }
        for invalid_value in self.invalid_yaml_values.iter_mut() {
            invalid_value.path.add(s.clone());
        }
    }
    pub fn union(&mut self, other: &Self) {
        self.missing_values.extend(other.missing_values.clone());
        self.invalid_yaml_values
            .extend(other.invalid_yaml_values.clone());
    }
    pub fn missing_fields(&self) -> Vec<InputCheckMissingData> {
        self.missing_values.clone()
    }
    pub fn invalid_yaml_fields(&self) -> Vec<InputCheckInvalidYamlData> {
        self.invalid_yaml_values.clone()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct InputCheckPath {
    parts: Vec<String>,
    last_part: Option<String>,
}

impl InputCheckPath {
    pub fn with_last(os: Option<String>) -> Self {
        InputCheckPath {
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

impl std::fmt::Display for InputCheckPath {
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
pub struct InputCheckMissingData {
    path: InputCheckPath,
}

impl std::fmt::Display for InputCheckMissingData {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.path.to_string())
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct InputCheckInvalidYamlData {
    path: InputCheckPath,
    data: serde_yaml::Value,
}

impl std::fmt::Display for InputCheckInvalidYamlData {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let p = self.path.to_string();
        write!(
            f,
            "{}",
            if let Ok(s) = serde_yaml::to_string(&self.data) {
                format!("{}\n With yaml:\n{}", p, s)
            } else {
                p
            }
        )
    }
}
