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
}

pub trait InputInverse {
    type Input;
}

impl<O: InputInverse> InputInverse for Vec<O> {
    type Input = Vec<<O as InputInverse>::Input>;
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
}

impl<O: InputInverse> InputInverse for HashMap<String, O> {
    type Input = HashMap<String, <O as InputInverse>::Input>;
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
}

impl<O: InputInverse> InputInverse for Box<O> {
    type Input = Box<<O as InputInverse>::Input>;
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
}

macro_rules! impl_input {
    ($($t: ty),*) => {
        $(
        impl InputInverse for $t {
            type Input = Self;
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
        }
        )*
    };
}

impl_input!(String);
impl_input!(f64, f32);
impl_input!(u128, u64, u32, u16, u8, usize);
impl_input!(i128, i64, i32, i16, i8, isize);
impl_input!(bool);

#[derive(Debug, Clone, PartialEq)]
pub struct InputCheckResult {
    // When adding a field, do also add it to is_empty
    missing_values: Vec<OptionalCheckMissingData>,
}

impl InputCheckResult {
    pub fn from_missing(os: Option<String>) -> InputCheckResult {
        InputCheckResult {
            missing_values: vec![OptionalCheckMissingData {
                path: OptionalCheckPath::with_last(os),
            }],
        }
    }
    pub fn empty() -> InputCheckResult {
        InputCheckResult {
            missing_values: vec![],
        }
    }
    pub fn is_empty(&self) -> bool {
        self.missing_values.len() == 0
    }
    pub fn extend_path(&mut self, s: String) {
        for missing_value in self.missing_values.iter_mut() {
            missing_value.path.add(s.clone());
        }
    }
    pub fn union(&mut self, other: &Self) {
        self.missing_values.extend(other.missing_values.clone());
    }
    pub fn missing_fields(&self) -> Vec<OptionalCheckMissingData> {
        self.missing_values.clone()
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
