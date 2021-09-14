use crate::support::noneable::Noneable;
use crate::support::to_numbas::ToNumbas;
use crate::support::to_numbas::*;
use crate::support::to_rumbas::*;
use crate::support::translatable::TranslatableString;
use crate::support::translatable::TranslatableStrings;
use crate::support::variable_valued::VariableValued;
use numbas::support::primitive::Primitive;
use rumbas_support::preamble::*;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::convert::Into;

pub mod choose_multiple;
pub mod choose_one;
pub mod match_answers;

#[derive(Input, Overwrite, RumbasCheck)]
#[input(name = "MultipleChoiceAnswerDataInput")]
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(untagged)]
pub enum MultipleChoiceAnswerData {
    ItemBased(Vec<MultipleChoiceAnswer>),
    NumbasLike(Box<MultipleChoiceAnswerDataNumbasLike>),
}

#[derive(Input, Overwrite, RumbasCheck)]
#[input(name = "MultipleChoiceAnswerDataNumbasLikeInput")]
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct MultipleChoiceAnswerDataNumbasLike {
    pub answers: VariableValued<Vec<TranslatableString>>,
    pub marks: VariableValued<Vec<Primitive>>,
    pub feedback: Noneable<Vec<TranslatableString>>,
}

#[derive(Input, Overwrite, RumbasCheck)]
#[input(name = "MatrixRowPrimitiveInput")]
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct MatrixRowPrimitive(pub Vec<Primitive>);

impl ToNumbas<numbas::question::part::match_answers::MultipleChoiceMatrix> for MatrixRowPrimitive {
    fn to_numbas(
        &self,
        _locale: &str,
    ) -> numbas::question::part::match_answers::MultipleChoiceMatrix {
        numbas::question::part::match_answers::MultipleChoiceMatrix::Row(self.0.clone())
    }
}

#[derive(Input, Overwrite, RumbasCheck)]
#[input(name = "MatrixRowInput")]
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct MatrixRow(pub TranslatableStrings);

impl ToNumbas<numbas::question::part::match_answers::MultipleChoiceMatrix> for MatrixRow {
    fn to_numbas(
        &self,
        locale: &str,
    ) -> numbas::question::part::match_answers::MultipleChoiceMatrix {
        numbas::question::part::match_answers::MultipleChoiceMatrix::Row(
            self.0
                .to_numbas(locale)
                .into_iter()
                .map(|a| a.into())
                .collect(),
        )
    }
}

#[derive(Input, Overwrite, RumbasCheck)]
#[input(name = "MatrixPrimitiveInput")]
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct MatrixPrimitive(pub Vec<VariableValued<Vec<numbas::support::primitive::Primitive>>>);

impl ToNumbas<numbas::question::part::match_answers::MultipleChoiceMatrix> for MatrixPrimitive {
    fn to_numbas(
        &self,
        locale: &str,
    ) -> numbas::question::part::match_answers::MultipleChoiceMatrix {
        numbas::question::part::match_answers::MultipleChoiceMatrix::Matrix(
            self.0.to_numbas(locale),
        )
    }
}

fn extract_multiple_choice_answer_data(
    answers: &numbas::support::primitive::VariableValued<Vec<String>>,
    marking_matrix: &Option<
        numbas::support::primitive::VariableValued<Vec<numbas::support::primitive::Primitive>>,
    >,
    distractors: &Option<Vec<String>>,
) -> MultipleChoiceAnswerData {
    if let (
        numbas::support::primitive::VariableValued::Value(answer_options),
        Some(numbas::support::primitive::VariableValued::Value(marking_matrix)),
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
                    statement: a.into(),
                    marks: b,
                    feedback: c.into(),
                })
                .collect(),
        )
    } else {
        MultipleChoiceAnswerData::NumbasLike(Box::new(MultipleChoiceAnswerDataNumbasLike {
            answers: answers.to_rumbas(),

            marks: marking_matrix
                .clone()
                .map(|m| m.to_rumbas())
                .expect("How can the marking matrix be optional?"),

            feedback: distractors
                .clone()
                .map(|v| Noneable::NotNone(v.to_rumbas()))
                .unwrap_or(Noneable::None),
        }))
    }
}

impl_to_numbas!(numbas::question::part::match_answers::MultipleChoiceMatrix);

impl_to_numbas!(Primitive);

#[derive(Input, Overwrite, RumbasCheck)]
#[input(name = "MultipleChoiceAnswerInput")]
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct MultipleChoiceAnswer {
    pub statement: TranslatableString,
    pub feedback: TranslatableString,
    pub marks: Primitive, // TODO: variable valued?
}
