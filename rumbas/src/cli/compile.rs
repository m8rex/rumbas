use crate::cli::check::CheckResult;
use rayon::prelude::*;
use rumbas::support::rc::within_repo;
use rumbas_support::path::RumbasPath;
use std::collections::HashSet;
use std::env;
use std::path::Path;
use std::path::PathBuf;

/// The name of the local folder used as cache
/// It caches the .exam files that are given to Numbas.
pub const CACHE_FOLDER: &str = ".rumbas";

/// The name of the local folder used for the output.
pub const OUTPUT_FOLDER: &str = "_output";

pub fn compile(compile_paths: Vec<String>, use_scorm: bool, as_zip: bool, no_minification: bool) {
    match compile_internal(
        CompilationContext { compile_paths },
        FileCompilationContext {
            use_scorm,
            as_zip,
            minify: !no_minification,
            output_folder: Path::new(OUTPUT_FOLDER).to_path_buf(),
        },
    )
    .has_failures
    {
        false => (),
        true => std::process::exit(1),
    }
}

#[derive(Debug, Clone)]
pub struct CompilationContext {
    pub compile_paths: Vec<String>,
}

pub struct InternalCompilationResult {
    pub has_failures: bool,
    pub created_outputs: Vec<PassedRumbasCompileData>,
}

pub fn compile_internal(
    context: CompilationContext,
    file_context: FileCompilationContext,
) -> InternalCompilationResult {
    let mut files: HashSet<_> = HashSet::new();
    for exam_question_path in context.compile_paths.iter() {
        let path = Path::new(&exam_question_path);
        log::info!("Compiling {:?}", path.display());
        let path = within_repo(&path);
        log::debug!("Found path within rumbas project {:?}", path);
        if let Some(path) = path {
            if crate::cli::rc::check_rc(&path, false) {
                files.extend(crate::cli::check::find_all_files(path).into_iter());
            } else {
                return InternalCompilationResult {
                    has_failures: true,
                    created_outputs: vec![],
                };
            }
        } else {
            log::error!(
                "{:?} doesn't seem to belong to a rumbas project.",
                exam_question_path
            );
            return InternalCompilationResult {
                has_failures: true,
                created_outputs: vec![],
            };
        }
    }
    let compile_results: Vec<(CompileResult, _)> = files
        .into_par_iter()
        .map(|file| (compile_file(&file_context, &file), file))
        .collect();

    let failures: Vec<_> = compile_results
        .par_iter()
        .filter(|(result, _)| match result {
            CompileResult::Partial(p) => p.has_failure(),
            CompileResult::Template => false,
            _ => true,
        })
        .collect();
    let has_failures = if !failures.is_empty() {
        for (check_result, path) in failures.iter() {
            log::error!("Compilation for {} failed:", path.display());
            check_result.log(path);
        }
        let successfull = compile_results.len() - failures.len();
        log::error!(
            "{} files successfully compiled and {} files failed.",
            successfull,
            failures.len()
        );
        true
    } else {
        log::info!("All {} compilations passed.", compile_results.len());
        false
    };
    let created_outputs = compile_results
        .par_iter()
        .into_par_iter()
        .flat_map(|(result, _)| match result {
            CompileResult::Partial(p) => p.created_outputs(),
            _ => vec![],
        })
        .collect();
    InternalCompilationResult {
        has_failures,
        created_outputs,
    }
}

pub enum CompileResult {
    FailedParsing(rumbas::exam::ParseError),
    LocalesNotSet,
    FailedInputCheck(rumbas_support::input::InputCheckResult),
    Partial(RumbasCompileData),
    Template,
}

#[derive(Debug, Clone)]
pub struct PassedRumbasCompileData {
    pub locale: String,
    pub generated_path: PathBuf,
    pub exam_name: String,
    pub exam_path: PathBuf,
}

pub struct RumbasCompileData {
    failed_check: Vec<(String, rumbas_support::rumbas_check::RumbasCheckResult)>,
    failed: Vec<String>, // locale
    passed: Vec<PassedRumbasCompileData>,
}

