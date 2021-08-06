use crate::data::question_part::QuestionPart;
use crate::data::translatable::TranslatableString;
use numbas::defaults::DEFAULTS;

pub trait ToRumbas<RumbasType>: Clone {
    fn to_rumbas(&self) -> RumbasType;
}

macro_rules! impl_to_rumbas {
    ($($type: ty), *) => {
        $(
        impl ToRumbas<$type> for $type {
            fn to_rumbas(&self) -> $type {
                self.clone()
            }
        }
        )*
    };
}

impl_to_rumbas!(bool, f64, usize, [f64; 2]);
impl_to_rumbas!(numbas::exam::Primitive);

impl<T, O: ToRumbas<T>> ToRumbas<Vec<T>> for Vec<O> {
    fn to_rumbas(&self) -> Vec<T> {
        self.iter().map(|item| item.to_rumbas()).collect()
    }
}

impl<T, O: ToRumbas<T>> ToRumbas<Option<T>> for Option<O> {
    fn to_rumbas(&self) -> Option<T> {
        self.clone().map(|item| item.to_rumbas())
    }
}

impl ToRumbas<TranslatableString> for String {
    fn to_rumbas(&self) -> TranslatableString {
        TranslatableString::s(self)
    }
}

pub fn extract_part_common_marks(
    pd: &numbas::exam::ExamQuestionPartSharedData,
) -> numbas::exam::Primitive {
    pd.marks
        .clone()
        .unwrap_or(numbas::exam::Primitive::Natural(DEFAULTS.part_common_marks))
}

pub fn extract_part_common_prompt(pd: &numbas::exam::ExamQuestionPartSharedData) -> String {
    pd.prompt.clone().unwrap_or_default()
}

pub fn extract_part_common_use_custom_name(pd: &numbas::exam::ExamQuestionPartSharedData) -> bool {
    pd.use_custom_name
        .unwrap_or(DEFAULTS.part_common_use_custom_name)
}

pub fn extract_part_common_custom_name(pd: &numbas::exam::ExamQuestionPartSharedData) -> String {
    pd.custom_name.clone().unwrap_or_default()
}

pub fn extract_part_common_steps_penalty(pd: &numbas::exam::ExamQuestionPartSharedData) -> usize {
    pd.steps_penalty
        .unwrap_or(DEFAULTS.part_common_steps_penalty)
}

pub fn extract_part_common_enable_minimum_marks(
    pd: &numbas::exam::ExamQuestionPartSharedData,
) -> bool {
    pd.enable_minimum_marks
        .unwrap_or(DEFAULTS.part_common_enable_minimum_marks)
}

pub fn extract_part_common_minimum_marks(pd: &numbas::exam::ExamQuestionPartSharedData) -> usize {
    pd.minimum_marks
        .unwrap_or(DEFAULTS.part_common_minimum_marks)
}

pub fn extract_part_common_show_correct_answer(
    pd: &numbas::exam::ExamQuestionPartSharedData,
) -> bool {
    pd.show_correct_answer
}

pub fn extract_part_common_show_feedback_icon(
    pd: &numbas::exam::ExamQuestionPartSharedData,
) -> bool {
    pd.show_feedback_icon
        .unwrap_or(DEFAULTS.part_common_show_feedback_icon)
}

pub fn extract_part_common_adaptive_marking_penalty(
    pd: &numbas::exam::ExamQuestionPartSharedData,
) -> usize {
    pd.adaptive_marking_penalty
        .unwrap_or(DEFAULTS.part_common_adaptive_marking_penalty)
}

pub fn extract_part_common_custom_marking_algorithm(
    pd: &numbas::exam::ExamQuestionPartSharedData,
) -> String {
    pd.custom_marking_algorithm.clone().unwrap_or_default()
}

pub fn extract_part_common_extend_base_marking_algorithm(
    pd: &numbas::exam::ExamQuestionPartSharedData,
) -> bool {
    pd.extend_base_marking_algorithm
        .unwrap_or(DEFAULTS.part_common_extend_base_marking_algorithm)
}

pub fn extract_part_common_steps(
    pd: &numbas::exam::ExamQuestionPartSharedData,
) -> Vec<QuestionPart> {
    pd.steps
        .clone()
        .unwrap_or_default()
        .into_iter()
        .map(|s| s.to_rumbas())
        .collect()
}