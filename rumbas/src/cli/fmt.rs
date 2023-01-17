use super::check;
use rayon::prelude::*;
use rumbas::support::file_manager::CACHE;
use rumbas_support::path::RumbasPath;
use rumbas_support::preamble::FileToLoad;
use std::collections::HashSet;
use yaml_subset::yaml::parse_yaml_file;

pub fn fmt(exam_question_paths: Vec<String>) {
    match fmt_internal(exam_question_paths) {
        Ok(_) => (),
        Err(_) => std::process::exit(1),
    }
}

pub fn fmt_internal(exam_question_paths: Vec<String>) -> Result<(), ()> {
    let files: HashSet<_> = check::files_from_paths(exam_question_paths)?;
    let check_results: Vec<(RumbasFormatResult, _)> = files
        .into_par_iter()
        .map(|file| (format_file(&file), file))
        .collect();

    let failures: Vec<_> = check_results
        .par_iter()
        .filter(|(result, _)| !matches!(result, RumbasFormatResult::Ok))
        .collect();
    if !failures.is_empty() {
        for (check_result, path) in failures.iter() {
            log::error!("Format for {} failed:", path.display());
            check_result.log(path);
        }
        log::error!("{} files failed.", failures.len());
        Err(())
    } else {
        log::info!("All checks passed.");
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub enum RumbasFormatResult {
    Ok,
    FailedReadingFile,
    FailedParsingYaml,
    FailedConvertingToYamlString,
    FailedWritingFile,
    NotFormattableFile,
}

impl RumbasFormatResult {
    pub fn log(&self, path: &RumbasPath) {
        log::error!("Error when processing {}.", path.display());
        match self {
            Self::FailedReadingFile => log::error!("Can't read the file."),
            Self::FailedParsingYaml => log::error!("Can't parse the file."),
            Self::NotFormattableFile => log::error!("File can't be formatted."),

            Self::FailedConvertingToYamlString => {
                log::error!("Failed generating the formatted yaml string.")
            }
            Self::FailedWritingFile => log::error!("Formatted yaml can't be written to file."),
            Self::Ok => log::error!("Formatting worked."),
        }
    }
}

pub fn format_file(path: &RumbasPath) -> RumbasFormatResult {
    log::info!("Formatting {:?}", path.display());
    match CACHE.read_file(FileToLoad {
        file_path: path.clone(),
        locale_dependant: false,
    }) {
        Some(a) => match a {
            rumbas_support::input::LoadedFile::Normal(n) => {
                let loaded = parse_yaml_file(&n.content[..]);
                match loaded {
                    Ok(yaml) => {
                        /*
                        let new_yaml = format(yaml[0].clone());
                        println!("{:#?}", new_yaml);
                        let mut out_str = String::new();
                        let mut emitter = YamlEmitter::new(&mut out_str);
                        emitter.multiline_strings(true);
                        let dump_res = emitter.dump(&new_yaml);
                        match dump_res {
                            Ok(_) => match std::fs::write(path, out_str) {
                                Ok(_) => RumbasFormatResult::Ok,
                                Err(_) => RumbasFormatResult::FailedWritingFile,
                            },
                            Err(_) => RumbasFormatResult::FailedConvertingToYamlString,
                        }*/
                        let dump_res = yaml.format();
                        match dump_res {
                            Ok(res) => match std::fs::write(path, res) {
                                Ok(_) => RumbasFormatResult::Ok,
                                Err(_) => RumbasFormatResult::FailedWritingFile,
                            },
                            Err(_) => RumbasFormatResult::FailedConvertingToYamlString,
                        }
                    }
                    Err(e) => {
                        log::debug!("Failed parsing {:?}\n{}", path.display(), e);
                        RumbasFormatResult::FailedParsingYaml
                    }
                }
            }
            _ => RumbasFormatResult::NotFormattableFile,
        },
        None => RumbasFormatResult::FailedReadingFile,
    }
}
