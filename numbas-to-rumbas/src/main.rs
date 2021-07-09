use numbas::exam::Exam as NExam;
use rumbas::data::diagnostic_exam::{
    Diagnostic, DiagnosticExam, DiagnosticScript, LearningObjective, LearningTopic,
};
use rumbas::data::exam::Exam as RExam;
use rumbas::data::extension::Extensions;
use rumbas::data::feedback::{Feedback, FeedbackMessage, Review};
use rumbas::data::file_reference::FileString;
use rumbas::data::function::Function;
use rumbas::data::gapfill::QuestionPartGapFill;
use rumbas::data::information::QuestionPartInformation;
use rumbas::data::jme::{
    CheckingType, CheckingTypeDataFloat, CheckingTypeDataNatural, JMEAnswerSimplification,
    JMELengthRestriction, JMEPatternRestriction, JMERestriction, JMEStringRestriction,
    JMEValueGenerator, QuestionPartJME,
};
use rumbas::data::locale::{Locale, SupportedLocale};
use rumbas::data::multiple_choice::{
    ChooseOneDisplay, MatchAnswersItem, MatchAnswersItemMarks, MultipleChoiceAnswer,
    MultipleChoiceAnswerData, MultipleChoiceAnswerDataNumbasLike, MultipleChoiceMatchAnswerData,
    MultipleChoiceMatchAnswerDataNumbasLike, MultipleChoiceMatchAnswers,
    QuestionPartChooseMultiple, QuestionPartChooseOne, QuestionPartMatchAnswersWithItems,
};
use rumbas::data::navigation::{
    DiagnosticNavigation, LeaveAction, NavigationSharedData, QuestionNavigation,
};
use rumbas::data::numbas_settings::NumbasSettings;
use rumbas::data::number_entry::{NumberEntryAnswer, QuestionPartNumberEntry};
use rumbas::data::optional_overwrite::Noneable;
use rumbas::data::pattern_match::QuestionPartPatternMatch;
use rumbas::data::preamble::Preamble;
use rumbas::data::question::{BuiltinConstants, CustomConstant, Question, VariablesTest};
use rumbas::data::question_group::{PickingStrategy, QuestionGroup, QuestionPath};
use rumbas::data::question_part::{QuestionPart, VariableReplacementStrategy};
use rumbas::data::template::ExamFileType;
use rumbas::data::template::Value;
use rumbas::data::timing::{TimeoutAction, Timing};
use rumbas::data::to_rumbas::ToRumbas;
use rumbas::data::translatable::TranslatableString;
use rumbas::data::variable::{Variable, VariableRepresentation, VariableTemplateType};

macro_rules! read {
    ($file_name: expr) => {{
        let content = std::fs::read_to_string($file_name).expect("Invalid file path");
        NExam::from_str(content.as_ref())
    }};
}

macro_rules! v {
    ($e: expr) => {
        Value::Normal($e)
    };
}

macro_rules! ts {
    ($s: expr) => {
        TranslatableString::NotTranslated(v!(FileString::s(&$s)))
    };
}

fn nn<T>() -> Noneable<T> {
    Noneable::None("none".to_string())
}

fn main() {
    let exam_res = read!("example.exam");
    match exam_res {
        Ok(exam) => {
            //println!("{:?}", exam);
            let rumbas_exam = convert_exam(exam);
            println!("{}", rumbas_exam.to_yaml().unwrap());
        }
        Err(e) => {
            eprintln!("Error: {:?}", e);
        }
    }
}

fn convert_exam(exam: NExam) -> ExamFileType {
    // TODO: check diagnostic vs normal
    ExamFileType::Diagnostic(convert_diagnostic_exam(exam))
}
/*
fn convert_normal_exam(exam: Exam) {

}*/

fn convert_diagnostic_exam(exam: NExam) -> DiagnosticExam {
    DiagnosticExam {
        locales: v!(vec![v!(Locale {
            name: v!("en".to_string()),
            numbas_locale: v!(SupportedLocale::EnGB)
        })]), // TODO: argument?
        name: v![ts!("todo".to_string())], // TODO: argument
        navigation: v![extract_diagnostic_navigation(&exam)],
        timing: v![extract_timing(&exam)],
        feedback: v![extract_feedback(&exam)],
        question_groups: v![extract_question_groups(&exam)],
        numbas_settings: v![NumbasSettings {
            locale: v!(SupportedLocale::EnGB),
            theme: v!("default".to_string())
        }], // TODO: argument?
        diagnostic: v![extract_diagnostic(&exam)],
    }
}

fn extract_shared_navigation(exam: &NExam) -> NavigationSharedData {
    NavigationSharedData {
        // TODO: fix numbas defaults
        start_password: v!(FileString::s(
            &exam
                .navigation
                .start_password
                .clone()
                .unwrap_or("".to_string())
        )),
        can_regenerate: v!(exam.navigation.allow_regenerate),
        show_steps: v!(exam.navigation.allow_steps.unwrap_or(true)),
        show_title_page: v!(exam.navigation.show_frontpage),
        prevent_leaving: v!(exam.navigation.prevent_leaving.unwrap_or(true)),
        show_names_of_question_groups: v!(exam
            .basic_settings
            .show_question_group_names
            .unwrap_or(true)),
        allow_printing: v!(exam.basic_settings.allow_printing.unwrap_or(true)),
    }
}

fn extract_diagnostic_navigation(exam: &NExam) -> DiagnosticNavigation {
    DiagnosticNavigation {
        shared_data: v!(extract_shared_navigation(exam)),
        on_leave: v!(exam
            .navigation
            .on_leave
            .clone()
            .map(|ol| {
                match ol {
                    numbas::exam::ExamLeaveAction::None { message: _ } => LeaveAction::None,
                    numbas::exam::ExamLeaveAction::WarnIfNotAttempted { message } => {
                        LeaveAction::WarnIfNotAttempted {
                            message: ts!(message),
                        }
                    }
                    numbas::exam::ExamLeaveAction::PreventIfNotAttempted { message } => {
                        LeaveAction::PreventIfNotAttempted {
                            message: ts!(message),
                        }
                    }
                }
            })
            .unwrap_or(LeaveAction::None)),
    }
}

fn extract_timeout_action(action: &numbas::exam::ExamTimeoutAction) -> TimeoutAction {
    match action {
        numbas::exam::ExamTimeoutAction::None { message: _ } => TimeoutAction::None,
        numbas::exam::ExamTimeoutAction::Warn { message } => TimeoutAction::Warn {
            message: ts!(message),
        },
    }
}

