use std::fmt::Display;
use std::process::Command;
use camino::Utf8Path;
use crate::install::installer::{execute, Installer};
use crate::install::installers::Homebrew;


pub trait HomebrewCommandInstaller: Display {
    fn command(&self) -> &'static str;
    fn package(&self) -> &'static str;
    fn dependencies(&self) -> Vec<Box<dyn Installer>> {
        vec![]
    }
}

impl<T> Installer for T where T: HomebrewCommandInstaller {

    fn installed(&self) -> anyhow::Result<bool> {
        Ok(Utf8Path::new("/opt/homebrew/bin/").join(self.command()).exists())
    }

    fn install(&self) -> anyhow::Result<bool> {
        execute(
            Command::new("brew")
                .arg("install")
                .arg(self.package())
        )
    }

    fn dependencies(&self) -> Vec<Box<dyn Installer>> {
        let mut dependencies = vec![];
        for dependency in self.dependencies() {
            dependencies.push(dependency);
        }
        dependencies.insert(0, Box::new(Homebrew));
        dependencies
    }
}
