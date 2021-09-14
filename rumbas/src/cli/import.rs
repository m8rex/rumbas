use numbas::exam::Exam as NExam;
use rumbas::exam::convert_numbas_exam;
use rumbas::exam::question_group::QuestionPath;
use rumbas::question::custom_part_type::CustomPartTypeDefinitionPath;
use rumbas::question::QuestionFileType;
use rumbas::support::to_rumbas::ToRumbas;

macro_rules! read_exam {
    ($file_name: expr) => {{
        let content = std::fs::read_to_string($file_name).expect("Invalid file path");
        NExam::from_exam_str(content.as_ref())
    }};
}

macro_rules! read_question {
    ($file_name: expr) => {{
        let content = std::fs::read_to_string($file_name).expect("Invalid file path");
        numbas::question::Question::from_question_exam_str(content.as_ref())
    }};
}

pub fn import(matches: &clap::ArgMatches) {
    let path = std::path::Path::new(matches.value_of("EXAM_PATH").unwrap());
    if matches.is_present("question") {
        let question_res = read_question!(path);
        match question_res {
            Ok(question) => {
                let rumbas_question: QuestionPath = question.to_rumbas();
                for cpt in rumbas_question.question_data.custom_part_types.iter() {
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
                let (name, rumbas_exam, qs, cpts) = convert_numbas_exam(exam);
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
