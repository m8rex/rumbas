use crate::support::noneable::Noneable;
use crate::support::to_numbas::ToNumbas;
use crate::support::to_rumbas::*;
use crate::support::translatable::ContentAreaTranslatableString;
use crate::support::translatable::JMETranslatableString;
use crate::support::variable_valued::VariableValued;
use comparable::Comparable;
use rumbas_support::preamble::*;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::convert::Into;
use structdoc::StructDoc;

pub mod choose_multiple;
pub mod choose_one;
pub mod match_answers;

#[derive(Input, Overwrite, RumbasCheck, Examples, StructDoc)]
#[input(name = "MultipleChoiceAnswerDataInput")]
#[derive(Serialize, Deserialize, Comparable, Debug, Clone, JsonSchema, PartialEq, Eq)]
#[serde(untagged)]
pub enum MultipleChoiceAnswerData {
    /// Specify a list of answer with it's marks and feedback
    ItemBased(Vec<MultipleChoiceAnswer>),
    /// Specify the answers, marks and feedback as separate lists.
    /// The first answers, matches the first mark and the first feedback etc
    NumbasLike(Box<MultipleChoiceAnswerDataNumbasLike>),
}

#[derive(Input, Overwrite, RumbasCheck, Examples, StructDoc)]
#[input(name = "MultipleChoiceAnswerDataNumbasLikeInput")]
#[derive(Serialize, Deserialize, Comparable, Debug, Clone, JsonSchema, PartialEq, Eq)]
pub struct MultipleChoiceAnswerDataNumbasLike {
    /// The possible answers
    pub answers: VariableValued<Vec<ContentAreaTranslatableString>>,
    /// The marks for the corresponding answers
    pub marks: VariableValued<Vec<JMETranslatableString>>,
    /// The feedback for the corresponding answers.
    pub feedback: Noneable<Vec<ContentAreaTranslatableString>>,
}

fn extract_multiple_choice_answer_data(
    answers: &numbas::support::primitive::VariableValued<Vec<numbas::jme::ContentAreaString>>,
    marking_matrix: &numbas::support::primitive::VariableValued<Vec<numbas::jme::JMEString>>,
    distractors: &Vec<numbas::jme::ContentAreaString>,
) -> MultipleChoiceAnswerData {
    if let (
        numbas::support::primitive::VariableValued::Value(answer_options),
        numbas::support::primitive::VariableValued::Value(marking_matrix),
    ) = (answers.clone(), marking_matrix.clone())
    {
        let answers_data: Vec<_> = if distractors.is_empty() {
            answer_options
                .into_iter()
                .zip(marking_matrix.into_iter())
                .map(|(a, b)| (a, b, numbas::jme::ContentAreaString::default()))
                .collect()
        } else {
            answer_options
                .into_iter()
                .zip(marking_matrix.into_iter())
                .zip(distractors.clone().into_iter())
                .map(|((a, b), c)| (a, b, c))
                .collect()
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

            marks: marking_matrix.to_rumbas(),

            feedback: if distractors.is_empty() {
                Noneable::None
            } else {
                Noneable::NotNone(distractors.to_rumbas())
            },
        }))
    }
}

#[derive(Input, Overwrite, RumbasCheck, Examples, StructDoc)]
#[input(name = "MultipleChoiceAnswerInput")]
#[derive(Serialize, Deserialize, Comparable, Debug, Clone, JsonSchema, PartialEq, Eq)]
pub struct MultipleChoiceAnswer {
    /// The statement of the answer
    pub statement: ContentAreaTranslatableString,
    /// The feedback shown when this answer is chosen
    pub feedback: ContentAreaTranslatableString,
    /// The marks to assign when this answer is chosen
    pub marks: JMETranslatableString,
}

#[derive(Input, Overwrite, RumbasCheck, Examples, StructDoc)]
#[input(name = "MultipleChoiceMarkingMethodInput")]
#[derive(Serialize, Deserialize, Comparable, JsonSchema, Debug, Copy, Clone, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum MultipleChoiceMarkingMethod {
    /// For each checkbox the student ticks, the corresponding entry in the marking matrix is added to their score. Unticked cells are ignored.
    ///
    /// This marking method is suitable for situations where the student should only select choices they’re sure about. You could apply negative marks for incorrect choices.
    SumTickedCells,
    /// For each checkbox, the student is awarded an equal proportion of the Maximum marks, if their selection for that cell matches the marking matrix. A positive value in the marking matrix signifies that the student should tick that checkbox, while a value of zero signifies that the student should not tick that box.
    ///
    /// This marking method is suitable for situations where the student must separate the available choices into two sets.
    ScorePerMatchedCell,
    /// the student is awarded the Maximum marks available if their selection exactly matches the marking matrix, and zero marks otherwise.
    ///
    /// This marking method is suitable for situations where the student must exactly match a certain pattern, and there is no meaningful “partially correct” answer.
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
    type ToNumbasHelper = ();
    fn to_numbas(
        &self,
        _locale: &str,
        _data: &Self::ToNumbasHelper
    ) -> numbas::question::part::choose_multiple::MultipleChoiceMarkingMethod {
        match self {
            MultipleChoiceMarkingMethod::SumTickedCells => numbas::question::part::choose_multiple::MultipleChoiceMarkingMethod::SumTickedCells,
            MultipleChoiceMarkingMethod::ScorePerMatchedCell => numbas::question::part::choose_multiple::MultipleChoiceMarkingMethod::ScorePerMatchedCell,
            MultipleChoiceMarkingMethod::AllOrNothing => numbas::question::part::choose_multiple::MultipleChoiceMarkingMethod::AllOrNothing,
        }
    }
}