fn extract_timing(exam: &NExam) -> Timing {
    Timing {
        duration_in_seconds: v!(exam
            .basic_settings
            .duration_in_seconds
            .map(|s| Noneable::NotNone(s))
            .unwrap_or(nn())),
        allow_pause: v!(exam.timing.allow_pause),
        on_timeout: v!(extract_timeout_action(&exam.timing.timeout)),
        timed_warning: v!(extract_timeout_action(&exam.timing.timed_warning)),
    }
}

fn extract_feedback(exam: &NExam) -> Feedback {
    Feedback {
        // TODO: fix numbas defaults
        percentage_needed_to_pass: v!(exam
            .basic_settings
            .percentage_needed_to_pass
            .map(|p| Noneable::NotNone(p))
            .unwrap_or(nn())),
        show_name_of_student: v!(exam.basic_settings.show_student_name.unwrap_or(true)),
        show_current_marks: v!(exam.feedback.show_actual_mark),
        show_maximum_marks: v!(exam.feedback.show_total_mark),
        show_answer_state: v!(exam.feedback.show_answer_state),
        allow_reveal_answer: v!(exam.feedback.allow_reveal_answer),
        review: v!(Review {
            show_score: v!(exam
                .feedback
                .review
                .clone()
                .map(|r| r.show_score.unwrap_or(true))
                .unwrap()),
            show_feedback: v!(exam
                .feedback
                .review
                .clone()
                .map(|r| r.show_feedback.unwrap_or(true))
                .unwrap()),
            show_expected_answer: v!(exam
                .feedback
                .review
                .clone()
                .map(|r| r.show_expected_answer.unwrap_or(true))
                .unwrap()),
            show_advice: v!(exam
                .feedback
                .review
                .clone()
                .map(|r| r.show_advice.unwrap_or(true))
                .unwrap()),
        }),
        advice: v!(ts!(exam.feedback.advice.clone().unwrap_or(String::new()))),
        intro: v!(ts!(exam.feedback.intro)),
        feedback_messages: v!(exam
            .feedback
            .feedback_messages
            .clone()
            .into_iter()
            .map(|m| {
                v!(FeedbackMessage {
                    message: m.message,
                    threshold: m.threshold
                })
            })
            .collect()),
    }
}

fn extract_builtin_constants(bc: numbas::exam::BuiltinConstants) -> BuiltinConstants {
    BuiltinConstants {
        e: v!(*bc.0.get(&"e".to_string()).unwrap_or(&false)),
        pi: v!(*bc.0.get(&"pi,\u{03c0}".to_string()).unwrap_or(&false)),
        i: v!(*bc.0.get(&"i".to_string()).unwrap_or(&false)),
    }
}

fn extract_variable_template_type(
    tt: numbas::exam::ExamVariableTemplateType,
) -> VariableTemplateType {
    match tt {
        numbas::exam::ExamVariableTemplateType::Anything => VariableTemplateType::Anything,
        numbas::exam::ExamVariableTemplateType::ListOfNumbers => {
            VariableTemplateType::ListOfNumbers
        }
        numbas::exam::ExamVariableTemplateType::ListOfStrings => {
            VariableTemplateType::ListOfStrings
        }
        numbas::exam::ExamVariableTemplateType::LongString => VariableTemplateType::LongString,
        numbas::exam::ExamVariableTemplateType::Number => VariableTemplateType::Number,
        numbas::exam::ExamVariableTemplateType::RandomRange => VariableTemplateType::RandomRange,
        numbas::exam::ExamVariableTemplateType::Range => VariableTemplateType::Range,
        numbas::exam::ExamVariableTemplateType::r#String => VariableTemplateType::r#String,
    }
}

