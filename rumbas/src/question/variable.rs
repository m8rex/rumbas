use crate::support::file_reference::FileString;
use crate::support::optional_overwrite::*;
use crate::support::template::{Value, ValueType};
use crate::support::to_numbas::ToNumbas;
use crate::support::to_rumbas::ToRumbas;
use regex::Regex;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

pub const UNGROUPED_GROUP: &str = "Ungrouped variables";

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq)]
#[serde(untagged)]
pub enum VariableRepresentation {
    ListOfStrings(Vec<Value<String>>),
    ListOfNumbers(Vec<Value<f64>>),
    Long(Box<Variable>),
    Number(f64),
    Other(VariableStringRepresentation),
}

impl RumbasCheck for VariableRepresentation {
    fn check(&self, locale: &str) -> RumbasCheckResult {
        match self {
            VariableRepresentation::ListOfStrings(v) => v.check(locale),
            VariableRepresentation::ListOfNumbers(v) => v.check(locale),
            VariableRepresentation::Long(v) => v.check(locale),
            VariableRepresentation::Number(v) => v.check(locale),
            VariableRepresentation::Other(v) => v.check(locale),
        }
    }
}

impl OptionalOverwrite<VariableRepresentation> for VariableRepresentation {
    fn overwrite(&mut self, _other: &VariableRepresentation) {
        //TODO?
    }
    fn insert_template_value(&mut self, key: &str, val: &serde_yaml::Value) {
        match self {
            VariableRepresentation::ListOfStrings(v) => v.insert_template_value(key, val),
            VariableRepresentation::ListOfNumbers(v) => v.insert_template_value(key, val),
            VariableRepresentation::Long(v) => v.insert_template_value(key, val),
            VariableRepresentation::Number(v) => v.insert_template_value(key, val),
            VariableRepresentation::Other(v) => v.insert_template_value(key, val),
        }
    }
}
impl_optional_overwrite_value!(VariableRepresentation);

impl ToNumbas<numbas::exam::ExamVariable> for VariableRepresentation {
    fn to_numbas_with_name(&self, locale: &str, name: String) -> numbas::exam::ExamVariable {
        self.to_variable().to_numbas_with_name(locale, name)
    }
    fn to_numbas(&self, _locale: &str) -> numbas::exam::ExamVariable {
        panic!(
            "{}",
            "Should not happen, don't call this method Missing name".to_string(),
        )
    }
}

impl ToRumbas<VariableRepresentation> for numbas::exam::ExamVariable {
    fn to_rumbas(&self) -> VariableRepresentation {
        VariableRepresentation::Long(Box::new(Variable {
            definition: self.definition.to_rumbas(),
            description: self.description.to_rumbas(),
            template_type: self.template_type.to_rumbas(),
            group: self.group.to_rumbas(),
        }))
    }
}

