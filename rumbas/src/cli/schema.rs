use rumbas::exam::ExamFileType;
use rumbas::exam::ExamFileTypeInput;
use rumbas::question::QuestionFileType;
use rumbas::question::QuestionFileTypeInput;
use schemars::schema_for;

pub fn schema() {
    let schema = schema_for!(ExamFileTypeInput);
    let file_name = "exam-schema.json";
    std::fs::write(file_name, serde_json::to_string_pretty(&schema).unwrap())
        .expect("writting exam schema to file");
    log::info!("{} created", file_name);
    let schema = schema_for!(QuestionFileTypeInput);
    let file_name = "question-schema.json";
    std::fs::write(file_name, serde_json::to_string_pretty(&schema).unwrap())
        .expect("writting question schema to file");
    log::info!("{} created", file_name);
    let schema = schema_for!(rumbas::question::custom_part_type::CustomPartTypeDefinitionInput);
    let file_name = "custom-part-type-schema.json";
    std::fs::write(file_name, serde_json::to_string_pretty(&schema).unwrap())
        .expect("writting custom-part-type schema to file");
    log::info!("{} created", file_name);

    let schema = schema_for!(ExamFileType);
    let file_name = "exam-schema2.json";
    std::fs::write(file_name, serde_json::to_string_pretty(&schema).unwrap())
        .expect("writting exam schema to file");
    log::info!("{} created", file_name);
    let schema = schema_for!(QuestionFileType);
    let file_name = "question-schema2.json";
    std::fs::write(file_name, serde_json::to_string_pretty(&schema).unwrap())
        .expect("writting question schema to file");
    log::info!("{} created", file_name);
    let schema = schema_for!(rumbas::question::custom_part_type::CustomPartTypeDefinition);
    let file_name = "custom-part-type-schema2.json";
    std::fs::write(file_name, serde_json::to_string_pretty(&schema).unwrap())
        .expect("writting custom-part-type schema to file");
    log::info!("{} created", file_name);
}
