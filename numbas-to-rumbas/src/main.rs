use numbas::exam::Exam as NExam;
use rumbas::data::custom_part_type::CustomPartTypeDefinitionPath;
use rumbas::data::diagnostic_exam::{
    Diagnostic, DiagnosticExam, DiagnosticScript, LearningObjective, LearningTopic,
};
use rumbas::data::extension::Extensions;
use rumbas::data::extension::QuestionPartExtension;
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
use rumbas::data::matrix::{
    QuestionPartMatrix, QuestionPartMatrixDimension, QuestionPartMatrixDimensions,
};
use rumbas::data::multiple_choice::{
    ChooseOneDisplay, MatchAnswersItem, MatchAnswersItemMarks, MultipleChoiceAnswer,
    MultipleChoiceAnswerData, MultipleChoiceAnswerDataNumbasLike, MultipleChoiceMatchAnswerData,
    MultipleChoiceMatchAnswerDataNumbasLike, MultipleChoiceMatchAnswers,
    QuestionPartChooseMultiple, QuestionPartChooseOne, QuestionPartMatchAnswersWithItems,
};
use rumbas::data::navigation::{
    DiagnosticNavigation, LeaveAction, MenuNavigation, NavigationSharedData, NormalNavigation,
    QuestionNavigation, SequentialNavigation, ShowResultsPage,
};
use rumbas::data::normal_exam::NormalExam;
use rumbas::data::numbas_settings::NumbasSettings;
use rumbas::data::number_entry::{NumberEntryAnswer, QuestionPartNumberEntry};
use rumbas::data::optional_overwrite::Noneable;
use rumbas::data::pattern_match::QuestionPartPatternMatch;
use rumbas::data::preamble::Preamble;
use rumbas::data::question::{BuiltinConstants, CustomConstant, Question, VariablesTest};
use rumbas::data::question_group::{PickingStrategy, QuestionGroup, QuestionPath};
use rumbas::data::question_part::{QuestionPart, VariableReplacementStrategy};
use rumbas::data::question_part::{QuestionPartBuiltin, QuestionPartCustom};
use rumbas::data::template::ExamFileType;
use rumbas::data::template::QuestionFileType;
use rumbas::data::template::Value;
use rumbas::data::timing::{TimeoutAction, Timing};
use rumbas::data::to_rumbas::ToRumbas;
use rumbas::data::translatable::TranslatableString;
use rumbas::data::variable::{Variable, VariableRepresentation, VariableTemplateType};
use sanitize_filename::sanitize;

pub struct NumbasDefaults {
    navigation_reverse: bool,
    navigation_browsing_enabled: bool,
    navigation_allow_steps: bool,
    navigation_prevent_leaving: bool,
    navigation_show_results_page: numbas::exam::ExamShowResultsPage,
    navigation_show_names_of_question_groups: bool,
    navigation_start_password: String,
    navigation_on_leave: LeaveAction,
    question_navigation_prevent_leaving: bool,
    basic_settings_show_student_name: bool,
    basic_settings_allow_printing: bool,
    feedback_review_show_score: bool,
    feedback_review_show_feedback: bool,
    feedback_review_show_expected_answer: bool,
    feedback_review_show_advice: bool,
    builtin_constants_e: bool,
    builtin_constants_i: bool,
    builtin_constants_pi: bool,
    part_common_marks: usize,
    part_common_use_custom_name: bool,
    part_common_steps_penalty: usize,
    part_common_enable_minimum_marks: bool,
    part_common_minimum_marks: usize,
    part_common_show_feedback_icon: bool,
    part_common_adaptive_marking_penalty: usize,
    part_common_extend_base_marking_algorithm: bool,
    number_entry_correct_answer_style: numbas::exam::AnswerStyle,
    length_restriction_length: usize,
    number_entry_fractions_must_be_reduced: bool,
    number_entry_partial_credit_if_fraction_not_reduced: numbas::exam::Primitive,
    number_entry_hint_fraction: bool,

    choose_one_has_to_select_option: bool,
    choose_one_show_cell_answer_state: bool,
    gapfill_sort_answers: bool,
    match_answers_with_items_min_answers: usize,
    choose_multiple_min_answers: usize,

