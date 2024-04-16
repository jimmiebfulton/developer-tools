use std::fmt::{Display, Formatter};
use std::process::Command;

use camino::{Utf8Path, Utf8PathBuf};
use tracing::debug;
use crate::install::installer::execute;

use crate::system::Installable;
use crate::installers::InstallerKey;
use crate::utils::homebrew_path;

pub struct HomebrewPackage {
    install_check: Utf8PathBuf,
    package: String,
    cask: bool,
    tap: Option<String>,
    dependencies: Vec<InstallerKey>
}

impl HomebrewPackage {
    pub fn new<P: Into<String>>(package: P, install_check: Utf8PathBuf) -> HomebrewPackage {
        HomebrewPackage {
            install_check,
            package: package.into(),
            cask: Default::default(),
            tap: None,
            dependencies: vec![InstallerKey::Homebrew],
        }
    }

    pub fn command<C: Into<String>>(command: C) -> HomebrewPackage {
        let command = command.into();
        Self::new(command.clone(), homebrew_path(command.clone()))
    }

    pub fn cask(mut self) -> HomebrewPackage {
        self.cask = true;
        self
    }

    pub fn tap<T: Into<String>>(mut self, tap: T) -> HomebrewPackage {
        self.tap = Some(tap.into());
        self
    }

    pub fn with_dependency(mut self, key: InstallerKey) -> HomebrewPackage {
        self.dependencies.push(key);
        self
    }
}

impl Installable for HomebrewPackage {
    fn install(&self) -> anyhow::Result<()> {
        if let Some(tap) = &self.tap {
            execute(
                Command::new("/opt/homebrew/bin/brew")
                    .arg("tap")
                    .arg(tap)
            )?;
        }
        let mut command = Command::new("brew");
        command.arg("install");
        if self.cask {
            command.arg("--cask");
        }
        command.arg(&self.package);
        execute(&mut command)?;
        Ok(())
    }

    fn installed(&self) -> anyhow::Result<bool> {
        debug!("Checking: {}", self.install_check);
        Ok(self.install_check.exists())
    }

    fn dependencies(&self) -> &[InstallerKey] {
        self.dependencies.as_slice()
    }
}

pub struct HomebrewPackageManager;

impl Installable for HomebrewPackageManager {
    fn installed(&self) -> anyhow::Result<bool> {
        Ok(Utf8Path::new("/opt/homebrew").exists())
    }

    fn install(&self) -> anyhow::Result<()> {
        execute(
            Command::new("/bin/bash")
                .arg("-c")
                .arg( "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/master/install.sh)")
        )?;
        Ok(())
    }

    fn dependencies(&self) -> &[InstallerKey] {
        Default::default()
    }
}

