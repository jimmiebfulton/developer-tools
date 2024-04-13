use anyhow::{Result};
use clap::ArgMatches;

pub fn execute(_matches: &ArgMatches) -> Result<()> {


    let source = include_str!("../../resource/abbr.fish");
    println!("{}", source);
    let source = include_str!("../../resource/functions.fish");
    println!("{}", source);

    Ok(())
}