use std::fmt::{Display, Formatter};
use std::process::Command;
use camino::Utf8Path;
use crate::install::installer::{execute, Installer};
use crate::install::installers::XCodeCliTools;

pub struct Homebrew;

impl Display for Homebrew {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
       write!(f, "Homebrew")
    }
}

impl Installer for Homebrew {
    fn installed(&self) -> anyhow::Result<bool> {
        Ok(Utf8Path::new("/opt/homebrew").exists())
    }

    fn install(&self) -> anyhow::Result<bool> {
        execute(
            Command::new("/bin/bash")
                .arg("-c")
                .arg( "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/master/install.sh)")
        )
    }

    fn dependencies(&self) -> Vec<Box<dyn Installer>> {
       vec![Box::new(XCodeCliTools)]
    }
}