    jme_single_letter_variables: bool,
    jme_allow_unknown_functions: bool,
    jme_implicit_function_composition: bool,
    jme_simplification_simplify_basic: bool,
    jme_simplification_simplify_unit_factor: bool,
    jme_simplification_simplify_unit_power: bool,
    jme_simplification_simplify_unit_denominator: bool,
    jme_simplification_simplify_zero_factor: bool,
    jme_simplification_simplify_zero_term: bool,
    jme_simplification_simplify_zero_power: bool,
    jme_simplification_simplify_zero_base: bool,
    jme_simplification_collect_numbers: bool,
    jme_simplification_constants_first: bool,
    jme_simplification_simplify_sqrt_products: bool,
    jme_simplification_simplify_sqrt_division: bool,
    jme_simplification_simplify_sqrt_square: bool,
    jme_simplification_simplify_other_numbers: bool,
    jme_simplification_simplify_no_leading_minus: bool,
    jme_simplification_simplify_fractions: bool,
    jme_simplification_simplify_trigonometric: bool,
    jme_simplification_cancel_terms: bool,
    jme_simplification_cancel_factors: bool,
    jme_simplification_collect_like_fractions: bool,
    jme_simplification_order_canonical: bool,
    jme_simplification_use_times_dot: bool,
    jme_simplification_expand_brackets: bool,
}

// TODO default values
const DEFAULTS: NumbasDefaults = NumbasDefaults {
    navigation_reverse: true,
    navigation_browsing_enabled: true,
    navigation_allow_steps: true,
    navigation_prevent_leaving: true,
    navigation_show_results_page: numbas::exam::ExamShowResultsPage::Never,
    navigation_show_names_of_question_groups: true,
    navigation_start_password: String::new(),
    navigation_on_leave: LeaveAction::None,
    question_navigation_prevent_leaving: true,
    basic_settings_show_student_name: true,
    basic_settings_allow_printing: true,
    feedback_review_show_score: true,
    feedback_review_show_feedback: true,
    feedback_review_show_expected_answer: true,
    feedback_review_show_advice: true,
    builtin_constants_e: false,
    builtin_constants_i: false,
    builtin_constants_pi: false,
    part_common_marks: 0,
    part_common_use_custom_name: false,
    part_common_steps_penalty: 0,
    part_common_enable_minimum_marks: true,
    part_common_minimum_marks: 0,
    part_common_show_feedback_icon: true,
    part_common_adaptive_marking_penalty: 0,
    part_common_extend_base_marking_algorithm: true,
    number_entry_correct_answer_style: numbas::exam::AnswerStyle::EnglishPlain,
    length_restriction_length: 0,

    choose_one_has_to_select_option: true,
    choose_one_show_cell_answer_state: true,
    gapfill_sort_answers: true,
    match_answers_with_items_min_answers: 0,
    choose_multiple_min_answers: 0,

    number_entry_fractions_must_be_reduced: false,
    number_entry_partial_credit_if_fraction_not_reduced: numbas::exam::Primitive::Natural(0),
    number_entry_hint_fraction: true,

    jme_single_letter_variables: false,
    jme_allow_unknown_functions: false,
    jme_implicit_function_composition: false,
    jme_simplification_simplify_basic: true,
    jme_simplification_simplify_unit_factor: true,
    jme_simplification_simplify_unit_power: true,
    jme_simplification_simplify_unit_denominator: true,
    jme_simplification_simplify_zero_factor: true,
    jme_simplification_simplify_zero_term: true,
    jme_simplification_simplify_zero_power: true,
    jme_simplification_simplify_zero_base: true,
    jme_simplification_collect_numbers: true,
    jme_simplification_constants_first: true,
    jme_simplification_simplify_sqrt_products: true,
    jme_simplification_simplify_sqrt_division: true,
    jme_simplification_simplify_sqrt_square: true,
    jme_simplification_simplify_other_numbers: true,
    jme_simplification_simplify_no_leading_minus: true,
    jme_simplification_simplify_fractions: true,
    jme_simplification_simplify_trigonometric: true,
    jme_simplification_cancel_terms: true,
    jme_simplification_cancel_factors: true,
    jme_simplification_collect_like_fractions: true,
    jme_simplification_order_canonical: false,
    jme_simplification_use_times_dot: true,
    jme_simplification_expand_brackets: false,
};

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
        TranslatableString::NotTranslated(FileString::s(&$s))
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
            let (name, rumbas_exam, qs, cpts) = convert_exam(exam);
            for qp in qs.into_iter() {
                let q_name = qp.question_name.clone().unwrap();
                let q_yaml = QuestionFileType::Normal(qp.question_data.unwrap())
                    .to_yaml()
                    .unwrap();
                let file = format!("output/questions/{}.yaml", q_name);
                println!("Writing to {}", file);
                std::fs::write(file, q_yaml).unwrap(); //fix handle result
            }
            for cpt in cpts.into_iter() {
                let c_name = cpt.custom_part_type_name.clone();
                let c_yaml = cpt.custom_part_type_data.to_yaml().unwrap();
                let file = format!("output/custom_part_types/{}.yaml", c_name);
                println!("Writing to {}", file);
                std::fs::write(file, c_yaml).unwrap(); //fix handle result
            }
            let exam_yaml = rumbas_exam.to_yaml().unwrap();
            std::fs::write(format!("output/exams/{}.yaml", name), exam_yaml).unwrap();
            //fix handle result
        }
        Err(e) => {
            eprintln!("Error: {:?}", e);
        }
    }
}

