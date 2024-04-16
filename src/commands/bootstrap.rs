use clap::ArgMatches;
use anyhow::Result;
use crate::install::installer::{install, Installer};
use crate::install::installers::Fonts;

pub fn execute(_matches: &ArgMatches) -> Result<()> {
    let fonts: Box<dyn Installer> = Box::new(Fonts);
    install(&fonts)
}