use rumbas::support::default::combine_with_default_files;

use rumbas::support::optional_overwrite::Input;
use rumbas::support::to_numbas::ToNumbas;
use std::env;
use std::path::Path;

/// The name of the local folder used as cache
/// It caches the .exam files that are given to Numbas.
const CACHE_FOLDER: &str = ".rumbas";

/// The name of the local folder used for the output.
const OUTPUT_FOLDER: &str = "_output";

pub fn compile(matches: &clap::ArgMatches) {
    let numbas_path = env::var(rumbas::NUMBAS_FOLDER_ENV)
        .expect(&format!("{} to be set", rumbas::NUMBAS_FOLDER_ENV)[..]);
    let path = Path::new(matches.value_of("EXAM_OR_QUESTION_PATH").unwrap());
    log::info!("Compiling {:?}", path.display());
    if path.is_absolute() {
        log::error!("Absolute path's are not supported");
        return;
    }

    let mut extra_args: Vec<&str> = Vec::new();
    if matches.is_present("scorm") {
        extra_args.push("-s");
    }
    let output_extension = if matches.is_present("zip") {
        extra_args.push("-z");
        "zip"
    } else {
        ""
    };

    let exam_input_result = rumbas::exam::ExamInput::from_file(path);
    match exam_input_result {
        Ok(mut exam_input) => {
            combine_with_default_files(path, &mut exam_input);
            let exam_result = exam_input.to_normal_safe();
            match exam_result {
                Ok(exam) => {
                    if exam.locales().is_empty() {
                        log::error!("Locales not set!");
                    } else {
                        let mut something_failed: bool = false;
                        for locale_item in exam.locales().iter() {
                            let locale = locale_item.name.to_owned();
                            let numbas = exam.to_numbas_safe(&locale);
                            match numbas {
                                Ok(res) => {
                                    let numbas_exam_name = path.with_extension("exam");
                                    let numbas_exam_path = Path::new(CACHE_FOLDER)
                                        .join(&locale) //TODO, in filename?
                                        .join(&numbas_exam_name);
                                    std::fs::create_dir_all(numbas_exam_path.parent().unwrap())
                                        .expect("Failed to create cache folders");
                                    let locale_folder_path = Path::new(OUTPUT_FOLDER).join(&locale);
                                    std::fs::create_dir_all(&locale_folder_path)
                                        .expect("Failed to create output locale folder fath");
                                    let absolute_path = locale_folder_path
                                        .canonicalize()
                                        .unwrap()
                                        .join(path.with_extension(output_extension));
                                    if output_extension.is_empty() {
                                        // Remove current folder
                                        std::fs::remove_dir_all(&absolute_path).unwrap_or(()); //If error, don't mind
                                                                                               // Create folder
                                        std::fs::create_dir_all(&absolute_path)
                                            .expect("Failed creating folder for output");
                                    } else {
                                        std::fs::create_dir_all(&absolute_path.parent().unwrap())
                                            .expect("Failed creating folder for output");
                                    };
                                    let numbas_output_path = absolute_path;
                                    //println!("{}", numbas_output_path.display());

                                    let exam_write_res =
                                        res.write(numbas_exam_path.to_str().unwrap());
                                    match exam_write_res {
                                        numbas::exam::exam::WriteResult::IOError(e) => {
                                            log::error!(
                                                "Failed saving the exam file because of {}.",
                                                e
                                            );
                                            something_failed = true;
                                        }
                                        numbas::exam::exam::WriteResult::JSONError(e) => {
                                            log::error!(
                                                "Failed generating the exam file because of {}.",
                                                e
                                            );
                                            something_failed = true;
                                        }
                                        numbas::exam::exam::WriteResult::Ok => {
                                            log::info!(
                                                "Generated and saved exam file for locale {}.",
                                                locale
                                            );

                                            let numbas_settings = exam.numbas_settings();

                                            let output = std::process::Command::new("python3")
                                                .current_dir(numbas_path.clone())
                                                .arg("bin/numbas.py")
                                                .arg("-l")
                                                .arg(locale_item.numbas_locale.to_str())
                                                .arg("-t")
                                                .arg(numbas_settings.theme)
                                                .args(&extra_args)
                                                .arg("-o")
                                                .arg(numbas_output_path)
                                                .arg(numbas_exam_path.canonicalize().unwrap()) //TODO?
                                                .output()
                                                .expect("failed to execute numbas process");
                                            if !output.stdout.is_empty() {
                                                log::debug!(
                                                    "{}",
                                                    std::str::from_utf8(&output.stdout).unwrap()
                                                );
                                            }
                                            if !output.stderr.is_empty() {
                                                log::error!(
                                                    "Compilation failed. Use -v to see more"
                                                );
                                                log::debug!(
                                                    "{}",
                                                    std::str::from_utf8(&output.stderr).unwrap()
                                                );
                                                something_failed = true;
                                            }
                                        }
                                    }
                                }
                                Err(check_result) => {
                                    something_failed = true;
                                    let missing_translations = check_result.missing_translations();
                                    let invalid_jme_fields = check_result.invalid_jme_fields();
                                    log::error!("Error when processing locale {}.", locale);
                                    if !missing_translations.is_empty() {
                                        log::error!(
                                            "Found {} missing translations:",
                                            missing_translations.len()
                                        );
                                        for (idx, error) in missing_translations.iter().enumerate()
                                        {
                                            log::error!("{}\t{}", idx + 1, error.to_string());
                                        }
                                    }
                                    if !invalid_jme_fields.is_empty() {
                                        log::error!(
                                            "Found {} invalid jme expressions:",
                                            invalid_jme_fields.len()
                                        );
                                        for (idx, error) in invalid_jme_fields.iter().enumerate() {
                                            log::error!("{}\t{}", idx + 1, error.to_string());
                                        }
                                    }
                                }
                            }
                        }
                        if something_failed {
                            std::process::exit(1)
                        }
                    }
                }
                Err(check_result) => {
                    let missing_fields = check_result.missing_fields();
                    let invalid_yaml_fields = check_result.invalid_yaml_fields();
                    log::error!("Error when processing to yaml input.");
                    if !missing_fields.is_empty() {
                        log::error!("Found {} missing fields:", missing_fields.len());
                        for (idx, error) in missing_fields.iter().enumerate() {
                            log::error!("{}\t{}", idx + 1, error.to_string());
                        }
                    }
                    if !invalid_yaml_fields.is_empty() {
                        log::error!("Found {} invalid fields:", invalid_yaml_fields.len());
                        for (idx, error) in invalid_yaml_fields.iter().enumerate() {
                            log::error!("{}\t{}", idx + 1, error.to_string());
                        }
                    }
                }
            }
        }
        Err(e) => {
            log::error!("{}", e);
            std::process::exit(1)
        }
    };
}
