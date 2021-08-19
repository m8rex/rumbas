use crate::support::template::{Value, ValueType};
use crate::data::translatable::TranslatableString;
use crate::support::optional_overwrite::*;
use crate::support::to_numbas::ToNumbas;
use crate::support::to_numbas::*;
use crate::support::to_rumbas::*;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::convert::Into;

pub mod choose_multiple;
pub mod choose_one;
pub mod match_answers;

optional_overwrite_enum! {
    #[serde(untagged)]
    pub enum MultipleChoiceAnswerData {
        ItemBased(Vec<MultipleChoiceAnswer>),
        NumbasLike(Box<MultipleChoiceAnswerDataNumbasLike>)
    }
}

optional_overwrite! {
    pub struct MultipleChoiceAnswerDataNumbasLike {
        answers: VariableValued<Vec<TranslatableString>>,
        marks: VariableValued<Vec<numbas::exam::Primitive>>,
        feedback: Noneable<Vec<TranslatableString>>
    }
}

#[derive(Debug, Deserialize, JsonSchema, Clone, PartialEq)]
struct MatrixRowPrimitive(Vec<numbas::exam::Primitive>);
impl_optional_overwrite!(MatrixRowPrimitive); // TODO: Does this do what it needs to do?

impl ToNumbas<numbas::exam::MultipleChoiceMatrix> for MatrixRowPrimitive {
    fn to_numbas(&self, _locale: &str) -> numbas::exam::MultipleChoiceMatrix {
        numbas::exam::MultipleChoiceMatrix::Row(self.0.clone())
    }
}

#[derive(Debug, Deserialize, JsonSchema, Clone, PartialEq)]
struct MatrixRow(Vec<TranslatableString>);
impl_optional_overwrite!(MatrixRow); // TODO: Does this do what it needs to do?

impl ToNumbas<numbas::exam::MultipleChoiceMatrix> for MatrixRow {
    fn to_numbas(&self, locale: &str) -> numbas::exam::MultipleChoiceMatrix {
        numbas::exam::MultipleChoiceMatrix::Row(
            self.0
                .to_numbas(locale)
                .into_iter()
                .map(|a| a.into())
                .collect(),
        )
    }
}

#[derive(Debug, Deserialize, JsonSchema, Clone, PartialEq)]
struct MatrixPrimitive(Vec<VariableValued<Vec<numbas::exam::Primitive>>>);
impl_optional_overwrite!(MatrixPrimitive); // TODO: Does this do what it needs to do?

impl ToNumbas<numbas::exam::MultipleChoiceMatrix> for MatrixPrimitive {
    fn to_numbas(&self, locale: &str) -> numbas::exam::MultipleChoiceMatrix {
        numbas::exam::MultipleChoiceMatrix::Matrix(self.0.to_numbas(locale))
    }
}

fn extract_multiple_choice_answer_data(
    answers: &numbas::exam::VariableValued<Vec<String>>,
    marking_matrix: &Option<numbas::exam::VariableValued<Vec<numbas::exam::Primitive>>>,
    distractors: &Option<Vec<String>>,
) -> MultipleChoiceAnswerData {
    if let (
        numbas::exam::VariableValued::Value(answer_options),
        Some(numbas::exam::VariableValued::Value(marking_matrix)),
    ) = (answers.clone(), marking_matrix.clone())
    {
        let answers_data: Vec<_> = match distractors.clone() {
            None => answer_options
                .into_iter()
                .zip(marking_matrix.into_iter())
                .map(|(a, b)| (a, b, "".to_string()))
                .collect(),
            Some(d) => answer_options
                .into_iter()
                .zip(marking_matrix.into_iter())
                .zip(d.into_iter())
                .map(|((a, b), c)| (a, b, c))
                .collect(),
        };
        MultipleChoiceAnswerData::ItemBased(
            answers_data
                .into_iter()
                .map(|(a, b, c)| MultipleChoiceAnswer {
                    statement: Value::Normal(a.into()),
                    marks: Value::Normal(b),
                    feedback: Value::Normal(c.into()),
                })
                .collect(),
        )
    } else {
        MultipleChoiceAnswerData::NumbasLike(Box::new(MultipleChoiceAnswerDataNumbasLike {
            answers: Value::Normal(
                answers
                    .clone()
                    .map(|v| {
                        v.iter()
                            .map(|vv| vv.clone().into())
                            .collect::<Vec<TranslatableString>>()
                    })
                    .to_rumbas(),
            ),
            marks: Value::Normal(
                marking_matrix
                    .clone()
                    .map(|m| m.to_rumbas())
                    .expect("How can the marking matrix be optional?"),
            ),
            feedback: Value::Normal(
                distractors
                    .clone()
                    .map(|v| {
                        Noneable::NotNone(
                            v.into_iter()
                                .map(|f| f.into())
                                .collect::<Vec<TranslatableString>>()
                                .to_rumbas(),
                        )
                    })
                    .unwrap_or_else(Noneable::nn),
            ),
        }))
    }
}

impl_to_numbas!(numbas::exam::MultipleChoiceMatrix);
impl_optional_overwrite!(numbas::exam::MultipleChoiceMatrix);

impl_to_numbas!(numbas::exam::Primitive);
impl_optional_overwrite!(numbas::exam::Primitive);

optional_overwrite! {
    pub struct MultipleChoiceAnswer {
        statement: TranslatableString,
        feedback: TranslatableString,
        marks: numbas::exam::Primitive // TODO: variable valued?
    }
}
