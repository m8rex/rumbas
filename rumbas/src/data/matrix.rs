use crate::data::optional_overwrite::*;
use crate::data::question_part::{QuestionPart, VariableReplacementStrategy};
use crate::data::template::{Value, ValueType};
use crate::data::to_numbas::{NumbasResult, ToNumbas};
use crate::data::translatable::TranslatableString;
use serde::{Deserialize, Serialize};

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
    fn to_numbas(&self, locale: &String) -> NumbasResult<Self::NumbasType> {
        let check = self.check();
        if check.is_empty() {
            let dimensions = self.dimensions.unwrap();
            let rows = dimensions.rows.unwrap();
            let columns = dimensions.columns.unwrap();
            Ok(Self::NumbasType {
                part_data: self.to_numbas_shared_data(&locale),
                correct_answer: self.correct_answer.unwrap(),
                correct_answer_fractions: self.display_correct_as_fraction.unwrap(),
                num_rows: rows.default().into(),
                num_columns: columns.default().into(),
                allow_resize: dimensions.is_resizable(),
                min_columns: columns.min(),
                max_columns: columns.max(),
                min_rows: rows.min(),
                max_rows: rows.max(),
                tolerance: self.max_absolute_deviation.unwrap(),
                mark_per_cell: self.mark_partial_by_cells.unwrap(),
                allow_fractions: self.allow_fractions.unwrap(),
            })
        } else {
            Err(check)
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
        Fixed(usize),
        Resizable(QuestionPartMatrixRangedDimension)
    }
}

impl QuestionPartMatrixDimension {
    pub fn default(&self) -> usize {
        match self {
            QuestionPartMatrixDimension::Fixed(f) => *f,
            QuestionPartMatrixDimension::Resizable(r) => r.default.unwrap(),
        }
    }
    pub fn min(&self) -> usize {
        match self {
            QuestionPartMatrixDimension::Fixed(f) => *f,
            QuestionPartMatrixDimension::Resizable(r) => r.min.unwrap(),
        }
    }
    pub fn max(&self) -> usize {
        match self {
            QuestionPartMatrixDimension::Fixed(f) => *f,
            QuestionPartMatrixDimension::Resizable(r) => match r.max.unwrap() {
                Noneable::None(_s) => 0,
                Noneable::NotNone(f) => f,
            },
        }
    }
    pub fn is_resizable(&self) -> bool {
        self.default() != self.min() || self.default() != self.max()
    }
    pub fn from_range(min: usize, default: usize, max: usize) -> Self {
        if min == default && default == max {
            Self::Fixed(min)
        } else {
            Self::Resizable(QuestionPartMatrixRangedDimension {
                default: Value::Normal(default),
                min: Value::Normal(min),
                max: Value::Normal(if max == 0 {
                    Noneable::None("none".to_string())
                } else {
                    Noneable::NotNone(max)
                }),
            })
        }
    }
}

optional_overwrite! {
    pub struct QuestionPartMatrixRangedDimension {
        /// The default size
        default: usize,
        /// The minimal size
        min: usize,
        /// The maximal size, if this is none, there is no limit
        max: Noneable<usize>
    }
}
