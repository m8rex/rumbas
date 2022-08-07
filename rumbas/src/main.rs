#![deny(clippy::print_stdout)]
#![deny(clippy::print_stderr)]

#[macro_use]
extern crate clap;
use clap::{crate_version, App};
use semver::Version;

mod cli;

/// The main cli function
/// # Requirements
/// Make sure `NUMBAS_FOLDER` is set
/// # Usage
/// See `rumbas help` for usage info
fn main() {
    let yaml = load_yaml!("cli.yml");
    let rumbas_version = crate_version!();
    let matches = App::from_yaml(yaml).version(rumbas_version).get_matches();
    let rumbas_version = Version::parse(rumbas_version).unwrap();

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

    // Check rc file
    let rc_res = rumbas::support::rc::read();
    match rc_res {
        Ok(rc) => {
            let rc_version = rc.version();
            if rc_version < rumbas_version && matches.subcommand_matches("update_repo").is_none() {
                log::error!("This repository uses an older rumbas version than the one that is compiling it ({} vs {}).", rc_version, rumbas_version);
                log::error!("Please execute `rumbas update_repo`.");
                std::process::exit(1)
            } else if rc_version > rumbas_version {
                log::error!("This repository uses a newer rumbas version than the one you are using ({} vs {}).", rc_version, rumbas_version);
                log::error!("Please update your rumbas version.");
                std::process::exit(1)
            }
        }
        Err(e) => {
            log::error!("Could not parse rc file: {}", e);
            std::process::exit(1)
        }
    }

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
