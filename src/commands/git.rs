use std::process::Command;
use std::sync::OnceLock;

use anyhow::{anyhow, Result};
use camino::Utf8PathBuf;
use clap::ArgMatches;
use tracing::{debug, error, info};
use regex::Regex;

fn ssh_git_pattern() -> &'static Regex {
    static REGEX: OnceLock<Regex> = OnceLock::new();
    REGEX.get_or_init(|| Regex::new(r"\S+@(\S+):(.*)").unwrap())
}

pub fn execute(matches: &ArgMatches) -> Result<()> {
    match matches.subcommand() {
        Some(("init", matches)) => init(matches)?,
        Some(("add-remote", matches)) => add_remote(matches)?,
        Some(("clone", matches)) => clone(matches)?,
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

    personalize_repo(matches, None)?;

    if !existing_repo {
        Command::new("git").args(["add", "."]).status()?;
        Command::new("git").args(["commit", "-m", "initial commit"]).status()?;
    }
    Ok(())
}

fn clone(matches: &ArgMatches) -> Result<()> {
    let url = personalize_url(matches)?;

    let mut command = Command::new("git");
    command
        .arg("clone".to_string())
        .arg(url);

    if let Some(destination) = matches.get_one::<String>("destination") {
        command.arg(destination);
    }
    command.status()?;

    personalize_repo(matches, personalization_directory(matches)?)?;

    Ok(())
}

fn add_remote(matches: &ArgMatches) -> Result<()> {
    let url = personalize_url(matches)?;

    Command::new("git")
        .arg("remote")
        .arg("add")
        .arg("origin")
        .arg(url)
        .status()?;

    Ok(())
}

fn personalize_url(matches: &ArgMatches) -> Result<String> {
    let url = matches.get_one::<String>("url").expect("URL is Required by Clap").to_string();

    if matches.get_flag("personal") {
        if url.starts_with("git@github.com") {
            info!("Personalizing Git URL");
            Ok(url.replace("git@github.com", "git@personal.github.com"))
        } else {
            debug!("Not personalizing Git URL");
            Ok(url)
        }
    } else {
        Ok(url)
    }
}

fn personalize_repo(matches: &ArgMatches, destination: Option<String>) -> Result<()> {
    if matches.get_flag("personal") {
        info!("Personalizing Repo");
        let mut command = Command::new("git");
        command.args(["config", "--local", "--add", "user.email", "jimmie.fulton@gmail.com"]);

        if let Some(destination) = destination {
            command.current_dir(destination);
        }
        command.status()?;
    } else {
        debug!("Not Personalizing Repo");
    }

    Ok(())
}

fn personalization_directory(matches: &ArgMatches) -> Result<Option<String>> {
    if let Some(destination) = matches.get_one::<String>("destination") {
        return Ok(Some(destination.into()))
    }

    let url = matches.get_one::<String>("url").expect("URL is required by Clap");

    if let Some(captures) = ssh_git_pattern().captures(url) {
        let directory = Utf8PathBuf::from(&captures[2])
            .file_stem().map(|stem| stem.to_string())
            .and_then(|directory| {
                debug!("Captured '{directory}' from '{url}'");
                Some(directory)
            });
        return Ok(directory);
    }

    Ok(None)
}