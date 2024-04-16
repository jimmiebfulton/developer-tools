use std::collections::HashSet;
use std::fs;
use std::path::PathBuf;
use clap::ArgMatches;
use anyhow::{anyhow, Result};
use camino::Utf8PathBuf;

pub fn execute(matches: &ArgMatches) -> Result<()> {
    let paths = matches.get_many::<PathBuf>("paths").expect("Required by Clap");

    for path in paths {
        handle_path(path)?;
    }

    Ok(())
}

fn handle_path(path: &PathBuf) -> Result<()> {
    let path = path.canonicalize().map_err(|err|anyhow!("Path '{}': {err}", path.to_str().unwrap()))?;
    let path = Utf8PathBuf::try_from(path)?;

    if !path.starts_with(home()?) {
        return external_to_home_error(path);
    }

    if restricted_directories()?.contains(&path) {
        return restricted_directory_error(path);
    }

    if let Some(parent) = path.parent() {
        let parent = parent.to_path_buf();
        if container_directories()?.contains(&parent) {
            return container_directory_error(path);
        }
    }

    if path.is_file() || path.is_symlink() {
        fs::remove_file(&path)?;
    } else if path.is_dir() {
        fs::remove_dir_all(&path)?;
    }

    Ok(())
}

fn home() -> Result<Utf8PathBuf> {
    let home = Utf8PathBuf::try_from(dirs::home_dir().expect("Home Directory Required"))?;
    Ok(home)
}

fn restricted_directories() -> Result<HashSet<Utf8PathBuf>> {
    let home = home()?;
    let mut results = HashSet::new();
    results.insert(home.clone());
    results.insert(home.join("projects"));
    results.insert(home.join("tmp"));
    results.insert(home.join("tmp/prototypes"));
    results.insert(home.join("tmp/generated"));
    Ok(results)
}

fn container_directories() -> Result<HashSet<Utf8PathBuf>> {
    let home = home()?;
    let mut results = HashSet::new();
    results.insert(home.clone());
    results.insert(home.join("projects"));
    Ok(results)
}

fn container_directory_error(path: Utf8PathBuf) -> Result<()> {
    Err(anyhow!("'{path}' is a container directory"))
}

fn external_to_home_error(path: Utf8PathBuf) -> Result<()> {
    Err(anyhow!("'{path}' is not within your HOME directory"))
}

fn restricted_directory_error(path: Utf8PathBuf) -> Result<()> {
    Err(anyhow!("'{path}' is a restricted directory"))
}

