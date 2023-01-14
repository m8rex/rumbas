use crate::question::part::multiple_choice::MultipleChoiceMarkingMethod;
use crate::question::part::question_part::JMENotes;
use crate::question::part::question_part::VariableReplacementStrategy;
use crate::question::part::question_part::{AdaptiveMarking, CustomMarking};
use crate::question::QuestionPart;
use crate::support::noneable::Noneable;
use crate::support::to_numbas::ToNumbas;
use crate::support::to_rumbas::*;
use crate::support::translatable::ContentAreaTranslatableString;
use crate::support::variable_valued::VariableValued;
use comparable::Comparable;
use numbas::jme::JMEString;
use rumbas_support::preamble::*;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::convert::Into;
use std::convert::TryInto;
use structdoc::StructDoc;

question_part_type! {
    #[derive(Input, Overwrite, RumbasCheck, Examples, StructDoc)]
    #[input(name = "QuestionPartMatchAnswersWithItemsInput")]
    #[derive(Serialize, Deserialize, Comparable, Debug, Clone, JsonSchema, PartialEq)]
    pub struct QuestionPartMatchAnswersWithItems {
        /// Specify the options, score per option and feedback per option.
        /// Old name was `answers`
        #[serde(alias = "answers")]
        answer_data: MultipleChoiceMatchAnswerData,
        /// If this is ticked, the choices are displayed in random order.
        shuffle_answers: bool,
        /// If this is ticked, the items (horizontal) are displayed in random order.
        shuffle_items: bool,
        /// If ticked, choices selected by the student will be highlighted as ‘correct’ if they have a positive score, and ‘incorrect’ if they are worth zero or negative marks. If this is not ticked, the ticked choices will be given a neutral highlight regardless of their scores.
        show_cell_answer_state: bool,
        /// The student must select at least this many choices. The value 0 means “no minimum”, though the student must make at least one choice to submit the part.
        should_select_at_least: usize,
        /// The student must select at most this many choices. The value 0 means “no maximum”.
        should_select_at_most: Noneable<usize>,
        /// How should the options be shown?
        display: MatchAnswerWithItemsDisplay,
        /// How should the options be shown?
        layout: MatchAnswersWithChoicesLayout,
        /// What to do if the student picks the wrong number of responses? Either "none" (do nothing), "prevent" (don’t let the student submit), or "warn" (show a warning but let them submit)
        wrong_nb_answers_warning_type: MultipleChoiceWarningType,
        /// If the student would have scored less than this many marks, they are instead awarded this many. Useful in combination with negative marking.
        minimal_achievable_marks: Noneable<usize>,
        /// If the student would have scored more than this many marks, they are instead awarded this many. The value 0 means “no maximum mark”.
        maximal_achievable_marks: Noneable<usize>

        //TODO other?
    }
}

