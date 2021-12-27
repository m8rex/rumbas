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

    let exam_input_result = rumbas::exam::ExamInput::from_file(path);
    match exam_input_result {
        Ok(mut exam_input) => {
            exam_input.combine_with_defaults(&path);

            let exam_result = exam_input.to_normal_safe();
            match exam_result {
                Ok(exam) => {
                    if exam.locales().is_empty() {
                        log::error!("Locales not set!");
                        std::process::exit(1)
                    } else {
                        let mut something_failed: bool = false;
                        for locale_item in exam.locales().iter() {
                            let locale = locale_item.name.to_owned();
                            let numbas = exam.to_numbas_safe(&locale);
                            match numbas {
                                Ok(_) => {}
                                Err(check_result) => {
                                    something_failed = true;
                                    log::error!("Error when processing locale {}.", locale);
                                    check_result.log();
                                }
                            }
                        }
                        if something_failed {
                            std::process::exit(1)
                        }
                    }
                }
                Err(check_result) => {
                    check_result.log();
                    std::process::exit(1)
                }
            }
        }
        Err(e) => {
            log::error!("{}", e);
            std::process::exit(1)
        }
    };
}
