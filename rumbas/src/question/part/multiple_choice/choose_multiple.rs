use crate::question::part::multiple_choice::{
    extract_multiple_choice_answer_data, MultipleChoiceAnswerData, MultipleChoiceMarkingMethod,
};
use crate::question::part::question_part::JMENotes;
use crate::question::part::question_part::VariableReplacementStrategy;
use crate::question::QuestionPart;
use crate::support::noneable::Noneable;
use crate::support::to_numbas::ToNumbas;
use crate::support::to_rumbas::*;
use crate::support::translatable::ContentAreaTranslatableString;
use crate::support::variable_valued::VariableValued;
use comparable::Comparable;
use rumbas_support::preamble::*;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::convert::Into;

question_part_type! {
    #[derive(Input, Overwrite, RumbasCheck, Examples)]
    #[input(name = "QuestionPartChooseMultipleInput")]
    #[derive(Serialize, Deserialize, Comparable, Debug, Clone, JsonSchema, PartialEq)]
    pub struct QuestionPartChooseMultiple {
        /// Old name was `answers`
        #[serde(alias = "answers")]
        answer_data: MultipleChoiceAnswerData,
        shuffle_answers: bool,
        show_cell_answer_state: bool,
        should_select_at_least: usize,
        should_select_at_most: Noneable<usize>,
        columns: usize,
        /// What to do if the student picks the wrong number of responses? Either "none" (do nothing), "prevent" (don’t let the student submit), or "warn" (show a warning but let them submit)
        wrong_nb_answers_warning_type: crate::question::part::multiple_choice::match_answers::MultipleChoiceWarningType,
        /// If the student would have scored less than this many marks, they are instead awarded this many. Useful in combination with negative marking.
        minimal_achievable_marks: Noneable<usize>,
        /// If the student would have scored more than this many marks, they are instead awarded this many. The value 0 means “no maximum mark”.
        maximal_achievable_marks: Noneable<usize>,

        marking_method: MultipleChoiceMarkingMethod
        //TODO other?
    }
}

impl ToNumbas<numbas::question::part::choose_multiple::QuestionPartChooseMultiple>
    for QuestionPartChooseMultiple
{
    fn to_numbas(
        &self,
        locale: &str,
    ) -> numbas::question::part::choose_multiple::QuestionPartChooseMultiple {
        // TODO: below is duplicated in CHooseOne
        let (choices, marking_matrix, distractors) = match &self.answer_data {
            MultipleChoiceAnswerData::ItemBased(answers) => (
                VariableValued::Value(
                    answers
                        .iter()
                        .map(|a| a.statement.clone())
                        .collect::<Vec<_>>(),
                )
                .to_numbas(locale),
                VariableValued::Value(answers.iter().map(|a| a.marks.clone()).collect::<Vec<_>>())
                    .to_numbas(locale),
                answers
                    .iter()
                    .map(|a| {
                        a.feedback.clone() //TODO
                    })
                    .collect::<Vec<_>>()
                    .to_numbas(locale),
            ),
            MultipleChoiceAnswerData::NumbasLike(data) => (
                data.answers.to_numbas(locale),
                data.marks.to_numbas(locale),
                data.feedback
                    .clone()
                    .map(|f| f.to_numbas(locale))
                    .unwrap_or_default(),
            ),
        };
        numbas::question::part::choose_multiple::QuestionPartChooseMultiple {
            part_data: self.to_numbas(locale),
            min_answers: self.should_select_at_least.into(),
            max_answers: self
                .should_select_at_most
                .to_numbas(locale)
                .unwrap_or(0.into()),
            min_marks: self
                .minimal_achievable_marks
                .to_numbas(locale)
                .unwrap_or(0.into()),
            max_marks: self
                .maximal_achievable_marks
                .to_numbas(locale)
                .unwrap_or(0.into()),
            shuffle_answers: self.shuffle_answers.to_numbas(locale),
            choices,
            display_columns: self.columns.to_numbas(locale),
            show_cell_answer_state: self.show_cell_answer_state.to_numbas(locale),
            wrong_nb_answers_warning: self.wrong_nb_answers_warning_type.to_numbas(locale),
            marking_matrix,
            distractors,
            marking_method: self.marking_method.to_numbas(locale),
        }
    }
}

impl ToRumbas<QuestionPartChooseMultiple>
    for numbas::question::part::choose_multiple::QuestionPartChooseMultiple
{
    fn to_rumbas(&self) -> QuestionPartChooseMultiple {
        create_question_part! {
            QuestionPartChooseMultiple with &self.part_data => {
                answer_data: self.to_rumbas(),
                shuffle_answers: self.shuffle_answers.to_rumbas(),
                show_cell_answer_state: self.show_cell_answer_state.to_rumbas(),
                should_select_at_least: self
                    .min_answers
                    .0
                    .to_rumbas(),
                should_select_at_most:
                    Noneable::NotNone(self.max_answers
                        .0.to_rumbas()),
                columns: self.display_columns.0.to_rumbas(),
                wrong_nb_answers_warning_type: self.wrong_nb_answers_warning.to_rumbas(),
                marking_method: self.marking_method.to_rumbas(),
                minimal_achievable_marks: Noneable::NotNone(self.min_marks.0.to_rumbas()),
                maximal_achievable_marks: Noneable::NotNone(self.max_marks.0.to_rumbas())
            }
        }
    }
}

impl ToRumbas<MultipleChoiceAnswerData>
    for numbas::question::part::choose_multiple::QuestionPartChooseMultiple
{
    fn to_rumbas(&self) -> MultipleChoiceAnswerData {
        extract_multiple_choice_answer_data(&self.choices, &self.marking_matrix, &self.distractors)
    }
}