fn extract_jme_answer_simplification(
    ov: &Option<Vec<numbas::exam::AnswerSimplificationType>>,
) -> JMEAnswerSimplification {
    let mut result = JMEAnswerSimplification {
        simplify_basic: v!(true),
        simplify_unit_factor: v!(true),
        simplify_unit_power: v!(true),
        simplify_unit_denominator: v!(true),
        simplify_zero_factor: v!(true),
        simplify_zero_term: v!(true),
        simplify_zero_power: v!(true),
        simplify_zero_base: v!(true),
        collect_numbers: v!(true),
        constants_first: v!(true),
        simplify_sqrt_products: v!(true),
        simplify_sqrt_division: v!(true),
        simplify_sqrt_square: v!(true),
        simplify_other_numbers: v!(true),
        simplify_no_leading_minus: v!(true),
        simplify_fractions: v!(true),
        simplify_trigonometric: v!(true),
        cancel_terms: v!(true),
        cancel_factors: v!(true),
        collect_like_fractions: v!(true),
        order_canonical: v!(false),
        use_times_dot: v!(true),
        expand_brackets: v!(false),
    }; // Numbas default
    if let Some(v) = ov {
        for a in v.iter() {
            match a {
                numbas::exam::AnswerSimplificationType::All(b) => {
                    result.simplify_basic = v!(*b);
                    result.simplify_unit_factor = v!(*b);
                    result.simplify_unit_power = v!(*b);
                    result.simplify_unit_denominator = v!(*b);
                    result.simplify_zero_factor = v!(*b);
                    result.simplify_zero_term = v!(*b);
                    result.simplify_zero_power = v!(*b);
                    result.simplify_zero_base = v!(*b);
                    result.collect_numbers = v!(*b);
                    result.constants_first = v!(*b);
                    result.simplify_sqrt_products = v!(*b);
                    result.simplify_sqrt_division = v!(*b);
                    result.simplify_sqrt_square = v!(*b);
                    result.simplify_other_numbers = v!(*b);
                    result.simplify_no_leading_minus = v!(*b);
                    result.simplify_fractions = v!(*b);
                    result.simplify_trigonometric = v!(*b);
                    result.cancel_terms = v!(*b);
                    result.cancel_factors = v!(*b);
                    result.collect_like_fractions = v!(*b);
                    result.use_times_dot = v!(*b);
                }
                numbas::exam::AnswerSimplificationType::Basic(b) => {
                    result.simplify_basic = v!(*b);
                }
                numbas::exam::AnswerSimplificationType::UnitFactor(b) => {
                    result.simplify_unit_factor = v!(*b);
                }
                numbas::exam::AnswerSimplificationType::UnitPower(b) => {
                    result.simplify_unit_power = v!(*b);
                }
                numbas::exam::AnswerSimplificationType::UnitDenominator(b) => {
                    result.simplify_unit_denominator = v!(*b);
                }
                numbas::exam::AnswerSimplificationType::ZeroFactor(b) => {
                    result.simplify_zero_factor = v!(*b);
                }
                numbas::exam::AnswerSimplificationType::ZeroTerm(b) => {
                    result.simplify_zero_term = v!(*b);
                }
                numbas::exam::AnswerSimplificationType::ZeroPower(b) => {
                    result.simplify_zero_power = v!(*b);
                }
                numbas::exam::AnswerSimplificationType::CollectNumbers(b) => {
                    result.collect_numbers = v!(*b);
                }
                numbas::exam::AnswerSimplificationType::ZeroBase(b) => {
                    result.simplify_zero_base = v!(*b);
                }
                numbas::exam::AnswerSimplificationType::ConstantsFirst(b) => {
                    result.constants_first = v!(*b);
                }
                numbas::exam::AnswerSimplificationType::SqrtProduct(b) => {
                    result.simplify_sqrt_products = v!(*b);
                }
                numbas::exam::AnswerSimplificationType::SqrtDivision(b) => {
                    result.simplify_sqrt_division = v!(*b);
                }
                numbas::exam::AnswerSimplificationType::SqrtSquare(b) => {
                    result.simplify_sqrt_square = v!(*b);
                }
                numbas::exam::AnswerSimplificationType::OtherNumbers(b) => {
                    result.simplify_other_numbers = v!(*b);
                }
                numbas::exam::AnswerSimplificationType::NoLeadingMinus(b) => {
                    result.simplify_no_leading_minus = v!(*b);
                }
                numbas::exam::AnswerSimplificationType::Fractions(b) => {
                    result.simplify_fractions = v!(*b);
                }
                numbas::exam::AnswerSimplificationType::Trigonometric(b) => {
                    result.simplify_trigonometric = v!(*b);
                }
                numbas::exam::AnswerSimplificationType::CancelTerms(b) => {
                    result.cancel_terms = v!(*b);
                }
                numbas::exam::AnswerSimplificationType::CancelFactors(b) => {
                    result.cancel_factors = v!(*b);
                }
                numbas::exam::AnswerSimplificationType::CollectLikeFractions(b) => {
                    result.collect_like_fractions = v!(*b);
                }
                numbas::exam::AnswerSimplificationType::TimesDot(b) => {
                    result.use_times_dot = v!(*b);
                }
                numbas::exam::AnswerSimplificationType::ExpandBrackets(b) => {
                    result.expand_brackets = v!(*b);
                }
                numbas::exam::AnswerSimplificationType::CanonicalOrder(b) => {
                    result.order_canonical = v!(*b);
                }
            }
        }
    }

    result
}

fn extract_checking_type(ct: &numbas::exam::JMECheckingType) -> CheckingType {
    // TODO
    CheckingType::DecimalPlaces(CheckingTypeDataNatural {
        checking_accuracy: v!(0),
    })
}

fn extract_part_common_marks(
    pd: &numbas::exam::ExamQuestionPartSharedData,
) -> numbas::exam::Primitive {
    pd.marks
        .clone()
        .unwrap_or(numbas::exam::Primitive::Natural(0)) // TODO
}

fn extract_part_common_prompt(pd: &numbas::exam::ExamQuestionPartSharedData) -> String {
    pd.prompt.clone().unwrap_or(String::new()) //TODO
}

fn extract_part_common_use_custom_name(pd: &numbas::exam::ExamQuestionPartSharedData) -> bool {
    pd.use_custom_name.unwrap_or(false) // TODO
}
fn extract_part_common_custom_name(pd: &numbas::exam::ExamQuestionPartSharedData) -> String {
    pd.custom_name.clone().unwrap_or(String::new()) // TODO
}
fn extract_part_common_steps_penalty(pd: &numbas::exam::ExamQuestionPartSharedData) -> usize {
    pd.steps_penalty.unwrap_or(0) // TODO
}
fn extract_part_common_enable_minimum_marks(pd: &numbas::exam::ExamQuestionPartSharedData) -> bool {
    pd.enable_minimum_marks.unwrap_or(false) // TODO
}
fn extract_part_common_minimum_marks(pd: &numbas::exam::ExamQuestionPartSharedData) -> usize {
    pd.minimum_marks.unwrap_or(0) // TODO
}
fn extract_part_common_show_correct_answer(pd: &numbas::exam::ExamQuestionPartSharedData) -> bool {
    pd.show_correct_answer // TODO
}
fn extract_part_common_show_feedback_icon(pd: &numbas::exam::ExamQuestionPartSharedData) -> bool {
    pd.show_feedback_icon.unwrap_or(false) // TODO
}
fn extract_part_common_variable_replacement_strategy(
    pd: &numbas::exam::ExamQuestionPartSharedData,
) -> VariableReplacementStrategy {
    VariableReplacementStrategy::OriginalFirst // TODO
}
fn extract_part_common_adaptive_marking_penalty(
    pd: &numbas::exam::ExamQuestionPartSharedData,
) -> usize {
    pd.adaptive_marking_penalty.unwrap_or(0) // TODO
}
fn extract_part_common_custom_marking_algorithm(
    pd: &numbas::exam::ExamQuestionPartSharedData,
) -> String {
    pd.custom_marking_algorithm.clone().unwrap_or(String::new()) // TODO
}
fn extract_part_common_extend_base_marking_algorithm(
    pd: &numbas::exam::ExamQuestionPartSharedData,
) -> bool {
    pd.extend_base_marking_algorithm.unwrap_or(false) // TODO
}
fn extract_part_common_steps(pd: &numbas::exam::ExamQuestionPartSharedData) -> Vec<QuestionPart> {
    pd.steps
        .clone()
        .unwrap_or(vec![])
        .into_iter()
        .map(|s| extract_part(&s))
        .filter(|s| s.is_some()) // TODO
        .map(|s| s.unwrap())
        .collect()
}

fn extract_restriction(r: &numbas::exam::JMERestriction) -> JMERestriction {
    JMERestriction {
        name: v!(ts!(r.name)),
        strings: v!(r.strings.clone().into_iter().map(|s| ts!(s)).collect()),
        partial_credit: v!(r.partial_credit),
        message: v!(ts!(r.message)),
    }
}

