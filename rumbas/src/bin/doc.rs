use rumbas::exam::ExamFileType;
use rumbas::question::QuestionFileType;
use structdoc::StructDoc;

const DATATYPES_PATH: &'static str = "../book/src/datatypes";

fn main() {
    clear_mds();

    println!("Writing ExamFileType documentation file");
    let mut created_tables = Default::default();
    let exam_docs = ExamFileType::document().markdown(&mut created_tables, true);
    write_md("complete_exam".to_string(), exam_docs);

    println!("Writing QuestionFileType documentation file");
    let mut created_tables = Default::default();
    let exam_docs = QuestionFileType::document().markdown(&mut created_tables, true);
    write_md("complete_question".to_string(), exam_docs);

    println!("Writing datatype md files for ExamFileType");
    let mut created_tables = Default::default();
    let tables = ExamFileType::document().markdown_tables(&mut created_tables, false);
    tables.into_iter().for_each(|(name, md)| write_md(name, md));

    println!("Writing datatype md files for QuestionFileType");
    let tables = QuestionFileType::document().markdown_tables(&mut created_tables, false);
    tables.into_iter().for_each(|(name, md)| write_md(name, md));
}

fn clear_mds() {
    std::fs::remove_dir_all(DATATYPES_PATH).unwrap();
    std::fs::create_dir(DATATYPES_PATH).unwrap();
}

fn write_md(name: String, md: String) {
    let filepath = format!("{}/{}.md", DATATYPES_PATH, name);
    println!("Writing {}", filepath);
    std::fs::write(filepath, md).expect("writing to work");
}
