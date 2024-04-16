use clap::ArgMatches;
use anyhow::Result;
use crate::installers::{init, InstallerKey};
use crate::system::install;

pub fn execute_install(matches: &ArgMatches) -> Result<()> {
    let key = matches.get_one::<InstallerKey>("installer").expect("Clap Required");
    init()?;
    install(key)?;
    Ok(())
}

pub fn execute_update(matches: &ArgMatches) -> Result<()> {
    let key = matches.get_one::<InstallerKey>("installer").expect("Clap Required");
    init()?;
    println!("Updating '{key:?}'");
    Ok(())
}
