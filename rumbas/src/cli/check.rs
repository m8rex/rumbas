use rayon::prelude::*;
use rumbas::support::dependency_manager::DEPENDENCIES;
use rumbas::support::file_manager::CACHE;
use rumbas::support::rc::within_repo;
use rumbas::support::to_numbas::ToNumbas;
use rumbas_support::path::RumbasPath;
use rumbas_support::preamble::Input;
use std::collections::HashSet;
use std::path::Path;

pub fn find_all_files(path: RumbasPath) -> Vec<RumbasPath> {
    if path.is_file() {
        vec![path]
    } else if path.is_dir() {
        if path.in_main_folder(rumbas::QUESTIONS_FOLDER) {
            CACHE.find_all_questions_in_folder(path)
        } else {
            CACHE.find_all_exams_in_folder(path)
        }
        .into_iter()
        .map(|f| f.file_path)
        .collect::<Vec<_>>()
    } else {
        log::error!("Symbolic links are not (yet) supported");
        std::process::exit(1);
    }
}

pub fn files_from_paths(exam_question_paths: Vec<String>) -> Result<HashSet<RumbasPath>, ()> {
    let mut files: HashSet<_> = HashSet::new();
    for exam_question_path in exam_question_paths.iter() {
        let path = Path::new(exam_question_path);
        log::info!("Looking for files for {:?}", path.display());
        let path = within_repo(&path);
        log::debug!("Found path within rumbas project {:?}", path);
        if let Some(path) = path {
            if crate::cli::rc::check_rc(&path, false) {
                files.extend(find_all_files(path).into_iter());
            } else {
                return Err(());
            }
        } else {
            log::error!(
                "{:?} doesn't seem to belong to a rumbas project.",
                exam_question_path
            );
            return Err(());
        }
    }
    Ok(files)
}

pub fn check(exam_question_paths: Vec<String>) {
    match check_internal(exam_question_paths) {
        Ok(_) => (),
        Err(_) => std::process::exit(1),
    }
}

pub fn check_internal(exam_question_paths: Vec<String>) -> Result<(), ()> {
    let mut files: HashSet<_> = files_from_paths(exam_question_paths)?;
    let check_results: Vec<(CheckResult, _)> = files
        .into_par_iter()
        .map(|file| (check_file(&file), file))
        .collect();

    let failures: Vec<_> = check_results
        .par_iter()
        .filter(|(result, _)| match result {
            CheckResult::Partial(p) => !p.failed.is_empty(),
            CheckResult::Template => false,
            _ => true,
        })
        .collect();
    if !failures.is_empty() {
        for (check_result, path) in failures.iter() {
            log::error!("Check for {} failed:", path.display());
            check_result.log(path);
        }
        log::error!("{} files failed.", failures.len());
        Err(())
    } else {
        log::info!("All checks passed.");
        Ok(())
    }
}

pub enum CheckResult {
    FailedParsing(rumbas::exam::ParseError),
    LocalesNotSet,
    FailedInputCheck(rumbas_support::input::InputCheckResult),
    /// The file is a template
    Template,
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
    pub fn log(&self, path: &RumbasPath) {
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
    pub fn log(&self, path: &RumbasPath) {
        match self {
            Self::FailedParsing(e) => log::error!("{}", e),
            Self::LocalesNotSet => log::error!("Locales not set for {}!", path.display()),
            Self::FailedInputCheck(e) => e.log(path),
            Self::Partial(r) => r.log(path),
            Self::Template => (),
        }
    }
}

/// Return true if parsing is ok
pub fn check_file(path: &RumbasPath) -> CheckResult {
    log::info!("Checking {:?}", path.display());
    let exam_input_result = rumbas::exam::RecursiveTemplateExamInput::from_file(path);
    match exam_input_result {
        Ok(mut exam_input) => {
            exam_input.normalize(path);

            DEPENDENCIES.add_dependencies(path.clone(), exam_input.dependencies(path));

            let exam_result = exam_input.to_normal_safe();
            match exam_result {
                Ok(exam) => {
                    if exam.data.locales().is_empty() {
                        CheckResult::LocalesNotSet
                    } else {
                        let mut failed_locales = Vec::new();
                        let mut passed_locales = Vec::new();
                        for locale_item in exam.data.locales().iter() {
                            let locale = locale_item.name.to_owned();
                            let numbas = exam.to_numbas_safe(&locale);
                            match numbas {
                                Ok(numbas_exam) => {
                                    passed_locales.push((
                                        locale,
                                        numbas_exam,
                                        locale_item.numbas_locale,
                                        exam.data.numbas_settings().theme,
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
                Err(check_result) => {
                    if check_result.is_empty_except_template_keys() {
                        CheckResult::Template
                    } else {
                        CheckResult::FailedInputCheck(check_result)
                    }
                }
            }
        }
        Err(e) => CheckResult::FailedParsing(e),
    }
}
