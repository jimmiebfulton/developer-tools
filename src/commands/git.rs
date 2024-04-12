use std::process::Command;

use anyhow::{anyhow, Result};
use camino::Utf8PathBuf;
use clap::ArgMatches;
use tracing::error;

pub fn execute(matches: &ArgMatches) -> Result<()> {
    match matches.subcommand() {
        Some(("init", matches)) => init(matches)?,
        Some(("add-remote", matches)) => add_remote(matches)?,
        Some((command, _)) => unimplemented!("Unimplemented command '{}'", command),
        None => error!("Unspecified command"),
    }

    Ok(())
}

fn init(matches: &ArgMatches) -> Result<()> {
    let existing_repo = Utf8PathBuf::from(".").join(".git").exists();
    let got_git_ignore = Utf8PathBuf::from(".").join(".gitignore");
    if !got_git_ignore.exists() {
        return Err(anyhow!(".gitignore does not exist!"));
    }
    let _status = Command::new("git").arg("init").status()?;
    if matches.get_flag("personal") {
        let _status = Command::new("git")
            .args(["config", "--local", "--add", "user.email", "jimmie.fulton@gmail.com"]).status()?;
    }
    if !existing_repo {
        Command::new("git").args(["add", ".").status()?;
        Command::new("git").args(["commit", "-m", "initial commit"]).status()?;

    }
    Ok(())
}

fn add_remote(matches: &ArgMatches) -> Result<()> {
    let mut url = matches.get_one::<String>("url").expect("URL is Required by Clap").to_string();

    if matches.get_flag("personal") {
        url = personalize_url(url);
    }

    let _status = Command::new("git")
        .arg("remote")
        .arg("add")
        .arg("origin")
        .arg(url)
        .status()?;

    Ok(())
}

fn personalize_url(url: String) -> String {
    if url.starts_with("git@github.com") {
        url.replace("git@github.com", "git@personal.github.com")
    } else {
        url
    }
}