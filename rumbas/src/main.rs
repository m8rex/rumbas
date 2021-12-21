#![deny(clippy::print_stdout)]
#![deny(clippy::print_stderr)]

#[macro_use]
extern crate clap;
use clap::{crate_version, App};

mod cli;

/// The main cli function
/// # Requirements
/// Make sure `NUMBAS_FOLDER` is set
/// # Usage
/// See `rumbas help` for usage info
fn main() {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).version(crate_version!()).get_matches();

    let log_level = match (
        matches.is_present("quiet"),
        matches.occurrences_of("verbose"),
    ) {
        (true, _) => log::LevelFilter::Off,
        //(false, 0) => log::LevelFilter::Error,
        //(false, 1) => log::LevelFilter::Warn,
        (false, 0) => log::LevelFilter::Info,
        _ => log::LevelFilter::Debug,
    };

    cli::logger::setup(log_level).expect("Working logger");

    if let Some(matches) = matches.subcommand_matches("import") {
        cli::import(matches)
    } else if let Some(matches) = matches.subcommand_matches("compile") {
        cli::compile(matches)
    } else if let Some(matches) = matches.subcommand_matches("init") {
        cli::init(matches)
    } else if let Some(matches) = matches.subcommand_matches("update_repo") {
        cli::update_repo(matches)
    } else if let Some(matches) = matches.subcommand_matches("schema") {
        cli::schema(matches)
    }
}
