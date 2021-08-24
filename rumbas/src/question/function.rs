use crate::support::optional_overwrite::*;
use crate::support::template::{Value, ValueType};
use crate::support::to_numbas::impl_to_numbas;
use crate::support::to_numbas::ToNumbas;
use crate::support::to_rumbas::ToRumbas;
use crate::support::translatable::{JMETranslatableString, TranslatableString};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

optional_overwrite! {
    pub struct Function {
        parameters: Vec<(String, numbas::question::function::FunctionType)>,
        output_type: numbas::question::function::FunctionType,
        #[serde(flatten)]
        definition: FunctionDefinition
    }
}
impl_optional_overwrite! {(String, numbas::question::function::FunctionType)}
impl_optional_overwrite!(numbas::question::function::FunctionType);

impl ToNumbas<numbas::question::function::Function> for Function {
    fn to_numbas(&self, locale: &str) -> numbas::question::function::Function {
        numbas::question::function::Function {
            parameters: self.parameters.to_numbas(locale),
            output_type: self.output_type.to_numbas(locale),
            definition: self.definition.to_numbas(&locale),
        }
    }
}
impl_to_numbas!(numbas::question::function::FunctionType);

impl ToRumbas<Function> for numbas::question::function::Function {
    fn to_rumbas(&self) -> Function {
        Function {
            definition: self.definition.to_rumbas(),
            output_type: Value::Normal(self.output_type),
            parameters: Value::Normal(self.parameters.clone().into_iter().collect()),
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
