use std::fmt::Display;
use std::process::Command;
use std::vec;
use anyhow::{anyhow, Result};
use camino::Utf8Path;
use crate::install::installers::{Fonts, Homebrew};

pub trait Installer: Display {
    fn installed(&self) -> Result<bool>;

    fn install(&self) -> Result<bool>;

    fn dependencies(&self) -> Vec<Box<dyn Installer>> {
        vec![]
    }

    fn update(&self) -> Result<bool> {
        Ok(true)
    }

    fn upgrade(&self) -> Result<bool> {
        Ok(true)
    }
}

pub trait InstallationGroup: Display {
    fn dependencies(&self) -> Vec<Box<dyn Installer>>;
}

impl Installer for dyn InstallationGroup {
    fn installed(&self) -> Result<bool> {
        for dependency in self.dependencies() {
            if !dependency.installed()? {
                return Ok(false);
            }
        }
        Ok(true)
    }

    fn install(&self) -> Result<bool> {
        Ok(true)
    }

    fn dependencies(&self) -> Vec<Box<dyn Installer>> {
        self.dependencies()
    }
}

pub fn install(installer: &Box<dyn Installer>) -> Result<()> {
    let installed = installer.installed()
        .map_err(|err| anyhow!("Error checking installation for '{installer}': {err}"))?;
    if !installed {
        for dependency in &installer.dependencies() {
            install(dependency)?;
        }
        installer.install()
            .map_err(|err| anyhow!("Error installing '{installer}': {err}"))?;
    }
    Ok(())
}

pub fn execute(command: &mut Command) -> Result<bool> {
    Ok(command.status()?.code().unwrap_or_default() == 0)
}