fn extract_length_restriction(r: &numbas::exam::JMELengthRestriction) -> JMELengthRestriction {
    JMELengthRestriction {
        restriction: v!(extract_restriction(&r.restriction)),
        length: v!(r.length.unwrap_or(0)), // TODO?
    }
}

fn extract_string_restriction(r: &numbas::exam::JMEStringRestriction) -> JMEStringRestriction {
    JMEStringRestriction {
        restriction: v!(extract_restriction(&r.restriction)),
        show_strings: v!(r.show_strings),
    }
}

fn extract_pattern_restriction(r: &numbas::exam::JMEPatternRestriction) -> JMEPatternRestriction {
    JMEPatternRestriction {
        partial_credit: v!(r.partial_credit),
        message: v!(ts!(r.message.clone())),
        pattern: v!(r.pattern.clone()),
        name_to_compare: v!(r.name_to_compare.clone()),
    }
}

fn extract_value_generator(g: &numbas::exam::JMEValueGenerator) -> JMEValueGenerator {
    JMEValueGenerator {
        name: v!(FileString::s(&g.name)),
        value: v!(FileString::s(&g.value)),
    }
}

fn extract_jme_part(qp: &numbas::exam::ExamQuestionPartJME) -> QuestionPart {
    QuestionPart::JME(QuestionPartJME {
        // Default section
        marks: v!(extract_part_common_marks(&qp.part_data)),
        prompt: v!(ts!(extract_part_common_prompt(&qp.part_data))),
        use_custom_name: v!(extract_part_common_use_custom_name(&qp.part_data)),
        custom_name: v!(extract_part_common_custom_name(&qp.part_data)),
        steps_penalty: v!(extract_part_common_steps_penalty(&qp.part_data)),
        enable_minimum_marks: v!(extract_part_common_enable_minimum_marks(&qp.part_data)),
        minimum_marks: v!(extract_part_common_minimum_marks(&qp.part_data)),
        show_correct_answer: v!(extract_part_common_show_correct_answer(&qp.part_data)),
        show_feedback_icon: v!(extract_part_common_show_feedback_icon(&qp.part_data)),
        variable_replacement_strategy: v!(extract_part_common_variable_replacement_strategy(
            &qp.part_data
        )),
        adaptive_marking_penalty: v!(extract_part_common_adaptive_marking_penalty(&qp.part_data)),
        custom_marking_algorithm: v!(extract_part_common_custom_marking_algorithm(&qp.part_data)),
        extend_base_marking_algorithm: v!(extract_part_common_extend_base_marking_algorithm(
            &qp.part_data
        )),
        steps: v!(extract_part_common_steps(&qp.part_data)),

        answer: v!(ts!(qp.answer)),
        answer_simplification: v!(extract_jme_answer_simplification(&qp.answer_simplification)),
        show_preview: v!(qp.show_preview),
        checking_type: v!(extract_checking_type(&qp.checking_type)), // TODO
        failure_rate: v!(qp.failure_rate),
        vset_range: v!(qp.vset_range),
        vset_range_points: v!(qp.vset_range_points),
        check_variable_names: v!(qp.check_variable_names),
        single_letter_variables: v!(qp.single_letter_variables.unwrap_or(false)), // TODO numbas default
        allow_unknown_functions: v!(qp.allow_unknown_functions.unwrap_or(false)), // TODO numbas default
        implicit_function_composition: v!(qp.implicit_function_composition.unwrap_or(false)), // TODO: numbas default

        max_length: v!(qp
            .max_length
            .clone()
            .map(|r| Noneable::NotNone(extract_length_restriction(&r)))
            .unwrap_or(nn())),
        min_length: v!(qp
            .min_length
            .clone()
            .map(|r| Noneable::NotNone(extract_length_restriction(&r)))
            .unwrap_or(nn())),
        must_have: v!(qp
            .must_have
            .clone()
            .map(|r| Noneable::NotNone(extract_string_restriction(&r)))
            .unwrap_or(nn())),
        may_not_have: v!(qp
            .may_not_have
            .clone()
            .map(|r| Noneable::NotNone(extract_string_restriction(&r)))
            .unwrap_or(nn())),
        must_match_pattern: v!(qp
            .must_match_pattern
            .clone()
            .map(|r| Noneable::NotNone(extract_pattern_restriction(&r)))
            .unwrap_or(nn())),
        value_generators: v!(qp
            .value_generators
            .clone()
            .map(|v| Noneable::NotNone(v.iter().map(|g| extract_value_generator(&g)).collect()))
            .unwrap_or(nn())),
    })
}

fn extract_number_entry_answer(a: &numbas::exam::NumberEntryAnswerType) -> NumberEntryAnswer {
    match a {
        numbas::exam::NumberEntryAnswerType::MinMax {
            min_value,
            max_value,
        } => NumberEntryAnswer::Range {
            from: FileString::s(&min_value.to_string()),
            to: FileString::s(&max_value.to_string()),
        },
        numbas::exam::NumberEntryAnswerType::Answer { answer } => {
            NumberEntryAnswer::Normal(FileString::s(&answer.to_string()))
        }
    }
}

