use crate::support::file_reference::FileString;
use crate::support::file_reference::FILE_PREFIX;
use crate::support::to_numbas::ToNumbas;
use crate::support::to_rumbas::ToRumbas;
use crate::support::translatable::TranslatableString;
use comparable::Comparable;
use numbas::jme::JMEString;
use regex::Regex;
use rumbas_support::preamble::*;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use structdoc::StructDoc;

pub const UNGROUPED_GROUP: &str = "Ungrouped variables";

#[derive(Input, Overwrite, RumbasCheck, Examples, StructDoc)]
#[input(name = "VariableRepresentationInput")]
#[derive(Serialize, Deserialize, Comparable, Debug, Clone, JsonSchema, PartialEq)]
#[serde(untagged)]
pub enum VariableRepresentation {
    //ListOfNumbers(Vec<f64>), not used for now, impossible to find difference with ListOfStrings
    //because of templating and invalid: TODO
    ListOfStrings(Vec<String>),
    Number(f64),
    Other(VariableStringRepresentation),
    TranslatableString(TranslatableString),
    Long(Box<Variable>),
}

#[derive(Debug, Clone)]
pub struct VariableToNumbasHelper {
    name: String,
    group: String
}

impl VariableToNumbasHelper {
    pub fn with_group(name: String, group: String) -> Self {
        VariableToNumbasHelper { name, group }
    }
    pub fn ungrouped(name: String) -> Self {
        Self::with_group(name, UNGROUPED_GROUP.to_string())
    }
}

impl ToNumbas<numbas::question::variable::Variable> for VariableRepresentation {
    type ToNumbasHelper = VariableToNumbasHelper;
    fn to_numbas(
        &self,
        locale: &str,
        data: &Self::ToNumbasHelper,
    ) -> numbas::question::variable::Variable {
        self.to_variable(locale).to_numbas(locale, data)
    }
}

impl ToRumbas<VariableRepresentation> for numbas::question::variable::Variable {
    fn to_rumbas(&self) -> VariableRepresentation {
        VariableRepresentation::Long(Box::new(Variable {
            definition: self.definition.to_rumbas(),
            description: self.description.to_rumbas(),
            template_type: self.template_type.to_rumbas(),
        }))
    }
}

impl VariableRepresentation {
    pub fn to_variable(&self, locale: &str) -> Variable {
        match self {
            VariableRepresentation::ListOfStrings(l) => Variable::new(
                VariableTemplateType::ListOfStrings,
                &serde_json::to_string(&l).unwrap(),
            ),
            /*VariableRepresentation::ListOfNumbers(l) => Variable::ungrouped(
                VariableTemplateType::ListOfNumbers,
                &serde_json::to_string(&l).unwrap(),
            ),*/
            VariableRepresentation::Long(v) => *(v.clone()),
            VariableRepresentation::Number(n) => {
                Variable::new(VariableTemplateType::Number, &n.to_string())
            }
            VariableRepresentation::Other(o) => match o {
                VariableStringRepresentation::Anything(s) => {
                    Variable::new(VariableTemplateType::Anything, &s.to_string())
                }
                VariableStringRepresentation::Range(r) => {
                    Variable::new(VariableTemplateType::Range, &r.as_range())
                }
                VariableStringRepresentation::RandomRange(r) => {
                    Variable::new(VariableTemplateType::RandomRange, &r.as_random_range())
                }
                VariableStringRepresentation::r#String(s) => {
                    Variable::new(VariableTemplateType::r#String, s)
                }
            },
            VariableRepresentation::TranslatableString(l) => Variable::new(
                VariableTemplateType::r#String,
                &l.to_string(locale).unwrap(),
            ),
        }
    }
}

#[derive(Input, Overwrite, RumbasCheck, JsonSchema, Examples, StructDoc)]
#[input(name = "VariableStringRepresentationInput")]
#[derive(Serialize, Deserialize, Comparable, Debug, Clone, PartialEq)]
#[serde(try_from = "String")]
#[serde(into = "String")]
pub enum VariableStringRepresentation {
    Anything(JMEString),
    r#String(String),
    Range(RangeData),
    RandomRange(RangeData),
}

impl JsonSchema for VariableStringRepresentationInput {
    fn schema_name() -> String {
        "VariableStringRepresentationInput".to_owned()
    }

    fn json_schema(_: &mut schemars::gen::SchemaGenerator) -> schemars::schema::Schema {
        schemars::schema::SchemaObject {
            instance_type: Some(schemars::schema::InstanceType::String.into()),
            ..Default::default()
        }
        .into()
    }
}

impl std::convert::TryFrom<String> for VariableStringRepresentation {
    type Error = &'static str;
    fn try_from(s: String) -> Result<Self, Self::Error> {
        if s.starts_with(&format!("{}:", FILE_PREFIX)[..]) {
            Err("can't start with file:")
        } else {
            Ok(if let Some(r) = RangeData::try_from_range(&s) {
                VariableStringRepresentation::Range(r)
            } else if let Some(r) = RangeData::try_from_random_range(&s) {
                VariableStringRepresentation::RandomRange(r)
            } else if let Ok(r) = JMEString::try_from(s.clone()) {
                VariableStringRepresentation::Anything(r)
            } else {
                VariableStringRepresentation::String(s)
            })
        }
    }
}

