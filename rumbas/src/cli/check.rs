use rayon::prelude::*;
use rumbas::support::file_manager::CACHE;
use rumbas::support::to_numbas::ToNumbas;
use rumbas_support::preamble::Input;
use std::path::{Path, PathBuf};

pub fn find_all_files(path: &Path) -> Vec<PathBuf> {
    if path.is_file() {
        vec![path.to_path_buf()]
    } else if path.is_dir() {
        if path.starts_with("questions") {
            CACHE.find_all_questions_in_folder(path.to_path_buf())
        } else {
            CACHE.find_all_exams_in_folder(path.to_path_buf())
        }
        .into_iter()
        .map(|f| f.file_path)
        .collect::<Vec<_>>()
    } else {
        log::error!("Symbolic links are not (yet) supported");
        std::process::exit(1);
    }
}

pub fn check(matches: &clap::ArgMatches) {
    let path = Path::new(matches.value_of("EXAM_OR_QUESTION_PATH").unwrap());
    log::info!("Checking {:?}", path.display());
    if path.is_absolute() {
        log::error!("Absolute path's are not supported");
        return;
    }
    let files = find_all_files(path);
    let check_results: Vec<(CheckResult, PathBuf)> = files
        .into_par_iter()
        .map(|file| (check_file(matches, &file), file))
        .collect();

    let nb_failures: usize = check_results
        .par_iter()
        .fold(
            || 0,
            |nb, (result, _)| match result {
                CheckResult::Partial(p) => {
                    if p.failed.is_empty() {
                        nb
                    } else {
                        nb + 1
                    }
                }
                _ => nb + 1,
            },
        )
        .sum();
    if nb_failures > 0 {
        for (check_result, path) in check_results.iter() {
            log::error!("Check for {} failed:", path.display());
            check_result.log(path);
        }
        log::error!("{} files failed.", nb_failures);
        std::process::exit(1);
    } else {
        log::info!("All checks passed.")
    }
}

pub enum CheckResult {
    FailedParsing(rumbas::exam::ParseError),
    LocalesNotSet,
    FailedInputCheck(rumbas_support::input::InputCheckResult),
    Partial(RumbasCheckData),
}

pub struct RumbasCheckData {
    failed: Vec<(String, rumbas_support::rumbas_check::RumbasCheckResult)>,
    passed: Vec<(
        String,
        numbas::exam::Exam,
        rumbas::exam::locale::SupportedLocale,
        String,
    )>,
}

impl RumbasCheckData {
    pub fn log(&self, path: &Path) {
        for (locale, check_result) in self.failed.iter() {
            log::error!(
                "Error when processing locale {} for {}.",
                locale,
                path.display()
            );

            check_result.log();
        }
    }
    pub fn passed(
        &self,
    ) -> Vec<(
        String,
        numbas::exam::Exam,
        rumbas::exam::locale::SupportedLocale,
        String,
    )> {
        self.passed.clone()
    }
    pub fn failed(&self) -> Vec<(String, rumbas_support::rumbas_check::RumbasCheckResult)> {
        self.failed.clone()
    }
}

impl CheckResult {
    pub fn log(&self, path: &Path) {
        match self {
            Self::FailedParsing(e) => log::error!("{}", e),
            Self::LocalesNotSet => log::error!("Locales not set for {}!", path.display()),
            Self::FailedInputCheck(e) => e.log(path),
            Self::Partial(r) => r.log(path),
        }
    }
}

/// Return true if parsing is ok
pub fn check_file(_matches: &clap::ArgMatches, path: &Path) -> CheckResult {
    log::info!("Checking {:?}", path.display());
    let exam_input_result = rumbas::exam::ExamInput::from_file(path);
    match exam_input_result {
        Ok(mut exam_input) => {
            exam_input.combine_with_defaults(path);

            let exam_result = exam_input.to_normal_safe();
            match exam_result {
                Ok(exam) => {
                    if exam.locales().is_empty() {
                        CheckResult::LocalesNotSet
                    } else {
                        let mut failed_locales = Vec::new();
                        let mut passed_locales = Vec::new();
                        for locale_item in exam.locales().iter() {
                            let locale = locale_item.name.to_owned();
                            let numbas = exam.to_numbas_safe(&locale);
                            match numbas {
                                Ok(numbas_exam) => {
                                    passed_locales.push((
                                        locale,
                                        numbas_exam,
                                        locale_item.numbas_locale,
                                        exam.numbas_settings().theme,
                                    ));
                                }
                                Err(check_result) => {
                                    failed_locales.push((locale, check_result));
                                }
                            }
                        }
                        CheckResult::Partial(RumbasCheckData {
                            passed: passed_locales,
                            failed: failed_locales,
                        })
                    }
                }
                Err(check_result) => CheckResult::FailedInputCheck(check_result),
            }
        }
        Err(e) => CheckResult::FailedParsing(e),
    }
}