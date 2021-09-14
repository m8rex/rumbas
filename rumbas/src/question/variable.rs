use crate::support::file_reference::FileString;
use crate::support::to_numbas::ToNumbas;
use crate::support::to_rumbas::ToRumbas;
use regex::Regex;
use rumbas_support::preamble::*;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

pub const UNGROUPED_GROUP: &str = "Ungrouped variables";

#[derive(Input, Overwrite, RumbasCheck)]
#[input(name = "VariableRepresentationInput")]
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(untagged)]
pub enum VariableRepresentation {
    ListOfStrings(Vec<String>),
    ListOfNumbers(Vec<f64>),
    Long(Box<Variable>),
    Number(f64),
    Other(VariableStringRepresentation),
}

impl ToNumbas<numbas::question::variable::Variable> for VariableRepresentation {
    fn to_numbas_with_name(
        &self,
        locale: &str,
        name: String,
    ) -> numbas::question::variable::Variable {
        self.to_variable().to_numbas_with_name(locale, name)
    }
    fn to_numbas(&self, _locale: &str) -> numbas::question::variable::Variable {
        panic!(
            "{}",
            "Should not happen, don't call this method Missing name".to_string(),
        )
    }
}

impl ToRumbas<VariableRepresentation> for numbas::question::variable::Variable {
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
                &serde_json::to_string(&l).unwrap(),
            ),
            VariableRepresentation::ListOfNumbers(l) => Variable::ungrouped(
                VariableTemplateType::ListOfNumbers,
                &serde_json::to_string(&l).unwrap(),
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
                    Variable::ungrouped(VariableTemplateType::Range, &r.as_range())
                }
                VariableStringRepresentation::RandomRange(r) => {
                    Variable::ungrouped(VariableTemplateType::RandomRange, &r.as_random_range())
                }
            },
        }
    }
}

#[derive(Input, Overwrite, RumbasCheck)]
#[input(name = "VariableStringRepresentationInput")]
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(from = "String")]
pub enum VariableStringRepresentation {
    Anything(String),
    Range(RangeData),
    RandomRange(RangeData),
}
/* TODO remove?
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
}*/

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

impl std::convert::From<String> for VariableStringRepresentationInput {
    fn from(s: String) -> Self {
        let v = VariableStringRepresentation::from(s);
        Self::from_normal(v)
    }
}

