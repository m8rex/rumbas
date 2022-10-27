use crate::question::part::extension::QuestionPartExtension;
use crate::question::part::gapfill::QuestionPartGapFill;
use crate::question::part::information::QuestionPartInformation;
use crate::question::part::jme::QuestionPartJME;
use crate::question::part::matrix::QuestionPartMatrix;
use crate::question::part::multiple_choice::choose_multiple::QuestionPartChooseMultiple;
use crate::question::part::multiple_choice::choose_one::QuestionPartChooseOne;
use crate::question::part::multiple_choice::match_answers::QuestionPartMatchAnswersWithItems;
use crate::question::part::number_entry::QuestionPartNumberEntry;
use crate::question::part::pattern_match::QuestionPartPatternMatch;
use crate::support::noneable::Noneable;
use crate::support::to_numbas::ToNumbas;
use crate::support::to_rumbas::*;
use crate::support::translatable::{ContentAreaTranslatableString, JMETranslatableString};
use comparable::Comparable;
use rumbas_support::preamble::*;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::convert::TryInto;
use structdoc::StructDoc;

#[derive(Input, Overwrite, RumbasCheck, Examples, StructDoc)]
#[input(name = "QuestionPartInput")]
#[derive(Serialize, Deserialize, Comparable, Debug, Clone, JsonSchema, PartialEq)]
#[serde(untagged)]
pub enum QuestionPart {
    /// A question part using a builtin question part type
    Builtin(QuestionPartBuiltin),
    /// A question part using a custom question part type
    Custom(QuestionPartCustom),
}

impl ToNumbas<numbas::question::part::QuestionPart> for QuestionPart {
    fn to_numbas(&self, locale: &str) -> numbas::question::part::QuestionPart {
        match self {
            QuestionPart::Builtin(b) => {
                numbas::question::part::QuestionPart::Builtin(b.to_numbas(locale))
            }
            QuestionPart::Custom(b) => {
                numbas::question::part::QuestionPart::Custom(b.to_numbas(locale))
            }
        }
    }
}

impl ToRumbas<QuestionPart> for numbas::question::part::QuestionPart {
    fn to_rumbas(&self) -> QuestionPart {
        match self {
            numbas::question::part::QuestionPart::Builtin(bqp) => {
                QuestionPart::Builtin(bqp.to_rumbas())
            }
            numbas::question::part::QuestionPart::Custom(cqp) => {
                QuestionPart::Custom(cqp.to_rumbas())
            }
        }
    }
}

impl QuestionPartInput {
    pub fn get_steps(&mut self) -> &mut Value<Vec<ValueType<QuestionPartInput>>> {
        match self {
            QuestionPartInput::Builtin(b) => b.get_steps(),
            QuestionPartInput::Custom(b) => b.0.get_steps(),
        }
    }
}

#[derive(Input, Overwrite, RumbasCheck, Examples, StructDoc)]
#[input(name = "QuestionPartBuiltinInput")]
#[derive(Serialize, Deserialize, Comparable, Debug, Clone, JsonSchema, PartialEq)]
#[serde(tag = "type")]
pub enum QuestionPartBuiltin {
    #[serde(rename = "jme")]
    /// Mathematical expression parts require the student to enter an algebraic expression, using JME syntax.
    JME(Box<QuestionPartJME>),
    #[serde(rename = "gapfill")]
    /// Gap-fill parts allow you to include answer inputs inline with the prompt text, instead of at the end of the part.
    /// Each gap is a question part in itself.
    GapFill(QuestionPartGapFill),
    #[serde(rename = "choose_one")]
    /// Multiple choice part where the student must choose one of several options
    ChooseOne(QuestionPartChooseOne),
    #[serde(rename = "choose_multiple")]
    /// Multiple choice part where the student can choose any of a list of options
    ChooseMultiple(QuestionPartChooseMultiple),
    #[serde(rename = "match_answers")]
    /// The student is presented with a 2D grid of choices and answers. Depending on how the part is set up, they must either match up each choice with an answer, or select any number of choice-answer pairs.
    MatchAnswersWithItems(QuestionPartMatchAnswersWithItems),
    #[serde(rename = "number_entry")]
    /// Number entry parts ask the student to enter a number, which is marked if it is in a specified range
    NumberEntry(QuestionPartNumberEntry),
    #[serde(rename = "pattern_match")]
    /// Use a text pattern part when you want the student to enter short, non-mathematical text.
    PatternMatch(QuestionPartPatternMatch),
    #[serde(rename = "information")]
    /// An information part contains only a prompt and no answer input. It is most often used as a Step to provide a hint for a parent part.
    Information(QuestionPartInformation),
    #[serde(rename = "extension")]
    /// An extension part acts as a placeholder for any interactive element added by an extension, or custom code in the question, which awards marks to the student.
    Extension(QuestionPartExtension),
    #[serde(rename = "matrix")]
    /// Matrix entry parts ask the student to enter a matrix of numbers. Marks are awarded if every cell in the student’s answer is equal to the corresponding cell in the correct answer, within the allowed margin of error.
    Matrix(QuestionPartMatrix),
}

