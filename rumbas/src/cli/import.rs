use numbas::exam::Exam as NExam;
use rumbas::exam::convert_numbas_exam;
use rumbas::exam::question_group::QuestionFromTemplate;
use rumbas::question::custom_part_type::CustomPartTypeDefinitionPath;
use rumbas::question::QuestionFileType;
use rumbas::support::to_rumbas::ToRumbas;

fn read_pretty_exam(path: &std::path::Path) -> String {
    let pretty_path = path.with_extension("exam.pretty");
    let exam_changed = pretty_path
        .metadata()
        .map(|m| m.modified())
        .and_then(std::convert::identity) // flatten result
        .map(|pretty_time| {
            path.metadata()
                .map(|m| m.modified())
                .and_then(std::convert::identity)
                .map(|normal_time| normal_time > pretty_time)
        })
        .and_then(std::convert::identity);
    let should_create_pretty =
        !pretty_path.exists() || exam_changed.is_err() || exam_changed.unwrap();
    if should_create_pretty {
        let normal_content = std::fs::read_to_string(path).expect("Invalid file path");
        let json_content = numbas::exam::Exam::clean_exam_str(&normal_content[..]);
        let v: serde_json::Value =
            serde_json::from_str(json_content).expect("failed parsing exam json");
        let pretty_exam =
            serde_json::to_string_pretty(&v).expect("failed generating json of parsed exam json");
        let pretty_exam_content = numbas::exam::Exam::to_exam_str(&pretty_exam[..]);
        std::fs::write(&pretty_path, pretty_exam_content)
            .expect("Writing pretty exam file to work");
    }
    std::fs::read_to_string(pretty_path).expect("Invalid file path")
}

macro_rules! read_exam {
    ($file_name: expr) => {{
        let content = read_pretty_exam($file_name);
        NExam::from_exam_str(content.as_ref())
    }};
}

macro_rules! read_question {
    ($file_name: expr) => {{
        let content = read_pretty_exam($file_name);
        numbas::question::Question::from_question_exam_str(content.as_ref())
    }};
}

pub fn import(path: String, is_question: bool) {
    let repo_path = std::path::Path::new(".");
    let repo_path = rumbas::support::rc::within_repo(&repo_path);
    if let Some(repo_path) = repo_path {
        if !crate::cli::rc::check_rc(&repo_path, false) {
            std::process::exit(1);
        }
        let path = std::path::Path::new(&path);
        if is_question {
            let question_res = read_question!(path);
            match question_res {
                Ok(question) => {
                    let rumbas_question: QuestionFromTemplate = question.to_rumbas();
                    let data = rumbas_question.data.clone();
                    for cpt in data.custom_part_types.iter() {
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
                    // TODO this will be done automatically on deserialization now?
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
    } else {
        log::error!(
            "{:?} doesn't seem to belong to a rumbas project.",
            repo_path
        );
        std::process::exit(1);
    }
}

fn create_question(qf: QuestionFromTemplate) {
    match qf.question_path {
        Some(path) => {
            let q_name = path.clone();
            let q_yaml = QuestionFileType::Normal(Box::new(qf.data))
                .to_yaml()
                .unwrap();
            let file = format!("{}/{}.yaml", rumbas::QUESTIONS_FOLDER, q_name);
            log::info!("Writing to {}", file);
            std::fs::write(file, q_yaml).unwrap(); //fix handle result
        }
        _ => unimplemented!(),
    }
}

fn create_custom_part_type(cpt: CustomPartTypeDefinitionPath) {
    let c_name = cpt.file_name.clone();
    let c_yaml = cpt.data.to_yaml().unwrap();
    let file = format!("{}/{}.yaml", rumbas::CUSTOM_PART_TYPES_FOLDER, c_name);
    log::info!("Writing to {}", file);
    std::fs::write(file, c_yaml).unwrap(); //fix handle result
}
