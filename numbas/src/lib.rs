pub mod exam;
pub mod question;
pub mod support;

extern crate pest;
#[macro_use]
extern crate pest_derive;
pub mod jme;
pub mod util;

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::Result;
    use std::fs;
    fn exam_from_file(file: &str) -> Result<exam::Exam> {
        let s = fs::read_to_string(file).expect(&format!("Failed to read {}", file)[..]);
        exam::Exam::from_exam_str(&s)
    }
    fn question_from_file(file: &str) -> Result<question::Question> {
        let s = fs::read_to_string(file).expect(&format!("Failed to read {}", file)[..]);
        question::Question::from_question_exam_str(&s)
    }
    #[test]
    #[ignore]
    fn it_works() -> Result<()> {
        let r = exam_from_file(
            "testfiles/exam-62579-ncm4-u1l1-practice-graphing-linear-functions.exam",
        )?;
        println!("{}", serde_json::to_string_pretty(&r).unwrap());
        Ok(())
    }

    #[test]
    fn parsing_works() {
        let _ = simple_logger::init();
        let files = vec![
            "question-132674-jesse-s-copy-of-numbas-demo-choose-one-from-a-list-part-type.exam",
            "question-132684-jesse-s-copy-of-numbas-demo-choose-several-from-a-list-part-type.exam",
            "question-132696-jesse-s-copy-of-numbas-demo-match-choices-with-answers-part-type.exam",
            "question-132698-jesse-s-copy-of-numbas-demo-match-text-pattern-part-type.exam",
            "question-132699-jesse-s-copy-of-numbas-demo-matrix-entry-part-type.exam",
        ];
        for file in files.into_iter() {
            println!("Parsing {}", file);
            let r = question_from_file(&format!("testfiles/{}", file)[..]);
            println!("{:?}", r);
            assert!(r.is_ok())
        }
    }
}