impl ToNumbas<numbas::question::part::QuestionPartBuiltin> for QuestionPartBuiltin {
    fn to_numbas(&self, locale: &str) -> numbas::question::part::QuestionPartBuiltin {
        match self {
            QuestionPartBuiltin::JME(d) => {
                numbas::question::part::QuestionPartBuiltin::JME((*d).to_numbas(locale))
            }
            QuestionPartBuiltin::GapFill(d) => {
                numbas::question::part::QuestionPartBuiltin::GapFill(d.to_numbas(locale))
            }
            QuestionPartBuiltin::ChooseOne(d) => {
                numbas::question::part::QuestionPartBuiltin::ChooseOne(d.to_numbas(locale))
            }
            QuestionPartBuiltin::ChooseMultiple(d) => {
                numbas::question::part::QuestionPartBuiltin::ChooseMultiple(d.to_numbas(locale))
            }
            QuestionPartBuiltin::MatchAnswersWithItems(d) => {
                numbas::question::part::QuestionPartBuiltin::MatchAnswersWithChoices(
                    d.to_numbas(locale),
                )
            }
            QuestionPartBuiltin::NumberEntry(d) => {
                numbas::question::part::QuestionPartBuiltin::NumberEntry(d.to_numbas(locale))
            }
            QuestionPartBuiltin::PatternMatch(d) => {
                numbas::question::part::QuestionPartBuiltin::PatternMatch(d.to_numbas(locale))
            }
            QuestionPartBuiltin::Information(d) => {
                numbas::question::part::QuestionPartBuiltin::Information(d.to_numbas(locale))
            }
            QuestionPartBuiltin::Extension(d) => {
                numbas::question::part::QuestionPartBuiltin::Extension(d.to_numbas(locale))
            }
            QuestionPartBuiltin::Matrix(d) => {
                numbas::question::part::QuestionPartBuiltin::Matrix(Box::new(d.to_numbas(locale)))
            }
        }
    }
}

impl ToRumbas<QuestionPartBuiltin> for numbas::question::part::QuestionPartBuiltin {
    fn to_rumbas(&self) -> QuestionPartBuiltin {
        match self {
            numbas::question::part::QuestionPartBuiltin::JME(p) => {
                QuestionPartBuiltin::JME(Box::new(p.to_rumbas()))
            }
            numbas::question::part::QuestionPartBuiltin::NumberEntry(p) => {
                QuestionPartBuiltin::NumberEntry(p.to_rumbas())
            }
            numbas::question::part::QuestionPartBuiltin::Matrix(p) => {
                QuestionPartBuiltin::Matrix((**p).to_rumbas())
            }
            numbas::question::part::QuestionPartBuiltin::PatternMatch(p) => {
                QuestionPartBuiltin::PatternMatch(p.to_rumbas())
            }
            numbas::question::part::QuestionPartBuiltin::ChooseOne(p) => {
                QuestionPartBuiltin::ChooseOne(p.to_rumbas())
            }
            numbas::question::part::QuestionPartBuiltin::ChooseMultiple(p) => {
                QuestionPartBuiltin::ChooseMultiple(p.to_rumbas())
            }
            numbas::question::part::QuestionPartBuiltin::MatchAnswersWithChoices(p) => {
                QuestionPartBuiltin::MatchAnswersWithItems(p.to_rumbas())
            }
            numbas::question::part::QuestionPartBuiltin::GapFill(p) => {
                QuestionPartBuiltin::GapFill(p.to_rumbas())
            }
            numbas::question::part::QuestionPartBuiltin::Information(p) => {
                QuestionPartBuiltin::Information(p.to_rumbas())
            }
            numbas::question::part::QuestionPartBuiltin::Extension(p) => {
                QuestionPartBuiltin::Extension(p.to_rumbas())
            }
        }
    }
}

