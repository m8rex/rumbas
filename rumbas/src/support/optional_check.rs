use std::collections::HashMap;

pub trait OptionalCheck {
    /// Check the optional data
    fn find_missing(&self) -> OptionalCheckResult;
}

impl<O: OptionalCheck> OptionalCheck for Vec<O> {
    fn find_missing(&self) -> OptionalCheckResult {
        let mut result = OptionalCheckResult::empty();
        for (i, item) in self.iter().enumerate() {
            let mut previous_result = item.find_missing();
            previous_result.extend_path(i.to_string());
            result.union(&previous_result)
        }
        result
    }
}

impl<T: OptionalCheck> OptionalCheck for HashMap<String, T> {
    fn find_missing(&self) -> OptionalCheckResult {
        let mut result = OptionalCheckResult::empty();
        // Key is not displayable, so show an index, just to differentiate
        for (i, (_key, item)) in self.iter().enumerate() {
            let mut previous_result = item.find_missing();
            previous_result.extend_path(i.to_string());
            result.union(&previous_result)
        }
        result
    }
}

impl<O: OptionalCheck> OptionalCheck for Box<O> {
    fn find_missing(&self) -> OptionalCheckResult {
        (**self).find_missing()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct OptionalCheckResult {
    // When adding a field, do also add it to is_empty
    missing_values: Vec<OptionalCheckMissingData>,
    invalid_yaml_values: Vec<OptionalCheckInvalidYamlData>,
}

impl OptionalCheckResult {
    pub fn from_missing(os: Option<String>) -> OptionalCheckResult {
        OptionalCheckResult {
            missing_values: vec![OptionalCheckMissingData {
                path: OptionalCheckPath::with_last(os),
            }],
            invalid_yaml_values: vec![],
        }
    }
    pub fn from_invalid(v: &serde_yaml::Value) -> OptionalCheckResult {
        OptionalCheckResult {
            missing_values: vec![],
            invalid_yaml_values: vec![OptionalCheckInvalidYamlData {
                path: OptionalCheckPath::without_last(),
                data: v.clone(),
            }],
        }
    }
    pub fn empty() -> OptionalCheckResult {
        OptionalCheckResult {
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
    pub fn missing_fields(&self) -> Vec<OptionalCheckMissingData> {
        self.missing_values.clone()
    }
    pub fn invalid_yaml_fields(&self) -> Vec<OptionalCheckInvalidYamlData> {
        self.invalid_yaml_values.clone()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct OptionalCheckPath {
    parts: Vec<String>,
    last_part: Option<String>,
}

impl OptionalCheckPath {
    pub fn with_last(os: Option<String>) -> Self {
        OptionalCheckPath {
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

impl std::fmt::Display for OptionalCheckPath {
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
pub struct OptionalCheckMissingData {
    path: OptionalCheckPath,
}

impl std::fmt::Display for OptionalCheckMissingData {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.path.to_string())
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct OptionalCheckInvalidYamlData {
    path: OptionalCheckPath,
    data: serde_yaml::Value,
}

impl std::fmt::Display for OptionalCheckInvalidYamlData {
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