fn extract_number_entry_part(qp: &numbas::exam::ExamQuestionPartNumberEntry) -> QuestionPart {
    QuestionPart::NumberEntry(QuestionPartNumberEntry {
        // Default section
        marks: v!(extract_part_common_marks(&qp.part_data)),
        prompt: v!(ts!(extract_part_common_prompt(&qp.part_data))),
        use_custom_name: v!(extract_part_common_use_custom_name(&qp.part_data)),
        custom_name: v!(extract_part_common_custom_name(&qp.part_data)),
        steps_penalty: v!(extract_part_common_steps_penalty(&qp.part_data)),
        enable_minimum_marks: v!(extract_part_common_enable_minimum_marks(&qp.part_data)),
        minimum_marks: v!(extract_part_common_minimum_marks(&qp.part_data)),
        show_correct_answer: v!(extract_part_common_show_correct_answer(&qp.part_data)),
        show_feedback_icon: v!(extract_part_common_show_feedback_icon(&qp.part_data)),
        variable_replacement_strategy: v!(extract_part_common_variable_replacement_strategy(
            &qp.part_data
        )),
        adaptive_marking_penalty: v!(extract_part_common_adaptive_marking_penalty(&qp.part_data)),
        custom_marking_algorithm: v!(extract_part_common_custom_marking_algorithm(&qp.part_data)),
        extend_base_marking_algorithm: v!(extract_part_common_extend_base_marking_algorithm(
            &qp.part_data
        )),
        steps: v!(extract_part_common_steps(&qp.part_data)),

        answer: v!(extract_number_entry_answer(&qp.answer)),
        display_correct_as_fraction: v!(qp.correct_answer_fraction),
        allow_fractions: v!(qp.allow_fractions),
        allowed_notation_styles: v!(qp.notation_styles.clone().unwrap_or(vec![])),
        display_correct_in_style: v!(qp
            .correct_answer_style
            .clone()
            .unwrap_or(numbas::exam::AnswerStyle::Plain)), // TODO default

        fractions_must_be_reduced: v!(qp.fractions_must_be_reduced.unwrap_or(true)), // TODO: default
        partial_credit_if_fraction_not_reduced: v!(qp
            .partial_credit_if_fraction_not_reduced
            .clone()
            .unwrap_or(numbas::exam::Primitive::Natural(0))), // TODO: default
        hint_fraction: v!(qp.show_fraction_hint.unwrap_or(true)), // TODO: default
    })
}
/* TODO
fn extract_matrix_part(qp: &numbas::exam::ExamQuestionPartMatrix) -> QuestionPart {
    QuestionPart::Matrix(None) // TODO
}*/

fn extract_pattern_match_part(qp: &numbas::exam::ExamQuestionPartPatternMatch) -> QuestionPart {
    QuestionPart::PatternMatch(QuestionPartPatternMatch {
        // Default section
        marks: v!(extract_part_common_marks(&qp.part_data)),
        prompt: v!(ts!(extract_part_common_prompt(&qp.part_data))),
        use_custom_name: v!(extract_part_common_use_custom_name(&qp.part_data)),
        custom_name: v!(extract_part_common_custom_name(&qp.part_data)),
        steps_penalty: v!(extract_part_common_steps_penalty(&qp.part_data)),
        enable_minimum_marks: v!(extract_part_common_enable_minimum_marks(&qp.part_data)),
        minimum_marks: v!(extract_part_common_minimum_marks(&qp.part_data)),
        show_correct_answer: v!(extract_part_common_show_correct_answer(&qp.part_data)),
        show_feedback_icon: v!(extract_part_common_show_feedback_icon(&qp.part_data)),
        variable_replacement_strategy: v!(extract_part_common_variable_replacement_strategy(
            &qp.part_data
        )),
        adaptive_marking_penalty: v!(extract_part_common_adaptive_marking_penalty(&qp.part_data)),
        custom_marking_algorithm: v!(extract_part_common_custom_marking_algorithm(&qp.part_data)),
        extend_base_marking_algorithm: v!(extract_part_common_extend_base_marking_algorithm(
            &qp.part_data
        )),
        steps: v!(extract_part_common_steps(&qp.part_data)),

        case_sensitive: v!(qp.case_sensitive),
        partial_credit: v!(qp.partial_credit),
        pattern: v!(ts!(qp.answer.to_string())),
        display_answer: v!(ts!(qp
            .display_answer
            .clone()
            .map(|d| d.to_string())
            .unwrap_or(qp.answer.to_string()))), // TDDO: check default
        match_mode: v!(qp.match_mode),
    })
}

fn extract_choose_one_part(qp: &numbas::exam::ExamQuestionPartChooseOne) -> QuestionPart {
    let answer_data = if let (
        numbas::exam::VariableValued::Value(answer_options),
        Some(numbas::exam::VariableValued::Value(marking_matrix)),
    ) = (qp.answers.clone(), qp.marking_matrix.clone())
    {
        let answers_data: Vec<_> = match qp.distractors.clone() {
            None => answer_options
                .into_iter()
                .zip(marking_matrix.into_iter())
                .map(|(a, b)| (a, b, "".to_string()))
                .collect(),
            Some(d) => answer_options
                .into_iter()
                .zip(marking_matrix.into_iter())
                .zip(d.into_iter())
                .map(|((a, b), c)| (a, b, c))
                .collect(),
        };
        v!(MultipleChoiceAnswerData::ItemBased(
            answers_data
                .into_iter()
                .map(|(a, b, c)| MultipleChoiceAnswer {
                    statement: v!(ts!(a)),
                    marks: v!(b),
                    feedback: v!(ts!(c))
                })
                .collect()
        ))
    } else {
        v!(MultipleChoiceAnswerData::NumbasLike(
            MultipleChoiceAnswerDataNumbasLike {
                answers: v!(qp
                    .answers
                    .clone()
                    .map(|v| v.iter().map(|vv| ts!(vv.clone())).collect::<Vec<_>>())
                    .to_rumbas()),
                marks: v!(qp
                    .marking_matrix
                    .clone()
                    .map(|m| m.to_rumbas())
                    .expect("How can the marking matrix be optional?")),
                feedback: v!(qp
                    .distractors
                    .clone()
                    .map(|v| Noneable::NotNone(
                        v.iter()
                            .map(|f| ts!(f).clone())
                            .collect::<Vec<_>>()
                            .to_rumbas()
                    ))
                    .unwrap_or(nn()))
            }
        ))
    };
    QuestionPart::ChooseOne(QuestionPartChooseOne {
        // Default section
        marks: v!(extract_part_common_marks(&qp.part_data)),
        prompt: v!(ts!(extract_part_common_prompt(&qp.part_data))),
        use_custom_name: v!(extract_part_common_use_custom_name(&qp.part_data)),
        custom_name: v!(extract_part_common_custom_name(&qp.part_data)),
        steps_penalty: v!(extract_part_common_steps_penalty(&qp.part_data)),
        enable_minimum_marks: v!(extract_part_common_enable_minimum_marks(&qp.part_data)),
        minimum_marks: v!(extract_part_common_minimum_marks(&qp.part_data)),
        show_correct_answer: v!(extract_part_common_show_correct_answer(&qp.part_data)),
        show_feedback_icon: v!(extract_part_common_show_feedback_icon(&qp.part_data)),
        variable_replacement_strategy: v!(extract_part_common_variable_replacement_strategy(
            &qp.part_data
        )),
        adaptive_marking_penalty: v!(extract_part_common_adaptive_marking_penalty(&qp.part_data)),
        custom_marking_algorithm: v!(extract_part_common_custom_marking_algorithm(&qp.part_data)),
        extend_base_marking_algorithm: v!(extract_part_common_extend_base_marking_algorithm(
            &qp.part_data
        )),
        steps: v!(extract_part_common_steps(&qp.part_data)),
        answer_data,
        display: v!(match qp.display_type {
            numbas::exam::ChooseOneDisplayType::Radio => ChooseOneDisplay::Radio {
                columns: qp.columns.0,
            },
            numbas::exam::ChooseOneDisplayType::DropDown => ChooseOneDisplay::DropDown,
        }),
        shuffle_answers: v!(qp.shuffle_answers),
        show_cell_answer_state: v!(qp.show_cell_answer_state),
        has_to_select_option: v!(qp.min_answers.map(|v| v == 1).unwrap_or(false)), // TODO: default
    })
}

