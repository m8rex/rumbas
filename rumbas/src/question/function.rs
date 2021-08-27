use crate::support::optional_overwrite::*;
use crate::support::template::Value;
use crate::support::to_numbas::impl_to_numbas;
use crate::support::to_numbas::ToNumbas;
use crate::support::to_rumbas::ToRumbas;
use crate::support::translatable::{JMETranslatableString, TranslatableString};
use crate::support::translatable::{JMETranslatableStringInput, TranslatableStringInput};
use numbas::question::function::FunctionType as NumbasFunctionType;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

// TODO: don't directly use numbas type
type StringFunctionTypeTuple = (String, NumbasFunctionType);
type StringFunctionTypeTuples = Vec<StringFunctionTypeTuple>;
type StringFunctionTypeTuplesInput = Vec<Value<StringFunctionTypeTuple>>;

impl_to_numbas!(NumbasFunctionType);

optional_overwrite! {
    pub struct Function {
        parameters: StringFunctionTypeTuples,
        output_type: NumbasFunctionType,
        #[serde(flatten)]
        definition: FunctionDefinition
    }
}
impl_optional_overwrite!(NumbasFunctionType);
impl_optional_overwrite! {StringFunctionTypeTuple}

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

optional_overwrite_enum! {
    #[serde(tag = "language")]
    pub enum FunctionDefinition {
        #[serde(rename="jme")]
        JME(FunctionDefinitionJME),
        #[serde(rename="js")]
        Javascript(FunctionDefinitionJavascript)
    }
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

optional_overwrite! {
    pub struct FunctionDefinitionJME {
        definition: JMETranslatableString
    }
}

optional_overwrite! {
    pub struct FunctionDefinitionJavascript {
        definition: TranslatableString
    }
}