impl ToNumbas<numbas::question::part::match_answers::QuestionPartMatchAnswersWithChoices>
    for QuestionPartMatchAnswersWithItems
{
    type ToNumbasHelper = ();
    fn to_numbas(
        &self,
        locale: &str,
        _data: &Self::ToNumbasHelper
    ) -> numbas::question::part::match_answers::QuestionPartMatchAnswersWithChoices {
        let (answers, choices, marking_matrix) = match &self.answer_data {
            MultipleChoiceMatchAnswerData::ItemBased(data) => (
                VariableValued::Value(data.answers.clone()).to_numbas(locale, &()),
                VariableValued::Value(
                    data.items
                        .iter()
                        .map(|a| a.clone().statement)
                        .collect::<Vec<_>>(),
                )
                .to_numbas(locale, &()),
                VariableValued::Value(
                    data.items.clone(), // TODO: better handling
                )
                .map(|v| {
                    v.iter()
                        .map(|i| {
                            data.answers
                                .iter()
                                .map(|a| {
                                    i.answer_marks
                                        .iter()
                                        .find(|am| &am.answer == a)
                                        .map_or_else(
                                            || "0".to_string().try_into().unwrap(),
                                            |v| v.marks.clone(),
                                        )
                                })
                                .collect::<Vec<_>>()
                        })
                        .collect::<Vec<_>>()
                })
                .to_numbas(locale, &()),
            ),
            MultipleChoiceMatchAnswerData::NumbasLike(data) => (
                data.answers.to_numbas(locale, &()),
                data.choices.to_numbas(locale, &()),
                data.marks.to_numbas(locale, &()),
            ),
        };
        numbas::question::part::match_answers::QuestionPartMatchAnswersWithChoices {
            part_data: self.to_numbas(locale, &()),
            min_answers: self.should_select_at_least.into(),
            max_answers: self
                .should_select_at_most
                .to_numbas(locale, &())
                .unwrap_or_default(),
            min_marks: self
                .minimal_achievable_marks
                .to_numbas(locale, &())
                .unwrap_or_default(),
            max_marks: self
                .maximal_achievable_marks
                .to_numbas(locale, &())
                .unwrap_or_default(),
            shuffle_answers: self.shuffle_answers.to_numbas(locale, &()),
            shuffle_choices: self.shuffle_items.to_numbas(locale, &()),
            answers,
            choices,
            wrong_nb_answers_warning: self.wrong_nb_answers_warning_type.to_numbas(locale, &()),
            layout: self.layout.to_numbas(locale, &()),
            show_cell_answer_state: self.show_cell_answer_state.to_numbas(locale, &()),
            marking_matrix,
            display_type: self.display.to_numbas(locale, &()),
        }
    }
}

impl ToRumbas<QuestionPartMatchAnswersWithItems>
    for numbas::question::part::match_answers::QuestionPartMatchAnswersWithChoices
{
    fn to_rumbas(&self) -> QuestionPartMatchAnswersWithItems {
        create_question_part! {
            QuestionPartMatchAnswersWithItems with &self.part_data => {

                answer_data: self.to_rumbas(),
                shuffle_answers: self.shuffle_answers.to_rumbas(),
                shuffle_items: self.shuffle_choices.to_rumbas(),
                show_cell_answer_state: self.show_cell_answer_state.to_rumbas(),
                should_select_at_least:
                    self.min_answers.0.to_rumbas(),
                should_select_at_most: if self.max_answers.0 > 0 {
                    Noneable::NotNone(self.max_answers.0.to_rumbas())
                } else {
                    Noneable::None
                },
                display: self.display_type.to_rumbas(),
                layout: self.layout.to_rumbas(),
                wrong_nb_answers_warning_type: self.wrong_nb_answers_warning.to_rumbas(),
                minimal_achievable_marks: if self.min_marks.0 > 0 { Noneable::NotNone(self.min_marks.0.to_rumbas()) } else { Noneable::None },
                maximal_achievable_marks: if self.max_marks.0 > 0 { Noneable::NotNone(self.max_marks.0.to_rumbas()) } else { Noneable::None }
            }
        }
    }
}

