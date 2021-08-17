use rumbas::data::default::combine_with_default_files;
use rumbas::data::to_numbas::ToNumbas;
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

    let exam_result = rumbas::data::exam::Exam::from_file(path);
    match exam_result {
        Ok(mut exam) => {
            combine_with_default_files(path, &mut exam);
            if exam.locales().0.is_none() {
                log::error!("Locales not set!");
            } else {
                for locale_item in exam.locales().unwrap().iter() {
                    let locale = locale_item.clone().unwrap().name.unwrap();
                    let numbas = exam.to_numbas(&locale);
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

                            let exam_write_res = res.write(numbas_exam_path.to_str().unwrap());
                            match exam_write_res {
                                numbas::exam::WriteResult::IOError(e) => {
                                    log::error!("Failed saving the exam file because of {}.", e);
                                    std::process::exit(1);
                                }
                                numbas::exam::WriteResult::JSONError(e) => {
                                    log::error!(
                                        "Failed generating the exam file because of {}.",
                                        e
                                    );
                                    std::process::exit(1);
                                }
                                numbas::exam::WriteResult::Ok => {
                                    log::info!("Generated and saved exam file.");

                                    let numbas_settings = exam.numbas_settings().clone().unwrap();

                                    let output = std::process::Command::new("python3")
                                        .current_dir(numbas_path.clone())
                                        .arg("bin/numbas.py")
                                        .arg("-l")
                                        .arg(
                                            locale_item
                                                .unwrap()
                                                .numbas_locale
                                                .clone()
                                                .unwrap()
                                                .to_str(),
                                        )
                                        .arg("-t")
                                        .arg(numbas_settings.theme.unwrap())
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
                                        log::error!("Compilation failed. Use -v to see more");
                                        log::debug!(
                                            "{}",
                                            std::str::from_utf8(&output.stderr).unwrap()
                                        );
                                        std::process::exit(1)
                                    }
                                }
                            }
                        }
                        Err(check_result) => {
                            let missing_fields = check_result.missing_fields();
                            let invalid_fields = check_result.invalid_fields();
                            log::error!(
                                "Error when processing locale {}.\nFound {} missing fields:\n{}\nFound {} invalid fields:\n{}",
                                locale,
                                missing_fields.len(),
                                missing_fields
                                    .iter()
                                    .map(|f| f.to_string())
                                    .collect::<Vec<_>>()
                                    .join("\n"),
                                invalid_fields.len(),
                                invalid_fields
                                    .iter()
                                    .map(|f| f.to_string())
                                    .collect::<Vec<_>>()
                                    .join("\n")
                            );
                            std::process::exit(1)
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
