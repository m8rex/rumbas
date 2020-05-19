pub mod exam;

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::Result;
    use std::fs;
    fn exam_from_file(file: &str) -> Result<exam::Exam> {
        let s = fs::read_to_string(file).expect("Failed to read the font json file");
        exam::Exam::from_str(&s)
    }
    #[test]
    fn it_works() -> Result<()> {
        let r = exam_from_file(
            "testfiles/exam-62579-ncm4-u1l1-practice-graphing-linear-functions.exam",
        )?;
        println!("{}", serde_json::to_string_pretty(&r).unwrap());
        //Ok(())
        exam::Exam::from_str("fail").map(|_| ())
    }
}