impl ToRumbas<MultipleChoiceMatchAnswerData>
    for numbas::question::part::match_answers::QuestionPartMatchAnswersWithChoices
{
    fn to_rumbas(&self) -> MultipleChoiceMatchAnswerData {
        if let (
            numbas::support::primitive::VariableValued::Value(answer_options),
            numbas::support::primitive::VariableValued::Value(choice_options),
            numbas::support::primitive::VariableValued::Value(marking_matrix),
        ) = (
            self.answers.clone(),
            self.choices.clone(),
            self.marking_matrix.clone(),
        ) {
            let items_data = choice_options.into_iter().zip(marking_matrix.into_iter());

            MultipleChoiceMatchAnswerData::ItemBased({
                let answers: Vec<_> = answer_options.iter().map(|a| a.clone().into()).collect();
                MultipleChoiceMatchAnswers {
                    answers: answers.clone(),
                    items: items_data
                        .into_iter()
                        .map(|(statement, marks)| {
                            MatchAnswersItem {
                                // TODO: extract to ToRumbas?
                                statement: statement.into(),
                                answer_marks: marks
                                    .into_iter()
                                    .enumerate()
                                    .map(|(i, marks)| MatchAnswersItemMarks {
                                        marks,
                                        answer: answers.get(i).unwrap().clone(),
                                    })
                                    .collect(),
                            }
                        })
                        .collect(),
                }
            })
        } else {
            MultipleChoiceMatchAnswerData::NumbasLike(Box::new(
                MultipleChoiceMatchAnswerDataNumbasLike {
                    answers: self.answers.to_rumbas(),
                    choices: self.choices.to_rumbas(),

                    marks: self.marking_matrix.to_rumbas(),
                },
            ))
        }
    }
}

#[derive(Input, Overwrite, RumbasCheck, Examples, StructDoc)]
#[input(name = "MatchAnswerWithItemsDisplayInput")]
#[derive(Serialize, Deserialize, Comparable, JsonSchema, Debug, Clone, PartialEq, Eq)]
#[serde(tag = "type")]
pub enum MatchAnswerWithItemsDisplay {
    #[serde(rename = "radio")]
    /// One from each row
    Radio,
    #[serde(rename = "check")]
    /// Any number from each row
    Check(MatchAnswersWithChoicesDisplayCheck),
}

impl ToNumbas<numbas::question::part::match_answers::MatchAnswersWithChoicesDisplayType>
    for MatchAnswerWithItemsDisplay
{
    type ToNumbasHelper = ();
    fn to_numbas(
        &self,
        locale: &str,
        _data: &Self::ToNumbasHelper

    ) -> numbas::question::part::match_answers::MatchAnswersWithChoicesDisplayType {
        match self {
            MatchAnswerWithItemsDisplay::Check(c) => {
                numbas::question::part::match_answers::MatchAnswersWithChoicesDisplayType::Check(
                    c.to_numbas(locale, &()),
                )
            }
            MatchAnswerWithItemsDisplay::Radio => {
                numbas::question::part::match_answers::MatchAnswersWithChoicesDisplayType::Radio
            }
        }
    }
}

impl ToRumbas<MatchAnswerWithItemsDisplay>
    for numbas::question::part::match_answers::MatchAnswersWithChoicesDisplayType
{
    fn to_rumbas(&self) -> MatchAnswerWithItemsDisplay {
        match self {
            numbas::question::part::match_answers::MatchAnswersWithChoicesDisplayType::Check(c) => {
                MatchAnswerWithItemsDisplay::Check(c.to_rumbas())
            }
            numbas::question::part::match_answers::MatchAnswersWithChoicesDisplayType::Radio => {
                MatchAnswerWithItemsDisplay::Radio
            }
        }
    }
}

#[derive(Input, Overwrite, RumbasCheck, Examples, StructDoc)]
#[input(name = "MatchAnswersWithChoicesDisplayCheckInput")]
#[derive(Serialize, Deserialize, Comparable, JsonSchema, Debug, Clone, PartialEq, Eq)]
pub struct MatchAnswersWithChoicesDisplayCheck {
    /// The marking method to use
    marking_method: MultipleChoiceMarkingMethod,
}

impl ToNumbas<numbas::question::part::match_answers::MatchAnswersWithChoicesDisplayTypeCheck>
    for MatchAnswersWithChoicesDisplayCheck
{
    type ToNumbasHelper = ();
    fn to_numbas(
        &self,
        locale: &str,        _data: &Self::ToNumbasHelper

    ) -> numbas::question::part::match_answers::MatchAnswersWithChoicesDisplayTypeCheck {
        numbas::question::part::match_answers::MatchAnswersWithChoicesDisplayTypeCheck {
            marking_method: self.marking_method.to_numbas(locale, &()),
        }
    }
}

