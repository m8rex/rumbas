use crate::data::template::{Value, ValueType};
use crate::data::translatable::{JMETranslatableString, TranslatableString};
use crate::support::optional_overwrite::*;
use crate::support::to_numbas::impl_to_numbas;
use crate::support::to_numbas::ToNumbas;
use crate::support::to_rumbas::ToRumbas;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

optional_overwrite! {
    pub struct Function {
        parameters: Vec<(String, numbas::exam::ExamFunctionType)>,
        output_type: numbas::exam::ExamFunctionType,
        #[serde(flatten)]
        definition: FunctionDefinition
    }
}
impl_optional_overwrite! {(String, numbas::exam::ExamFunctionType)}

impl ToNumbas<numbas::exam::ExamFunction> for Function {
    fn to_numbas(&self, locale: &str) -> numbas::exam::ExamFunction {
        numbas::exam::ExamFunction {
            parameters: self.parameters.to_numbas(locale),
            output_type: self.output_type.to_numbas(locale),
            definition: self.definition.to_numbas(&locale),
        }
    }
}

impl ToRumbas<Function> for numbas::exam::ExamFunction {
    fn to_rumbas(&self) -> Function {
        Function {
            definition: Value::Normal(self.definition.to_rumbas()),
            output_type: Value::Normal(self.output_type),
            parameters: Value::Normal(self.parameters.clone().into_iter().collect()),
        }
    }
}

impl_to_numbas!(numbas::exam::ExamFunctionType);
impl_optional_overwrite!(numbas::exam::ExamFunctionType);

optional_overwrite_enum! {
    #[serde(tag = "language")]
    pub enum FunctionDefinition {
        #[serde(rename="jme")]
        JME(FunctionDefinitionJME),
        #[serde(rename="js")]
        Javascript(FunctionDefinitionJavascript)
    }
}

impl ToNumbas<numbas::exam::ExamFunctionDefinition> for FunctionDefinition {
    fn to_numbas(&self, locale: &str) -> numbas::exam::ExamFunctionDefinition {
        match self {
            FunctionDefinition::JME(c) => numbas::exam::ExamFunctionDefinition::JME {
                definition: c.definition.to_numbas(locale),
            },
            FunctionDefinition::Javascript(c) => numbas::exam::ExamFunctionDefinition::Javascript {
                definition: c.definition.to_numbas(locale),
            },
        }
    }
}

impl ToRumbas<FunctionDefinition> for numbas::exam::ExamFunctionDefinition {
    fn to_rumbas(&self) -> FunctionDefinition {
        match self {
            numbas::exam::ExamFunctionDefinition::JME { definition } => {
                FunctionDefinition::JME(FunctionDefinitionJME {
                    definition: Value::Normal(definition.to_rumbas()),
                })
            }
            numbas::exam::ExamFunctionDefinition::Javascript { definition } => {
                FunctionDefinition::Javascript(FunctionDefinitionJavascript {
                    definition: Value::Normal(definition.to_rumbas()),
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