fn convert_exam(
    exam: NExam,
) -> (
    String,
    ExamFileType,
    Vec<QuestionPath>,
    Vec<CustomPartTypeDefinitionPath>,
) {
    let (name, exam, qgs, cpts) = match exam.navigation.navigation_mode {
        numbas::exam::ExamNavigationMode::Diagnostic => {
            let (exam, qgs, cpts) = convert_diagnostic_exam(exam);
            (
                exam.name.clone().unwrap(),
                ExamFileType::Diagnostic(exam),
                qgs,
                cpts,
            )
        }
        _ => {
            let (exam, qgs, cpts) = convert_normal_exam(exam);
            (
                exam.name.clone().unwrap(),
                ExamFileType::Normal(exam),
                qgs,
                cpts,
            )
        }
    };
    (
        {
            if let TranslatableString::NotTranslated(n) = name {
                n.get_content(&String::new())
            } else {
                panic!("Should not happen");
            }
        },
        exam,
        qgs,
        cpts,
    )
}

fn convert_normal_exam(
    exam: NExam,
) -> (
    NormalExam,
    Vec<QuestionPath>,
    Vec<CustomPartTypeDefinitionPath>,
) {
    let question_groups = extract_question_groups(&exam);
    let custom_part_types = exam.custom_part_types.to_rumbas();
    (
        NormalExam {
            locales: v!(vec![v!(Locale {
                name: v!("en".to_string()),
                numbas_locale: v!(SupportedLocale::EnGB)
            })]), // todo: argument?
            name: v![ts!("todo".to_string())], // todo: argument
            navigation: v![extract_normal_navigation(&exam).unwrap()],
            timing: v![extract_timing(&exam)],
            feedback: v![extract_feedback(&exam)],
            question_groups: v![question_groups.clone()],
            numbas_settings: v![NumbasSettings {
                locale: v!(SupportedLocale::EnGB),
                theme: v!("default".to_string())
            }], // todo: argument?
            custom_part_types: v!(custom_part_types.clone()),
        },
        question_groups
            .into_iter()
            .flat_map(|qg| {
                qg.unwrap()
                    .questions
                    .unwrap()
                    .into_iter()
                    .map(|q| q.unwrap())
            })
            .collect(),
        custom_part_types,
    )
}

fn convert_diagnostic_exam(
    exam: NExam,
) -> (
    DiagnosticExam,
    Vec<QuestionPath>,
    Vec<CustomPartTypeDefinitionPath>,
) {
    let question_groups = extract_question_groups(&exam);
    let custom_part_types = exam.custom_part_types.to_rumbas();
    (
        DiagnosticExam {
            locales: v!(vec![v!(Locale {
                name: v!("en".to_string()),
                numbas_locale: v!(SupportedLocale::EnGB)
            })]), // todo: argument?
            name: v![ts!("todo".to_string())], // todo: argument
            navigation: v![extract_diagnostic_navigation(&exam)],
            timing: v![extract_timing(&exam)],
            feedback: v![extract_feedback(&exam)],
            question_groups: v![question_groups.clone()],
            numbas_settings: v![NumbasSettings {
                locale: v!(SupportedLocale::EnGB),
                theme: v!("default".to_string())
            }], // todo: argument?
            diagnostic: v![extract_diagnostic(&exam)],
            custom_part_types: v!(custom_part_types.clone()),
        },
        question_groups
            .into_iter()
            .flat_map(|qg| {
                qg.unwrap()
                    .questions
                    .unwrap()
                    .into_iter()
                    .map(|q| q.unwrap())
            })
            .collect(),
        custom_part_types,
    )
}

