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
        matches.is_present("verbose"),
        matches.occurrences_of("verbose"),
    ) {
        (true, _, _) => log::LevelFilter::Off,
        (false, true, 1) => log::LevelFilter::Error,
        (false, true, 2) => log::LevelFilter::Warn,
        (false, true, 4) => log::LevelFilter::Debug,
        _ => log::LevelFilter::Info,
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
    } else if let Some(matches) = matches.subcommand_matches("check") {
        cli::check(matches)
    } else if let Some(matches) = matches.subcommand_matches("watch") {
        cli::watch(matches)
    } else if let Some(matches) = matches.subcommand_matches("fmt") {
        cli::fmt(matches)
    }
}