impl std::convert::TryFrom<String> for VariableStringRepresentationInput {
    type Error = &'static str;
    fn try_from(s: String) -> Result<Self, Self::Error> {
        VariableStringRepresentation::try_from(s).map(Self::from_normal)
    }
}

impl std::convert::From<VariableStringRepresentation> for String {
    fn from(v: VariableStringRepresentation) -> Self {
        match v {
            VariableStringRepresentation::Anything(s) => s.to_string(),
            VariableStringRepresentation::Range(r) => r.as_range(),
            VariableStringRepresentation::RandomRange(r) => r.as_random_range(),
            VariableStringRepresentation::r#String(s) => s,
        }
    }
}
// TODO a StringInput derive that creates a struct which equals it's Input and reads and writes to
// string
impl std::convert::From<VariableStringRepresentationInput> for String {
    fn from(v: VariableStringRepresentationInput) -> Self {
        v.to_normal().into()
    }
}

#[derive(Input, Overwrite, RumbasCheck, StructDoc)]
#[input(name = "RangeDataInput")]
#[derive(Serialize, Deserialize, Comparable, Debug, Clone, JsonSchema, PartialEq)]
pub struct RangeData {
    pub from: f64,
    pub to: f64,
    pub step: f64,
}

impl Examples for RangeDataInputEnum {
    fn examples() -> Vec<Self> {
        RangeDataInput::examples()
            .into_iter()
            .map(RangeDataInputEnum)
            .collect()
    }
}

impl Examples for RangeDataInput {
    fn examples() -> Vec<Self> {
        vec![
            RangeData {
                from: 0.0,
                to: 1.0,
                step: 0.1,
            },
            RangeData {
                from: 1.0,
                to: 1.0,
                step: -0.1,
            },
            RangeData {
                from: 10.0,
                to: 1.0,
                step: 5.0,
            },
            RangeData {
                from: -10.0,
                to: 1.0,
                step: 0.1,
            },
        ]
        .into_iter()
        .map(Self::from_normal)
        .collect()
    }
}

impl RangeData {
    pub fn try_from_range(s: &str) -> Option<RangeData> {
        let re = Regex::new(
            r"^((-)?\d+(?:\.\d*)?)\s*\.\.\s*((-)?\d+(?:\.\d*)?)(\#((-)?\d+(?:\.\d*)?))?$",
        )
        .expect("It to be a valid regex");
        if let Some(c) = re.captures(s) {
            return Some(RangeData {
                from: c.get(1).unwrap().as_str().parse().unwrap(),
                to: c.get(3).unwrap().as_str().parse().unwrap(),
                step: c.get(6).map(|m| m.as_str()).unwrap_or("1").parse().unwrap(),
            });
        }
        None
    }
    pub fn as_range(&self) -> String {
        format!("{} .. {}#{}", self.from, self.to, self.step)
    }
    pub fn try_from_random_range(s: &str) -> Option<RangeData> {
        if s.starts_with("random(") && s.ends_with(')') {
            return RangeData::try_from_range(&s[7..s.len() - 1]);
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
    fn range_back_and_forth() {
        for input in RangeDataInput::examples().into_iter() {
            let data = input.to_normal();
            let range = data.as_range();
            assert_eq!(Some(data.clone()), RangeData::try_from_range(&range));

            let random_range = data.as_random_range();
            assert_eq!(
                Some(data.clone()),
                RangeData::try_from_random_range(&random_range)
            );
        }
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

#[derive(Input, Overwrite, RumbasCheck, Examples, StructDoc)]
#[input(name = "VariableInput")]
#[derive(Serialize, Deserialize, Comparable, Debug, Clone, JsonSchema, PartialEq, Eq)]
pub struct Variable {
    pub definition: FileString, //TODO: definition dependant of template type, for random_range: start, end and step instead
    pub description: String,
    pub template_type: VariableTemplateType,
}

impl ToNumbas<numbas::question::variable::Variable> for Variable {
    type ToNumbasHelper = VariableToNumbasHelper;
    fn to_numbas(
        &self,
        locale: &str,
        data: &Self::ToNumbasHelper,
    ) -> numbas::question::variable::Variable {
        let data = data.to_owned();
        numbas::question::variable::Variable {
            name: data.name,
            definition: self.definition.to_numbas(locale, &()),
            description: self.description.to_numbas(locale, &()),
            template_type: self.template_type.to_numbas(locale, &()),
            group: data.group,
            can_override: false, // Don't support overriding variables (yet?)
        }
    }
}

impl Variable {
    fn new(template_type: VariableTemplateType, definition: &str) -> Variable {
        Variable {
            template_type,
            definition: definition.to_string().to_rumbas(),
            description: String::new().to_rumbas(),
        }
    }
}

#[derive(Input, Overwrite, RumbasCheck, Examples, StructDoc)]
#[input(name = "VariableTemplateTypeInput")]
/// The different template_types for a variable
#[derive(Serialize, Deserialize, Comparable, JsonSchema, Debug, Clone, PartialEq, Eq)]
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
    type ToNumbasHelper = ();
    fn to_numbas(&self, _locale: &str, _data: &Self::ToNumbasHelper) -> numbas::question::variable::VariableTemplateType {
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
