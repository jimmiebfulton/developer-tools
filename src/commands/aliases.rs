use anyhow::{Result};
use clap::ArgMatches;

pub fn execute(_matches: &ArgMatches) -> Result<()> {
    println!("abbr -a gi dt git init");
    println!("abbr -a gip dt git init -p");
    println!("abbr -a gar dt git add-remote ");
    println!("abbr -a garp dt git add-remote -p ");
    Ok(())
}