fn extract_shared_navigation(exam: &NExam) -> NavigationSharedData {
    NavigationSharedData {
        start_password: v!(FileString::s(
            &exam
                .navigation
                .start_password
                .clone()
                .unwrap_or(DEFAULTS.navigation_start_password.clone())
        )),
        can_regenerate: v!(exam.navigation.allow_regenerate),
        show_steps: v!(exam
            .navigation
            .allow_steps
            .unwrap_or(DEFAULTS.navigation_allow_steps)),
        show_title_page: v!(exam.navigation.show_frontpage),
        prevent_leaving: v!(exam
            .navigation
            .prevent_leaving
            .unwrap_or(DEFAULTS.navigation_prevent_leaving)),
        show_names_of_question_groups: v!(exam
            .basic_settings
            .show_question_group_names
            .unwrap_or(DEFAULTS.navigation_show_names_of_question_groups)),
        allow_printing: v!(exam
            .basic_settings
            .allow_printing
            .unwrap_or(DEFAULTS.basic_settings_allow_printing)),
    }
}

fn extract_sequential_navigation_show_results_page(
    v: numbas::exam::ExamShowResultsPage,
) -> ShowResultsPage {
    match v {
        numbas::exam::ExamShowResultsPage::Never => ShowResultsPage::Never,
        numbas::exam::ExamShowResultsPage::OnCompletion => ShowResultsPage::OnCompletion,
    }
}