impl QuestionPartBuiltinInput {
    pub fn get_steps(&mut self) -> &mut Value<Vec<ValueType<QuestionPartInput>>> {
        match self {
            QuestionPartBuiltinInput::JME(d) => d.as_mut().0.get_steps(),
            QuestionPartBuiltinInput::GapFill(d) => d.0.get_steps(),
            QuestionPartBuiltinInput::ChooseOne(d) => d.0.get_steps(),
            QuestionPartBuiltinInput::ChooseMultiple(d) => d.0.get_steps(),
            QuestionPartBuiltinInput::MatchAnswersWithItems(d) => d.0.get_steps(),
            QuestionPartBuiltinInput::NumberEntry(d) => d.0.get_steps(),
            QuestionPartBuiltinInput::PatternMatch(d) => d.0.get_steps(),
            QuestionPartBuiltinInput::Information(d) => d.0.get_steps(),
            QuestionPartBuiltinInput::Extension(d) => d.0.get_steps(),
            QuestionPartBuiltinInput::Matrix(d) => d.0.get_steps(),
        }
    }
}

#[derive(Input, Overwrite, RumbasCheck, Examples, StructDoc)]
#[input(name = "JMENotesInput")]
#[derive(Debug, Clone, JsonSchema, Deserialize, Serialize, Comparable, PartialEq, Eq, Default)]
#[serde(transparent)]
pub struct JMENotes(pub Vec<JMENote>);

impl ToNumbas<numbas::jme::JMENotesString> for JMENotes {
    fn to_numbas(&self, locale: &str) -> numbas::jme::JMENotesString {
        self.0
            .iter()
            .map(|v| {
                let description = if let Noneable::NotNone(d) = &v.description {
                    format!("({})", d)
                } else {
                    "".to_string()
                };
                format!(
                    "{}{}:{}",
                    v.name,
                    description,
                    v.expression.to_numbas(locale)
                )
            })
            .collect::<Vec<_>>()
            .join("\n\n")
            .try_into()
            .unwrap()
    }
}

impl ToRumbas<JMENotes> for numbas::jme::JMENotesString {
    fn to_rumbas(&self) -> JMENotes {
        JMENotes(if let Some(ref notes) = self.notes {
            notes
                .iter()
                .map(|n| JMENote {
                    name: n.name.to_string().to_rumbas(),
                    description: n.description.to_rumbas(),
                    expression: n.expression_string().to_rumbas(),
                })
                .collect()
        } else {
            vec![]
        })
    }
}

#[derive(Input, Overwrite, RumbasCheck, Examples, StructDoc)]
#[input(name = "JMENoteInput")]
#[derive(Serialize, Deserialize, Comparable, Debug, Clone, JsonSchema, PartialEq, Eq)]
pub struct JMENote {
    pub name: String,
    pub description: Noneable<String>,
    pub expression: JMETranslatableString,
}

impl ToNumbas<numbas::question::custom_part_type::CustomPartMarkingNote> for JMENote {
    fn to_numbas(&self, locale: &str) -> numbas::question::custom_part_type::CustomPartMarkingNote {
        numbas::question::custom_part_type::CustomPartMarkingNote {
            name: self.name.to_numbas(locale),
            definition: self.expression.to_numbas(locale),
            description: self.description.unwrap_or("".to_string()),
        }
    }
}

impl ToRumbas<JMENote> for numbas::question::custom_part_type::CustomPartMarkingNote {
    fn to_rumbas(&self) -> JMENote {
        JMENote {
            name: self.name.to_rumbas(),
            expression: self.definition.to_rumbas(),
            description: Noneable::NotNone(self.description.to_rumbas()),
        }
    }
}