impl VariableRepresentation {
    pub fn to_variable(&self) -> Variable {
        match self {
            VariableRepresentation::ListOfStrings(l) => Variable::ungrouped(
                VariableTemplateType::ListOfStrings,
                &serde_json::to_string(&l.iter().map(|e| e.unwrap()).collect::<Vec<_>>()).unwrap(),
            ),
            VariableRepresentation::ListOfNumbers(l) => Variable::ungrouped(
                VariableTemplateType::ListOfNumbers,
                &serde_json::to_string(&l.iter().map(|e| e.unwrap()).collect::<Vec<_>>()).unwrap(),
            ),
            VariableRepresentation::Long(v) => *(v.clone()),
            VariableRepresentation::Number(n) => {
                Variable::ungrouped(VariableTemplateType::Number, &n.to_string())
            }
            VariableRepresentation::Other(o) => match o {
                VariableStringRepresentation::Anything(s) => {
                    Variable::ungrouped(VariableTemplateType::Anything, &s)
                }
                VariableStringRepresentation::Range(r) => {
                    Variable::ungrouped(VariableTemplateType::Range, &r.to_range())
                }
                VariableStringRepresentation::RandomRange(r) => {
                    Variable::ungrouped(VariableTemplateType::RandomRange, &r.to_random_range())
                }
            },
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(from = "String")]
pub enum VariableStringRepresentation {
    Anything(String),
    Range(RangeData),
    RandomRange(RangeData),
}
impl_optional_overwrite!(VariableStringRepresentation);

impl JsonSchema for VariableStringRepresentation {
    fn schema_name() -> String {
        "VariableStringRepresentation".to_owned()
    }

    fn json_schema(_: &mut schemars::gen::SchemaGenerator) -> schemars::schema::Schema {
        schemars::schema::SchemaObject {
            instance_type: Some(schemars::schema::InstanceType::String.into()),
            ..Default::default()
        }
        .into()
    }
}

impl std::convert::From<String> for VariableStringRepresentation {
    fn from(s: String) -> Self {
        if let Some(r) = RangeData::try_from_range(&s) {
            VariableStringRepresentation::Range(r)
        } else if let Some(r) = RangeData::try_from_random_range(&s) {
            VariableStringRepresentation::RandomRange(r)
        } else {
            VariableStringRepresentation::Anything(s)
        }
    }
}

// TODO: let this be parsed as JME?
#[derive(Serialize, Deserialize, JsonSchema, Debug, Copy, Clone, PartialEq)]
pub struct RangeData {
    from: f64,
    to: f64,
    step: f64,
}

impl RangeData {
    pub fn try_from_range(s: &str) -> Option<RangeData> {
        let re = Regex::new(r"^(\d+(?:\.\d*)?)\s*\.\.\s*(\d+(?:\.\d*)?)(\#(\d+(?:\.\d*)?))?$")
            .expect("It to be a valid regex");
        if let Some(c) = re.captures(s) {
            return Some(RangeData {
                from: c.get(1).unwrap().as_str().parse().unwrap(),
                to: c.get(2).unwrap().as_str().parse().unwrap(),
                step: c.get(4).map(|m| m.as_str()).unwrap_or("1").parse().unwrap(),
            });
        }
        None
    }
    pub fn to_range(self) -> String {
        format!("{} .. {}#{}", self.from, self.to, self.step)
    }
    pub fn try_from_random_range(s: &str) -> Option<RangeData> {
        if s.starts_with("random(") && s.ends_with(')') {
            return RangeData::try_from_range(&s[7..s.len() - 1].to_string());
        }
        None
    }
    pub fn to_random_range(self) -> String {
        format!("random({})", self.to_range())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn range_ints() {
        let s = "2 .. 10#1".to_string();
        assert_eq!(
            Some(RangeData {
                from: 2.0,
                to: 10.0,
                step: 1.0
            }),
            RangeData::try_from_range(&s)
        );
        assert_eq!(None, RangeData::try_from_random_range(&s));
    }

    #[test]
    fn random_range_ints() {
        let s = "random(2 .. 10#2)".to_string();
        assert_eq!(
            Some(RangeData {
                from: 2.0,
                to: 10.0,
                step: 2.0
            }),
            RangeData::try_from_random_range(&s)
        );
        assert_eq!(None, RangeData::try_from_range(&s));
    }

    #[test]
    fn random_range_ints_without_step() {
        let s = "random(2..10)".to_string();
        assert_eq!(
            Some(RangeData {
                from: 2.0,
                to: 10.0,
                step: 1.0
            }),
            RangeData::try_from_random_range(&s)
        );
        assert_eq!(None, RangeData::try_from_range(&s));
    }

    #[test]
    fn range_floats() {
        let s = "2.1 .. 10.5#1.99".to_string();
        assert_eq!(
            Some(RangeData {
                from: 2.1,
                to: 10.5,
                step: 1.99
            }),
            RangeData::try_from_range(&s)
        );
        assert_eq!(None, RangeData::try_from_random_range(&s));
    }

    #[test]
    fn random_range_floats() {
        let s = "random(20.222 .. 100.0#19.57)".to_string();
        assert_eq!(
            Some(RangeData {
                from: 20.222,
                to: 100.0,
                step: 19.57
            }),
            RangeData::try_from_random_range(&s)
        );
        assert_eq!(None, RangeData::try_from_range(&s));
    }
}

optional_overwrite! {
    pub struct Variable {
        definition: FileString,//TODO: definition dependant of template type, for random_range: start, end and step instead
        description: String,
        template_type: VariableTemplateType,
        group: String //TODO "Ungrouped variables" -> real optional? if not -> ungrouped?
    }
}

impl ToNumbas<numbas::exam::ExamVariable> for Variable {
    fn to_numbas_with_name(&self, locale: &str, name: String) -> numbas::exam::ExamVariable {
        numbas::exam::ExamVariable {
            name,
            definition: self.definition.to_numbas(locale),
            description: self.description.to_numbas(locale),
            template_type: self.template_type.to_numbas(locale),
            group: self.group.to_numbas(locale),
            can_override: false, // Don't support overriding variables (yet?)
        }
    }
    fn to_numbas(&self, _locale: &str) -> numbas::exam::ExamVariable {
        panic!(
            "{}",
            "Should not happen, don't call this method Missing name".to_string(),
        )
    }
}

impl Variable {
    fn ungrouped(template_type: VariableTemplateType, definition: &str) -> Variable {
        Variable {
            template_type: Value::Normal(template_type.clone()),
            definition: definition.to_string().to_rumbas(),
            description: String::new().to_rumbas(),
            group: UNGROUPED_GROUP.to_string().to_rumbas(),
        }
    }
}

/// The different template_types for a variable
#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum VariableTemplateType {
    /// Not specified
    Anything, // TODO: allow this?
    /// A list of numbers
    ListOfNumbers,
    /// A list of strings
    ListOfStrings,
    /// A long string
    LongString,
    /// A number
    Number,
    /// A random number from a range
    RandomRange,
    /// A range
    Range,
    /// A string
    r#String,
}
impl_optional_overwrite!(VariableTemplateType);

impl ToNumbas<numbas::exam::ExamVariableTemplateType> for VariableTemplateType {
    fn to_numbas(&self, _locale: &str) -> numbas::exam::ExamVariableTemplateType {
        match self {
            VariableTemplateType::Anything => numbas::exam::ExamVariableTemplateType::Anything,
            VariableTemplateType::ListOfNumbers => {
                numbas::exam::ExamVariableTemplateType::ListOfNumbers
            }
            VariableTemplateType::ListOfStrings => {
                numbas::exam::ExamVariableTemplateType::ListOfStrings
            }
            VariableTemplateType::LongString => numbas::exam::ExamVariableTemplateType::LongString,
            VariableTemplateType::Number => numbas::exam::ExamVariableTemplateType::Number,
            VariableTemplateType::Range => numbas::exam::ExamVariableTemplateType::Range,
            VariableTemplateType::RandomRange => {
                numbas::exam::ExamVariableTemplateType::RandomRange
            }
            VariableTemplateType::r#String => numbas::exam::ExamVariableTemplateType::r#String,
        }
    }
}

impl ToRumbas<VariableTemplateType> for numbas::exam::ExamVariableTemplateType {
    fn to_rumbas(&self) -> VariableTemplateType {
        match self {
            numbas::exam::ExamVariableTemplateType::Anything => VariableTemplateType::Anything,
            numbas::exam::ExamVariableTemplateType::ListOfNumbers => {
                VariableTemplateType::ListOfNumbers
            }
            numbas::exam::ExamVariableTemplateType::ListOfStrings => {
                VariableTemplateType::ListOfStrings
            }
            numbas::exam::ExamVariableTemplateType::LongString => VariableTemplateType::LongString,
            numbas::exam::ExamVariableTemplateType::Number => VariableTemplateType::Number,
            numbas::exam::ExamVariableTemplateType::RandomRange => {
                VariableTemplateType::RandomRange
            }
            numbas::exam::ExamVariableTemplateType::Range => VariableTemplateType::Range,
            numbas::exam::ExamVariableTemplateType::r#String => VariableTemplateType::r#String,
        }
    }
}