impl ToRumbas<MatchAnswersWithChoicesDisplayCheck>
    for numbas::question::part::match_answers::MatchAnswersWithChoicesDisplayTypeCheck
{
    fn to_rumbas(&self) -> MatchAnswersWithChoicesDisplayCheck {
        MatchAnswersWithChoicesDisplayCheck {
            marking_method: self.marking_method.to_rumbas(),
        }
    }
}

#[derive(Input, Overwrite, RumbasCheck, Examples, StructDoc)]
#[input(name = "MultipleChoiceMatchAnswerDataInput")]
#[derive(Serialize, Deserialize, Comparable, JsonSchema, Debug, Clone, PartialEq, Eq)]
#[serde(tag = "type")]
pub enum MultipleChoiceMatchAnswerData {
    #[serde(rename = "item_based")]
    /// Specify a list of answers and a list of items with marks for different answers
    ItemBased(MultipleChoiceMatchAnswers),
    #[serde(rename = "numbas_like")]
    /// Specify a list of answers, choices and marks in separate lists.
    NumbasLike(Box<MultipleChoiceMatchAnswerDataNumbasLike>),
}

#[derive(Input, Overwrite, RumbasCheck, Examples, StructDoc)]
#[input(name = "MultipleChoiceMatchAnswerDataNumbasLikeInput")]
#[derive(Serialize, Deserialize, Comparable, Debug, Clone, JsonSchema, PartialEq, Eq)]
pub struct MultipleChoiceMatchAnswerDataNumbasLike {
    /// The possible answers
    pub answers: VariableValued<Vec<ContentAreaTranslatableString>>,
    /// The possible choices
    pub choices: VariableValued<Vec<ContentAreaTranslatableString>>,
    /// The marks for the corresponding answers
    pub marks: VariableValued<Vec<Vec<JMEString>>>,
}

#[derive(Input, Overwrite, RumbasCheck, Examples, StructDoc)]
#[input(name = "MultipleChoiceMatchAnswersInput")]
#[derive(Serialize, Deserialize, Comparable, Debug, Clone, JsonSchema, PartialEq, Eq)]
pub struct MultipleChoiceMatchAnswers {
    /// Values of the answers
    pub answers: Vec<ContentAreaTranslatableString>,
    /// Items for which the answer can be selected
    pub items: Vec<MatchAnswersItem>,
}

#[derive(Input, Overwrite, RumbasCheck, Examples, StructDoc)]
#[input(name = "MatchAnswersItemInput")]
#[derive(Serialize, Deserialize, Comparable, Debug, Clone, JsonSchema, PartialEq, Eq)]
pub struct MatchAnswersItem {
    /// The statement for the item
    pub statement: ContentAreaTranslatableString,
    /// Map points to strings of answers ! use anchors in yaml
    pub answer_marks: Vec<MatchAnswersItemMarks>,
}

#[derive(Input, Overwrite, RumbasCheck, Examples, StructDoc)]
#[input(name = "MatchAnswersItemMarksInput")]
#[derive(Serialize, Deserialize, Comparable, Debug, Clone, JsonSchema, PartialEq, Eq)]
pub struct MatchAnswersItemMarks {
    /// The marks a student get's for selecting the answer
    pub marks: JMEString,
    /// The answer that yields marks for the item
    pub answer: ContentAreaTranslatableString,
}

#[derive(Input, Overwrite, RumbasCheck, Examples, StructDoc)]
#[input(name = "MultipleChoiceWarningTypeInput")]
#[derive(Serialize, Deserialize, Comparable, Debug, Clone, JsonSchema, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum MultipleChoiceWarningType {
    /// Do nothing
    None,
    /// Prevent submission until they pick an acceptable number of answers
    Prevent,
}

