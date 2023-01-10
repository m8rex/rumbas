use super::check::files_from_paths;
use rayon::prelude::*;
use rumbas_support::path::RumbasPath;
use std::collections::HashSet;

pub fn export(exam_question_paths: Vec<String>) {
    match export_internal(exam_question_paths) {
        Ok(_) => (),
        Err(_) => std::process::exit(1),
    }
}

pub fn export_internal(exam_question_paths: Vec<String>) -> Result<(), ()> {
    let files: HashSet<_> = files_from_paths(exam_question_paths)?;
    let export_results: Vec<(ExportResult, _)> = files
        .into_par_iter()
        .map(|file| (export_file(&file), file))
        .collect();

    let failures: Vec<_> = export_results
        .par_iter()
        .filter(|(result, _)| match result {
            ExportResult::Ok => false,
            _ => true,
        })
        .collect();
    if !failures.is_empty() {
        for (export_result, path) in failures.iter() {
            log::error!("Export for {} failed:", path.display());
            export_result.log(path);
        }
        log::error!("{} files failed.", failures.len());
        Err(())
    } else {
        log::info!("All exports passed.");
        Ok(())
    }
}

pub enum ExportResult {
    FailedParsing(rumbas::exam::ParseError),
    FailedSerializing(serde_yaml::Error),
    InvalidPath,
    Ok,
}

impl ExportResult {
    pub fn log(&self, path: &RumbasPath) {
        match self {
            Self::FailedParsing(e) => log::error!("{}", e),
            Self::FailedSerializing(e) => log::error!("{}", e),
            Self::InvalidPath => log::error!(
                "{} is not within the questions or exams folder.",
                path.display()
            ),
            Self::Ok => (),
        }
    }
}

pub fn export_file(path: &RumbasPath) -> ExportResult {
    log::info!("Exporting {:?}", path.display());

    let exam_input_result = rumbas::exam::RecursiveTemplateExamInput::from_file(path);
    match exam_input_result {
        Ok(mut exam_input) => {
            exam_input.normalize(path);

            if path.in_main_folder(rumbas::EXAMS_FOLDER) {
                match serde_yaml::to_string(&exam_input) {
                    Ok(yaml) => {
                        println!("{}", yaml);
                        ExportResult::Ok
                    }
                    Err(e) => ExportResult::FailedSerializing(e),
                }
            } else if path.in_main_folder(rumbas::QUESTIONS_FOLDER) {
                let exam_file_type_input: rumbas::exam::ExamFileTypeInput = exam_input.into();
                let question = exam_file_type_input.find_question_of_preview();
                match serde_yaml::to_string(&question) {
                    Ok(yaml) => {
                        println!("{}", yaml);
                        ExportResult::Ok
                    }
                    Err(e) => ExportResult::FailedSerializing(e),
                }
            } else {
                ExportResult::InvalidPath
            }
        }
        Err(e) => ExportResult::FailedParsing(e),
    }
}