// TODO major refactor: add fields that are used to right ones
macro_rules! question_part_type {
    (
        $(#[$outer:meta])*
        pub struct $struct: ident {
            $($(
                $(#[$inner:meta])*
                $field: ident: $type: ty
             ),+)?
        }
    )=>
    {
        // TODO: add unit tests
        // TODO: alternative answers
        $(#[$outer])*
        pub struct $struct {
            /// A content area used to prompt the student for an answer.
            pub prompt: ContentAreaTranslatableString,
            /// The number of marks to award for answering the part correctly.
            pub marks: numbas::support::primitive::Number,
            #[serde(alias="custom_name")]
            /// An optional custom part name, to use in part path's
            pub part_name: Noneable<String>,
            /// When the student reveals answers to the question, or views the question in review mode, should a correct answer be shown? You might want to turn this off if you’re doing custom marking and the part has no “correct” answer.

            pub show_correct_answer: bool,
            /// After the student submits an answer to this part, should an icon describing their score be shown? This is usually shown next to the input field, as well as in the feedback box. This option also controls whether feedback messages are shown for this part. You might want to turn this off if you’ve set up a question with a custom marking script which assigns a score based on the answers to two or more parts (or gapfills), meaning the individual parts have no independent “correct” or “incorrect” state.
            pub show_feedback_icon: bool,
            /// The marking algorithm tab allows you to customise the script used to mark the student’s answer, and test that it works correctly on answers that you provide.
            pub custom_marking: Noneable<CustomMarking>,
            #[input(skip)]
            /// A (possibly empty) list of sub-parts which the student can reveal by clicking on a button. Marks awarded for steps don’t increase the total available for the part, but are given in case the student gets a lower score for the main part.
            pub steps: Vec<QuestionPart>,
    /// If the student reveals the Steps, reduce the total available marks by this amount. Credit for the part is scaled down accordingly. For example, if there are 6 marks available and the penalty for revealing steps is 2 marks, the total available after revealing steps is 4. An answer worth 3 marks without revealing steps is instead worth 3 * 4/6 = 2 marks after revealing steps.
            pub steps_penalty: numbas::support::primitive::Number,

            // Adaptive marking
            /// Adaptive marking allows you to incorporate the student’s answers to earlier parts when marking their answer to another part. You could use this to allow an “error carried forward” marking scheme, or in more free-form questions where one part has no correct answer - for example, “think of a number and find its square root”. This is achieved by replacing the values of question variables with the student’s answers to other parts. When a variable is replaced, any other variables depending on that one are recalculated using the new value. All other variables keep their original values.
            /// See for more info and a warning https://numbas-editor.readthedocs.io/en/latest/question/parts/reference.html#adaptive-marking
            pub adaptive_marking: Noneable<AdaptiveMarking>
            $(,
            $(
                $(#[$inner])*
                pub $field: $type
            ),+
            )?
        }
        impl ToNumbas<numbas::question::part::QuestionPartSharedData> for $struct {
            fn to_numbas(&self, locale: &str) -> numbas::question::part::QuestionPartSharedData {
                numbas::question::part::QuestionPartSharedData {
                    marks: self.marks.to_numbas(locale),
                    prompt: self.prompt.to_numbas(locale),
                    use_custom_name: self.part_name.to_numbas(locale).is_some(),
                    custom_name: self.part_name.to_numbas(locale).unwrap_or_default(),
                    steps_penalty: self.steps_penalty.to_numbas(locale),
                    show_correct_answer: self.show_correct_answer.to_numbas(locale),
                    show_feedback_icon: self.show_feedback_icon.to_numbas(locale),
                    adaptive_marking: self.adaptive_marking.to_numbas(locale).unwrap_or_default(),
                    custom_marking: self.custom_marking.to_numbas(locale).unwrap_or_default(),
                    steps: self.steps.to_numbas(&locale),
                }

            }
        }
        paste::paste! {
            impl [<$struct Input>] {
                pub fn get_steps(&mut self) -> &mut Value<Vec<ValueType<<crate::question::part::question_part::QuestionPart as InputInverse>::Input>>> {
                    &mut self.steps
                }
            }
        }
    }
}

question_part_type! {
    #[derive(Input, Overwrite, RumbasCheck, Examples, StructDoc)]
    #[input(name = "QuestionPartCustomInput")]
    #[derive(Serialize, Deserialize, Comparable, Debug, Clone, JsonSchema, PartialEq)]
    pub struct QuestionPartCustom {
        #[serde(rename="type")]
        /// The name of the custom part name
        type_name: String, // Renamed because of bug in Comparable
        /// The settings for the CustomPartType
        settings: std::collections::BTreeMap<String, CustomPartInputTypeValue>
    }
}

#[derive(Input, Overwrite, RumbasCheck, Examples, StructDoc)]
#[input(name = "CustomPartInputTypeValueInput")]
#[derive(Serialize, Deserialize, Comparable, Debug, Clone, JsonSchema, PartialEq, Eq)]
#[serde(untagged)]
// TODO
pub enum CustomPartInputTypeValue {
    CheckBox(bool),
    Code(String),
}

impl ToNumbas<numbas::question::part::CustomPartInputTypeValue> for CustomPartInputTypeValue {
    fn to_numbas(&self, _locale: &str) -> numbas::question::part::CustomPartInputTypeValue {
        match self {
            CustomPartInputTypeValue::CheckBox(v) => {
                numbas::question::part::CustomPartInputTypeValue::CheckBox(*v)
            }
            CustomPartInputTypeValue::Code(v) => {
                numbas::question::part::CustomPartInputTypeValue::Code(v.clone())
            }
        }
    }
}

impl ToRumbas<CustomPartInputTypeValue> for numbas::question::part::CustomPartInputTypeValue {
    fn to_rumbas(&self) -> CustomPartInputTypeValue {
        match self {
            numbas::question::part::CustomPartInputTypeValue::CheckBox(v) => {
                CustomPartInputTypeValue::CheckBox(*v)
            }
            numbas::question::part::CustomPartInputTypeValue::Code(v) => {
                CustomPartInputTypeValue::Code(v.to_string())
            }
        }
    }
}

impl ToNumbas<numbas::question::part::QuestionPartCustom> for QuestionPartCustom {
    fn to_numbas(&self, locale: &str) -> numbas::question::part::QuestionPartCustom {
        numbas::question::part::QuestionPartCustom {
            part_data: self.to_numbas(locale),
            r#type: self.type_name.to_numbas(locale),
            settings: self.settings.to_numbas(locale),
        }
    }
}

impl ToRumbas<QuestionPartCustom> for numbas::question::part::QuestionPartCustom {
    fn to_rumbas(&self) -> QuestionPartCustom {
        QuestionPartCustom {
            // Default section
            marks: extract_part_common_marks(&self.part_data),
            prompt: extract_part_common_prompt(&self.part_data),
            part_name: extract_part_common_custom_name(&self.part_data),
            steps_penalty: extract_part_common_steps_penalty(&self.part_data),
            show_correct_answer: extract_part_common_show_correct_answer(&self.part_data),
            show_feedback_icon: extract_part_common_show_feedback_icon(&self.part_data),
            adaptive_marking: self.part_data.adaptive_marking.to_rumbas(),
            custom_marking: self.part_data.custom_marking.to_rumbas(),

            steps: extract_part_common_steps(&self.part_data),

            type_name: self.r#type.clone(),
            settings: self.settings.to_rumbas(),
        }
    }
}

#[derive(Input, Overwrite, RumbasCheck, Examples, StructDoc)]
#[input(name = "AdaptiveMarkingInput")]
#[derive(Serialize, Deserialize, Comparable, Debug, Clone, JsonSchema, PartialEq, Eq)]
pub struct AdaptiveMarking {
    /// The variable replacements to do
    variable_replacements: Vec<VariableReplacement>,
    /// The circumstances under which the variable replacements are used, and adaptive marking is applied.
    variable_replacement_strategy: VariableReplacementStrategy,
    /// If adaptive marking is used, reduce the total available marks by this amount. Credit for the part is scaled down accordingly. See steps_penalty for an example.
    penalty: usize,
}

impl ToNumbas<numbas::question::part::AdaptiveMarking> for AdaptiveMarking {
    fn to_numbas(&self, locale: &str) -> numbas::question::part::AdaptiveMarking {
        numbas::question::part::AdaptiveMarking {
            variable_replacements: self.variable_replacements.to_numbas(&locale),
            variable_replacement_strategy: self.variable_replacement_strategy.to_numbas(&locale),
            penalty: self.penalty.to_numbas(locale),
        }
    }
}

impl ToRumbas<Noneable<AdaptiveMarking>> for numbas::question::part::AdaptiveMarking {
    fn to_rumbas(&self) -> Noneable<AdaptiveMarking> {
        if self.variable_replacements.is_empty() {
            Noneable::None
        } else {
            Noneable::NotNone(AdaptiveMarking {
                variable_replacement_strategy: self.variable_replacement_strategy.to_rumbas(),
                variable_replacements: self.variable_replacements.to_rumbas(),
                penalty: self.penalty,
            })
        }
    }
}

#[derive(Input, Overwrite, RumbasCheck, Examples, StructDoc)]
#[input(name = "CustomMarkingInput")]
#[derive(Serialize, Deserialize, Comparable, Debug, Clone, JsonSchema, PartialEq, Eq)]
pub struct CustomMarking {
    /// This allows you to customise the script used to mark the student’s answer, and
    /// test that it works correctly on answers that you provide.
    algorithm_notes: JMENotes,
    /// If this is ticked, all marking notes provided by the part’s standard marking algorithm will be available. If the same note is defined in both the standard algorithm and your custom algorithm, your version will be used.
    extend_base_marking_algorithm: bool,
}

impl ToNumbas<numbas::question::part::CustomMarking> for CustomMarking {
    fn to_numbas(&self, locale: &str) -> numbas::question::part::CustomMarking {
        numbas::question::part::CustomMarking {
            algorithm: self.algorithm_notes.to_numbas(&locale),
            extend_base_marking_algorithm: self.extend_base_marking_algorithm.to_numbas(locale),
        }
    }
}

impl ToRumbas<Noneable<CustomMarking>> for numbas::question::part::CustomMarking {
    fn to_rumbas(&self) -> Noneable<CustomMarking> {
        if self.algorithm.is_empty() {
            Noneable::None
        } else {
            Noneable::NotNone(CustomMarking {
                algorithm_notes: self.algorithm.to_rumbas(),
                extend_base_marking_algorithm: self.extend_base_marking_algorithm,
            })
        }
    }
}

#[derive(Input, Overwrite, RumbasCheck, Examples, StructDoc)]
#[input(name = "VariableReplacementInput")]
#[derive(Serialize, Deserialize, Comparable, Debug, Clone, JsonSchema, PartialEq, Eq)]
pub struct VariableReplacement {
    /// The name of the variable to replace
    variable: String,
    /// The path to the part whose answer the variable’s value should be replaced with. Different part types produce different types of values.
    part_answer_to_use: String,
    /// If this is ticked, the student must submit an answer to the referenced part before they can submit an answer to this part.
    must_be_answered: bool,
}

impl ToNumbas<numbas::question::part::VariableReplacement> for VariableReplacement {
    fn to_numbas(&self, _locale: &str) -> numbas::question::part::VariableReplacement {
        numbas::question::part::VariableReplacement {
            variable: self.variable.clone(),
            part_answer_to_use: self.part_answer_to_use.clone(),
            must_be_answered: self.must_be_answered,
        }
    }
}

impl ToRumbas<VariableReplacement> for numbas::question::part::VariableReplacement {
    fn to_rumbas(&self) -> VariableReplacement {
        VariableReplacement {
            variable: self.variable.clone(),
            part_answer_to_use: self.part_answer_to_use.clone(),
            must_be_answered: self.must_be_answered,
        }
    }
}

#[derive(Input, Overwrite, RumbasCheck, Examples, StructDoc)]
#[input(name = "VariableReplacementStrategyInput")]
#[derive(Serialize, Deserialize, Comparable, Debug, Clone, JsonSchema, PartialEq, Eq)]
pub enum VariableReplacementStrategy {
    #[serde(rename = "original_first")]
    ///  The student’s answer is first marked using the original values of the question variables. If the credit given by this method is less than the maximum available, the marking is repeated using the defined variable replacements. If the credit gained with variable replacements is greater than the credit gained under the original marking, that score is used, and the student is told that their answers to previous parts have been used in the marking for this part.
    OriginalFirst,
    #[serde(rename = "always_replace")]
    /// The student’s answer is only marked once, with the defined variable replacements applied.
    AlwaysReplace,
}

impl ToNumbas<numbas::question::part::VariableReplacementStrategy> for VariableReplacementStrategy {
    fn to_numbas(&self, _locale: &str) -> numbas::question::part::VariableReplacementStrategy {
        match self {
            VariableReplacementStrategy::OriginalFirst => {
                numbas::question::part::VariableReplacementStrategy::OriginalFirst
            }
            VariableReplacementStrategy::AlwaysReplace => {
                numbas::question::part::VariableReplacementStrategy::AlwaysReplace
            }
        }
    }
}

impl ToRumbas<VariableReplacementStrategy> for numbas::question::part::VariableReplacementStrategy {
    fn to_rumbas(&self) -> VariableReplacementStrategy {
        match self {
            numbas::question::part::VariableReplacementStrategy::OriginalFirst => {
                VariableReplacementStrategy::OriginalFirst
            }
            numbas::question::part::VariableReplacementStrategy::AlwaysReplace => {
                VariableReplacementStrategy::AlwaysReplace
            }
        }
    }
}
