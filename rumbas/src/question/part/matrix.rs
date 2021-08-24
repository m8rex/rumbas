use crate::question::part::question_part::JMENotes;
use crate::question::part::question_part::JMENotesInput;
use crate::question::part::question_part::{QuestionPart, VariableReplacementStrategy};
use crate::question::part::question_part::{QuestionPartInput, VariableReplacementStrategyInput};
use crate::question::QuestionParts;
use crate::question::QuestionPartsInput;
use crate::support::optional_overwrite::*;
use crate::support::rumbas_types::*;
use crate::support::template::{Value, ValueType};
use crate::support::to_numbas::ToNumbas;
use crate::support::to_rumbas::*;
use crate::support::translatable::ContentAreaTranslatableString;
use crate::support::translatable::ContentAreaTranslatableStringInput;
use crate::support::variable_valued::VariableValued;
use numbas::support::primitive::Primitive;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

// See https://docs.numbas.org.uk/en/latest/question/parts/matrixentry.html#matrix-entry
question_part_type! {
    pub struct QuestionPartMatrix {
        correct_answer: Primitive,
        dimensions: QuestionPartMatrixDimensions,

        /// If the absolute difference between the student’s value for a particular cell and the correct answer’s is less than this value, then it will be marked as correct.
        max_absolute_deviation: RumbasFloat,
        /// If this is set to true, the student will be awarded marks according to the proportion of cells that are marked correctly. If this is not ticked, they will only receive the marks for the part if they get every cell right. If their answer does not have the same dimensions as the correct answer, they are always awarded zero marks.
        mark_partial_by_cells: RumbasBool,

        display_correct_as_fraction: RumbasBool,
        allow_fractions: RumbasBool
        // todo: precision
    }
}

impl ToNumbas<numbas::question::part::matrix::QuestionPartMatrix> for QuestionPartMatrix {
    fn to_numbas(&self, locale: &str) -> numbas::question::part::matrix::QuestionPartMatrix {
        let dimensions = self.dimensions.unwrap();
        let rows = dimensions.rows.unwrap();
        let columns = dimensions.columns.unwrap();
        numbas::question::part::matrix::QuestionPartMatrix {
            part_data: self.to_numbas(locale),
            correct_answer: self.correct_answer.to_numbas(locale),
            correct_answer_fractions: self.display_correct_as_fraction.to_numbas(locale),
            num_rows: rows.default().to_numbas(locale),
            num_columns: columns.default().to_numbas(locale),
            allow_resize: dimensions.is_resizable(),
            min_columns: columns.min().to_numbas(locale),
            max_columns: columns.max().to_numbas(locale),
            min_rows: rows.min().to_numbas(locale),
            max_rows: rows.max().to_numbas(locale),
            tolerance: self.max_absolute_deviation.to_numbas(locale),
            mark_per_cell: self.mark_partial_by_cells.to_numbas(locale),
            allow_fractions: self.allow_fractions.to_numbas(locale),
        }
    }
}

impl ToRumbas<QuestionPartMatrix> for numbas::question::part::matrix::QuestionPartMatrix {
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
        create_question_part! {
            QuestionPartMatrix with &self.part_data  => {
                correct_answer: self.correct_answer.to_rumbas(),
                display_correct_as_fraction: self.correct_answer_fractions.to_rumbas(),
                dimensions: Value::Normal(dimensions),
                max_absolute_deviation: self.tolerance.to_rumbas(),
                mark_partial_by_cells: self.mark_per_cell.to_rumbas(),
                allow_fractions: self.allow_fractions.to_rumbas()
            }
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
        Fixed(VariableValuedNatural),
        Resizable(BoxQuestionPartMatrixRangedDimension)
    }
}

type BoxQuestionPartMatrixRangedDimension = Box<QuestionPartMatrixRangedDimension>;
type BoxQuestionPartMatrixRangedDimensionInput = Box<QuestionPartMatrixRangedDimensionInput>;

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
                Noneable::None => VariableValued::Value(0),
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
                    Noneable::None
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
        default: VariableValuedNatural,
        /// The minimal size
        min: VariableValuedNatural,
        /// The maximal size, if this is none, there is no limit
        max: NoneableVariableValuedNatural
    }
}
