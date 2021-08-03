use crate::data::optional_overwrite::*;
use crate::data::question_part::JMENotes;
use crate::data::question_part::{QuestionPart, VariableReplacementStrategy};
use crate::data::template::{Value, ValueType};
use crate::data::to_numbas::{NumbasResult, ToNumbas};
use crate::data::to_rumbas::*;
use crate::data::translatable::ContentAreaTranslatableString;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::convert::TryInto;

// See https://docs.numbas.org.uk/en/latest/question/parts/matrixentry.html#matrix-entry
question_part_type! {
    pub struct QuestionPartMatrix {
        correct_answer: numbas::exam::Primitive,
        dimensions: QuestionPartMatrixDimensions,

        /// If the absolute difference between the student’s value for a particular cell and the correct answer’s is less than this value, then it will be marked as correct.
        max_absolute_deviation: f64,
        /// If this is set to true, the student will be awarded marks according to the proportion of cells that are marked correctly. If this is not ticked, they will only receive the marks for the part if they get every cell right. If their answer does not have the same dimensions as the correct answer, they are always awarded zero marks.
        mark_partial_by_cells: bool,

        display_correct_as_fraction: bool,
        allow_fractions: bool
        // todo: precision
    }
}

impl ToNumbas for QuestionPartMatrix {
    type NumbasType = numbas::exam::ExamQuestionPartMatrix;
    fn to_numbas(&self, locale: &str) -> NumbasResult<Self::NumbasType> {
        let check = self.check();
        if check.is_empty() {
            let dimensions = self.dimensions.unwrap();
            let rows = dimensions.rows.unwrap();
            let columns = dimensions.columns.unwrap();
            Ok(Self::NumbasType {
                part_data: self.to_numbas_shared_data(locale),
                correct_answer: self.correct_answer.unwrap(),
                correct_answer_fractions: self.display_correct_as_fraction.unwrap(),
                num_rows: rows.default().to_numbas(locale).unwrap().map(|v| v.into()),
                num_columns: columns
                    .default()
                    .to_numbas(locale)
                    .unwrap()
                    .map(|v| v.into()),
                allow_resize: dimensions.is_resizable(),
                min_columns: columns.min().to_numbas(locale).unwrap(),
                max_columns: columns.max().to_numbas(locale).unwrap(),
                min_rows: rows.min().to_numbas(locale).unwrap(),
                max_rows: rows.max().to_numbas(locale).unwrap(),
                tolerance: self.max_absolute_deviation.unwrap(),
                mark_per_cell: self.mark_partial_by_cells.unwrap(),
                allow_fractions: self.allow_fractions.unwrap(),
            })
        } else {
            Err(check)
        }
    }
}

