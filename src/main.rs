use std::process::exit;
use anyhow::Result;
use clap::ArgMatches;
use tracing::error;

mod cli;
mod commands;
mod traces;
mod system;
mod installers;
mod utils;

fn main() -> Result<()> {
    let matches = cli::command().get_matches();
    traces::init(&matches);

    match execute(&matches) {
        Ok(_) => {}
        Err(err) => {
            error!("{}", err);
            exit(-1);
        }
    }
    Ok(())
}

fn execute(matches: &ArgMatches) -> Result<()> {
    match matches.subcommand() {
        Some(("init", matches)) => commands::init::execute(matches)?,
        Some(("install", matches)) => commands::install::execute_install(matches)?,
        Some(("update", matches)) => commands::install::execute_update(matches)?,
        Some(("git", matches)) => commands::git::execute(matches)?,
        Some(("purge", matches)) => commands::purge::execute(matches)?,
        Some(("rd", matches)) => commands::rd::execute(matches)?,
        None => unreachable!(),
        _ =>  unreachable!(),
    }
    Ok(())
}

