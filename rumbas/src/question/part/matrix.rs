use crate::question::part::question_part::JMENotes;
use crate::question::part::question_part::VariableReplacementStrategy;
use crate::question::part::question_part::{AdaptiveMarking, CustomMarking};
use crate::question::QuestionPart;
use crate::support::noneable::Noneable;
use crate::support::to_numbas::ToNumbas;
use crate::support::to_rumbas::*;
use crate::support::translatable::ContentAreaTranslatableString;
use crate::support::variable_valued::ReverseVariableValued;
use comparable::Comparable;
use rumbas_support::preamble::*;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use structdoc::StructDoc;

// See https://docs.numbas.org.uk/en/latest/question/parts/matrixentry.html#matrix-entry
question_part_type! {
    #[derive(Input, Overwrite, RumbasCheck, Examples, StructDoc)]
    #[input(name = "QuestionPartMatrixInput")]
    #[derive(Serialize, Deserialize, Comparable, Debug, Clone, JsonSchema, PartialEq)]
    pub struct QuestionPartMatrix {
        /// The expected answer to the part. This is a JME expression which must evaluate to a matrix.
        correct_answer: numbas::jme::JMEString,
        /// The dimensions of the student's answer field
        dimensions: QuestionPartMatrixDimensions,

        /// If the absolute difference between the student’s value for a particular cell and the correct answer’s is less than this value, then it will be marked as correct.
        max_absolute_deviation: f64,
        /// If this is set to true, the student will be awarded marks according to the proportion of cells that are marked correctly. If this is not ticked, they will only receive the marks for the part if they get every cell right. If their answer does not have the same dimensions as the correct answer, they are always awarded zero marks.
        mark_partial_by_cells: bool,

        /// If this is ticked, then non-integer numbers in the correct answer will be displayed as fractions instead of decimals.
        display_correct_as_fraction: bool,
        // This option is only available when no precision restriction is applied, since they apply to decimal numbers.
        /// If this is ticked, the student can enter a ratio of two whole numbers, e.g. -3/8, as their answer.
        allow_fractions: bool

        // todo: precision
    }
}

impl ToNumbas<numbas::question::part::matrix::QuestionPartMatrix> for QuestionPartMatrix {
    type ToNumbasHelper = ();
    fn to_numbas(&self, locale: &str, _data: &Self::ToNumbasHelper) -> numbas::question::part::matrix::QuestionPartMatrix {
        let dimensions = self.dimensions.clone();
        let rows = dimensions.rows.clone();
        let columns = dimensions.columns.clone();
        numbas::question::part::matrix::QuestionPartMatrix {
            part_data: self.to_numbas(locale, &()),
            correct_answer: self.correct_answer.to_numbas(locale, &()),
            correct_answer_fractions: self.display_correct_as_fraction.to_numbas(locale, &()),
            num_rows: rows.default().to_numbas(locale, &()),
            num_columns: columns.default().to_numbas(locale, &()),
            allow_resize: dimensions.is_resizable(),
            min_columns: columns.min().to_numbas(locale, &()),
            max_columns: columns.max().to_numbas(locale, &()),
            min_rows: rows.min().to_numbas(locale, &()),
            max_rows: rows.max().to_numbas(locale, &()),
            tolerance: self.max_absolute_deviation.to_numbas(locale, &()),
            mark_per_cell: self.mark_partial_by_cells.to_numbas(locale, &()),
            allow_fractions: self.allow_fractions.to_numbas(locale, &()),
        }
    }
}

