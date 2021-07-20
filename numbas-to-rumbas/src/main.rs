use numbas::defaults::DEFAULTS;
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

macro_rules! read {
    ($file_name: expr) => {{
        let content = std::fs::read_to_string($file_name).expect("Invalid file path");
        NExam::from_exam_str(content.as_ref())
    }};
}

fn main() {
    let exam_res = read!("example.exam");
    match exam_res {
        Ok(exam) => {
            //println!("{:?}", exam);
            let (name, rumbas_exam, qs, cpts) = convert_exam(exam);
            for qp in qs.into_iter() {
                let q_name = qp.question_name.clone().unwrap();
                let q_yaml = QuestionFileType::Normal(Box::new(qp.question_data.unwrap()))
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
            locales: Value::Normal(vec![Value::Normal(Locale {
                name: Value::Normal("en".to_string()),
                numbas_locale: Value::Normal(SupportedLocale::EnGB),
            })]), // todo: argument?
            name: Value::Normal(TranslatableString::s(&"todo".to_string())), // todo: argument
            navigation: Value::Normal(extract_normal_navigation(&exam).unwrap()),
            timing: Value::Normal(extract_timing(&exam)),
            feedback: Value::Normal(extract_feedback(&exam)),
            question_groups: Value::Normal(question_groups.clone()),
            numbas_settings: Value::Normal(NumbasSettings {
                locale: Value::Normal(SupportedLocale::EnGB),
                theme: Value::Normal("default".to_string()),
            }), // todo: argument?
            custom_part_types: Value::Normal(custom_part_types.clone()),
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
            locales: Value::Normal(vec![Value::Normal(Locale {
                name: Value::Normal("en".to_string()),
                numbas_locale: Value::Normal(SupportedLocale::EnGB),
            })]), // todo: argument?
            name: Value::Normal(TranslatableString::s(&"todo".to_string())), // todo: argument
            navigation: Value::Normal(extract_diagnostic_navigation(&exam)),
            timing: Value::Normal(extract_timing(&exam)),
            feedback: Value::Normal(extract_feedback(&exam)),
            question_groups: Value::Normal(question_groups.clone()),
            numbas_settings: Value::Normal(NumbasSettings {
                locale: Value::Normal(SupportedLocale::EnGB),
                theme: Value::Normal("default".to_string()),
            }), // todo: argument?
            diagnostic: Value::Normal(extract_diagnostic(&exam)),
            custom_part_types: Value::Normal(custom_part_types.clone()),
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
        start_password: Value::Normal(FileString::s(
            &exam
                .navigation
                .start_password
                .clone()
                .unwrap_or(DEFAULTS.navigation_start_password),
        )),
        can_regenerate: Value::Normal(exam.navigation.allow_regenerate),
        show_steps: Value::Normal(
            exam.navigation
                .allow_steps
                .unwrap_or(DEFAULTS.navigation_allow_steps),
        ),
        show_title_page: Value::Normal(exam.navigation.show_frontpage),
        prevent_leaving: Value::Normal(
            exam.navigation
                .prevent_leaving
                .unwrap_or(DEFAULTS.navigation_prevent_leaving),
        ),
        show_names_of_question_groups: Value::Normal(
            exam.basic_settings
                .show_question_group_names
                .unwrap_or(DEFAULTS.navigation_show_names_of_question_groups),
        ),
        allow_printing: Value::Normal(
            exam.basic_settings
                .allow_printing
                .unwrap_or(DEFAULTS.basic_settings_allow_printing),
        ),
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
                shared_data: Value::Normal(extract_shared_navigation(exam)),
                can_move_to_previous: Value::Normal(
                    exam.navigation
                        .reverse
                        .unwrap_or(DEFAULTS.navigation_reverse),
                ),
                browsing_enabled: Value::Normal(
                    exam.navigation
                        .browsing_enabled
                        .unwrap_or(DEFAULTS.navigation_browsing_enabled),
                ),
                show_results_page: Value::Normal(extract_sequential_navigation_show_results_page(
                    exam.navigation
                        .show_results_page
                        .clone()
                        .unwrap_or(DEFAULTS.navigation_show_results_page),
                )),
                on_leave: Value::Normal(
                    match exam
                        .navigation
                        .on_leave
                        .clone()
                        .unwrap_or(DEFAULTS.navigation_on_leave)
                    {
                        numbas::exam::ExamLeaveAction::None { message: _ } => LeaveAction::None,
                        numbas::exam::ExamLeaveAction::WarnIfNotAttempted { message } => {
                            LeaveAction::WarnIfNotAttempted {
                                message: TranslatableString::s(&message),
                            }
                        }
                        numbas::exam::ExamLeaveAction::PreventIfNotAttempted { message } => {
                            LeaveAction::PreventIfNotAttempted {
                                message: TranslatableString::s(&message),
                            }
                        }
                    },
                ),
            }))
        }
        numbas::exam::ExamNavigationMode::Menu => Some(NormalNavigation::Menu(MenuNavigation {
            shared_data: Value::Normal(extract_shared_navigation(exam)),
        })),
        numbas::exam::ExamNavigationMode::Diagnostic => None,
    }
}

fn extract_diagnostic_navigation(exam: &NExam) -> DiagnosticNavigation {
    DiagnosticNavigation {
        shared_data: Value::Normal(extract_shared_navigation(exam)),
        on_leave: Value::Normal(
            match exam
                .navigation
                .on_leave
                .clone()
                .unwrap_or(DEFAULTS.navigation_on_leave)
            {
                numbas::exam::ExamLeaveAction::None { message: _ } => LeaveAction::None,
                numbas::exam::ExamLeaveAction::WarnIfNotAttempted { message } => {
                    LeaveAction::WarnIfNotAttempted {
                        message: TranslatableString::s(&message),
                    }
                }
                numbas::exam::ExamLeaveAction::PreventIfNotAttempted { message } => {
                    LeaveAction::PreventIfNotAttempted {
                        message: TranslatableString::s(&message),
                    }
                }
            },
        ),
    }
}

fn extract_timeout_action(action: &numbas::exam::ExamTimeoutAction) -> TimeoutAction {
    match action {
        numbas::exam::ExamTimeoutAction::None { message: _ } => TimeoutAction::None,
        numbas::exam::ExamTimeoutAction::Warn { message } => TimeoutAction::Warn {
            message: TranslatableString::s(&message),
        },
    }
}

fn extract_timing(exam: &NExam) -> Timing {
    Timing {
        duration_in_seconds: Value::Normal(
            exam.basic_settings
                .duration_in_seconds
                .map(Noneable::NotNone)
                .unwrap_or_else(Noneable::nn),
        ),
        allow_pause: Value::Normal(exam.timing.allow_pause),
        on_timeout: Value::Normal(extract_timeout_action(&exam.timing.timeout)),
        timed_warning: Value::Normal(extract_timeout_action(&exam.timing.timed_warning)),
    }
}

