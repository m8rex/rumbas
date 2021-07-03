use numbas::exam::Exam;

macro_rules! read {
    ($file_name: expr) => {{
        let content = std::fs::read_to_string($file_name).expect("Invalid file path");
        Exam::from_str(content.as_ref())
    }};
}

fn main() {
    let exam = read!("example.exam");
    println!("{:?}", exam);
}