impl ToRumbas<QuestionPartMatrix> for numbas::exam::ExamQuestionPartMatrix {
    fn to_rumbas(&self) -> QuestionPartMatrix {
        let rows = Value::Normal(QuestionPartMatrixDimension::from_range(
            self.min_rows.to_rumbas(),
            self.num_rows.clone().map(|v| v.0).to_rumbas(),
            self.max_rows.to_rumbas(),
        ));
        let columns = Value::Normal(QuestionPartMatrixDimension::from_range(
            self.min_columns.to_rumbas(),
            self.num_columns.clone().map(|v| v.0).to_rumbas(),
            self.max_columns.to_rumbas(),
        ));
        let dimensions = QuestionPartMatrixDimensions { rows, columns };
        QuestionPartMatrix {
            marks: Value::Normal(extract_part_common_marks(&self.part_data)),
            prompt: Value::Normal(extract_part_common_prompt(&self.part_data)),
            use_custom_name: Value::Normal(extract_part_common_use_custom_name(&self.part_data)),
            custom_name: Value::Normal(extract_part_common_custom_name(&self.part_data)),
            steps_penalty: Value::Normal(extract_part_common_steps_penalty(&self.part_data)),
            enable_minimum_marks: Value::Normal(extract_part_common_enable_minimum_marks(
                &self.part_data,
            )),
            minimum_marks: Value::Normal(extract_part_common_minimum_marks(&self.part_data)),
            show_correct_answer: Value::Normal(extract_part_common_show_correct_answer(
                &self.part_data,
            )),
            show_feedback_icon: Value::Normal(extract_part_common_show_feedback_icon(
                &self.part_data,
            )),
            variable_replacement_strategy: Value::Normal(
                self.part_data.variable_replacement_strategy.to_rumbas(),
            ),
            adaptive_marking_penalty: Value::Normal(extract_part_common_adaptive_marking_penalty(
                &self.part_data,
            )),
            custom_marking_algorithm_notes: Value::Normal(
                self.part_data
                    .custom_marking_algorithm
                    .to_rumbas()
                    .unwrap_or_default(),
            ),
            extend_base_marking_algorithm: Value::Normal(
                extract_part_common_extend_base_marking_algorithm(&self.part_data),
            ),
            steps: Value::Normal(extract_part_common_steps(&self.part_data)),

            correct_answer: Value::Normal(self.correct_answer.clone()),
            display_correct_as_fraction: Value::Normal(self.correct_answer_fractions),
            dimensions: Value::Normal(dimensions),
            max_absolute_deviation: Value::Normal(self.tolerance),
            mark_partial_by_cells: Value::Normal(self.mark_per_cell),
            allow_fractions: Value::Normal(self.allow_fractions),
        }
    }
}

optional_overwrite! {
    pub struct QuestionPartMatrixDimensions {
        rows: QuestionPartMatrixDimension,
        columns: QuestionPartMatrixDimension
    }
}

impl QuestionPartMatrixDimensions {
    pub fn is_resizable(&self) -> bool {
        self.rows.unwrap().is_resizable() || self.columns.unwrap().is_resizable()
    }
}

optional_overwrite_enum! {
    pub enum QuestionPartMatrixDimension {
        Fixed(VariableValued<usize>),
        Resizable(Box<QuestionPartMatrixRangedDimension>)
    }
}

impl QuestionPartMatrixDimension {
    pub fn default(&self) -> VariableValued<usize> {
        match self {
            QuestionPartMatrixDimension::Fixed(f) => f.clone(),
            QuestionPartMatrixDimension::Resizable(r) => r.default.unwrap(),
        }
    }
    pub fn min(&self) -> VariableValued<usize> {
        match self {
            QuestionPartMatrixDimension::Fixed(f) => f.clone(),
            QuestionPartMatrixDimension::Resizable(r) => r.min.unwrap(),
        }
    }
    pub fn max(&self) -> VariableValued<usize> {
        match self {
            QuestionPartMatrixDimension::Fixed(f) => f.clone(),
            QuestionPartMatrixDimension::Resizable(r) => match r.max.unwrap() {
                Noneable::None(_s) => VariableValued::Value(0),
                Noneable::NotNone(f) => f,
            },
        }
    }
    pub fn is_resizable(&self) -> bool {
        self.default() != self.min() || self.default() != self.max()
    }
    pub fn from_range(
        min: VariableValued<usize>,
        default: VariableValued<usize>,
        max: VariableValued<usize>,
    ) -> Self {
        if min == default && default == max {
            Self::Fixed(min)
        } else {
            Self::Resizable(Box::new(QuestionPartMatrixRangedDimension {
                default: Value::Normal(default),
                min: Value::Normal(min),
                max: Value::Normal(if max == VariableValued::Value(0) {
                    Noneable::None("none".to_string())
                } else {
                    Noneable::NotNone(max)
                }),
            }))
        }
    }
}

optional_overwrite! {
    pub struct QuestionPartMatrixRangedDimension {
        /// The default size
        default: VariableValued<usize>,
        /// The minimal size
        min: VariableValued<usize>,
        /// The maximal size, if this is none, there is no limit
        max: Noneable<VariableValued<usize>>
    }
}
