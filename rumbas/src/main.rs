#![deny(clippy::print_stdout)]
#![deny(clippy::print_stderr)]

use clap::CommandFactory;
use clap::Parser;
use rumbas::support::cli::{Cli, Command};

#[macro_use]
extern crate clap;

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
        Command::GenerateShellCompletion { shell } => cli::complete(Cli::command(), shell),
        Command::Fmt {
            exam_or_question_paths,
        } => cli::fmt(exam_or_question_paths),
        Command::Export {
            exam_or_question_paths,
        } => cli::export(exam_or_question_paths),
        Command::EditorOutput {
            output_path,
            url_prefix,
        } => cli::create_editor_output(output_path, url_prefix),
    }
}

#[cfg(test)]
mod tests {
    use super::Cli;
    use clap::CommandFactory;
    #[test]
    fn verify_cli() {
        Cli::command().debug_assert()
    }
}
