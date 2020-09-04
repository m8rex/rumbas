use crate::data::extension::Extensions;
use crate::data::to_numbas::{NumbasResult, ToNumbas};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct CustomPartTypeDefinition {
    type_name: String,
    description: String,
    settings: numbas::exam::CustomPartTypeSettings,
    can_be_gap: bool,
    can_be_step: bool,
    marking_notes: Vec<numbas::exam::CustomPartMarkingNotes>,
    published: bool,
    extensions: Extensions,
    #[serde(flatten)]
    input_widget: CustomPartInputWidget,
    //TODO source
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum CustomPartInputWidget {
    //TODO other types: https://numbas-editor.readthedocs.io/en/latest/custom-part-types/reference.html
    #[serde(rename = "string")]
    String {
        // The student enters a single line of text.
        input_options: CustomPartStringInputOptions,
    },
    #[serde(rename = "number")]
    Number {
        // The student enters a number, using any of the allowed notation styles. If the student’s answer is not a valid number, they are shown a warning and can not submit the part.
        input_options: CustomPartNumberInputOptions,
    },
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct CustomPartStringInputOptions {
    //TODO? hint & correctAnswer is shared for all...
    hint: String, // A string displayed next to the input field, giving any necessary information about how to enter their answer.
    correct_answer: String, // A JME expression which evaluates to the expected answer to the part.
    allow_empty: bool, // If false, the part will only be marked if their answer is non-empty.
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct CustomPartNumberInputOptions {
    //TODO? hint & correctAnswer is shared for all...
    hint: String, // A string displayed next to the input field, giving any necessary information about how to enter their answer.
    correct_answer: String, // A JME expression which evaluates to the expected answer to the part.
    allow_fractions: bool, //Allow the student to enter their answer as a fraction?
    allowed_notation_styles: Vec<String>,
}
