use clap::Command;
use clap_complete::{generate, Generator, Shell};
use std::io;

fn print_completions<G: Generator>(gen: G, cmd: &mut Command) {
    generate(gen, cmd, cmd.get_name().to_string(), &mut io::stdout());
}

pub fn complete(mut cmd: Command, s: Shell) {
    print_completions(s, &mut cmd);
}