#[derive(Input, Overwrite, RumbasCheck)]
#[input(name = "RangeDataInput")]
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema, PartialEq)]
pub struct RangeData {
    pub from: f64,
    pub to: f64,
    pub step: f64,
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
    pub fn as_range(&self) -> String {
        format!("{} .. {}#{}", self.from, self.to, self.step)
    }
    pub fn try_from_random_range(s: &str) -> Option<RangeData> {
        if s.starts_with("random(") && s.ends_with(')') {
            return RangeData::try_from_range(&s[7..s.len() - 1].to_string());
        }
        None
    }
    pub fn as_random_range(&self) -> String {
        format!("random({})", self.as_range())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    macro_rules! i {
        ($expr: expr) => {
            RangeDataInput::from_normal($expr)
        };
    }

    #[test]
    fn range_ints() {
        let s = "2 .. 10#1".to_string();
        assert_eq!(
            Some(i!(RangeData {
                from: 2.0,
                to: 10.0,
                step: 1.0
            })),
            RangeData::try_from_range(&s).map(|a| i!(a))
        );
        assert_eq!(None, RangeData::try_from_random_range(&s).map(|a| i!(a)));
    }

    #[test]
    fn random_range_ints() {
        let s = "random(2 .. 10#2)".to_string();
        assert_eq!(
            Some(i!(RangeData {
                from: 2.0,
                to: 10.0,
                step: 2.0
            })),
            RangeData::try_from_random_range(&s).map(|a| i!(a))
        );
        assert_eq!(None, RangeData::try_from_range(&s).map(|a| i!(a)));
    }

    #[test]
    fn random_range_ints_without_step() {
        let s = "random(2..10)".to_string();
        assert_eq!(
            Some(i!(RangeData {
                from: 2.0,
                to: 10.0,
                step: 1.0
            })),
            RangeData::try_from_random_range(&s).map(|a| i!(a))
        );
        assert_eq!(None, RangeData::try_from_range(&s).map(|a| i!(a)));
    }

    #[test]
    fn range_floats() {
        let s = "2.1 .. 10.5#1.99".to_string();
        assert_eq!(
            Some(i!(RangeData {
                from: 2.1,
                to: 10.5,
                step: 1.99
            })),
            RangeData::try_from_range(&s).map(|a| i!(a))
        );
        assert_eq!(None, RangeData::try_from_random_range(&s).map(|a| i!(a)));
    }

    #[test]
    fn random_range_floats() {
        let s = "random(20.222 .. 100.0#19.57)".to_string();
        assert_eq!(
            Some(i!(RangeData {
                from: 20.222,
                to: 100.0,
                step: 19.57
            })),
            RangeData::try_from_random_range(&s).map(|a| i!(a))
        );
        assert_eq!(None, RangeData::try_from_range(&s).map(|a| i!(a)));
    }
}

#[derive(Input, Overwrite, RumbasCheck)]
#[input(name = "VariableInput")]
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct Variable {
    pub definition: FileString, //TODO: definition dependant of template type, for random_range: start, end and step instead
    pub description: String,
    pub template_type: VariableTemplateType,
    pub group: String, //TODO "Ungrouped variables" -> real optional? if not -> ungrouped?
}

impl ToNumbas<numbas::question::variable::Variable> for Variable {
    fn to_numbas_with_name(
        &self,
        locale: &str,
        name: String,
    ) -> numbas::question::variable::Variable {
        numbas::question::variable::Variable {
            name,
            definition: self.definition.to_numbas(locale),
            description: self.description.to_numbas(locale),
            template_type: self.template_type.to_numbas(locale),
            group: self.group.to_numbas(locale),
            can_override: false, // Don't support overriding variables (yet?)
        }
    }
    fn to_numbas(&self, _locale: &str) -> numbas::question::variable::Variable {
        panic!(
            "{}",
            "Should not happen, don't call this method Missing name".to_string(),
        )
    }
}

impl Variable {
    fn ungrouped(template_type: VariableTemplateType, definition: &str) -> Variable {
        Variable {
            template_type,
            definition: definition.to_string().to_rumbas(),
            description: String::new().to_rumbas(),
            group: UNGROUPED_GROUP.to_string().to_rumbas(),
        }
    }
}

#[derive(Input, Overwrite, RumbasCheck)]
#[input(name = "VariableTemplateTypeInput")]
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

impl ToNumbas<numbas::question::variable::VariableTemplateType> for VariableTemplateType {
    fn to_numbas(&self, _locale: &str) -> numbas::question::variable::VariableTemplateType {
        match self {
            VariableTemplateType::Anything => {
                numbas::question::variable::VariableTemplateType::Anything
            }
            VariableTemplateType::ListOfNumbers => {
                numbas::question::variable::VariableTemplateType::ListOfNumbers
            }
            VariableTemplateType::ListOfStrings => {
                numbas::question::variable::VariableTemplateType::ListOfStrings
            }
            VariableTemplateType::LongString => {
                numbas::question::variable::VariableTemplateType::LongString
            }
            VariableTemplateType::Number => {
                numbas::question::variable::VariableTemplateType::Number
            }
            VariableTemplateType::Range => numbas::question::variable::VariableTemplateType::Range,
            VariableTemplateType::RandomRange => {
                numbas::question::variable::VariableTemplateType::RandomRange
            }
            VariableTemplateType::r#String => {
                numbas::question::variable::VariableTemplateType::r#String
            }
        }
    }
}

impl ToRumbas<VariableTemplateType> for numbas::question::variable::VariableTemplateType {
    fn to_rumbas(&self) -> VariableTemplateType {
        match self {
            numbas::question::variable::VariableTemplateType::Anything => {
                VariableTemplateType::Anything
            }
            numbas::question::variable::VariableTemplateType::ListOfNumbers => {
                VariableTemplateType::ListOfNumbers
            }
            numbas::question::variable::VariableTemplateType::ListOfStrings => {
                VariableTemplateType::ListOfStrings
            }
            numbas::question::variable::VariableTemplateType::LongString => {
                VariableTemplateType::LongString
            }
            numbas::question::variable::VariableTemplateType::Number => {
                VariableTemplateType::Number
            }
            numbas::question::variable::VariableTemplateType::RandomRange => {
                VariableTemplateType::RandomRange
            }
            numbas::question::variable::VariableTemplateType::Range => VariableTemplateType::Range,
            numbas::question::variable::VariableTemplateType::r#String => {
                VariableTemplateType::r#String
            }
        }
    }
}
