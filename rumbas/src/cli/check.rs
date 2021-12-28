use rumbas::support::file_manager::CACHE;
use rumbas::support::to_numbas::ToNumbas;
use rumbas_support::preamble::Input;
use std::path::Path;

pub fn check(matches: &clap::ArgMatches) {
    let path = Path::new(matches.value_of("EXAM_OR_QUESTION_PATH").unwrap());
    log::info!("Compiling {:?}", path.display());
    if path.is_absolute() {
        log::error!("Absolute path's are not supported");
        return;
    }

    let files = if path.is_file() {
        vec![path.to_path_buf()]
    } else if path.is_dir() {
        if path.starts_with("question") {
            CACHE.find_all_questions_in_folder(path.to_path_buf())
        } else {
            CACHE.find_all_exams_in_folder(path.to_path_buf())
        }
        .into_iter()
        .map(|f| f.file_path)
        .collect()
    } else {
        log::error!("Symbolic links are not (yet) supported");
        std::process::exit(1);
    };
    let nb_failures = files.iter().fold(0, |nb, file| {
        if check_file(matches, &file) {
            nb
        } else {
            nb + 1
        }
    });
    if nb_failures > 0 {
        log::error!("{} files failed.", nb_failures);
        std::process::exit(1);
    } else {
        log::info!("All checks passed.")
    }
}

/// Return true if parsing is ok
fn check_file(_matches: &clap::ArgMatches, path: &Path) -> bool {
    log::info!("Compiling {:?}", path.display());
    let exam_input_result = rumbas::exam::ExamInput::from_file(path);
    match exam_input_result {
        Ok(mut exam_input) => {
            exam_input.combine_with_defaults(&path);

            let exam_result = exam_input.to_normal_safe();
            match exam_result {
                Ok(exam) => {
                    if exam.locales().is_empty() {
                        log::error!("Locales not set for {}!", path.display());
                        return false;
                    } else {
                        let mut something_failed: bool = false;
                        for locale_item in exam.locales().iter() {
                            let locale = locale_item.name.to_owned();
                            let numbas = exam.to_numbas_safe(&locale);
                            match numbas {
                                Ok(_) => {}
                                Err(check_result) => {
                                    something_failed = true;
                                    log::error!(
                                        "Error when processing locale {} for {}.",
                                        locale,
                                        path.display()
                                    );
                                    check_result.log();
                                }
                            }
                        }
                        return !something_failed;
                    }
                }
                Err(check_result) => {
                    log::info!("Failed compiling {:?}", path.display());
                    check_result.log();
                    return false;
                }
            }
        }
        Err(e) => {
            log::error!("{}", e);
            return false;
        }
    };
}
