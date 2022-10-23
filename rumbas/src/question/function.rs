use crate::support::to_numbas::ToNumbas;
use crate::support::to_rumbas::ToRumbas;
use crate::support::translatable::{JMETranslatableString, TranslatableString};
use comparable::Comparable;
use rumbas_support::preamble::*;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use structdoc::StructDoc;

#[derive(Input, Overwrite, RumbasCheck, Examples, StructDoc)]
#[input(name = "FunctionInput")]
#[derive(Serialize, Deserialize, Comparable, Debug, Clone, JsonSchema, PartialEq, Eq)]
pub struct Function {
    // TODO: don't directly use numbas type
    pub parameters: Vec<(String, FunctionType)>,
    pub output_type: FunctionType,
    #[serde(flatten)]
    pub definition: FunctionDefinition,
}

impl ToNumbas<numbas::question::function::Function> for Function {
    fn to_numbas(&self, locale: &str) -> numbas::question::function::Function {
        numbas::question::function::Function {
            parameters: self.parameters.to_numbas(locale),
            output_type: self.output_type.to_numbas(locale),
            definition: self.definition.to_numbas(locale),
        }
    }
}

impl ToRumbas<Function> for numbas::question::function::Function {
    fn to_rumbas(&self) -> Function {
        Function {
            definition: self.definition.to_rumbas(),
            output_type: self.output_type.to_rumbas(),
            parameters: self.parameters.to_rumbas(),
        }
    }
}

#[derive(Input, Overwrite, RumbasCheck, Examples, StructDoc)]
#[input(name = "FunctionDefinitionInput")]
#[derive(Serialize, Deserialize, Comparable, Debug, Clone, JsonSchema, PartialEq, Eq)]
#[serde(tag = "language")]
pub enum FunctionDefinition {
    #[serde(rename = "jme")]
    JME(FunctionDefinitionJME),
    #[serde(rename = "javascript")]
    Javascript(FunctionDefinitionJavascript),
}

impl ToNumbas<numbas::question::function::FunctionDefinition> for FunctionDefinition {
    fn to_numbas(&self, locale: &str) -> numbas::question::function::FunctionDefinition {
        match self {
            FunctionDefinition::JME(c) => numbas::question::function::FunctionDefinition::JME {
                definition: c.definition.to_numbas(locale),
            },
            FunctionDefinition::Javascript(c) => {
                numbas::question::function::FunctionDefinition::Javascript {
                    definition: c.definition.to_numbas(locale),
                }
            }
        }
    }
}

impl ToRumbas<FunctionDefinition> for numbas::question::function::FunctionDefinition {
    fn to_rumbas(&self) -> FunctionDefinition {
        match self {
            numbas::question::function::FunctionDefinition::JME { definition } => {
                FunctionDefinition::JME(FunctionDefinitionJME {
                    definition: definition.to_rumbas(),
                })
            }
            numbas::question::function::FunctionDefinition::Javascript { definition } => {
                FunctionDefinition::Javascript(FunctionDefinitionJavascript {
                    definition: definition.to_rumbas(),
                })
            }
        }
    }
}

#[derive(Input, Overwrite, RumbasCheck, Examples, StructDoc)]
#[input(name = "FunctionDefinitionJMEInput")]
#[derive(Serialize, Deserialize, Comparable, Debug, Clone, JsonSchema, PartialEq, Eq)]
pub struct FunctionDefinitionJME {
    pub definition: JMETranslatableString,
}

#[derive(Input, Overwrite, RumbasCheck, Examples, StructDoc)]
#[input(name = "FunctionDefinitionJavascriptInput")]
#[derive(Serialize, Deserialize, Comparable, Debug, Clone, JsonSchema, PartialEq, Eq)]
pub struct FunctionDefinitionJavascript {
    pub definition: TranslatableString,
}

#[derive(Input, Overwrite, RumbasCheck, Examples, StructDoc)]
#[input(name = "FunctionTypeInput")]
#[derive(Serialize, Deserialize, Comparable, Debug, Clone, JsonSchema, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum FunctionType {
    Boolean,
    Decimal,
    Dictionary,
    Expression,
    #[serde(rename = "html")]
    HTML,
    Integer,
    KeyPair,
    List,
    Matrix,
    Nothing,
    Number,
    Range,
    Rational,
    Set,
    r#String,
    Vector,
    ExtensionGeogebraApplet,
}

impl ToNumbas<numbas::question::function::FunctionType> for FunctionType {
    fn to_numbas(&self, _locale: &str) -> numbas::question::function::FunctionType {
        match self {
            Self::Boolean => numbas::question::function::FunctionType::Boolean,
            Self::Decimal => numbas::question::function::FunctionType::Decimal,
            Self::Dictionary => numbas::question::function::FunctionType::Dictionary,
            Self::Expression => numbas::question::function::FunctionType::Expression,
            Self::HTML => numbas::question::function::FunctionType::HTML,
            Self::Integer => numbas::question::function::FunctionType::Integer,
            Self::KeyPair => numbas::question::function::FunctionType::KeyPair,
            Self::List => numbas::question::function::FunctionType::List,
            Self::Matrix => numbas::question::function::FunctionType::Matrix,
            Self::Nothing => numbas::question::function::FunctionType::Nothing,
            Self::Number => numbas::question::function::FunctionType::Number,
            Self::Range => numbas::question::function::FunctionType::Range,
            Self::Rational => numbas::question::function::FunctionType::Rational,
            Self::Set => numbas::question::function::FunctionType::Set,
            Self::r#String => numbas::question::function::FunctionType::r#String,
            Self::Vector => numbas::question::function::FunctionType::Vector,
            Self::ExtensionGeogebraApplet => {
                numbas::question::function::FunctionType::ExtensionGeogebraApplet
            }
        }
    }
}

impl ToRumbas<FunctionType> for numbas::question::function::FunctionType {
    fn to_rumbas(&self) -> FunctionType {
        match self {
            Self::Boolean => FunctionType::Boolean,
            Self::Decimal => FunctionType::Decimal,
            Self::Dictionary => FunctionType::Dictionary,
            Self::Expression => FunctionType::Expression,
            Self::HTML => FunctionType::HTML,
            Self::Integer => FunctionType::Integer,
            Self::KeyPair => FunctionType::KeyPair,
            Self::List => FunctionType::List,
            Self::Matrix => FunctionType::Matrix,
            Self::Nothing => FunctionType::Nothing,
            Self::Number => FunctionType::Number,
            Self::Range => FunctionType::Range,
            Self::Rational => FunctionType::Rational,
            Self::Set => FunctionType::Set,
            Self::r#String => FunctionType::r#String,
            Self::Vector => FunctionType::Vector,
            Self::ExtensionGeogebraApplet => FunctionType::ExtensionGeogebraApplet,
        }
    }
}
