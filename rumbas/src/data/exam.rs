use crate::data::optional_overwrite::OptionalOverwrite;
use serde::Deserialize;
use serde::Serialize;

optional_overwrite! {
    Exam,
    name: String,
    duration_in_seconds: usize,
    percentage_needed_to_pass: f64,
    show_names_of_question_groups: bool,
    show_name_of_student: bool
}

pub struct TmpNumbasExam {
    //TODO: remove
    name: String,
    duration_in_seconds: Option<usize>,     // in seconds
    percentage_needed_to_pass: Option<f64>, //TODO: is this a float?
    //resources: Vec<[String; 2]>,
    //extensions: Vec<String>,
    //custom_part_types: Vec<CustomPartType>,
    show_question_group_names: Option<bool>,
    show_student_name: Option<bool>,
}

impl Exam {
    //TODO use numbas::Exam
    fn to_numbas_exam(&self) -> TmpNumbasExam {
        //TODO: check for empty fields
        TmpNumbasExam {
            name: self.name.clone().unwrap(),
            duration_in_seconds: self.duration_in_seconds,
            percentage_needed_to_pass: self.percentage_needed_to_pass,
            show_question_group_names: self.show_names_of_question_groups,
            show_student_name: self.show_name_of_student,
        }
    }
}
