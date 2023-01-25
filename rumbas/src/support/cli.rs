use clap::{Parser, Subcommand};
use clap_complete::Shell;

/// The rumbas cli
#[derive(Debug, Parser)] // requires `derive` feature
#[clap[author, version, about, long_about = None]]
pub struct Cli {
    #[clap(subcommand)]
    pub command: Command,

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
    pub verbose: u8,
    /// Enables quiet mode. Nothing is logged. This has precedence over the verbose option.
    #[clap(short)]
    pub quiet: bool,
}

#[derive(Debug, Subcommand)]
pub enum Command {
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
        #[clap(required = true, value_parser)]
        exam_or_question_paths: Vec<String>,
        /// Include the files necessary to make a SCORM package
        #[clap(value_parser, long, short)]
        scorm: bool,
        /// Create a zip file instead of a directory
        #[clap(value_parser, long, short)]
        zip: bool,
        /// Don't perform minification on the created js in the exam. Useful if you don't have uglifyjs or want to debug something.
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
        #[clap(required = true, value_parser)]
        exam_or_question_paths: Vec<String>,
    },
    /// Watch a path
    #[clap(arg_required_else_help = true)]
    Watch {
        /// The path to watch
        path: String,
        /// Only check exams and questions that change due to file changes, but don't compile them with numbas.
        #[clap(short)]
        only_check: bool,
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
        #[clap(required = true, value_parser)]
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
    Init {
        /// Whether the defaults should be of summative nature (instead of formative)
        #[clap(value_parser, long)]
        summative: bool,
    },
    /// Update the repository to the next rumbas version
    UpdateRepo,
    /// Export a rumbas exam as one yaml.
    /// All default files and templating is resolved.
    ///
    /// Can be useful to debug exams / questions that don't behave as expected
    Export {
        /// The path to the exams to export
        ///
        /// If a folder within the questions or exams folder is used, all questions/exams in that folder will be exported.
        ///
        /// It is possible to specify multiple paths to folder/files.
        #[clap(required = true, value_parser)]
        exam_or_question_paths: Vec<String>,
    },
    /// Creates files with the json schemas (beta).
    /// See https://github.com/m8rex/rumbas-examples/tree/main/.vscode for usage instructions
    Schema,
    /// Generate shell completion file. Placing this file at the right location and reloading your
    /// shell, will give you shell completion for rumbas.
    ///
    /// Example for bash: rumbas generate-shell-completion bash | sudo tee /usr/share/bash-completion/completions/rumbas
    #[clap(arg_required_else_help = true)]
    GenerateShellCompletion {
        /// The shell for which the shell competion should be generated
        shell: Shell,
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
        matches!(
            self,
            Self::UpdateRepo
                | Self::Init { summative: _ }
                | Self::GenerateShellCompletion { shell: _ }
        )
    }
}