impl RumbasCompileData {
    pub fn has_failure(&self) -> bool {
        !(self.failed.is_empty() && self.failed_check.is_empty())
    }
    pub fn created_outputs(&self) -> Vec<PassedRumbasCompileData> {
        self.passed.clone()
    }
    pub fn log(&self, path: &RumbasPath) {
        for (locale, check_result) in self.failed_check.iter() {
            log::error!(
                "Error when processing locale {} for {}.",
                locale,
                path.display()
            );

            check_result.log();
        }
        for locale in self.failed.iter() {
            log::error!(
                "Error when compiling locale {} for {} with numbas.",
                locale,
                path.display()
            );
        }
        for passed in self.passed.iter() {
            log::info!(
                "Succesfully compiled locale {} for {} ({}) with numbas. The output can be found at {}.",
                passed.locale,
                passed.exam_path.display(),
                passed.exam_name,
                passed.generated_path.display()
            );
        }
    }
}

impl CompileResult {
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

#[derive(Debug, Clone)]
pub struct FileCompilationContext {
    pub use_scorm: bool,
    pub as_zip: bool,
    pub minify: bool,
    pub output_folder: PathBuf,
}

pub fn compile_file(context: &FileCompilationContext, path: &RumbasPath) -> CompileResult {
    let check_result = crate::cli::check::check_file(path);
    match check_result {
        CheckResult::FailedParsing(f) => CompileResult::FailedParsing(f),
        CheckResult::FailedInputCheck(f) => CompileResult::FailedInputCheck(f),
        CheckResult::LocalesNotSet => CompileResult::LocalesNotSet,
        CheckResult::Template => CompileResult::Template,
        CheckResult::Partial(p) => {
            let mut passed_compilations = Vec::new();
            let mut failed_compilations = Vec::new();
            for (locale, numbas_exam, numbas_locale, theme) in p.passed() {
                let exam_name = numbas_exam.basic_settings.name.clone();
                let compiler = NumbasCompiler {
                    use_scorm: context.use_scorm,
                    as_zip: context.as_zip,
                    exam_path: path.clone(),
                    numbas_locale: numbas_locale.to_str().to_string(),
                    locale: locale.clone(),
                    theme,
                    exam: numbas_exam,
                    minify: context.minify,
                    output_folder: path.keep_root(context.output_folder.as_path()),
                };
                if compiler.compile() {
                    passed_compilations.push(PassedRumbasCompileData {
                        locale,
                        generated_path: compiler.output_path(),
                        exam_path: path.project().to_path_buf(),
                        exam_name,
                    })
                } else {
                    failed_compilations.push(locale)
                }
            }
            CompileResult::Partial(RumbasCompileData {
                passed: passed_compilations,
                failed: failed_compilations,
                failed_check: p.failed(),
            })
        }
    }
}

pub struct NumbasCompiler {
    use_scorm: bool,
    as_zip: bool,
    exam_path: RumbasPath,
    locale: String,
    numbas_locale: String,
    theme: String,
    minify: bool,
    exam: numbas::exam::Exam,
    output_folder: RumbasPath,
}

impl NumbasCompiler {
    /// Return the locale folder within the cache folder
    fn numbas_exam_folder(&self) -> RumbasPath {
        self.exam_path
            .keep_root(Path::new(CACHE_FOLDER).join(&self.locale).as_path())
    }
    /// Returns the path where the numbas exam should be saved
    fn numbas_exam_path(&self) -> PathBuf {
        let numbas_exam_name = self.exam_path.project().with_extension("exam");
        self.numbas_exam_folder().absolute().join(&numbas_exam_name)
    }
    /// Returns the locale folder within the output folder
    fn locale_output_folder(&self) -> PathBuf {
        self.output_folder.absolute().join(&self.locale)
    }
    /// Creates the output path for the generated html
    pub fn output_path(&self) -> PathBuf {
        let output_file = self
            .exam_path
            .project()
            .with_extension(self.output_extension());
        self.locale_output_folder().join(output_file)
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
    fn create_folder_structure(&self) {
        std::fs::create_dir_all(self.numbas_exam_path().parent().unwrap())
            .expect("Failed to create cache folders for the .exam file");
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

        let mut args: Vec<&str> = vec!["-l", &self.numbas_locale[..], "-t", &self.theme[..]];

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
            .current_dir(numbas_path)
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
                log::info!(
                    "Generated and saved exam file for exam {} with locale {}.",
                    self.exam_path.project().display(),
                    self.locale
                );

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