impl ToRumbas<QuestionPartMatrix> for numbas::question::part::matrix::QuestionPartMatrix {
    fn to_rumbas(&self) -> QuestionPartMatrix {
        let rows = QuestionPartMatrixDimension::from_range(
            self.min_rows.to_rumbas(),
            self.num_rows.clone().map(|v| v.0).to_rumbas(),
            self.max_rows.to_rumbas(),
        );
        let columns = QuestionPartMatrixDimension::from_range(
            self.min_columns.to_rumbas(),
            self.num_columns.clone().map(|v| v.0).to_rumbas(),
            self.max_columns.to_rumbas(),
        );
        let dimensions = QuestionPartMatrixDimensions { rows, columns };
        create_question_part! {
            QuestionPartMatrix with &self.part_data  => {
                correct_answer: self.correct_answer.to_rumbas(),
                display_correct_as_fraction: self.correct_answer_fractions.to_rumbas(),
                dimensions,
                max_absolute_deviation: self.tolerance.to_rumbas(),
                mark_partial_by_cells: self.mark_per_cell.to_rumbas(),
                allow_fractions: self.allow_fractions.to_rumbas()
            }
        }
    }
}

#[derive(Input, Overwrite, RumbasCheck, Examples, StructDoc)]
#[input(name = "QuestionPartMatrixDimensionsInput")]
#[derive(Serialize, Deserialize, Comparable, Debug, Clone, JsonSchema, PartialEq, Eq)]
pub struct QuestionPartMatrixDimensions {
    /// The number of rows in the student’s answer field.
    pub rows: QuestionPartMatrixDimension,
    /// The number of columns in the student’s answer field.
    pub columns: QuestionPartMatrixDimension,
}

impl QuestionPartMatrixDimensions {
    pub fn is_resizable(&self) -> bool {
        self.rows.is_resizable() || self.columns.is_resizable()
    }
}

#[derive(Input, Overwrite, RumbasCheck, Examples, StructDoc)]
#[input(name = "QuestionPartMatrixDimensionInput")]
#[derive(Serialize, Deserialize, Comparable, Debug, Clone, JsonSchema, PartialEq, Eq)]
pub enum QuestionPartMatrixDimension {
    /// The dimensions are fixed
    Fixed(ReverseVariableValued<usize>),
    /// The student can change the dimensions
    Resizable(Box<QuestionPartMatrixRangedDimension>),
}

impl QuestionPartMatrixDimension {
    pub fn default(&self) -> ReverseVariableValued<usize> {
        match self {
            QuestionPartMatrixDimension::Fixed(f) => f.clone(),
            QuestionPartMatrixDimension::Resizable(r) => r.default.clone(),
        }
    }
    pub fn min(&self) -> ReverseVariableValued<usize> {
        match self {
            QuestionPartMatrixDimension::Fixed(f) => f.clone(),
            QuestionPartMatrixDimension::Resizable(r) => r.min.clone(),
        }
    }
    pub fn max(&self) -> ReverseVariableValued<usize> {
        match self {
            QuestionPartMatrixDimension::Fixed(f) => f.clone(),
            QuestionPartMatrixDimension::Resizable(r) => match &r.max {
                Noneable::None => ReverseVariableValued::Value(0),
                Noneable::NotNone(f) => f.clone(),
            },
        }
    }
    pub fn is_resizable(&self) -> bool {
        self.default() != self.min() || self.default() != self.max()
    }
    pub fn from_range(
        min: ReverseVariableValued<usize>,
        default: ReverseVariableValued<usize>,
        max: ReverseVariableValued<usize>,
    ) -> Self {
        if min == default && default == max {
            Self::Fixed(min)
        } else {
            Self::Resizable(Box::new(QuestionPartMatrixRangedDimension {
                default,
                min,
                max: if max == ReverseVariableValued::Value(0) {
                    Noneable::None
                } else {
                    Noneable::NotNone(max)
                },
            }))
        }
    }
}

#[derive(Input, Overwrite, RumbasCheck, Examples, StructDoc)]
#[input(name = "QuestionPartMatrixRangedDimensionInput")]
#[derive(Serialize, Deserialize, Comparable, Debug, Clone, JsonSchema, PartialEq, Eq)]
pub struct QuestionPartMatrixRangedDimension {
    /// The default size
    pub default: ReverseVariableValued<usize>,
    /// The minimal size
    pub min: ReverseVariableValued<usize>,
    /// The maximal size, if this is none, there is no limit
    pub max: Noneable<ReverseVariableValued<usize>>,
}
