use anyhow::{Result};
use clap::ArgMatches;

pub fn execute(_matches: &ArgMatches) -> Result<()> {
    println!("abbr -a gi dt git init");
    println!("abbr -a gip dt git init -p");
    println!("abbr -a gar dt git add-remote ");
    println!("abbr -a garp dt git add-remote -p ");

    println!("abbr -a a archetect");
    println!("abbr -a acm archetect cache manager");

    println!("abbr -a rr rustrover");
    println!("abbr -a i idea");

    println!("abbr -a ga git add");
    println!("abbr -a ga. git add .");
    println!("abbr -a gc git commit");
    println!("abbr -a gcl git clone");
    println!("abbr -a gclp dt git clone -p");
    println!("abbr -a gco git checkout");
    println!("abbr -a gp git checkout");
    println!("abbr -a gl git log");

    Ok(())
}