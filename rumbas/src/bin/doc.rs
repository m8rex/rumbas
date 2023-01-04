use rumbas::exam::ExamFileType;
use rumbas::question::custom_part_type::CustomPartTypeDefinition;
use rumbas::question::QuestionFileType;
use structdoc::{MarkdownSettings, StructDoc};

const DATATYPES_PATH: &'static str = "../book/src/datatypes";

fn main() {
    clear_mds();

    let links = MarkdownSettings::new().without_optional();
    let no_links = MarkdownSettings::new().without_optional().without_links();

    println!("Writing ExamFileType documentation file");
    let mut created_tables = Default::default();
    let docs = ExamFileType::document().markdown(&mut created_tables, links);
    write_md("complete_exam".to_string(), docs);

    println!("Writing QuestionFileType documentation file");
    let mut created_tables = Default::default();
    let docs = QuestionFileType::document().markdown(&mut created_tables, links);
    write_md("complete_question".to_string(), docs);

    println!("Writing CustomPartTypeDefinition documentation file");
    let mut created_tables = Default::default();
    let docs = CustomPartTypeDefinition::document().markdown(&mut created_tables, links);
    write_md("complete_custom_part_type".to_string(), docs);

    println!("Writing datatype md files for ExamFileType");
    let mut created_tables = Default::default();
    let tables = ExamFileType::document().markdown_tables(&mut created_tables, no_links);
    tables.into_iter().for_each(|(name, md)| write_md(name, md));

    println!("Writing datatype md files for QuestionFileType");
    let tables = QuestionFileType::document().markdown_tables(&mut created_tables, no_links);
    tables.into_iter().for_each(|(name, md)| write_md(name, md));

    println!("Writing datatype md files for CustomPartTypeDefinition");
    let tables =
        CustomPartTypeDefinition::document().markdown_tables(&mut created_tables, no_links);
    tables.into_iter().for_each(|(name, md)| write_md(name, md));
}

fn clear_mds() {
    std::fs::remove_dir_all(DATATYPES_PATH).ok();
    std::fs::create_dir(DATATYPES_PATH).unwrap();
}

fn write_md(name: String, md: String) {
    let filepath = format!("{}/{}.md", DATATYPES_PATH, name);
    println!("Writing {}", filepath);
    std::fs::write(filepath, md).expect("writing to work");
}