fn extract_feedback(exam: &NExam) -> Feedback {
    Feedback {
        percentage_needed_to_pass: Value::Normal(
            exam.basic_settings
                .percentage_needed_to_pass
                .map(Noneable::NotNone)
                .unwrap_or_else(Noneable::nn),
        ),
        show_name_of_student: Value::Normal(
            exam.basic_settings
                .show_student_name
                .unwrap_or(DEFAULTS.basic_settings_show_student_name),
        ),
        show_current_marks: Value::Normal(exam.feedback.show_actual_mark),
        show_maximum_marks: Value::Normal(exam.feedback.show_total_mark),
        show_answer_state: Value::Normal(exam.feedback.show_answer_state),
        allow_reveal_answer: Value::Normal(exam.feedback.allow_reveal_answer),
        review: Value::Normal(Review {
            show_score: Value::Normal(
                exam.feedback
                    .review
                    .clone()
                    .map(|r| r.show_score.unwrap_or(DEFAULTS.feedback_review_show_score))
                    .unwrap(),
            ),
            show_feedback: Value::Normal(
                exam.feedback
                    .review
                    .clone()
                    .map(|r| {
                        r.show_feedback
                            .unwrap_or(DEFAULTS.feedback_review_show_feedback)
                    })
                    .unwrap(),
            ),
            show_expected_answer: Value::Normal(
                exam.feedback
                    .review
                    .clone()
                    .map(|r| {
                        r.show_expected_answer
                            .unwrap_or(DEFAULTS.feedback_review_show_expected_answer)
                    })
                    .unwrap(),
            ),
            show_advice: Value::Normal(
                exam.feedback
                    .review
                    .clone()
                    .map(|r| {
                        r.show_advice
                            .unwrap_or(DEFAULTS.feedback_review_show_advice)
                    })
                    .unwrap(),
            ),
        }),
        advice: Value::Normal(TranslatableString::s(
            &exam.feedback.advice.clone().unwrap_or_default(),
        )),
        intro: Value::Normal(TranslatableString::s(&exam.feedback.intro)),
        feedback_messages: Value::Normal(
            exam.feedback
                .feedback_messages
                .clone()
                .into_iter()
                .map(|m| {
                    Value::Normal(FeedbackMessage {
                        message: m.message,
                        threshold: m.threshold,
                    })
                })
                .collect(),
        ),
    }
}