fn extract_choose_multiple_part(qp: &numbas::exam::ExamQuestionPartChooseMultiple) -> QuestionPart {
    // TODO: less duplicate code?: Extract following as function
    let answer_data = if let (
        numbas::exam::VariableValued::Value(answer_options),
        Some(numbas::exam::VariableValued::Value(marking_matrix)),
    ) = (qp.choices.clone(), qp.marking_matrix.clone())
    {
        let answers_data: Vec<_> = match qp.distractors.clone() {
            None => answer_options
                .into_iter()
                .zip(marking_matrix.into_iter())
                .map(|(a, b)| (a, b, "".to_string()))
                .collect(),
            Some(d) => answer_options
                .into_iter()
                .zip(marking_matrix.into_iter())
                .zip(d.into_iter())
                .map(|((a, b), c)| (a, b, c))
                .collect(),
        };
        v!(MultipleChoiceAnswerData::ItemBased(
            answers_data
                .into_iter()
                .map(|(a, b, c)| MultipleChoiceAnswer {
                    statement: v!(ts!(a)),
                    marks: v!(b),
                    feedback: v!(ts!(c))
                })
                .collect()
        ))
    } else {
        v!(MultipleChoiceAnswerData::NumbasLike(
            MultipleChoiceAnswerDataNumbasLike {
                answers: v!(qp
                    .choices
                    .clone()
                    .map(|v| v.iter().map(|vv| ts!(vv.clone())).collect::<Vec<_>>())
                    .to_rumbas()),
                marks: v!(qp
                    .marking_matrix
                    .clone()
                    .map(|m| m.to_rumbas())
                    .expect("How can the marking matrix be optional?")),
                feedback: v!(qp
                    .distractors
                    .clone()
                    .map(|v| Noneable::NotNone(
                        v.iter()
                            .map(|f| ts!(f).clone())
                            .collect::<Vec<_>>()
                            .to_rumbas()
                    ))
                    .unwrap_or(nn()))
            }
        ))
    };
    QuestionPart::ChooseMultiple(QuestionPartChooseMultiple {
        // Default section
        marks: v!(extract_part_common_marks(&qp.part_data)),
        prompt: v!(ts!(extract_part_common_prompt(&qp.part_data))),
        use_custom_name: v!(extract_part_common_use_custom_name(&qp.part_data)),
        custom_name: v!(extract_part_common_custom_name(&qp.part_data)),
        steps_penalty: v!(extract_part_common_steps_penalty(&qp.part_data)),
        enable_minimum_marks: v!(extract_part_common_enable_minimum_marks(&qp.part_data)),
        minimum_marks: v!(extract_part_common_minimum_marks(&qp.part_data)),
        show_correct_answer: v!(extract_part_common_show_correct_answer(&qp.part_data)),
        show_feedback_icon: v!(extract_part_common_show_feedback_icon(&qp.part_data)),
        variable_replacement_strategy: v!(extract_part_common_variable_replacement_strategy(
            &qp.part_data
        )),
        adaptive_marking_penalty: v!(extract_part_common_adaptive_marking_penalty(&qp.part_data)),
        custom_marking_algorithm: v!(extract_part_common_custom_marking_algorithm(&qp.part_data)),
        extend_base_marking_algorithm: v!(extract_part_common_extend_base_marking_algorithm(
            &qp.part_data
        )),
        steps: v!(extract_part_common_steps(&qp.part_data)),

        answer_data,
        shuffle_answers: v!(qp.shuffle_answers),
        show_cell_answer_state: v!(qp.show_cell_answer_state),
        should_select_at_least: v!(qp.min_answers.unwrap_or(0)),
        should_select_at_most: v!(qp
            .max_answers
            .map(|ma| Noneable::NotNone(ma))
            .unwrap_or(nn())),
        columns: v!(qp.display_columns.0),
    })
}

