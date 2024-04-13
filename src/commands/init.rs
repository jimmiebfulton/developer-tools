use std::io;
use anyhow::{Result};
use clap::ArgMatches;
use clap_complete::{generate, Shell};
use tracing::error;
use crate::cli;

pub fn execute(matches: &ArgMatches) -> Result<()> {
    match matches.subcommand() {
        Some(("fish", matches)) => fish(matches)?,
        Some((command, _)) => unimplemented!("Unimplemented command '{}'", command),
        None => error!("Unspecified command"),
    }
    Ok(())
}

pub fn fish(_matches: &ArgMatches) -> Result<()> {
    let source = include_str!("../../resource/abbr.fish");
    println!("{}", source);
    let source = include_str!("../../resource/functions.fish");
    println!("{}", source);

    let mut cmd = cli::command();
    generate(Shell::Fish, &mut cmd, "dt", &mut io::stdout());

    Ok(())
}