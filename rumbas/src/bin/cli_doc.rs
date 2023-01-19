use clap::CommandFactory;
use rumbas::support::cli::Cli;
use std::fmt::Write;

const CLI_PATH: &'static str = "../book/src/cli.md";

fn main() -> Result<(), std::fmt::Error> {
    let mut out = String::new();

    let mut cmd = Cli::command();

    writeln!(out, "# Rumbas CLI commands")?;

    writeln!(out, "## All commands")?;

    writeln!(out, "```console")?;
    writeln!(out, "$ {} --help", cmd.get_name())?;
    writeln!(out, "{}", cmd.render_long_help())?;
    writeln!(out, "```")?;

    for mut subcommand in cmd
        .get_subcommands()
        .map(|a| a.clone())
        .collect::<Vec<_>>()
        .clone()
        .into_iter()
    {
        writeln!(out, "## {}", subcommand.get_name())?;

        writeln!(out, "```console")?;
        writeln!(out, "$ {} {} --help", cmd.get_name(), subcommand.get_name())?;
        writeln!(out, "{}", subcommand.render_long_help())?;
        writeln!(out, "```")?;
    }

    std::fs::write(CLI_PATH, out).expect("writing to work");
    Ok(())
}