fn extract_builtin_constants(bc: numbas::exam::BuiltinConstants) -> BuiltinConstants {
    BuiltinConstants {
        e: Value::Normal(
            *bc.0
                .get(&"e".to_string())
                .unwrap_or(&DEFAULTS.builtin_constants_e),
        ),
        pi: Value::Normal(
            *bc.0
                .get(&"pi,\u{03c0}".to_string())
                .unwrap_or(&DEFAULTS.builtin_constants_pi),
        ),
        i: Value::Normal(
            *bc.0
                .get(&"i".to_string())
                .unwrap_or(&DEFAULTS.builtin_constants_i),
        ),
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
        simplify_basic: Value::Normal(DEFAULTS.jme_simplification_simplify_basic),
        simplify_unit_factor: Value::Normal(DEFAULTS.jme_simplification_simplify_unit_factor),
        simplify_unit_power: Value::Normal(DEFAULTS.jme_simplification_simplify_unit_power),
        simplify_unit_denominator: Value::Normal(
            DEFAULTS.jme_simplification_simplify_unit_denominator,
        ),
        simplify_zero_factor: Value::Normal(DEFAULTS.jme_simplification_simplify_zero_factor),
        simplify_zero_term: Value::Normal(DEFAULTS.jme_simplification_simplify_zero_term),
        simplify_zero_power: Value::Normal(DEFAULTS.jme_simplification_simplify_zero_power),
        simplify_zero_base: Value::Normal(DEFAULTS.jme_simplification_simplify_zero_base),
        collect_numbers: Value::Normal(DEFAULTS.jme_simplification_collect_numbers),
        constants_first: Value::Normal(DEFAULTS.jme_simplification_constants_first),
        simplify_sqrt_products: Value::Normal(DEFAULTS.jme_simplification_simplify_sqrt_products),
        simplify_sqrt_division: Value::Normal(DEFAULTS.jme_simplification_simplify_sqrt_division),
        simplify_sqrt_square: Value::Normal(DEFAULTS.jme_simplification_simplify_sqrt_square),
        simplify_other_numbers: Value::Normal(DEFAULTS.jme_simplification_simplify_other_numbers),
        simplify_no_leading_minus: Value::Normal(
            DEFAULTS.jme_simplification_simplify_no_leading_minus,
        ),
        simplify_fractions: Value::Normal(DEFAULTS.jme_simplification_simplify_fractions),
        simplify_trigonometric: Value::Normal(DEFAULTS.jme_simplification_simplify_trigonometric),
        cancel_terms: Value::Normal(DEFAULTS.jme_simplification_cancel_terms),
        cancel_factors: Value::Normal(DEFAULTS.jme_simplification_cancel_factors),
        collect_like_fractions: Value::Normal(DEFAULTS.jme_simplification_collect_like_fractions),
        order_canonical: Value::Normal(DEFAULTS.jme_simplification_order_canonical),
        use_times_dot: Value::Normal(DEFAULTS.jme_simplification_use_times_dot),
        expand_brackets: Value::Normal(DEFAULTS.jme_simplification_expand_brackets),
    }; // Numbas default
    if let Some(v) = ov {
        for a in v.iter() {
            match a {
                numbas::exam::AnswerSimplificationType::All(b) => {
                    result.simplify_basic = Value::Normal(*b);
                    result.simplify_unit_factor = Value::Normal(*b);
                    result.simplify_unit_power = Value::Normal(*b);
                    result.simplify_unit_denominator = Value::Normal(*b);
                    result.simplify_zero_factor = Value::Normal(*b);
                    result.simplify_zero_term = Value::Normal(*b);
                    result.simplify_zero_power = Value::Normal(*b);
                    result.simplify_zero_base = Value::Normal(*b);
                    result.collect_numbers = Value::Normal(*b);
                    result.constants_first = Value::Normal(*b);
                    result.simplify_sqrt_products = Value::Normal(*b);
                    result.simplify_sqrt_division = Value::Normal(*b);
                    result.simplify_sqrt_square = Value::Normal(*b);
                    result.simplify_other_numbers = Value::Normal(*b);
                    result.simplify_no_leading_minus = Value::Normal(*b);
                    result.simplify_fractions = Value::Normal(*b);
                    result.simplify_trigonometric = Value::Normal(*b);
                    result.cancel_terms = Value::Normal(*b);
                    result.cancel_factors = Value::Normal(*b);
                    result.collect_like_fractions = Value::Normal(*b);
                    result.use_times_dot = Value::Normal(*b);
                }
                numbas::exam::AnswerSimplificationType::Basic(b) => {
                    result.simplify_basic = Value::Normal(*b);
                }
                numbas::exam::AnswerSimplificationType::UnitFactor(b) => {
                    result.simplify_unit_factor = Value::Normal(*b);
                }
                numbas::exam::AnswerSimplificationType::UnitPower(b) => {
                    result.simplify_unit_power = Value::Normal(*b);
                }
                numbas::exam::AnswerSimplificationType::UnitDenominator(b) => {
                    result.simplify_unit_denominator = Value::Normal(*b);
                }
                numbas::exam::AnswerSimplificationType::ZeroFactor(b) => {
                    result.simplify_zero_factor = Value::Normal(*b);
                }
                numbas::exam::AnswerSimplificationType::ZeroTerm(b) => {
                    result.simplify_zero_term = Value::Normal(*b);
                }
                numbas::exam::AnswerSimplificationType::ZeroPower(b) => {
                    result.simplify_zero_power = Value::Normal(*b);
                }
                numbas::exam::AnswerSimplificationType::CollectNumbers(b) => {
                    result.collect_numbers = Value::Normal(*b);
                }
                numbas::exam::AnswerSimplificationType::ZeroBase(b) => {
                    result.simplify_zero_base = Value::Normal(*b);
                }
                numbas::exam::AnswerSimplificationType::ConstantsFirst(b) => {
                    result.constants_first = Value::Normal(*b);
                }
                numbas::exam::AnswerSimplificationType::SqrtProduct(b) => {
                    result.simplify_sqrt_products = Value::Normal(*b);
                }
                numbas::exam::AnswerSimplificationType::SqrtDivision(b) => {
                    result.simplify_sqrt_division = Value::Normal(*b);
                }
                numbas::exam::AnswerSimplificationType::SqrtSquare(b) => {
                    result.simplify_sqrt_square = Value::Normal(*b);
                }
                numbas::exam::AnswerSimplificationType::OtherNumbers(b) => {
                    result.simplify_other_numbers = Value::Normal(*b);
                }
                numbas::exam::AnswerSimplificationType::NoLeadingMinus(b) => {
                    result.simplify_no_leading_minus = Value::Normal(*b);
                }
                numbas::exam::AnswerSimplificationType::Fractions(b) => {
                    result.simplify_fractions = Value::Normal(*b);
                }
                numbas::exam::AnswerSimplificationType::Trigonometric(b) => {
                    result.simplify_trigonometric = Value::Normal(*b);
                }
                numbas::exam::AnswerSimplificationType::CancelTerms(b) => {
                    result.cancel_terms = Value::Normal(*b);
                }
                numbas::exam::AnswerSimplificationType::CancelFactors(b) => {
                    result.cancel_factors = Value::Normal(*b);
                }
                numbas::exam::AnswerSimplificationType::CollectLikeFractions(b) => {
                    result.collect_like_fractions = Value::Normal(*b);
                }
                numbas::exam::AnswerSimplificationType::TimesDot(b) => {
                    result.use_times_dot = Value::Normal(*b);
                }
                numbas::exam::AnswerSimplificationType::ExpandBrackets(b) => {
                    result.expand_brackets = Value::Normal(*b);
                }
                numbas::exam::AnswerSimplificationType::CanonicalOrder(b) => {
                    result.order_canonical = Value::Normal(*b);
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
                checking_accuracy: Value::Normal(v.checking_accuracy),
            })
        }
        numbas::exam::JMECheckingType::AbsoluteDifference(v) => {
            CheckingType::AbsoluteDifference(CheckingTypeDataFloat {
                checking_accuracy: Value::Normal(v.checking_accuracy),
            })
        }
        numbas::exam::JMECheckingType::DecimalPlaces(v) => {
            CheckingType::DecimalPlaces(CheckingTypeDataNatural {
                checking_accuracy: Value::Normal(v.checking_accuracy),
            })
        }
        numbas::exam::JMECheckingType::SignificantFigures(v) => {
            CheckingType::SignificantFigures(CheckingTypeDataNatural {
                checking_accuracy: Value::Normal(v.checking_accuracy),
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
    pd.prompt.clone().unwrap_or_default()
}

fn extract_part_common_use_custom_name(pd: &numbas::exam::ExamQuestionPartSharedData) -> bool {
    pd.use_custom_name
        .unwrap_or(DEFAULTS.part_common_use_custom_name)
}
fn extract_part_common_custom_name(pd: &numbas::exam::ExamQuestionPartSharedData) -> String {
    pd.custom_name.clone().unwrap_or_default()
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
    pd.custom_marking_algorithm.clone().unwrap_or_default()
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
        .unwrap_or_default()
        .into_iter()
        .map(|s| extract_part(&s))
        .collect()
}

fn extract_restriction(r: &numbas::exam::JMERestriction) -> JMERestriction {
    JMERestriction {
        name: Value::Normal(TranslatableString::s(&r.name)),
        strings: Value::Normal(
            r.strings
                .clone()
                .into_iter()
                .map(|s| TranslatableString::s(&s))
                .collect(),
        ),
        partial_credit: Value::Normal(r.partial_credit),
        message: Value::Normal(TranslatableString::s(&r.message)),
    }
}

fn extract_length_restriction(r: &numbas::exam::JMELengthRestriction) -> JMELengthRestriction {
    JMELengthRestriction {
        restriction: Value::Normal(extract_restriction(&r.restriction)),
        length: Value::Normal(r.length.unwrap_or(DEFAULTS.length_restriction_length)),
    }
}

fn extract_string_restriction(r: &numbas::exam::JMEStringRestriction) -> JMEStringRestriction {
    JMEStringRestriction {
        restriction: Value::Normal(extract_restriction(&r.restriction)),
        show_strings: Value::Normal(r.show_strings),
    }
}

fn extract_pattern_restriction(r: &numbas::exam::JMEPatternRestriction) -> JMEPatternRestriction {
    JMEPatternRestriction {
        partial_credit: Value::Normal(r.partial_credit),
        message: Value::Normal(TranslatableString::s(&r.message.clone())),
        pattern: Value::Normal(r.pattern.clone()),
        name_to_compare: Value::Normal(r.name_to_compare.clone()),
    }
}

fn extract_value_generator(g: &numbas::exam::JMEValueGenerator) -> JMEValueGenerator {
    JMEValueGenerator {
        name: Value::Normal(FileString::s(&g.name)),
        value: Value::Normal(FileString::s(&g.value)),
    }
}

fn extract_jme_part(qp: &numbas::exam::ExamQuestionPartJME) -> QuestionPartBuiltin {
    QuestionPartBuiltin::JME(QuestionPartJME {
        // Default section
        marks: Value::Normal(extract_part_common_marks(&qp.part_data)),
        prompt: Value::Normal(TranslatableString::s(&extract_part_common_prompt(
            &qp.part_data,
        ))),
        use_custom_name: Value::Normal(extract_part_common_use_custom_name(&qp.part_data)),
        custom_name: Value::Normal(extract_part_common_custom_name(&qp.part_data)),
        steps_penalty: Value::Normal(extract_part_common_steps_penalty(&qp.part_data)),
        enable_minimum_marks: Value::Normal(extract_part_common_enable_minimum_marks(
            &qp.part_data,
        )),
        minimum_marks: Value::Normal(extract_part_common_minimum_marks(&qp.part_data)),
        show_correct_answer: Value::Normal(extract_part_common_show_correct_answer(&qp.part_data)),
        show_feedback_icon: Value::Normal(extract_part_common_show_feedback_icon(&qp.part_data)),
        variable_replacement_strategy: Value::Normal(
            extract_part_common_variable_replacement_strategy(&qp.part_data),
        ),
        adaptive_marking_penalty: Value::Normal(extract_part_common_adaptive_marking_penalty(
            &qp.part_data,
        )),
        custom_marking_algorithm: Value::Normal(extract_part_common_custom_marking_algorithm(
            &qp.part_data,
        )),
        extend_base_marking_algorithm: Value::Normal(
            extract_part_common_extend_base_marking_algorithm(&qp.part_data),
        ),
        steps: Value::Normal(extract_part_common_steps(&qp.part_data)),

        answer: Value::Normal(TranslatableString::s(&qp.answer)),
        answer_simplification: Value::Normal(extract_jme_answer_simplification(
            &qp.answer_simplification,
        )),
        show_preview: Value::Normal(qp.show_preview),
        checking_type: Value::Normal(extract_checking_type(&qp.checking_type)),
        failure_rate: Value::Normal(qp.failure_rate),
        vset_range: Value::Normal([qp.vset_range[0].0, qp.vset_range[1].0]),
        vset_range_points: Value::Normal(qp.vset_range_points.0),
        check_variable_names: Value::Normal(qp.check_variable_names),
        single_letter_variables: Value::Normal(
            qp.single_letter_variables
                .unwrap_or(DEFAULTS.jme_single_letter_variables),
        ),
        allow_unknown_functions: Value::Normal(
            qp.allow_unknown_functions
                .unwrap_or(DEFAULTS.jme_allow_unknown_functions),
        ),
        implicit_function_composition: Value::Normal(
            qp.implicit_function_composition
                .unwrap_or(DEFAULTS.jme_implicit_function_composition),
        ),

        max_length: Value::Normal(
            qp.max_length
                .clone()
                .map(|r| Noneable::NotNone(extract_length_restriction(&r)))
                .unwrap_or_else(Noneable::nn),
        ),
        min_length: Value::Normal(
            qp.min_length
                .clone()
                .map(|r| Noneable::NotNone(extract_length_restriction(&r)))
                .unwrap_or_else(Noneable::nn),
        ),
        must_have: Value::Normal(
            qp.must_have
                .clone()
                .map(|r| Noneable::NotNone(extract_string_restriction(&r)))
                .unwrap_or_else(Noneable::nn),
        ),
        may_not_have: Value::Normal(
            qp.may_not_have
                .clone()
                .map(|r| Noneable::NotNone(extract_string_restriction(&r)))
                .unwrap_or_else(Noneable::nn),
        ),
        must_match_pattern: Value::Normal(
            qp.must_match_pattern
                .clone()
                .map(|r| Noneable::NotNone(extract_pattern_restriction(&r)))
                .unwrap_or_else(Noneable::nn),
        ),
        value_generators: Value::Normal(
            qp.value_generators
                .clone()
                .map(|v| Noneable::NotNone(v.iter().map(|g| extract_value_generator(&g)).collect()))
                .unwrap_or_else(Noneable::nn),
        ),
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
        marks: Value::Normal(extract_part_common_marks(&qp.part_data)),
        prompt: Value::Normal(TranslatableString::s(&extract_part_common_prompt(
            &qp.part_data,
        ))),
        use_custom_name: Value::Normal(extract_part_common_use_custom_name(&qp.part_data)),
        custom_name: Value::Normal(extract_part_common_custom_name(&qp.part_data)),
        steps_penalty: Value::Normal(extract_part_common_steps_penalty(&qp.part_data)),
        enable_minimum_marks: Value::Normal(extract_part_common_enable_minimum_marks(
            &qp.part_data,
        )),
        minimum_marks: Value::Normal(extract_part_common_minimum_marks(&qp.part_data)),
        show_correct_answer: Value::Normal(extract_part_common_show_correct_answer(&qp.part_data)),
        show_feedback_icon: Value::Normal(extract_part_common_show_feedback_icon(&qp.part_data)),
        variable_replacement_strategy: Value::Normal(
            extract_part_common_variable_replacement_strategy(&qp.part_data),
        ),
        adaptive_marking_penalty: Value::Normal(extract_part_common_adaptive_marking_penalty(
            &qp.part_data,
        )),
        custom_marking_algorithm: Value::Normal(extract_part_common_custom_marking_algorithm(
            &qp.part_data,
        )),
        extend_base_marking_algorithm: Value::Normal(
            extract_part_common_extend_base_marking_algorithm(&qp.part_data),
        ),
        steps: Value::Normal(extract_part_common_steps(&qp.part_data)),

        answer: Value::Normal(extract_number_entry_answer(&qp.answer)),
        display_correct_as_fraction: Value::Normal(qp.correct_answer_fraction),
        allow_fractions: Value::Normal(qp.allow_fractions),
        allowed_notation_styles: Value::Normal(
            qp.notation_styles.clone().unwrap_or_default().to_rumbas(),
        ),
        display_correct_in_style: Value::Normal(
            qp.correct_answer_style
                .clone()
                .unwrap_or(DEFAULTS.number_entry_correct_answer_style)
                .to_rumbas(),
        ),

        fractions_must_be_reduced: Value::Normal(
            qp.fractions_must_be_reduced
                .unwrap_or(DEFAULTS.number_entry_fractions_must_be_reduced),
        ),
        partial_credit_if_fraction_not_reduced: Value::Normal(
            qp.partial_credit_if_fraction_not_reduced
                .clone()
                .unwrap_or(DEFAULTS.number_entry_partial_credit_if_fraction_not_reduced),
        ),
        hint_fraction: Value::Normal(
            qp.show_fraction_hint
                .unwrap_or(DEFAULTS.number_entry_hint_fraction),
        ),
    })
}

/*impl ToRumbas for numbas::exam::ExamQuestionPartMatrix {
type RumbasType = QuestionPartMatrix;
fn to_rumbas(&self) -> Self::RumbasType {*/
fn extract_matrix_part(sel: &numbas::exam::ExamQuestionPartMatrix) -> QuestionPartBuiltin {
    QuestionPartBuiltin::Matrix({
        let rows = Value::Normal(QuestionPartMatrixDimension::from_range(
            sel.num_rows.0,
            sel.min_rows,
            sel.max_rows,
        ));
        let columns = Value::Normal(QuestionPartMatrixDimension::from_range(
            sel.num_columns.0,
            sel.min_columns,
            sel.max_columns,
        ));
        let dimensions = QuestionPartMatrixDimensions { rows, columns };
        QuestionPartMatrix {
            // Default section
            marks: Value::Normal(extract_part_common_marks(&sel.part_data)),
            prompt: Value::Normal(TranslatableString::s(&extract_part_common_prompt(
                &sel.part_data,
            ))),
            use_custom_name: Value::Normal(extract_part_common_use_custom_name(&sel.part_data)),
            custom_name: Value::Normal(extract_part_common_custom_name(&sel.part_data)),
            steps_penalty: Value::Normal(extract_part_common_steps_penalty(&sel.part_data)),
            enable_minimum_marks: Value::Normal(extract_part_common_enable_minimum_marks(
                &sel.part_data,
            )),
            minimum_marks: Value::Normal(extract_part_common_minimum_marks(&sel.part_data)),
            show_correct_answer: Value::Normal(extract_part_common_show_correct_answer(
                &sel.part_data,
            )),
            show_feedback_icon: Value::Normal(extract_part_common_show_feedback_icon(
                &sel.part_data,
            )),
            variable_replacement_strategy: Value::Normal(
                extract_part_common_variable_replacement_strategy(&sel.part_data),
            ),
            adaptive_marking_penalty: Value::Normal(extract_part_common_adaptive_marking_penalty(
                &sel.part_data,
            )),
            custom_marking_algorithm: Value::Normal(extract_part_common_custom_marking_algorithm(
                &sel.part_data,
            )),
            extend_base_marking_algorithm: Value::Normal(
                extract_part_common_extend_base_marking_algorithm(&sel.part_data),
            ),
            steps: Value::Normal(extract_part_common_steps(&sel.part_data)),

            correct_answer: Value::Normal(sel.correct_answer.clone()),
            display_correct_as_fraction: Value::Normal(sel.correct_answer_fractions),
            dimensions: Value::Normal(dimensions),
            max_absolute_deviation: Value::Normal(sel.tolerance),
            mark_partial_by_cells: Value::Normal(sel.mark_per_cell),
            allow_fractions: Value::Normal(sel.allow_fractions),
        }
    })
}
//}

fn extract_pattern_match_part(
    qp: &numbas::exam::ExamQuestionPartPatternMatch,
) -> QuestionPartBuiltin {
    QuestionPartBuiltin::PatternMatch(QuestionPartPatternMatch {
        // Default section
        marks: Value::Normal(extract_part_common_marks(&qp.part_data)),
        prompt: Value::Normal(TranslatableString::s(&extract_part_common_prompt(
            &qp.part_data,
        ))),
        use_custom_name: Value::Normal(extract_part_common_use_custom_name(&qp.part_data)),
        custom_name: Value::Normal(extract_part_common_custom_name(&qp.part_data)),
        steps_penalty: Value::Normal(extract_part_common_steps_penalty(&qp.part_data)),
        enable_minimum_marks: Value::Normal(extract_part_common_enable_minimum_marks(
            &qp.part_data,
        )),
        minimum_marks: Value::Normal(extract_part_common_minimum_marks(&qp.part_data)),
        show_correct_answer: Value::Normal(extract_part_common_show_correct_answer(&qp.part_data)),
        show_feedback_icon: Value::Normal(extract_part_common_show_feedback_icon(&qp.part_data)),
        variable_replacement_strategy: Value::Normal(
            extract_part_common_variable_replacement_strategy(&qp.part_data),
        ),
        adaptive_marking_penalty: Value::Normal(extract_part_common_adaptive_marking_penalty(
            &qp.part_data,
        )),
        custom_marking_algorithm: Value::Normal(extract_part_common_custom_marking_algorithm(
            &qp.part_data,
        )),
        extend_base_marking_algorithm: Value::Normal(
            extract_part_common_extend_base_marking_algorithm(&qp.part_data),
        ),
        steps: Value::Normal(extract_part_common_steps(&qp.part_data)),

        case_sensitive: Value::Normal(qp.case_sensitive),
        partial_credit: Value::Normal(qp.partial_credit),
        pattern: Value::Normal(TranslatableString::s(&qp.answer.to_string())),
        display_answer: Value::Normal(TranslatableString::s(
            &qp.display_answer
                .clone()
                .map(|d| d.to_string())
                .unwrap_or_else(|| qp.answer.to_string()),
        )), // TDDO: check default
        match_mode: Value::Normal(qp.match_mode),
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
        Value::Normal(MultipleChoiceAnswerData::ItemBased(
            answers_data
                .into_iter()
                .map(|(a, b, c)| MultipleChoiceAnswer {
                    statement: Value::Normal(TranslatableString::s(&a)),
                    marks: Value::Normal(b),
                    feedback: Value::Normal(TranslatableString::s(&c)),
                })
                .collect(),
        ))
    } else {
        Value::Normal(MultipleChoiceAnswerData::NumbasLike(Box::new(
            MultipleChoiceAnswerDataNumbasLike {
                answers: Value::Normal(
                    qp.answers
                        .clone()
                        .map(|v| {
                            v.iter()
                                .map(|vv| TranslatableString::s(&vv.clone()))
                                .collect::<Vec<_>>()
                        })
                        .to_rumbas(),
                ),
                marks: Value::Normal(
                    qp.marking_matrix
                        .clone()
                        .map(|m| m.to_rumbas())
                        .expect("How can the marking matrix be optional?"),
                ),
                feedback: Value::Normal(
                    qp.distractors
                        .clone()
                        .map(|v| {
                            Noneable::NotNone(
                                v.iter()
                                    .map(|f| TranslatableString::s(&f))
                                    .collect::<Vec<_>>()
                                    .to_rumbas(),
                            )
                        })
                        .unwrap_or_else(Noneable::nn),
                ),
            },
        )))
    };
    QuestionPartBuiltin::ChooseOne(QuestionPartChooseOne {
        // Default section
        marks: Value::Normal(extract_part_common_marks(&qp.part_data)),
        prompt: Value::Normal(TranslatableString::s(&extract_part_common_prompt(
            &qp.part_data,
        ))),
        use_custom_name: Value::Normal(extract_part_common_use_custom_name(&qp.part_data)),
        custom_name: Value::Normal(extract_part_common_custom_name(&qp.part_data)),
        steps_penalty: Value::Normal(extract_part_common_steps_penalty(&qp.part_data)),
        enable_minimum_marks: Value::Normal(extract_part_common_enable_minimum_marks(
            &qp.part_data,
        )),
        minimum_marks: Value::Normal(extract_part_common_minimum_marks(&qp.part_data)),
        show_correct_answer: Value::Normal(extract_part_common_show_correct_answer(&qp.part_data)),
        show_feedback_icon: Value::Normal(extract_part_common_show_feedback_icon(&qp.part_data)),
        variable_replacement_strategy: Value::Normal(
            extract_part_common_variable_replacement_strategy(&qp.part_data),
        ),
        adaptive_marking_penalty: Value::Normal(extract_part_common_adaptive_marking_penalty(
            &qp.part_data,
        )),
        custom_marking_algorithm: Value::Normal(extract_part_common_custom_marking_algorithm(
            &qp.part_data,
        )),
        extend_base_marking_algorithm: Value::Normal(
            extract_part_common_extend_base_marking_algorithm(&qp.part_data),
        ),
        steps: Value::Normal(extract_part_common_steps(&qp.part_data)),
        answer_data,
        display: Value::Normal(match qp.display_type {
            numbas::exam::ChooseOneDisplayType::Radio => ChooseOneDisplay::Radio {
                columns: qp.columns.0,
            },
            numbas::exam::ChooseOneDisplayType::DropDown => ChooseOneDisplay::DropDown,
        }),
        shuffle_answers: Value::Normal(qp.shuffle_answers),
        show_cell_answer_state: Value::Normal(
            qp.show_cell_answer_state
                .unwrap_or(DEFAULTS.choose_one_show_cell_answer_state),
        ),
        has_to_select_option: Value::Normal(
            qp.min_answers
                .map(|v| v == 1)
                .unwrap_or(DEFAULTS.choose_one_has_to_select_option),
        ),
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
        Value::Normal(MultipleChoiceAnswerData::ItemBased(
            answers_data
                .into_iter()
                .map(|(a, b, c)| MultipleChoiceAnswer {
                    statement: Value::Normal(TranslatableString::s(&a)),
                    marks: Value::Normal(b),
                    feedback: Value::Normal(TranslatableString::s(&c)),
                })
                .collect(),
        ))
    } else {
        Value::Normal(MultipleChoiceAnswerData::NumbasLike(Box::new(
            MultipleChoiceAnswerDataNumbasLike {
                answers: Value::Normal(
                    qp.choices
                        .clone()
                        .map(|v| {
                            v.iter()
                                .map(|vv| TranslatableString::s(&vv.clone()))
                                .collect::<Vec<_>>()
                        })
                        .to_rumbas(),
                ),
                marks: Value::Normal(
                    qp.marking_matrix
                        .clone()
                        .map(|m| m.to_rumbas())
                        .expect("How can the marking matrix be optional?"),
                ),
                feedback: Value::Normal(
                    qp.distractors
                        .clone()
                        .map(|v| {
                            Noneable::NotNone(
                                v.iter()
                                    .map(|f| TranslatableString::s(&f))
                                    .collect::<Vec<_>>()
                                    .to_rumbas(),
                            )
                        })
                        .unwrap_or_else(Noneable::nn),
                ),
            },
        )))
    };
    QuestionPartBuiltin::ChooseMultiple(QuestionPartChooseMultiple {
        // Default section
        marks: Value::Normal(extract_part_common_marks(&qp.part_data)),
        prompt: Value::Normal(TranslatableString::s(&extract_part_common_prompt(
            &qp.part_data,
        ))),
        use_custom_name: Value::Normal(extract_part_common_use_custom_name(&qp.part_data)),
        custom_name: Value::Normal(extract_part_common_custom_name(&qp.part_data)),
        steps_penalty: Value::Normal(extract_part_common_steps_penalty(&qp.part_data)),
        enable_minimum_marks: Value::Normal(extract_part_common_enable_minimum_marks(
            &qp.part_data,
        )),
        minimum_marks: Value::Normal(extract_part_common_minimum_marks(&qp.part_data)),
        show_correct_answer: Value::Normal(extract_part_common_show_correct_answer(&qp.part_data)),
        show_feedback_icon: Value::Normal(extract_part_common_show_feedback_icon(&qp.part_data)),
        variable_replacement_strategy: Value::Normal(
            extract_part_common_variable_replacement_strategy(&qp.part_data),
        ),
        adaptive_marking_penalty: Value::Normal(extract_part_common_adaptive_marking_penalty(
            &qp.part_data,
        )),
        custom_marking_algorithm: Value::Normal(extract_part_common_custom_marking_algorithm(
            &qp.part_data,
        )),
        extend_base_marking_algorithm: Value::Normal(
            extract_part_common_extend_base_marking_algorithm(&qp.part_data),
        ),
        steps: Value::Normal(extract_part_common_steps(&qp.part_data)),

        answer_data,
        shuffle_answers: Value::Normal(qp.shuffle_answers),
        show_cell_answer_state: Value::Normal(qp.show_cell_answer_state),
        should_select_at_least: Value::Normal(
            qp.min_answers
                .unwrap_or(DEFAULTS.choose_multiple_min_answers),
        ),
        should_select_at_most: Value::Normal(
            qp.max_answers
                .map(Noneable::NotNone)
                .unwrap_or_else(Noneable::nn),
        ),
        columns: Value::Normal(qp.display_columns.0),
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

        Value::Normal(MultipleChoiceMatchAnswerData::ItemBased({
            let answers: Vec<_> = answer_options
                .iter()
                .map(|a| Value::Normal(TranslatableString::s(&a.clone())))
                .collect();
            MultipleChoiceMatchAnswers {
                answers: Value::Normal(answers.clone()),
                items: Value::Normal(
                    items_data
                        .into_iter()
                        .map(|(statement, marks)| {
                            Value::Normal(MatchAnswersItem {
                                statement: Value::Normal(TranslatableString::s(&statement)),
                                answer_marks: Value::Normal(
                                    marks
                                        .into_iter()
                                        .enumerate()
                                        .map(|(i, m)| MatchAnswersItemMarks {
                                            marks: Value::Normal(m),
                                            answer: answers.get(i).unwrap().clone(),
                                        })
                                        .collect(),
                                ),
                            })
                        })
                        .collect(),
                ),
            }
        }))
    } else {
        Value::Normal(MultipleChoiceMatchAnswerData::NumbasLike(
            MultipleChoiceMatchAnswerDataNumbasLike {
                answers: Value::Normal(
                    qp.answers
                        .clone()
                        .map(|v| {
                            v.iter()
                                .map(|vv| TranslatableString::s(&vv.clone()))
                                .collect::<Vec<_>>()
                        })
                        .to_rumbas(),
                ),
                choices: Value::Normal(
                    qp.choices
                        .clone()
                        .map(|v| {
                            v.iter()
                                .map(|vv| TranslatableString::s(&vv.clone()))
                                .collect::<Vec<_>>()
                        })
                        .to_rumbas(),
                ),
                marks: Value::Normal(
                    qp.marking_matrix
                        .clone()
                        .map(|m| m.to_rumbas())
                        .expect("How can the marking matrix be optional?"),
                ),
            },
        ))
    };
    QuestionPartBuiltin::MatchAnswersWithItems(QuestionPartMatchAnswersWithItems {
        // Default section
        marks: Value::Normal(extract_part_common_marks(&qp.part_data)),
        prompt: Value::Normal(TranslatableString::s(&extract_part_common_prompt(
            &qp.part_data,
        ))),
        use_custom_name: Value::Normal(extract_part_common_use_custom_name(&qp.part_data)),
        custom_name: Value::Normal(extract_part_common_custom_name(&qp.part_data)),
        steps_penalty: Value::Normal(extract_part_common_steps_penalty(&qp.part_data)),
        enable_minimum_marks: Value::Normal(extract_part_common_enable_minimum_marks(
            &qp.part_data,
        )),
        minimum_marks: Value::Normal(extract_part_common_minimum_marks(&qp.part_data)),
        show_correct_answer: Value::Normal(extract_part_common_show_correct_answer(&qp.part_data)),
        show_feedback_icon: Value::Normal(extract_part_common_show_feedback_icon(&qp.part_data)),
        variable_replacement_strategy: Value::Normal(
            extract_part_common_variable_replacement_strategy(&qp.part_data),
        ),
        adaptive_marking_penalty: Value::Normal(extract_part_common_adaptive_marking_penalty(
            &qp.part_data,
        )),
        custom_marking_algorithm: Value::Normal(extract_part_common_custom_marking_algorithm(
            &qp.part_data,
        )),
        extend_base_marking_algorithm: Value::Normal(
            extract_part_common_extend_base_marking_algorithm(&qp.part_data),
        ),
        steps: Value::Normal(extract_part_common_steps(&qp.part_data)),

        answer_data,
        shuffle_answers: Value::Normal(qp.shuffle_answers),
        shuffle_items: Value::Normal(qp.shuffle_choices),
        show_cell_answer_state: Value::Normal(qp.show_cell_answer_state),
        should_select_at_least: Value::Normal(
            qp.min_answers
                .unwrap_or(DEFAULTS.match_answers_with_items_min_answers),
        ),
        should_select_at_most: Value::Normal(
            qp.max_answers
                .map(Noneable::NotNone)
                .unwrap_or_else(Noneable::nn),
        ),
        display: Value::Normal(qp.display_type.to_rumbas()),
        layout: Value::Normal(qp.layout.clone()),
    })
}

fn extract_gapfill_part(qp: &numbas::exam::ExamQuestionPartGapFill) -> QuestionPartBuiltin {
    QuestionPartBuiltin::GapFill(QuestionPartGapFill {
        marks: Value::Normal(extract_part_common_marks(&qp.part_data)),
        prompt: Value::Normal(TranslatableString::s(&extract_part_common_prompt(
            &qp.part_data,
        ))),
        use_custom_name: Value::Normal(extract_part_common_use_custom_name(&qp.part_data)),
        custom_name: Value::Normal(extract_part_common_custom_name(&qp.part_data)),
        steps_penalty: Value::Normal(extract_part_common_steps_penalty(&qp.part_data)),
        enable_minimum_marks: Value::Normal(extract_part_common_enable_minimum_marks(
            &qp.part_data,
        )),
        minimum_marks: Value::Normal(extract_part_common_minimum_marks(&qp.part_data)),
        show_correct_answer: Value::Normal(extract_part_common_show_correct_answer(&qp.part_data)),
        show_feedback_icon: Value::Normal(extract_part_common_show_feedback_icon(&qp.part_data)),
        variable_replacement_strategy: Value::Normal(
            extract_part_common_variable_replacement_strategy(&qp.part_data),
        ),
        adaptive_marking_penalty: Value::Normal(extract_part_common_adaptive_marking_penalty(
            &qp.part_data,
        )),
        custom_marking_algorithm: Value::Normal(extract_part_common_custom_marking_algorithm(
            &qp.part_data,
        )),
        extend_base_marking_algorithm: Value::Normal(
            extract_part_common_extend_base_marking_algorithm(&qp.part_data),
        ),
        steps: Value::Normal(extract_part_common_steps(&qp.part_data)),

        sort_answers: Value::Normal(qp.sort_answers.unwrap_or(DEFAULTS.gapfill_sort_answers)),

        gaps: Value::Normal(qp.gaps.iter().map(|s| extract_part(&s)).collect()),
    })
}

fn extract_information_part(qp: &numbas::exam::ExamQuestionPartInformation) -> QuestionPartBuiltin {
    QuestionPartBuiltin::Information(QuestionPartInformation {
        marks: Value::Normal(extract_part_common_marks(&qp.part_data)),
        prompt: Value::Normal(TranslatableString::s(&extract_part_common_prompt(
            &qp.part_data,
        ))),
        use_custom_name: Value::Normal(extract_part_common_use_custom_name(&qp.part_data)),
        custom_name: Value::Normal(extract_part_common_custom_name(&qp.part_data)),
        steps_penalty: Value::Normal(extract_part_common_steps_penalty(&qp.part_data)),
        enable_minimum_marks: Value::Normal(extract_part_common_enable_minimum_marks(
            &qp.part_data,
        )),
        minimum_marks: Value::Normal(extract_part_common_minimum_marks(&qp.part_data)),
        show_correct_answer: Value::Normal(extract_part_common_show_correct_answer(&qp.part_data)),
        show_feedback_icon: Value::Normal(extract_part_common_show_feedback_icon(&qp.part_data)),
        variable_replacement_strategy: Value::Normal(
            extract_part_common_variable_replacement_strategy(&qp.part_data),
        ),
        adaptive_marking_penalty: Value::Normal(extract_part_common_adaptive_marking_penalty(
            &qp.part_data,
        )),
        custom_marking_algorithm: Value::Normal(extract_part_common_custom_marking_algorithm(
            &qp.part_data,
        )),
        extend_base_marking_algorithm: Value::Normal(
            extract_part_common_extend_base_marking_algorithm(&qp.part_data),
        ),
        steps: Value::Normal(extract_part_common_steps(&qp.part_data)),
    })
}
fn extract_extension_part(qp: &numbas::exam::ExamQuestionPartExtension) -> QuestionPartBuiltin {
    QuestionPartBuiltin::Extension(QuestionPartExtension {
        marks: Value::Normal(extract_part_common_marks(&qp.part_data)),
        prompt: Value::Normal(TranslatableString::s(&extract_part_common_prompt(
            &qp.part_data,
        ))),
        use_custom_name: Value::Normal(extract_part_common_use_custom_name(&qp.part_data)),
        custom_name: Value::Normal(extract_part_common_custom_name(&qp.part_data)),
        steps_penalty: Value::Normal(extract_part_common_steps_penalty(&qp.part_data)),
        enable_minimum_marks: Value::Normal(extract_part_common_enable_minimum_marks(
            &qp.part_data,
        )),
        minimum_marks: Value::Normal(extract_part_common_minimum_marks(&qp.part_data)),
        show_correct_answer: Value::Normal(extract_part_common_show_correct_answer(&qp.part_data)),
        show_feedback_icon: Value::Normal(extract_part_common_show_feedback_icon(&qp.part_data)),
        variable_replacement_strategy: Value::Normal(
            extract_part_common_variable_replacement_strategy(&qp.part_data),
        ),
        adaptive_marking_penalty: Value::Normal(extract_part_common_adaptive_marking_penalty(
            &qp.part_data,
        )),
        custom_marking_algorithm: Value::Normal(extract_part_common_custom_marking_algorithm(
            &qp.part_data,
        )),
        extend_base_marking_algorithm: Value::Normal(
            extract_part_common_extend_base_marking_algorithm(&qp.part_data),
        ),
        steps: Value::Normal(extract_part_common_steps(&qp.part_data)),
    })
}

fn extract_custom_part(qp: &numbas::exam::ExamQuestionPartCustom) -> QuestionPartCustom {
    QuestionPartCustom {
        // Default section
        marks: Value::Normal(extract_part_common_marks(&qp.part_data)),
        prompt: Value::Normal(TranslatableString::s(&extract_part_common_prompt(
            &qp.part_data,
        ))),
        use_custom_name: Value::Normal(extract_part_common_use_custom_name(&qp.part_data)),
        custom_name: Value::Normal(extract_part_common_custom_name(&qp.part_data)),
        steps_penalty: Value::Normal(extract_part_common_steps_penalty(&qp.part_data)),
        enable_minimum_marks: Value::Normal(extract_part_common_enable_minimum_marks(
            &qp.part_data,
        )),
        minimum_marks: Value::Normal(extract_part_common_minimum_marks(&qp.part_data)),
        show_correct_answer: Value::Normal(extract_part_common_show_correct_answer(&qp.part_data)),
        show_feedback_icon: Value::Normal(extract_part_common_show_feedback_icon(&qp.part_data)),
        variable_replacement_strategy: Value::Normal(
            extract_part_common_variable_replacement_strategy(&qp.part_data),
        ),
        adaptive_marking_penalty: Value::Normal(extract_part_common_adaptive_marking_penalty(
            &qp.part_data,
        )),
        custom_marking_algorithm: Value::Normal(extract_part_common_custom_marking_algorithm(
            &qp.part_data,
        )),
        extend_base_marking_algorithm: Value::Normal(
            extract_part_common_extend_base_marking_algorithm(&qp.part_data),
        ),
        steps: Value::Normal(extract_part_common_steps(&qp.part_data)),

        r#type: Value::Normal(qp.r#type.clone()),
        settings: Value::Normal(
            qp.settings
                .clone()
                .into_iter()
                .map(|(k, v)| (k, v.to_rumbas()))
                .collect(),
        ),
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
            Value::Normal(QuestionGroup {
                name: Value::Normal(TranslatableString::s(&q.name.unwrap_or_default())),
                picking_strategy: Value::Normal(match q.picking_strategy {
                    numbas::exam::ExamQuestionGroupPickingStrategy::AllOrdered => {
                        PickingStrategy::AllOrdered
                    }
                    numbas::exam::ExamQuestionGroupPickingStrategy::AllShuffled => {
                        PickingStrategy::AllShuffled
                    }
                    numbas::exam::ExamQuestionGroupPickingStrategy::RandomSubset {
                        pick_questions,
                    } => PickingStrategy::RandomSubset { pick_questions },
                }),
                questions: Value::Normal(
                    q.questions
                        .into_iter()
                        .map(|q| {
                            Value::Normal(QuestionPath {
                                question_name: Value::Normal(sanitize(q.name)),
                                question_data: Value::Normal(Question {
                                    statement: Value::Normal(TranslatableString::s(&q.statement)),
                                    advice: Value::Normal(TranslatableString::s(&q.advice)),
                                    parts: Value::Normal(
                                        q.parts
                                            .iter()
                                            .map(|p| Value::Normal(extract_part(p)))
                                            .collect(),
                                    ),
                                    builtin_constants: Value::Normal(extract_builtin_constants(
                                        q.builtin_constants,
                                    )),
                                    custom_constants: Value::Normal(
                                        q.constants
                                            .iter()
                                            .map(|cc| CustomConstant {
                                                name: Value::Normal(cc.name.clone()),
                                                value: Value::Normal(cc.value.clone()),
                                                tex: Value::Normal(cc.tex.clone()),
                                            })
                                            .collect(),
                                    ),
                                    variables: Value::Normal(
                                        q.variables
                                            .iter()
                                            .map(|(k, v)| {
                                                (
                                                    k.clone(),
                                                    Value::Normal(VariableRepresentation::Long(
                                                        Box::new(Value::Normal(Variable {
                                                            definition: Value::Normal(
                                                                FileString::s(&v.definition),
                                                            ),
                                                            description: Value::Normal(
                                                                v.description.clone(),
                                                            ),
                                                            template_type: Value::Normal(
                                                                extract_variable_template_type(
                                                                    v.template_type.clone(),
                                                                ),
                                                            ),
                                                            group: Value::Normal(v.group.clone()),
                                                        })),
                                                    )),
                                                )
                                            })
                                            .collect::<std::collections::HashMap<_, _>>(),
                                    ),
                                    variables_test: Value::Normal(VariablesTest {
                                        condition: Value::Normal(
                                            q.variables_test.condition.clone(),
                                        ),
                                        max_runs: Value::Normal(q.variables_test.max_runs.0),
                                    }),
                                    functions: Value::Normal(
                                        q.functions
                                            .iter()
                                            .map(|(k, f)| {
                                                (
                                                    k.clone(),
                                                    Value::Normal(Function {
                                                        definition: Value::Normal(FileString::s(
                                                            &f.definition,
                                                        )),
                                                        output_type: Value::Normal(f.output_type),
                                                        language: Value::Normal(f.language),
                                                        parameters: Value::Normal(
                                                            f.parameters
                                                                .clone()
                                                                .into_iter()
                                                                .collect(),
                                                        ),
                                                    }),
                                                )
                                            })
                                            .collect::<std::collections::HashMap<_, _>>(),
                                    ),
                                    preamble: Value::Normal(Preamble {
                                        js: Value::Normal(FileString::s(&q.preamble.js)),
                                        css: Value::Normal(FileString::s(&q.preamble.css)),
                                    }),
                                    navigation: Value::Normal(QuestionNavigation {
                                        can_regenerate: Value::Normal(
                                            q.navigation.allow_regenerate,
                                        ),
                                        show_title_page: Value::Normal(q.navigation.show_frontpage),
                                        prevent_leaving: Value::Normal(
                                            q.navigation.prevent_leaving.unwrap_or(
                                                DEFAULTS.question_navigation_prevent_leaving,
                                            ),
                                        ),
                                    }),
                                    extensions: Value::Normal(Extensions::from(&q.extensions)),
                                    diagnostic_topic_names: Value::Normal(
                                        q.tags
                                            .iter()
                                            .filter(|t| t.starts_with("skill: "))
                                            .map(|t| {
                                                TranslatableString::s(
                                                    &t.splitn(2, ": ").collect::<Vec<_>>()[1]
                                                        .to_string(),
                                                )
                                            })
                                            .collect(),
                                    ),
                                    resources: Value::Normal(
                                        q.resources
                                            .to_rumbas()
                                            .into_iter()
                                            .map(|r| Value::Normal(r))
                                            .collect(),
                                    ),
                                }),
                            })
                        })
                        .collect(),
                ),
            })
        })
        .collect()
}

fn extract_diagnostic(exam: &NExam) -> Diagnostic {
    let diagnostic = exam.diagnostic.clone().unwrap();
    Diagnostic {
        script: Value::Normal(match diagnostic.script {
            numbas::exam::ExamDiagnosticScript::Mastery => DiagnosticScript::Mastery,
            numbas::exam::ExamDiagnosticScript::Diagnosys => DiagnosticScript::Diagnosys,
            numbas::exam::ExamDiagnosticScript::Custom => {
                DiagnosticScript::Custom(TranslatableString::s(&diagnostic.custom_script))
            }
        }),
        objectives: Value::Normal(
            diagnostic
                .knowledge_graph
                .clone()
                .learning_objectives
                .into_iter()
                .map(|l| LearningObjective {
                    name: Value::Normal(TranslatableString::s(&l.name)),
                    description: Value::Normal(TranslatableString::s(&l.description)),
                })
                .collect(),
        ),
        topics: Value::Normal(
            diagnostic
                .knowledge_graph
                .topics
                .into_iter()
                .map(|l| LearningTopic {
                    name: Value::Normal(TranslatableString::s(&l.name)),
                    description: Value::Normal(TranslatableString::s(&l.description)),
                    objectives: Value::Normal(
                        l.learning_objectives
                            .clone()
                            .into_iter()
                            .map(|o| TranslatableString::s(&o))
                            .collect(),
                    ),
                    depends_on: Value::Normal(
                        l.depends_on
                            .clone()
                            .into_iter()
                            .map(|o| TranslatableString::s(&o))
                            .collect(),
                    ),
                })
                .collect(),
        ),
    }
}
