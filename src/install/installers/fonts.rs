use std::fmt::{Display, Formatter};
use std::process::Command;
use crate::install::installer::{execute, Installer};
use crate::install::installers::Homebrew;

pub struct Fonts;

impl Display for Fonts {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Fonts")
    }
}

impl Installer for Fonts {
    fn installed(&self) -> anyhow::Result<bool> {
        execute(
            Command::new("brew")
                .arg("list")
                .arg("font-jetbrains-mono-nerd-font")
        )
    }

    fn install(&self) -> anyhow::Result<bool> {
        execute(
            Command::new("brew")
                .arg("tap")
                .arg("homebrew/cask-fonts")
        )?;
        execute(
            Command::new("/bin/bash")
                .arg("-c")
                .arg("brew search '/font-.*-nerd-font/' | awk '{ print $1 }' | xargs -I{} brew install --cask {} || true")
        )
    }

    fn dependencies(&self) -> Vec<Box<dyn Installer>> {
        vec![Box::new(Homebrew)]
    }
}