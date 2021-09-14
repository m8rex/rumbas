use crate::support::to_numbas::impl_to_numbas;
use crate::support::to_numbas::ToNumbas;
use crate::support::to_rumbas::ToRumbas;
use crate::support::translatable::{JMETranslatableString, TranslatableString};
use numbas::question::function::FunctionType as NumbasFunctionType;
use rumbas_support::preamble::*;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

// TODO: don't directly use numbas type
type StringFunctionTypeTuple = (String, NumbasFunctionType);
type StringFunctionTypeTuples = Vec<StringFunctionTypeTuple>;

impl_to_numbas!(NumbasFunctionType);

#[derive(Input, Overwrite, RumbasCheck)]
#[input(name = "FunctionInput")]
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct Function {
    parameters: StringFunctionTypeTuples,
    output_type: NumbasFunctionType,
    #[serde(flatten)]
    definition: FunctionDefinition,
}

impl ToNumbas<numbas::question::function::Function> for Function {
    fn to_numbas(&self, locale: &str) -> numbas::question::function::Function {
        numbas::question::function::Function {
            parameters: self.parameters.to_numbas(locale),
            output_type: self.output_type.to_numbas(locale),
            definition: self.definition.to_numbas(&locale),
        }
    }
}

impl ToRumbas<Function> for numbas::question::function::Function {
    fn to_rumbas(&self) -> Function {
        Function {
            definition: self.definition.to_rumbas(),
            output_type: self.output_type,
            parameters: self.parameters.clone().into_iter().collect(),
        }
    }
}

#[derive(Input, Overwrite, RumbasCheck)]
#[input(name = "FunctionDefinitionInput")]
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(tag = "language")]
pub enum FunctionDefinition {
    #[serde(rename = "jme")]
    JME(FunctionDefinitionJME),
    #[serde(rename = "js")]
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

#[derive(Input, Overwrite, RumbasCheck)]
#[input(name = "FunctionDefinitionJMEInput")]
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct FunctionDefinitionJME {
    definition: JMETranslatableString,
}

#[derive(Input, Overwrite, RumbasCheck)]
#[input(name = "FunctionDefinitionJavascriptInput")]
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct FunctionDefinitionJavascript {
    definition: TranslatableString,
}