impl ToNumbas<numbas::question::part::match_answers::MultipleChoiceWarningType>
    for MultipleChoiceWarningType
{
    type ToNumbasHelper = ();
    fn to_numbas(
        &self,
        _locale: &str,        _data: &Self::ToNumbasHelper

    ) -> numbas::question::part::match_answers::MultipleChoiceWarningType {
        match self {
            Self::None => numbas::question::part::match_answers::MultipleChoiceWarningType::None,
            Self::Prevent => {
                numbas::question::part::match_answers::MultipleChoiceWarningType::Prevent
            }
        }
    }
}

impl ToRumbas<MultipleChoiceWarningType>
    for numbas::question::part::match_answers::MultipleChoiceWarningType
{
    fn to_rumbas(&self) -> MultipleChoiceWarningType {
        match self {
            Self::None => MultipleChoiceWarningType::None,
            Self::Prevent => MultipleChoiceWarningType::Prevent,
        }
    }
}

#[derive(Input, Overwrite, RumbasCheck, Examples, StructDoc)]
#[input(name = "MatchAnswersWithChoicesLayoutTypeInput")]
#[derive(Serialize, Deserialize, Comparable, Debug, Clone, JsonSchema, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum MatchAnswersWithChoicesLayoutType {
    /// All options are shown
    All,
    /// Only the lower triangle is shown
    LowerTriangle,
}

impl ToNumbas<numbas::question::part::match_answers::MatchAnswersWithChoicesLayoutType>
    for MatchAnswersWithChoicesLayoutType
{
    type ToNumbasHelper = ();
    fn to_numbas(
        &self,
        _locale: &str,        _data: &Self::ToNumbasHelper

    ) -> numbas::question::part::match_answers::MatchAnswersWithChoicesLayoutType {
        match self {
            Self::All => numbas::question::part::match_answers::MatchAnswersWithChoicesLayoutType::All,
            Self::LowerTriangle => {
                numbas::question::part::match_answers::MatchAnswersWithChoicesLayoutType::LowerTriangle
            }
        }
    }
}

impl ToRumbas<MatchAnswersWithChoicesLayoutType>
    for numbas::question::part::match_answers::MatchAnswersWithChoicesLayoutType
{
    fn to_rumbas(&self) -> MatchAnswersWithChoicesLayoutType {
        match self {
            Self::All => MatchAnswersWithChoicesLayoutType::All,
            Self::LowerTriangle => MatchAnswersWithChoicesLayoutType::LowerTriangle,
        }
    }
}

#[derive(Input, Overwrite, RumbasCheck, Examples, StructDoc)]
#[input(name = "MatchAnswersWithChoicesLayoutInput")]
#[derive(Serialize, Deserialize, Comparable, Debug, Clone, JsonSchema, PartialEq, Eq)]
pub struct MatchAnswersWithChoicesLayout {
    #[serde(rename = "type")]
    /// Which fields should be shown
    layout_type: MatchAnswersWithChoicesLayoutType, // Renamed because of bug in Comparable
}

impl ToNumbas<numbas::question::part::match_answers::MatchAnswersWithChoicesLayout>
    for MatchAnswersWithChoicesLayout
{
    type ToNumbasHelper = ();
    fn to_numbas(
        &self,
        locale: &str,       
        _data: &Self::ToNumbasHelper
    ) -> numbas::question::part::match_answers::MatchAnswersWithChoicesLayout {
        numbas::question::part::match_answers::MatchAnswersWithChoicesLayout {
            r#type: self.layout_type.to_numbas(locale, &()),
            expression: String::new(),
        }
    }
}

impl ToRumbas<MatchAnswersWithChoicesLayout>
    for numbas::question::part::match_answers::MatchAnswersWithChoicesLayout
{
    fn to_rumbas(&self) -> MatchAnswersWithChoicesLayout {
        MatchAnswersWithChoicesLayout {
            layout_type: self.r#type.to_rumbas(),
        }
    }
}
