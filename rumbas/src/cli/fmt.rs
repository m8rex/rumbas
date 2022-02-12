use super::check;
use rayon::prelude::*;
use rumbas::support::file_manager::CACHE;
use rumbas_support::preamble::FileToLoad;
use std::collections::HashSet;
use std::path::{Path, PathBuf};
use yaml_rust::{yaml::Yaml, YamlEmitter, YamlLoader};

pub fn fmt(matches: &clap::ArgMatches) {
    match fmt_internal(
        matches
            .values_of("EXAM_OR_QUESTION_PATH")
            .map(|vals| vals.collect::<Vec<_>>())
            .unwrap_or_default(),
    ) {
        Ok(_) => (),
        Err(_) => std::process::exit(1),
    }
}

pub fn fmt_internal(exam_question_paths: Vec<&str>) -> Result<(), ()> {
    let mut files: HashSet<PathBuf> = HashSet::new();
    for exam_question_path in exam_question_paths.iter() {
        let path = Path::new(exam_question_path);
        log::info!("Formatting {:?}", path.display());
        if path.is_absolute() {
            log::error!("Absolute path's are not supported");
            return Err(());
        }
        files.extend(check::find_all_files(path).into_iter());
    }
    let check_results: Vec<(RumbasFormatResult, PathBuf)> = files
        .into_par_iter()
        .map(|file| (format_file(&file), file))
        .collect();

    let failures: Vec<_> = check_results
        .par_iter()
        .filter(|(result, _)| match result {
            RumbasFormatResult::Ok => false,
            _ => true,
        })
        .collect();
    if failures.len() > 0 {
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
    pub fn log(&self, path: &Path) {
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

pub fn format_file(path: &Path) -> RumbasFormatResult {
    log::info!("Formatting {:?}", path.display());
    match CACHE.read_file(FileToLoad {
        file_path: path.to_path_buf(),
        locale_dependant: false,
    }) {
        Some(a) => match a {
            rumbas_support::input::LoadedFile::Normal(n) => {
                let loaded = YamlLoader::load_from_str(&n.content[..]);
                match loaded {
                    Ok(yaml) => {
                        let new_yaml = format(yaml[0].clone());

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
                        }
                    }
                    Err(_) => RumbasFormatResult::FailedParsingYaml,
                }
            }
            _ => RumbasFormatResult::NotFormattableFile,
        },
        None => RumbasFormatResult::FailedReadingFile,
    }
}

fn format(y: Yaml) -> Yaml {
    y
}
