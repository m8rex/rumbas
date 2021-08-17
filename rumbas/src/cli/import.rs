use numbas::exam::Exam as NExam;
use rumbas::data::custom_part_type::CustomPartTypeDefinitionPath;
use rumbas::data::diagnostic_exam::DiagnosticExam;
use rumbas::data::locale::{Locale, SupportedLocale};
use rumbas::data::normal_exam::NormalExam;
use rumbas::data::numbas_settings::NumbasSettings;
use rumbas::data::question_group::QuestionPath;
use rumbas::data::template::ExamFileType;
use rumbas::data::template::QuestionFileType;
use rumbas::data::template::Value;
use rumbas::data::to_rumbas::ToRumbas;
use rumbas::data::translatable::TranslatableString;

macro_rules! read_exam {
    ($file_name: expr) => {{
        let content = std::fs::read_to_string($file_name).expect("Invalid file path");
        NExam::from_exam_str(content.as_ref())
    }};
}

macro_rules! read_question {
    ($file_name: expr) => {{
        let content = std::fs::read_to_string($file_name).expect("Invalid file path");
        numbas::exam::ExamQuestion::from_question_exam_str(content.as_ref())
    }};
}

fn create_question(qp: QuestionPath) {
    let q_name = qp.question_name.clone();
    let q_yaml = QuestionFileType::Normal(Box::new(qp.question_data))
        .to_yaml()
        .unwrap();
    let file = format!("{}/{}.yaml", rumbas::QUESTIONS_FOLDER, q_name);
    log::info!("Writing to {}", file);
    std::fs::write(file, q_yaml).unwrap(); //fix handle result
}

fn create_custom_part_type(cpt: CustomPartTypeDefinitionPath) {
    let c_name = cpt.custom_part_type_name.clone();
    let c_yaml = cpt.custom_part_type_data.to_yaml().unwrap();
    let file = format!("{}/{}.yaml", rumbas::CUSTOM_PART_TYPES_FOLDER, c_name);
    log::info!("Writing to {}", file);
    std::fs::write(file, c_yaml).unwrap(); //fix handle result
}

pub fn import(matches: &clap::ArgMatches) {
    let path = std::path::Path::new(matches.value_of("EXAM_PATH").unwrap());
    if matches.is_present("question") {
        let question_res = read_question!(path);
        match question_res {
            Ok(question) => {
                let rumbas_question: QuestionPath = question.to_rumbas();
                for cpt in rumbas_question
                    .question_data
                    .custom_part_types
                    .unwrap()
                    .iter()
                {
                    create_custom_part_type(cpt.to_owned());
                }
                create_question(rumbas_question);
            }
            Err(e) => {
                log::error!("{:?}", e);
                std::process::exit(1)
            }
        }
    } else {
        let exam_res = read_exam!(path);
        match exam_res {
            Ok(exam) => {
                //println!("{:?}", exam);
                let (name, rumbas_exam, qs, cpts) = convert_exam(exam);
                for qp in qs.into_iter() {
                    create_question(qp)
                }
                for cpt in cpts.into_iter() {
                    create_custom_part_type(cpt);
                }
                let exam_yaml = rumbas_exam.to_yaml().unwrap();
                std::fs::write(format!("{}/{}.yaml", rumbas::EXAMS_FOLDER, name), exam_yaml)
                    .unwrap();
                //fix handle result
            }
            Err(e) => {
                log::error!("{:?}", e);
                std::process::exit(1)
            }
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
        numbas::exam::ExamNavigationMode::Diagnostic(ref _d) => {
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
                n.get_content("").unwrap()
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
    let question_groups = exam
        .question_groups
        .to_rumbas()
        .into_iter()
        .map(Value::Normal)
        .collect::<Vec<_>>();
    let custom_part_types = exam.custom_part_types.to_rumbas();
    (
        NormalExam {
            locales: Value::Normal(vec![Value::Normal(Locale {
                name: Value::Normal("en".to_string()),
                numbas_locale: Value::Normal(SupportedLocale::EnGB),
            })]), // todo: argument?
            name: Value::Normal(TranslatableString::s(&exam.basic_settings.name)), // todo: argument
            navigation: Value::Normal(exam.to_rumbas()),
            timing: Value::Normal(exam.to_rumbas()),
            feedback: Value::Normal(exam.to_rumbas()),
            question_groups: Value::Normal(question_groups.clone()),
            numbas_settings: Value::Normal(NumbasSettings {
                locale: Value::Normal(SupportedLocale::EnGB),
                theme: Value::Normal("default".to_string()),
            }), // todo: argument?
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
    let question_groups = exam
        .question_groups
        .to_rumbas()
        .into_iter()
        .map(Value::Normal)
        .collect::<Vec<_>>();
    let custom_part_types = exam.custom_part_types.to_rumbas();
    (
        DiagnosticExam {
            locales: Value::Normal(vec![Value::Normal(Locale {
                name: Value::Normal("en".to_string()),
                numbas_locale: Value::Normal(SupportedLocale::EnGB),
            })]), // todo: argument?
            name: Value::Normal(TranslatableString::s(&exam.basic_settings.name)), // todo: argument
            navigation: Value::Normal(exam.to_rumbas()),
            timing: Value::Normal(exam.to_rumbas()),
            feedback: Value::Normal(exam.to_rumbas()),
            question_groups: Value::Normal(question_groups.clone()),
            numbas_settings: Value::Normal(NumbasSettings {
                locale: Value::Normal(SupportedLocale::EnGB),
                theme: Value::Normal("default".to_string()),
            }), // todo: argument?
            diagnostic: Value::Normal(exam.diagnostic.unwrap().to_rumbas()),
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
