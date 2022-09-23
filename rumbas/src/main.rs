#![deny(clippy::print_stdout)]
#![deny(clippy::print_stderr)]

#[macro_use]
extern crate clap;
use clap::crate_version;
use clap::{Parser, Subcommand};
use semver::Version;
use std::path::Path;

mod cli;

/// The main cli function
/// # Requirements
/// Make sure `NUMBAS_FOLDER` is set
/// # Usage
/// See `rumbas help` for usage info
fn main() {
    let args = Cli::parse();

    let log_level = match (args.quiet, args.verbose) {
        (true, _) => log::LevelFilter::Off,
        (false, 1) => log::LevelFilter::Error,
        (false, 2) => log::LevelFilter::Warn,
        (false, 0) | (false, 3) => log::LevelFilter::Info,
        (false, _) => log::LevelFilter::Debug, // debug for 4 or more v's
    };

    cli::logger::setup(log_level).expect("Working logger");

    match args.command {
        Command::Import {
            exam_path,
            question,
        } => cli::import(exam_path, question),
        Command::Compile {
            exam_or_question_paths,
            scorm,
            zip,
            no_minification,
        } => cli::compile(exam_or_question_paths, scorm, zip, no_minification),
        Command::Watch { path, only_check } => cli::watch(path, only_check),
        Command::Check {
            exam_or_question_paths,
        } => cli::check(exam_or_question_paths),
        Command::UpdateRepo => cli::update_repo(),
        Command::Init => cli::init(),
        Command::Schema => cli::schema(),
        Command::Fmt {
            exam_or_question_paths,
        } => cli::fmt(exam_or_question_paths),
        Command::EditorOutput {
            output_path,
            url_prefix,
        } => cli::create_editor_output(output_path, url_prefix),
    }
}

/// The rumbas cli
#[derive(Debug, Parser)] // requires `derive` feature
#[clap[author, version, about, long_about = None]]
struct Cli {
    #[clap(subcommand)]
    command: Command,

    /// Sets the level of verbosity
    ///
    /// -v    sets the level to ERROR
    ///
    /// -vv   sets the level to WARN
    ///
    /// -vvv   sets the level to INFO
    ///
    /// -vvvv   sets the level to DEBUG
    ///
    /// The default is -vvv
    #[clap(short, action = clap::ArgAction::Count)]
    verbose: u8,
    /// Enables quiet mode. Nothing is logged. This has precedence over the verbose option.
    #[clap(short)]
    quiet: bool,
}

#[derive(Debug, Subcommand)]
enum Command {
    /// Compile a rumbas exam (or question)
    ///
    /// You can pass a path to a folder to compile all files in the folder.
    #[clap(arg_required_else_help = true)]
    Compile {
        /// The path to the exam or question file to compile.
        ///
        /// If a folder within the questions or exams folder is used, all questions/exams in that folder will be compiled.
        ///
        /// It is possible to specify multiple paths to folder/files.
        #[clap(required = true, multiple = true, value_parser)]
        exam_or_question_paths: Vec<String>,
        /// Include the files necessary to make a SCORM package
        #[clap(value_parser, long, short)]
        scorm: bool,
        /// Create a zip file instead of a directory
        #[clap(value_parser, long, short)]
        zip: bool,
        /// on't perform minification on the created js in the exam. Useful if you don't have uglifyjs or want to debug something.
        #[clap(value_parser, long)]
        no_minification: bool,
    },
    /// Check a rumbas exam (or question)
    ///
    /// You can pass a path to a folder to check all files in the folder.
    #[clap(arg_required_else_help = true)]
    Check {
        /// The path to the exam or question file to check.
        ///
        /// If a folder within the questions or exams folder is used, all questions/exams in that folder will be checked.
        ///
        /// It is possible to specify multiple paths to folder/files.
        #[clap(required = true, multiple = true, value_parser)]
        exam_or_question_paths: Vec<String>,
    },
    /// Format a rumbas exam (or question).
    ///
    /// You can pass a path to a folder to format all files in the folder.
    #[clap(arg_required_else_help = true)]
    Fmt {
        /// The path to the exam or question file to format.
        ///
        /// If a folder within the questions or exams folder is used, all questions/exams in that folder will be formatted.
        ///
        /// It is possible to specify multiple paths to folder/files.
        #[clap(required = true, multiple = true, value_parser)]
        exam_or_question_paths: Vec<String>,
    },
    /// Import a numbas .exam file
    ///    
    /// Resources have to be manually placed in the resources folder
    #[clap(arg_required_else_help = true)]
    Import {
        /// The path to the numbas .exam file
        #[clap(value_parser)]
        exam_path: String,
        /// Tells rumbas that this is the exam file of a numbas question instead of of a numbas exam.
        #[clap(short)]
        question: bool,
    },
    /// Initialize a rumbas project in this folder
    Init,
    /// Update the repository to the next rumbas version
    UpdateRepo,
    /// Creates files with the json schemas (beta).
    /// See https://github.com/m8rex/rumbas-examples/tree/main/.vscode for usage instructions
    Schema,
    /// Watch a path
    #[clap(arg_required_else_help = true)]
    Watch {
        /// The path to watch
        path: String,
        /// Only check exams and questions that change due to file changes, but don't compile them with numbas.
        #[clap(short)]
        only_check: bool,
    },
    /// Generates a folder structure that can be hosted and used as an 'editor' in the numbas lti provider
    ///
    /// Only exams are compiled.
    #[clap(arg_required_else_help = true)]
    EditorOutput {
        /// The path to the folder where the output should be generated.
        output_path: String,
        /// The url prefix for all editor api calls
        url_prefix: String,
    },
}

impl Command {
    fn can_execute_in_old_version(&self) -> bool {
        matches!(self, Self::UpdateRepo | Self::Init)
    }
}
