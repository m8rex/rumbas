use numbas::exam::Exam as NExam;
use rumbas::data::diagnostic_exam::{Diagnostic, DiagnosticExam, DiagnosticScript};
use rumbas::data::exam::Exam as RExam;
use rumbas::data::feedback::{Feedback, FeedbackMessage, Review};
use rumbas::data::file_reference::FileString;
use rumbas::data::locale::{Locale, SupportedLocale};
use rumbas::data::navigation::DiagnosticNavigation;
use rumbas::data::navigation::LeaveAction;
use rumbas::data::navigation::NavigationSharedData;
use rumbas::data::numbas_settings::NumbasSettings;
use rumbas::data::optional_overwrite::Noneable;
use rumbas::data::question_group::QuestionGroup;
use rumbas::data::template::Value;
use rumbas::data::timing::{TimeoutAction, Timing};
use rumbas::data::translatable::TranslatableString;

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

fn extract_question_groups(exam: &NExam) -> Vec<Value<QuestionGroup>> {
    vec![] // TODO
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
        objectives: v!(vec![]), // TODO
        topics: v!(vec![]),     // TODO
    }
}
