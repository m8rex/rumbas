use numbas::exam::Exam as NExam;
use rumbas::data::diagnostic_exam::{
    Diagnostic, DiagnosticExam, DiagnosticScript, LearningObjective, LearningTopic,
};
use rumbas::data::exam::Exam as RExam;
use rumbas::data::extension::Extensions;
use rumbas::data::feedback::{Feedback, FeedbackMessage, Review};
use rumbas::data::file_reference::FileString;
use rumbas::data::function::Function;
use rumbas::data::locale::{Locale, SupportedLocale};
use rumbas::data::navigation::{
    DiagnosticNavigation, LeaveAction, NavigationSharedData, QuestionNavigation,
};
use rumbas::data::numbas_settings::NumbasSettings;
use rumbas::data::optional_overwrite::Noneable;
use rumbas::data::preamble::Preamble;
use rumbas::data::question::{BuiltinConstants, CustomConstant, Question, VariablesTest};
use rumbas::data::question_group::{PickingStrategy, QuestionGroup, QuestionPath};
use rumbas::data::question_part::QuestionPart;
use rumbas::data::template::Value;
use rumbas::data::timing::{TimeoutAction, Timing};
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

fn convert_exam(exam: NExam) -> RExam {
    // TODO: check diagnostic vs normal
    RExam::Diagnostic(convert_diagnostic_exam(exam))
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
            .unwrap_or(Noneable::None(String::new()))),
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
            .unwrap_or(Noneable::None(String::new()))),
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

fn extract_jme_part(qp: &numbas::exam::ExamQuestionPartJME) -> QuestionPart {
    QuestionPart::JME(None) // TODO
}

fn extract_number_entry_part(qp: &numbas::exam::ExamQuestionPartNumberEntry) -> QuestionPart {
    QuestionPart::NumberEntry(None) // TODO
}

fn extract_matrix_part(qp: &numbas::exam::ExamQuestionPartMatrix) -> QuestionPart {
    QuestionPart::Matrix(None) // TODO
}

fn extract_pattern_match_part(qp: &numbas::exam::ExamQuestionPartPatternMatch) -> QuestionPart {
    QuestionPart::PatternMatch(None) // TODO
}

fn extract_choose_one_part(qp: &numbas::exam::ExamQuestionPartChooseOne) -> QuestionPart {
    QuestionPart::ChooseOne(None) // TODO
}

fn extract_choose_multiple_part(qp: &numbas::exam::ExamQuestionPartChooseMultiple) -> QuestionPart {
    QuestionPart::ChooseMultiple(None) // TODO
}

fn extract_match_answers_with_choices_part(
    qp: &numbas::exam::ExamQuestionPartMatchAnswersWithChoices,
) -> QuestionPart {
    QuestionPart::MatchAnswersWithItems(None) // TODO
}

fn extract_gapfill_part(qp: &numbas::exam::ExamQuestionPartGapFill) -> QuestionPart {
    QuestionPart::GapFill(None) // TODO
}

fn extract_information_part(qp: &numbas::exam::ExamQuestionPartInformation) -> QuestionPart {
    QuestionPart::Information(None) // TODO
}

fn extract_extension_part(qp: &numbas::exam::ExamQuestionPart) -> QuestionPart {
    QuestionPart::Extension(None) // TODO
}

fn extract_part(qp: &numbas::exam::ExamQuestionPart) -> QuestionPart {
    match qp {
        numbas::exam::ExamQuestionPart::JME(p) => extract_jme_part(p),
        numbas::exam::ExamQuestionPart::NumberEntry(p) => extract_number_entry_part(p),
        numbas::exam::ExamQuestionPart::Matrix(p) => extract_matrix_part(p),
        numbas::exam::ExamQuestionPart::PatternMatch(p) => extract_pattern_match_part(p),
        numbas::exam::ExamQuestionPart::ChooseOne(p) => extract_choose_one_part(p),
        numbas::exam::ExamQuestionPart::ChooseMultiple(p) => extract_choose_multiple_part(p),
        numbas::exam::ExamQuestionPart::MatchAnswersWithChoices(p) => {
            extract_match_answers_with_choices_part(p)
        }
        numbas::exam::ExamQuestionPart::GapFill(p) => extract_gapfill_part(p),
        numbas::exam::ExamQuestionPart::Information(p) => extract_information_part(p),
        numbas::exam::ExamQuestionPart::Extension(p) => extract_extension_part(p),
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
                                parts: v!(q.parts.iter().map(|p| v!(extract_part(p))).collect()), // TODO
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
                                        t.splitn(2, ":").collect::<Vec<_>>()[1].to_string()
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
