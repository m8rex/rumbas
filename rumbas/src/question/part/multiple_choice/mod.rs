use crate::support::noneable::Noneable;
use crate::support::to_numbas::ToNumbas;
use crate::support::to_rumbas::*;
use crate::support::translatable::ContentAreaTranslatableString;
use crate::support::translatable::EmbracedJMETranslatableString;
use crate::support::translatable::JMETranslatableString;
use crate::support::variable_valued::VariableValued;
use rumbas_support::preamble::*;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use comparable::Comparable;
use std::convert::Into;

pub mod choose_multiple;
pub mod choose_one;
pub mod match_answers;

#[derive(Input, Overwrite, RumbasCheck, Examples)]
#[input(name = "MultipleChoiceAnswerDataInput")]
#[derive(Serialize, Deserialize, Comparable, Debug, Clone, JsonSchema, PartialEq)]
#[serde(untagged)]
pub enum MultipleChoiceAnswerData {
    ItemBased(Vec<MultipleChoiceAnswer>),
    NumbasLike(Box<MultipleChoiceAnswerDataNumbasLike>),
}

#[derive(Input, Overwrite, RumbasCheck, Examples)]
#[input(name = "MultipleChoiceAnswerDataNumbasLikeInput")]
#[derive(Serialize, Deserialize, Comparable, Debug, Clone, JsonSchema, PartialEq)]
pub struct MultipleChoiceAnswerDataNumbasLike {
    pub answers: VariableValued<Vec<ContentAreaTranslatableString>>,
    pub marks: VariableValued<Vec<JMETranslatableString>>,
    pub feedback: Noneable<Vec<EmbracedJMETranslatableString>>,
}

fn extract_multiple_choice_answer_data(
    answers: &numbas::support::primitive::VariableValued<Vec<numbas::jme::ContentAreaString>>,
    marking_matrix: &Option<
        numbas::support::primitive::VariableValued<Vec<numbas::jme::JMEString>>,
    >,
    distractors: &Option<Vec<numbas::jme::EmbracedJMEString>>,
) -> MultipleChoiceAnswerData {
    if let (
        numbas::support::primitive::VariableValued::Value(answer_options),
        Some(numbas::support::primitive::VariableValued::Value(marking_matrix)),
    ) = (answers.clone(), marking_matrix.clone())
    {
        let answers_data: Vec<_> = match distractors.clone() {
            Some(d) => answer_options
                .into_iter()
                .zip(marking_matrix.into_iter())
                .zip(d.into_iter())
                .map(|((a, b), c)| (a, b, c))
                .collect(),
            None => answer_options
                .into_iter()
                .zip(marking_matrix.into_iter())
                .map(|(a, b)| (a, b, numbas::jme::EmbracedJMEString::new()))
                .collect(),
        };
        MultipleChoiceAnswerData::ItemBased(
            answers_data
                .into_iter()
                .map(|(statement, marks, feedback)| MultipleChoiceAnswer {
                    statement: statement.into(),
                    marks: marks.into(),
                    feedback: feedback.into(),
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

#[derive(Input, Overwrite, RumbasCheck, Examples)]
#[input(name = "MultipleChoiceAnswerInput")]
#[derive(Serialize, Deserialize, Comparable, Debug, Clone, JsonSchema, PartialEq)]
pub struct MultipleChoiceAnswer {
    pub statement: ContentAreaTranslatableString,
    pub feedback: EmbracedJMETranslatableString,
    pub marks: JMETranslatableString,
}

#[derive(Input, Overwrite, RumbasCheck, Examples)]
#[input(name = "MultipleChoiceMarkingMethodInput")]
#[derive(Serialize, Deserialize, Comparable, JsonSchema, Debug, Copy, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum MultipleChoiceMarkingMethod {
    SumTickedCells,
    ScorePerMatchedCell,
    AllOrNothing,
}

impl ToRumbas<MultipleChoiceMarkingMethod>
    for numbas::question::part::choose_multiple::MultipleChoiceMarkingMethod
{
    fn to_rumbas(&self) -> MultipleChoiceMarkingMethod {
        match self {
            numbas::question::part::choose_multiple::MultipleChoiceMarkingMethod::SumTickedCells => MultipleChoiceMarkingMethod::SumTickedCells,
            numbas::question::part::choose_multiple::MultipleChoiceMarkingMethod::ScorePerMatchedCell => MultipleChoiceMarkingMethod::ScorePerMatchedCell,
            numbas::question::part::choose_multiple::MultipleChoiceMarkingMethod::AllOrNothing => MultipleChoiceMarkingMethod::AllOrNothing,
        }
    }
}

impl ToNumbas<numbas::question::part::choose_multiple::MultipleChoiceMarkingMethod>
    for MultipleChoiceMarkingMethod
{
    fn to_numbas(
        &self,
        _locale: &str,
    ) -> numbas::question::part::choose_multiple::MultipleChoiceMarkingMethod {
        match self {
            MultipleChoiceMarkingMethod::SumTickedCells => numbas::question::part::choose_multiple::MultipleChoiceMarkingMethod::SumTickedCells,
            MultipleChoiceMarkingMethod::ScorePerMatchedCell => numbas::question::part::choose_multiple::MultipleChoiceMarkingMethod::ScorePerMatchedCell,
            MultipleChoiceMarkingMethod::AllOrNothing => numbas::question::part::choose_multiple::MultipleChoiceMarkingMethod::AllOrNothing,
        }
    }
}
