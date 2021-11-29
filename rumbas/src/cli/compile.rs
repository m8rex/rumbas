use rumbas::support::default::combine_with_default_files;

use rumbas::support::to_numbas::ToNumbas;
use rumbas_support::preamble::Input;
use std::env;
use std::path::Path;
use std::path::PathBuf;

/// The name of the local folder used as cache
/// It caches the .exam files that are given to Numbas.
const CACHE_FOLDER: &str = ".rumbas";

/// The name of the local folder used for the output.
const OUTPUT_FOLDER: &str = "_output";

pub fn compile(matches: &clap::ArgMatches) {
    let path = Path::new(matches.value_of("EXAM_OR_QUESTION_PATH").unwrap());
    log::info!("Compiling {:?}", path.display());
    if path.is_absolute() {
        log::error!("Absolute path's are not supported");
        return;
    }

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
                                    //println!("{}", numbas_output_path.display());
                                    let compiler = NumbasCompiler {
                                        use_scorm: matches.is_present("scorm"),
                                        as_zip: matches.is_present("zip"),
                                        exam_path: path.to_path_buf(),
                                        numbas_locale: locale_item
                                            .numbas_locale
                                            .to_str()
                                            .to_string(),
                                        locale,
                                        theme: exam.numbas_settings().theme,
                                        exam: res,
                                        minify: !matches.is_present("no-minification"),
                                    };
                                    if !compiler.compile() {
                                        something_failed = true;
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

pub struct NumbasCompiler {
    use_scorm: bool,
    as_zip: bool,
    exam_path: PathBuf,
    locale: String,
    numbas_locale: String,
    theme: String,
    minify: bool,
    exam: numbas::exam::Exam,
}

impl NumbasCompiler {
    /// Return the locale folder within the cache folder
    fn numbas_exam_folder(&self) -> PathBuf {
        Path::new(CACHE_FOLDER).join(&self.locale) //TODO, in filename?
    }
    /// Returns the path where the numbas exam should be saved
    fn numbas_exam_path(&self) -> PathBuf {
        let numbas_exam_name = self.exam_path.with_extension("exam");
        let numbas_exam_path = self.numbas_exam_folder().join(&numbas_exam_name);
        numbas_exam_path
    }
    /// Returns the locale folder within the output folder
    fn locale_output_folder(&self) -> PathBuf {
        Path::new(OUTPUT_FOLDER).join(&self.locale)
    }
    /// Creates the output path for the generated html
    fn output_path(&self) -> PathBuf {
        let output_file = self.exam_path.with_extension(self.output_extension());
        self.locale_output_folder()
            .canonicalize()
            .unwrap()
            .join(output_file)
    }
    /// Return the extension of the output
    fn output_extension(&self) -> &'static str {
        if self.as_zip {
            "zip"
        } else {
            ""
        }
    }
    /// Create the needed folder structure
    /// Creates the folders in the cache folder
    /// Creates the folders in the output folder
    fn create_folder_structure(&self) -> () {
        std::fs::create_dir_all(self.numbas_exam_folder()).expect("Failed to create cache folders");
        std::fs::create_dir_all(self.locale_output_folder())
            .expect("Failed to create output locale folder fath");
        let output_path = self.output_path();
        if !self.as_zip {
            // Remove current folder
            std::fs::remove_dir_all(&output_path).unwrap_or(()); //If error, don't mind
                                                                 // Create folder
            std::fs::create_dir_all(&output_path).expect("Failed creating folder for output");
        } else {
            std::fs::create_dir_all(&output_path.parent().unwrap())
                .expect("Failed creating folder for output");
        };
    }
    /// Execute numbas through the python3 cli interface
    fn execute_numbas(&self) -> std::process::Output {
        let numbas_path = env::var(rumbas::NUMBAS_FOLDER_ENV)
            .expect(&format!("{} to be set", rumbas::NUMBAS_FOLDER_ENV)[..]);

        let mut args: Vec<&str> = Vec::new();

        args.push("-l");
        args.push(&self.numbas_locale[..]);

        args.push("-t");
        args.push(&self.theme[..]);

        if self.use_scorm {
            args.push("-s");
        }
        if self.as_zip {
            args.push("-z");
        }
        if self.minify {
            args.push("--minify_js");
            args.push("uglifyjs");

            args.push("--minify_css");
            args.push("uglifycss");
        }

        args.push("-o");
        let output_path = self.output_path();
        args.push(output_path.to_str().unwrap());

        let exam_path = self.numbas_exam_path().canonicalize().unwrap();
        args.push(exam_path.to_str().unwrap());

        log::debug!("Compile numbas with args {:?}", args.join(", "));

        std::process::Command::new("python3")
            .current_dir(numbas_path.clone())
            .arg("bin/numbas.py")
            .args(&args)
            .output()
            .expect("failed to execute numbas process")
    }
    /// Compile the numbas exam
    pub fn compile(&self) -> bool {
        self.create_folder_structure();
        let exam_file_path = self.numbas_exam_path();
        let exam_write_res = self.exam.write(exam_file_path.to_str().unwrap());
        match exam_write_res {
            numbas::exam::WriteResult::IOError(e) => {
                log::error!(
                    "Failed saving the exam file {} because of {}.",
                    exam_file_path.to_str().unwrap(),
                    e
                );
                return false;
            }
            numbas::exam::WriteResult::JSONError(e) => {
                log::error!(
                    "Failed generating the exam file {} because of {}.",
                    exam_file_path.to_str().unwrap(),
                    e
                );
                return false;
            }
            numbas::exam::WriteResult::Ok => {
                log::info!("Generated and saved exam file for locale {}.", self.locale);

                let output = self.execute_numbas();
                if !output.stdout.is_empty() {
                    log::debug!("{}", std::str::from_utf8(&output.stdout).unwrap());
                }
                if !output.stderr.is_empty() {
                    log::error!("Compilation failed. Use -v to see more");
                    log::debug!("{}", std::str::from_utf8(&output.stderr).unwrap());
                    return false;
                }
            }
        }
        true
    }
}