fn extract_match_answers_with_choices_part(
    qp: &numbas::exam::ExamQuestionPartMatchAnswersWithChoices,
) -> QuestionPart {
    let answer_data = if let (
        numbas::exam::VariableValued::Value(answer_options),
        numbas::exam::VariableValued::Value(choice_options),
        Some(numbas::exam::VariableValued::Value(marking_matrix)),
    ) = (
        qp.answers.clone(),
        qp.choices.clone(),
        qp.marking_matrix.clone(),
    ) {
        // inverted_matrix[choice][answer]
        let inverted_matrix: Vec<Vec<_>> = (0..choice_options.len())
            .map(|choice_idx| {
                marking_matrix
                    .clone()
                    .into_iter()
                    .map(|m| {
                        m.get(choice_idx)
                            .expect("marking_matrix does not have enough columns?")
                            .clone()
                    })
                    .collect()
            })
            .collect();

        let items_data: Vec<_> = choice_options
            .into_iter()
            .zip(inverted_matrix.into_iter())
            .collect();

        v!(MultipleChoiceMatchAnswerData::ItemBased({
            let answers: Vec<_> = answer_options.iter().map(|a| v!(ts!(a.clone()))).collect();
            MultipleChoiceMatchAnswers {
                answers: v!(answers.clone()),
                items: v!(items_data
                    .into_iter()
                    .map(|(statement, marks)| v!(MatchAnswersItem {
                        statement: v!(ts!(statement)),
                        answer_marks: v!(marks
                            .into_iter()
                            .enumerate()
                            .map(|(i, m)| {
                                MatchAnswersItemMarks {
                                    marks: v!(m),
                                    answer: answers.get(i).unwrap().clone(),
                                }
                            })
                            .collect()),
                    }))
                    .collect()),
            }
        }))
    } else {
        v!(MultipleChoiceMatchAnswerData::NumbasLike(
            MultipleChoiceMatchAnswerDataNumbasLike {
                answers: v!(qp
                    .answers
                    .clone()
                    .map(|v| v.iter().map(|vv| ts!(vv.clone())).collect::<Vec<_>>())
                    .to_rumbas()),
                choices: v!(qp
                    .choices
                    .clone()
                    .map(|v| v.iter().map(|vv| ts!(vv.clone())).collect::<Vec<_>>())
                    .to_rumbas()),
                marks: v!(qp
                    .marking_matrix
                    .clone()
                    .map(|m| m.to_rumbas())
                    .expect("How can the marking matrix be optional?")),
            }
        ))
    };
    QuestionPart::MatchAnswersWithItems(QuestionPartMatchAnswersWithItems {
        // Default section
        marks: v!(extract_part_common_marks(&qp.part_data)),
        prompt: v!(ts!(extract_part_common_prompt(&qp.part_data))),
        use_custom_name: v!(extract_part_common_use_custom_name(&qp.part_data)),
        custom_name: v!(extract_part_common_custom_name(&qp.part_data)),
        steps_penalty: v!(extract_part_common_steps_penalty(&qp.part_data)),
        enable_minimum_marks: v!(extract_part_common_enable_minimum_marks(&qp.part_data)),
        minimum_marks: v!(extract_part_common_minimum_marks(&qp.part_data)),
        show_correct_answer: v!(extract_part_common_show_correct_answer(&qp.part_data)),
        show_feedback_icon: v!(extract_part_common_show_feedback_icon(&qp.part_data)),
        variable_replacement_strategy: v!(extract_part_common_variable_replacement_strategy(
            &qp.part_data
        )),
        adaptive_marking_penalty: v!(extract_part_common_adaptive_marking_penalty(&qp.part_data)),
        custom_marking_algorithm: v!(extract_part_common_custom_marking_algorithm(&qp.part_data)),
        extend_base_marking_algorithm: v!(extract_part_common_extend_base_marking_algorithm(
            &qp.part_data
        )),
        steps: v!(extract_part_common_steps(&qp.part_data)),

        answer_data,
        shuffle_answers: v!(qp.shuffle_answers),
        shuffle_items: v!(qp.shuffle_choices),
        show_cell_answer_state: v!(qp.show_cell_answer_state),
        should_select_at_least: v!(qp.min_answers.unwrap_or(0)),
        should_select_at_most: v!(qp
            .max_answers
            .map(|ma| Noneable::NotNone(ma))
            .unwrap_or(nn())),
        display: v!(qp.display_type.to_rumbas()),
        layout: v!(qp.layout.clone()),
    }) // TODO
}

fn extract_gapfill_part(qp: &numbas::exam::ExamQuestionPartGapFill) -> QuestionPart {
    QuestionPart::GapFill(QuestionPartGapFill {
        marks: v!(extract_part_common_marks(&qp.part_data)),
        prompt: v!(ts!(extract_part_common_prompt(&qp.part_data))),
        use_custom_name: v!(extract_part_common_use_custom_name(&qp.part_data)),
        custom_name: v!(extract_part_common_custom_name(&qp.part_data)),
        steps_penalty: v!(extract_part_common_steps_penalty(&qp.part_data)),
        enable_minimum_marks: v!(extract_part_common_enable_minimum_marks(&qp.part_data)),
        minimum_marks: v!(extract_part_common_minimum_marks(&qp.part_data)),
        show_correct_answer: v!(extract_part_common_show_correct_answer(&qp.part_data)),
        show_feedback_icon: v!(extract_part_common_show_feedback_icon(&qp.part_data)),
        variable_replacement_strategy: v!(extract_part_common_variable_replacement_strategy(
            &qp.part_data
        )),
        adaptive_marking_penalty: v!(extract_part_common_adaptive_marking_penalty(&qp.part_data)),
        custom_marking_algorithm: v!(extract_part_common_custom_marking_algorithm(&qp.part_data)),
        extend_base_marking_algorithm: v!(extract_part_common_extend_base_marking_algorithm(
            &qp.part_data
        )),
        steps: v!(extract_part_common_steps(&qp.part_data)),

        sort_answers: v!(qp.sort_answers.unwrap_or(false)), // TODO default

        gaps: v!(qp
            .gaps
            .iter()
            .map(|s| extract_part(&s))
            .filter(|s| s.is_some()) // TODO
            .map(|s| s.unwrap())
            .collect()),
    })
}

fn extract_information_part(qp: &numbas::exam::ExamQuestionPartInformation) -> QuestionPart {
    QuestionPart::Information(QuestionPartInformation {
        marks: v!(extract_part_common_marks(&qp.part_data)),
        prompt: v!(ts!(extract_part_common_prompt(&qp.part_data))),
        use_custom_name: v!(extract_part_common_use_custom_name(&qp.part_data)),
        custom_name: v!(extract_part_common_custom_name(&qp.part_data)),
        steps_penalty: v!(extract_part_common_steps_penalty(&qp.part_data)),
        enable_minimum_marks: v!(extract_part_common_enable_minimum_marks(&qp.part_data)),
        minimum_marks: v!(extract_part_common_minimum_marks(&qp.part_data)),
        show_correct_answer: v!(extract_part_common_show_correct_answer(&qp.part_data)),
        show_feedback_icon: v!(extract_part_common_show_feedback_icon(&qp.part_data)),
        variable_replacement_strategy: v!(extract_part_common_variable_replacement_strategy(
            &qp.part_data
        )),
        adaptive_marking_penalty: v!(extract_part_common_adaptive_marking_penalty(&qp.part_data)),
        custom_marking_algorithm: v!(extract_part_common_custom_marking_algorithm(&qp.part_data)),
        extend_base_marking_algorithm: v!(extract_part_common_extend_base_marking_algorithm(
            &qp.part_data
        )),
        steps: v!(extract_part_common_steps(&qp.part_data)),
    }) // TODO
}
/* TODO
fn extract_extension_part(qp: &numbas::exam::ExamQuestionPart) -> QuestionPart {
    QuestionPart::Extension(None) // TODO
}*/

