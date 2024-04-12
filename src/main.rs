use anyhow::Result;

mod cli;
mod commands;
mod traces;

fn main() -> Result<()> {
    let matches = cli::command().get_matches();
    traces::init(&matches);

    match matches.subcommand() {
        Some(("aliases", matches)) => commands::aliases::execute(matches)?,
        Some(("completions", matches)) => commands::completions::execute(matches)?,
        Some(("git", matches)) => commands::git::execute(matches)?,
        None => unreachable!(),
        _ =>  unreachable!(),
    }

    Ok(())
}

