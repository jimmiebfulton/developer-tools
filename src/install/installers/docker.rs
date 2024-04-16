use std::fmt::{Display, Formatter};
use std::process::Command;
use camino::Utf8Path;
use crate::install::installer::{execute, Installer};
use crate::install::installers::Homebrew;

pub struct Docker;

impl Display for Docker {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Docker")
    }
}

impl Installer for Docker {
    fn installed(&self) -> anyhow::Result<bool> {
        Ok(Utf8Path::new("/opt/homebrew/bin/docker").exists())
    }

    fn install(&self) -> anyhow::Result<bool> {
        execute(
            Command::new("brew")
                .arg("cask")
                .arg("install")
                .arg("docker")
        )
    }

    fn dependencies(&self) -> Vec<Box<dyn Installer>> {
        vec![Box::new(Homebrew)]
    }
}