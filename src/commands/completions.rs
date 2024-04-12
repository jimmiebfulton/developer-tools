use std::io;

use anyhow::{Result, Error};
use clap::ArgMatches;
use clap_complete::{generate, Shell};

use crate::cli;

pub fn execute(matches: &ArgMatches) -> Result<()> {
    if let Some(generator) = matches.get_one::<Shell>("generator") {
        let mut cmd = cli::command();
        generate(*generator, &mut cmd, "dt", &mut io::stdout());
    } else {
        return Err(Error::msg("Invalid completions shell"));
    }

    Ok(())
}
