use crate::question::part::multiple_choice::{
    extract_multiple_choice_answer_data, MultipleChoiceAnswerData,
};
use crate::question::part::question_part::JMENotes;
use crate::question::part::question_part::VariableReplacementStrategy;
use crate::question::QuestionParts;
use crate::support::rumbas_types::*;
use crate::support::to_numbas::ToNumbas;
use crate::support::to_numbas::*;
use crate::support::to_rumbas::*;
use crate::support::translatable::ContentAreaTranslatableString;
use crate::support::variable_valued::VariableValued;
use numbas::defaults::DEFAULTS;
use numbas::support::primitive::Primitive;
use rumbas_support::preamble::*;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::convert::Into;

question_part_type! {
    #[derive(Input, Overwrite, RumbasCheck)]
    #[input(name = "QuestionPartChooseMultipleInput")]
    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct QuestionPartChooseMultiple {
        /// Old name was `answers`
        #[serde(alias = "answers")]
        answer_data: MultipleChoiceAnswerData,
        shuffle_answers: RumbasBool,
        show_cell_answer_state: RumbasBool,
        should_select_at_least: RumbasNatural,
        should_select_at_most: NoneableNatural,
        columns: RumbasNatural,
        /// What to do if the student picks the wrong number of responses? Either "none" (do nothing), "prevent" (don’t let the student submit), or "warn" (show a warning but let them submit)
        wrong_nb_answers_warning_type:  MultipleChoiceWarningType
        //min_marks & max_marks?
        //TODO other?
    }
}
type MultipleChoiceWarningType = numbas::question::part::match_answers::MultipleChoiceWarningType;
type MultipleChoiceWarningTypeInput = MultipleChoiceWarningType;
impl_to_numbas!(numbas::question::part::match_answers::MultipleChoiceWarningType);
impl_to_rumbas!(numbas::question::part::match_answers::MultipleChoiceWarningType);

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
                Some(
                    VariableValued::Value(
                        answers.iter().map(|a| a.marks.clone()).collect::<Vec<_>>(),
                    )
                    .to_numbas(locale),
                ),
                Some(
                    answers
                        .iter()
                        .map(|a| {
                            a.feedback.clone() //TODO
                        })
                        .collect::<Vec<_>>()
                        .to_numbas(locale),
                ),
            ),
            MultipleChoiceAnswerData::NumbasLike(data) => (
                data.answers.to_numbas(locale),
                Some(data.marks.to_numbas(locale)),
                data.feedback.clone().map(|f| f.to_numbas(locale)).into(),
            ),
        };
        numbas::question::part::choose_multiple::QuestionPartChooseMultiple {
            part_data: self.to_numbas(locale),
            min_answers: Some(self.should_select_at_least.to_numbas(locale)),
            max_answers: self.should_select_at_most.to_numbas(locale),
            min_marks: Some(0usize), // todo?
            max_marks: Some(0usize.into()),
            shuffle_answers: self.shuffle_answers.to_numbas(locale),
            choices,
            display_columns: self.columns.to_numbas(locale),
            wrong_nb_choices_warning: self.wrong_nb_answers_warning_type.to_numbas(locale),
            show_cell_answer_state: self.show_cell_answer_state.to_numbas(locale),
            marking_matrix,
            distractors,
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
                    .unwrap_or(DEFAULTS.choose_multiple_min_answers)
                    .0
                    .to_rumbas(),
                should_select_at_most:
                    self.max_answers
                        .map(|v| v.0).to_rumbas(),
                columns: self.display_columns.0.to_rumbas(),
                wrong_nb_answers_warning_type: self.wrong_nb_choices_warning.to_rumbas()
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
