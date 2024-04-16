use std::fmt::{Display, Formatter};
use std::process::Command;
use camino::Utf8Path;
use crate::install::installer::{execute, Installer};

pub struct XCodeCliTools;

impl Display for XCodeCliTools {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "XCode Command Line Tools")
    }
}

impl Installer for XCodeCliTools {
    fn installed(&self) -> anyhow::Result<bool> {
        Ok(Utf8Path::new("/Library/Developer/CommandLineTools").exists())
    }

    fn install(&self) -> anyhow::Result<bool> {
        execute(Command::new("xcode-select")
            .arg("--install"))
    }
}