fn extract_part(qp: &numbas::exam::ExamQuestionPart) -> Option<QuestionPart> {
    match qp {
        numbas::exam::ExamQuestionPart::JME(p) => Some(extract_jme_part(p)),
        numbas::exam::ExamQuestionPart::NumberEntry(p) => Some(extract_number_entry_part(p)),
        numbas::exam::ExamQuestionPart::Matrix(p) => None, //extract_matrix_part(p),
        numbas::exam::ExamQuestionPart::PatternMatch(p) => Some(extract_pattern_match_part(p)),
        numbas::exam::ExamQuestionPart::ChooseOne(p) => Some(extract_choose_one_part(p)),
        numbas::exam::ExamQuestionPart::ChooseMultiple(p) => Some(extract_choose_multiple_part(p)),
        numbas::exam::ExamQuestionPart::MatchAnswersWithChoices(p) => {
            Some(extract_match_answers_with_choices_part(p))
        }
        numbas::exam::ExamQuestionPart::GapFill(p) => Some(extract_gapfill_part(p)),
        numbas::exam::ExamQuestionPart::Information(p) => Some(extract_information_part(p)),
        numbas::exam::ExamQuestionPart::Extension(p) => None, // extract_extension_part(p),
    }
}

fn extract_question_groups(exam: &NExam) -> Vec<Value<QuestionGroup>> {
    exam.clone()
        .question_groups
        .into_iter()
        .map(|q| {
            v!(QuestionGroup {
                name: v!(ts!(q.name.unwrap_or(String::new()))),
                picking_strategy: v!(match q.picking_strategy {
                    numbas::exam::ExamQuestionGroupPickingStrategy::AllOrdered => {
                        PickingStrategy::AllOrdered
                    }
                    numbas::exam::ExamQuestionGroupPickingStrategy::AllShuffled => {
                        PickingStrategy::AllShuffled
                    }
                    numbas::exam::ExamQuestionGroupPickingStrategy::RandomSubset {
                        pick_questions,
                    } => {
                        PickingStrategy::RandomSubset { pick_questions }
                    }
                }),
                questions: v!(q
                    .questions
                    .clone()
                    .into_iter()
                    .map(|q| {
                        v!(QuestionPath {
                            question_name: v!(q.name),
                            question_data: v!(Question {
                                statement: v!(ts!(q.statement)),
                                advice: v!(ts!(q.advice)),
                                parts: v!(q
                                    .parts
                                    .iter()
                                    .map(|p| extract_part(p))
                                    .filter(|p| p.is_some())
                                    .map(|p| v!(p.unwrap()))
                                    .collect()), // TODO remove unwrap (remove option above)
                                builtin_constants: v!(extract_builtin_constants(
                                    q.builtin_constants
                                )),
                                custom_constants: v!(q
                                    .constants
                                    .iter()
                                    .map(|cc| CustomConstant {
                                        name: v!(cc.name.clone()),
                                        value: v!(cc.value.clone()),
                                        tex: v!(cc.tex.clone()),
                                    })
                                    .collect()),
                                variables: v!(q
                                    .variables
                                    .iter()
                                    .map(|(k, v)| (
                                        k.clone(),
                                        v!(VariableRepresentation::Long(v!(Variable {
                                            definition: v!(FileString::s(&v.definition)),
                                            description: v!(v.description.clone()),
                                            template_type: v!(extract_variable_template_type(
                                                v.template_type.clone()
                                            )),
                                            group: v!(v.group.clone()),
                                        })))
                                    ))
                                    .collect::<std::collections::HashMap<_, _>>()),
                                variables_test: v!(VariablesTest {
                                    condition: v!(q.variables_test.condition.clone()),
                                    max_runs: v!(q.variables_test.max_runs.0)
                                }),
                                functions: v!(q
                                    .functions
                                    .iter()
                                    .map(|(k, f)| (
                                        k.clone(),
                                        v!(Function {
                                            definition: v!(FileString::s(&f.definition)),
                                            output_type: v!(f.output_type),
                                            language: v!(f.language),
                                            parameters: v!(f
                                                .parameters
                                                .clone()
                                                .into_iter()
                                                .map(|a| a)
                                                .collect())
                                        })
                                    ))
                                    .collect::<std::collections::HashMap<_, _>>()),
                                preamble: v!(Preamble {
                                    js: v!(FileString::s(&q.preamble.js)),
                                    css: v!(FileString::s(&q.preamble.css)),
                                }), // TODO
                                navigation: v!(QuestionNavigation {
                                    can_regenerate: v!(q.navigation.allow_regenerate),
                                    show_title_page: v!(q.navigation.show_frontpage),
                                    prevent_leaving: v!(q
                                        .navigation
                                        .prevent_leaving
                                        .unwrap_or(false)), // TODO: check default
                                }),
                                extensions: v!(Extensions {
                                    jsx_graph: v!(q.extensions.contains(&"jsx_graph".to_string())),
                                    stats: v!(q.extensions.contains(&"stats".to_string())),
                                }),
                                diagnostic_topic_names: v!(q
                                    .tags
                                    .iter()
                                    .filter(|t| t.starts_with("skill: "))
                                    .map(|t| ts!(
                                        t.splitn(2, ": ").collect::<Vec<_>>()[1].to_string()
                                    ))
                                    .collect()),
                            })
                        })
                    })
                    .collect())
            })
        })
        .collect()
}

fn extract_diagnostic(exam: &NExam) -> Diagnostic {
    let diagnostic = exam.diagnostic.clone().unwrap();
    Diagnostic {
        script: v!(match diagnostic.script {
            numbas::exam::ExamDiagnosticScript::Mastery => DiagnosticScript::Mastery,
            numbas::exam::ExamDiagnosticScript::Diagnosys => DiagnosticScript::Diagnosys,
            numbas::exam::ExamDiagnosticScript::Custom => {
                DiagnosticScript::Custom(ts!(diagnostic.custom_script))
            }
        }),
        objectives: v!(diagnostic
            .knowledge_graph
            .clone()
            .learning_objectives
            .into_iter()
            .map(|l| LearningObjective {
                name: v!(ts!(l.name)),
                description: v!(ts!(l.description))
            })
            .collect()),
        topics: v!(diagnostic
            .knowledge_graph
            .clone()
            .topics
            .into_iter()
            .map(|l| LearningTopic {
                name: v!(ts!(l.name)),
                description: v!(ts!(l.description)),
                objectives: v!(l
                    .learning_objectives
                    .clone()
                    .into_iter()
                    .map(|o| ts!(o))
                    .collect()),
                depends_on: v!(l.depends_on.clone().into_iter().map(|o| ts!(o)).collect()),
            })
            .collect()),
    }
}