fn extract_normal_navigation(exam: &NExam) -> Option<NormalNavigation> {
    match exam.navigation.navigation_mode {
        numbas::exam::ExamNavigationMode::Sequence => {
            Some(NormalNavigation::Sequential(SequentialNavigation {
                shared_data: v!(extract_shared_navigation(exam)),
                can_move_to_previous: v!(exam
                    .navigation
                    .reverse
                    .unwrap_or(DEFAULTS.navigation_reverse)),
                browsing_enabled: v!(exam
                    .navigation
                    .browsing_enabled
                    .unwrap_or(DEFAULTS.navigation_browsing_enabled)),
                show_results_page: v!(extract_sequential_navigation_show_results_page(
                    exam.navigation
                        .show_results_page
                        .clone()
                        .unwrap_or(DEFAULTS.navigation_show_results_page)
                )),
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
                    .unwrap_or(DEFAULTS.navigation_on_leave)),
            }))
        }
        numbas::exam::ExamNavigationMode::Menu => Some(NormalNavigation::Menu(MenuNavigation {
            shared_data: v!(extract_shared_navigation(exam)),
        })),
        numbas::exam::ExamNavigationMode::Diagnostic => None,
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
            .unwrap_or(DEFAULTS.navigation_on_leave)),
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
        percentage_needed_to_pass: v!(exam
            .basic_settings
            .percentage_needed_to_pass
            .map(|p| Noneable::NotNone(p))
            .unwrap_or(nn())),
        show_name_of_student: v!(exam
            .basic_settings
            .show_student_name
            .unwrap_or(DEFAULTS.basic_settings_show_student_name)),
        show_current_marks: v!(exam.feedback.show_actual_mark),
        show_maximum_marks: v!(exam.feedback.show_total_mark),
        show_answer_state: v!(exam.feedback.show_answer_state),
        allow_reveal_answer: v!(exam.feedback.allow_reveal_answer),
        review: v!(Review {
            show_score: v!(exam
                .feedback
                .review
                .clone()
                .map(|r| r.show_score.unwrap_or(DEFAULTS.feedback_review_show_score))
                .unwrap()),
            show_feedback: v!(exam
                .feedback
                .review
                .clone()
                .map(|r| r
                    .show_feedback
                    .unwrap_or(DEFAULTS.feedback_review_show_feedback))
                .unwrap()),
            show_expected_answer: v!(exam
                .feedback
                .review
                .clone()
                .map(|r| r
                    .show_expected_answer
                    .unwrap_or(DEFAULTS.feedback_review_show_expected_answer))
                .unwrap()),
            show_advice: v!(exam
                .feedback
                .review
                .clone()
                .map(|r| r
                    .show_advice
                    .unwrap_or(DEFAULTS.feedback_review_show_advice))
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
        e: v!(*bc
            .0
            .get(&"e".to_string())
            .unwrap_or(&DEFAULTS.builtin_constants_e)),
        pi: v!(*bc
            .0
            .get(&"pi,\u{03c0}".to_string())
            .unwrap_or(&DEFAULTS.builtin_constants_pi)),
        i: v!(*bc
            .0
            .get(&"i".to_string())
            .unwrap_or(&DEFAULTS.builtin_constants_i)),
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
        simplify_basic: v!(DEFAULTS.jme_simplification_simplify_basic),
        simplify_unit_factor: v!(DEFAULTS.jme_simplification_simplify_unit_factor),
        simplify_unit_power: v!(DEFAULTS.jme_simplification_simplify_unit_power),
        simplify_unit_denominator: v!(DEFAULTS.jme_simplification_simplify_unit_denominator),
        simplify_zero_factor: v!(DEFAULTS.jme_simplification_simplify_zero_factor),
        simplify_zero_term: v!(DEFAULTS.jme_simplification_simplify_zero_term),
        simplify_zero_power: v!(DEFAULTS.jme_simplification_simplify_zero_power),
        simplify_zero_base: v!(DEFAULTS.jme_simplification_simplify_zero_base),
        collect_numbers: v!(DEFAULTS.jme_simplification_collect_numbers),
        constants_first: v!(DEFAULTS.jme_simplification_constants_first),
        simplify_sqrt_products: v!(DEFAULTS.jme_simplification_simplify_sqrt_products),
        simplify_sqrt_division: v!(DEFAULTS.jme_simplification_simplify_sqrt_division),
        simplify_sqrt_square: v!(DEFAULTS.jme_simplification_simplify_sqrt_square),
        simplify_other_numbers: v!(DEFAULTS.jme_simplification_simplify_other_numbers),
        simplify_no_leading_minus: v!(DEFAULTS.jme_simplification_simplify_no_leading_minus),
        simplify_fractions: v!(DEFAULTS.jme_simplification_simplify_fractions),
        simplify_trigonometric: v!(DEFAULTS.jme_simplification_simplify_trigonometric),
        cancel_terms: v!(DEFAULTS.jme_simplification_cancel_terms),
        cancel_factors: v!(DEFAULTS.jme_simplification_cancel_factors),
        collect_like_fractions: v!(DEFAULTS.jme_simplification_collect_like_fractions),
        order_canonical: v!(DEFAULTS.jme_simplification_order_canonical),
        use_times_dot: v!(DEFAULTS.jme_simplification_use_times_dot),
        expand_brackets: v!(DEFAULTS.jme_simplification_expand_brackets),
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
    match ct {
        numbas::exam::JMECheckingType::RelativeDifference(v) => {
            CheckingType::RelativeDifference(CheckingTypeDataFloat {
                checking_accuracy: v!(v.checking_accuracy),
            })
        }
        numbas::exam::JMECheckingType::AbsoluteDifference(v) => {
            CheckingType::AbsoluteDifference(CheckingTypeDataFloat {
                checking_accuracy: v!(v.checking_accuracy),
            })
        }
        numbas::exam::JMECheckingType::DecimalPlaces(v) => {
            CheckingType::DecimalPlaces(CheckingTypeDataNatural {
                checking_accuracy: v!(v.checking_accuracy),
            })
        }
        numbas::exam::JMECheckingType::SignificantFigures(v) => {
            CheckingType::SignificantFigures(CheckingTypeDataNatural {
                checking_accuracy: v!(v.checking_accuracy),
            })
        }
    }
}

fn extract_part_common_marks(
    pd: &numbas::exam::ExamQuestionPartSharedData,
) -> numbas::exam::Primitive {
    pd.marks
        .clone()
        .unwrap_or(numbas::exam::Primitive::Natural(DEFAULTS.part_common_marks))
}

fn extract_part_common_prompt(pd: &numbas::exam::ExamQuestionPartSharedData) -> String {
    pd.prompt.clone().unwrap_or(String::new())
}

fn extract_part_common_use_custom_name(pd: &numbas::exam::ExamQuestionPartSharedData) -> bool {
    pd.use_custom_name
        .unwrap_or(DEFAULTS.part_common_use_custom_name)
}
fn extract_part_common_custom_name(pd: &numbas::exam::ExamQuestionPartSharedData) -> String {
    pd.custom_name.clone().unwrap_or(String::new())
}
fn extract_part_common_steps_penalty(pd: &numbas::exam::ExamQuestionPartSharedData) -> usize {
    pd.steps_penalty
        .unwrap_or(DEFAULTS.part_common_steps_penalty)
}
fn extract_part_common_enable_minimum_marks(pd: &numbas::exam::ExamQuestionPartSharedData) -> bool {
    pd.enable_minimum_marks
        .unwrap_or(DEFAULTS.part_common_enable_minimum_marks)
}
fn extract_part_common_minimum_marks(pd: &numbas::exam::ExamQuestionPartSharedData) -> usize {
    pd.minimum_marks
        .unwrap_or(DEFAULTS.part_common_minimum_marks)
}
fn extract_part_common_show_correct_answer(pd: &numbas::exam::ExamQuestionPartSharedData) -> bool {
    pd.show_correct_answer
}
fn extract_part_common_show_feedback_icon(pd: &numbas::exam::ExamQuestionPartSharedData) -> bool {
    pd.show_feedback_icon
        .unwrap_or(DEFAULTS.part_common_show_feedback_icon)
}
fn extract_part_common_variable_replacement_strategy(
    pd: &numbas::exam::ExamQuestionPartSharedData,
) -> VariableReplacementStrategy {
    match pd.variable_replacement_strategy {
        numbas::exam::VariableReplacementStrategy::OriginalFirst => {
            VariableReplacementStrategy::OriginalFirst
        }
    }
}
fn extract_part_common_adaptive_marking_penalty(
    pd: &numbas::exam::ExamQuestionPartSharedData,
) -> usize {
    pd.adaptive_marking_penalty
        .unwrap_or(DEFAULTS.part_common_adaptive_marking_penalty)
}
fn extract_part_common_custom_marking_algorithm(
    pd: &numbas::exam::ExamQuestionPartSharedData,
) -> String {
    pd.custom_marking_algorithm.clone().unwrap_or(String::new())
}
fn extract_part_common_extend_base_marking_algorithm(
    pd: &numbas::exam::ExamQuestionPartSharedData,
) -> bool {
    pd.extend_base_marking_algorithm
        .unwrap_or(DEFAULTS.part_common_extend_base_marking_algorithm)
}
fn extract_part_common_steps(pd: &numbas::exam::ExamQuestionPartSharedData) -> Vec<QuestionPart> {
    pd.steps
        .clone()
        .unwrap_or(vec![])
        .into_iter()
        .map(|s| extract_part(&s))
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
        length: v!(r.length.unwrap_or(DEFAULTS.length_restriction_length)),
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

fn extract_jme_part(qp: &numbas::exam::ExamQuestionPartJME) -> QuestionPartBuiltin {
    QuestionPartBuiltin::JME(QuestionPartJME {
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
        checking_type: v!(extract_checking_type(&qp.checking_type)),
        failure_rate: v!(qp.failure_rate),
        vset_range: v!([qp.vset_range[0].0, qp.vset_range[1].0]),
        vset_range_points: v!(qp.vset_range_points.0),
        check_variable_names: v!(qp.check_variable_names),
        single_letter_variables: v!(qp
            .single_letter_variables
            .unwrap_or(DEFAULTS.jme_single_letter_variables)),
        allow_unknown_functions: v!(qp
            .allow_unknown_functions
            .unwrap_or(DEFAULTS.jme_allow_unknown_functions)),
        implicit_function_composition: v!(qp
            .implicit_function_composition
            .unwrap_or(DEFAULTS.jme_implicit_function_composition)),

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

fn extract_number_entry_part(
    qp: &numbas::exam::ExamQuestionPartNumberEntry,
) -> QuestionPartBuiltin {
    QuestionPartBuiltin::NumberEntry(QuestionPartNumberEntry {
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
        allowed_notation_styles: v!(qp.notation_styles.clone().unwrap_or(vec![]).to_rumbas()),
        display_correct_in_style: v!(qp
            .correct_answer_style
            .clone()
            .unwrap_or(DEFAULTS.number_entry_correct_answer_style)
            .to_rumbas()),

        fractions_must_be_reduced: v!(qp
            .fractions_must_be_reduced
            .unwrap_or(DEFAULTS.number_entry_fractions_must_be_reduced)),
        partial_credit_if_fraction_not_reduced: v!(qp
            .partial_credit_if_fraction_not_reduced
            .clone()
            .unwrap_or(DEFAULTS.number_entry_partial_credit_if_fraction_not_reduced)),
        hint_fraction: v!(qp
            .show_fraction_hint
            .unwrap_or(DEFAULTS.number_entry_hint_fraction)),
    })
}

/*impl ToRumbas for numbas::exam::ExamQuestionPartMatrix {
type RumbasType = QuestionPartMatrix;
fn to_rumbas(&self) -> Self::RumbasType {*/
fn extract_matrix_part(sel: &numbas::exam::ExamQuestionPartMatrix) -> QuestionPartBuiltin {
    QuestionPartBuiltin::Matrix({
        let rows = v!(QuestionPartMatrixDimension::from_range(
            sel.num_rows.0,
            sel.min_rows,
            sel.max_rows
        ));
        let columns = v!(QuestionPartMatrixDimension::from_range(
            sel.num_columns.0,
            sel.min_columns,
            sel.max_columns,
        ));
        let dimensions = QuestionPartMatrixDimensions { rows, columns };
        QuestionPartMatrix {
            // Default section
            marks: v!(extract_part_common_marks(&sel.part_data)),
            prompt: v!(ts!(extract_part_common_prompt(&sel.part_data))),
            use_custom_name: v!(extract_part_common_use_custom_name(&sel.part_data)),
            custom_name: v!(extract_part_common_custom_name(&sel.part_data)),
            steps_penalty: v!(extract_part_common_steps_penalty(&sel.part_data)),
            enable_minimum_marks: v!(extract_part_common_enable_minimum_marks(&sel.part_data)),
            minimum_marks: v!(extract_part_common_minimum_marks(&sel.part_data)),
            show_correct_answer: v!(extract_part_common_show_correct_answer(&sel.part_data)),
            show_feedback_icon: v!(extract_part_common_show_feedback_icon(&sel.part_data)),
            variable_replacement_strategy: v!(extract_part_common_variable_replacement_strategy(
                &sel.part_data
            )),
            adaptive_marking_penalty: v!(extract_part_common_adaptive_marking_penalty(
                &sel.part_data
            )),
            custom_marking_algorithm: v!(extract_part_common_custom_marking_algorithm(
                &sel.part_data
            )),
            extend_base_marking_algorithm: v!(extract_part_common_extend_base_marking_algorithm(
                &sel.part_data
            )),
            steps: v!(extract_part_common_steps(&sel.part_data)),

            correct_answer: v!(sel.correct_answer.clone()),
            display_correct_as_fraction: v!(sel.correct_answer_fractions),
            dimensions: v!(dimensions),
            max_absolute_deviation: v!(sel.tolerance),
            mark_partial_by_cells: v!(sel.mark_per_cell),
            allow_fractions: v!(sel.allow_fractions),
        }
    })
}
//}

fn extract_pattern_match_part(
    qp: &numbas::exam::ExamQuestionPartPatternMatch,
) -> QuestionPartBuiltin {
    QuestionPartBuiltin::PatternMatch(QuestionPartPatternMatch {
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

fn extract_choose_one_part(qp: &numbas::exam::ExamQuestionPartChooseOne) -> QuestionPartBuiltin {
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
    QuestionPartBuiltin::ChooseOne(QuestionPartChooseOne {
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
        show_cell_answer_state: v!(qp
            .show_cell_answer_state
            .unwrap_or(DEFAULTS.choose_one_show_cell_answer_state)),
        has_to_select_option: v!(qp
            .min_answers
            .map(|v| v == 1)
            .unwrap_or(DEFAULTS.choose_one_has_to_select_option)),
    })
}

fn extract_choose_multiple_part(
    qp: &numbas::exam::ExamQuestionPartChooseMultiple,
) -> QuestionPartBuiltin {
    // todo: less duplicate code?: Extract following as function
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
    QuestionPartBuiltin::ChooseMultiple(QuestionPartChooseMultiple {
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
        should_select_at_least: v!(qp
            .min_answers
            .unwrap_or(DEFAULTS.choose_multiple_min_answers)),
        should_select_at_most: v!(qp
            .max_answers
            .map(|ma| Noneable::NotNone(ma))
            .unwrap_or(nn())),
        columns: v!(qp.display_columns.0),
    })
}

fn extract_match_answers_with_choices_part(
    qp: &numbas::exam::ExamQuestionPartMatchAnswersWithChoices,
) -> QuestionPartBuiltin {
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
    QuestionPartBuiltin::MatchAnswersWithItems(QuestionPartMatchAnswersWithItems {
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
        should_select_at_least: v!(qp
            .min_answers
            .unwrap_or(DEFAULTS.match_answers_with_items_min_answers)),
        should_select_at_most: v!(qp
            .max_answers
            .map(|ma| Noneable::NotNone(ma))
            .unwrap_or(nn())),
        display: v!(qp.display_type.to_rumbas()),
        layout: v!(qp.layout.clone()),
    })
}

fn extract_gapfill_part(qp: &numbas::exam::ExamQuestionPartGapFill) -> QuestionPartBuiltin {
    QuestionPartBuiltin::GapFill(QuestionPartGapFill {
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

        sort_answers: v!(qp.sort_answers.unwrap_or(DEFAULTS.gapfill_sort_answers)),

        gaps: v!(qp.gaps.iter().map(|s| extract_part(&s)).collect()),
    })
}

fn extract_information_part(qp: &numbas::exam::ExamQuestionPartInformation) -> QuestionPartBuiltin {
    QuestionPartBuiltin::Information(QuestionPartInformation {
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
    })
}
fn extract_extension_part(qp: &numbas::exam::ExamQuestionPartExtension) -> QuestionPartBuiltin {
    QuestionPartBuiltin::Extension(QuestionPartExtension {
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
    })
}

fn extract_custom_part(qp: &numbas::exam::ExamQuestionPartCustom) -> QuestionPartCustom {
    QuestionPartCustom {
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

        r#type: v!(qp.r#type.clone()),
        settings: v!(qp
            .settings
            .clone()
            .into_iter()
            .map(|(k, v)| (k, v.to_rumbas()))
            .collect()),
    }
}

fn extract_part(qp: &numbas::exam::ExamQuestionPart) -> QuestionPart {
    match qp {
        numbas::exam::ExamQuestionPart::Builtin(bqp) => QuestionPart::Builtin(match bqp {
            numbas::exam::ExamQuestionPartBuiltin::JME(p) => extract_jme_part(p),
            numbas::exam::ExamQuestionPartBuiltin::NumberEntry(p) => extract_number_entry_part(p),
            numbas::exam::ExamQuestionPartBuiltin::Matrix(p) => extract_matrix_part(p),
            numbas::exam::ExamQuestionPartBuiltin::PatternMatch(p) => extract_pattern_match_part(p),
            numbas::exam::ExamQuestionPartBuiltin::ChooseOne(p) => extract_choose_one_part(p),
            numbas::exam::ExamQuestionPartBuiltin::ChooseMultiple(p) => {
                extract_choose_multiple_part(p)
            }
            numbas::exam::ExamQuestionPartBuiltin::MatchAnswersWithChoices(p) => {
                extract_match_answers_with_choices_part(p)
            }
            numbas::exam::ExamQuestionPartBuiltin::GapFill(p) => extract_gapfill_part(p),
            numbas::exam::ExamQuestionPartBuiltin::Information(p) => extract_information_part(p),
            numbas::exam::ExamQuestionPartBuiltin::Extension(p) => extract_extension_part(p),
        }),
        numbas::exam::ExamQuestionPart::Custom(cqp) => {
            QuestionPart::Custom(extract_custom_part(cqp))
        }
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
                            question_name: v!(sanitize(q.name)),
                            question_data: v!(Question {
                                statement: v!(ts!(q.statement)),
                                advice: v!(ts!(q.advice)),
                                parts: v!(q.parts.iter().map(|p| v!(extract_part(p))).collect()),
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
                                }),
                                navigation: v!(QuestionNavigation {
                                    can_regenerate: v!(q.navigation.allow_regenerate),
                                    show_title_page: v!(q.navigation.show_frontpage),
                                    prevent_leaving: v!(q
                                        .navigation
                                        .prevent_leaving
                                        .unwrap_or(DEFAULTS.question_navigation_prevent_leaving)),
                                }),
                                extensions: v!(Extensions::from(&q.extensions)),
                                diagnostic_topic_names: v!(q
                                    .tags
                                    .iter()
                                    .filter(|t| t.starts_with("skill: "))
                                    .map(|t| ts!(
                                        t.splitn(2, ": ").collect::<Vec<_>>()[1].to_string()
                                    ))
                                    .collect()),
                                resources: v!(q
                                    .resources
                                    .to_rumbas()
                                    .into_iter()
                                    .map(|r| v!(r))
                                    .collect())
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
