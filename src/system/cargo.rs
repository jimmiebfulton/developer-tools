use std::fmt::{Display, Formatter};
use std::process::Command;

use crate::installers::InstallerKey;
use crate::system::Installable;
use crate::utils::{execute, home_path};

pub struct CargoCommand {
    command: String,
    package: Option<String>,
    dependencies: Vec<InstallerKey>,
}

impl CargoCommand {
    pub fn new<C: Into<String>>(command: C) -> CargoCommand {
        CargoCommand {
            command: command.into(),
            package: Default::default(),
            dependencies: vec![InstallerKey::Rust],
        }
    }

    pub fn command(&self) -> &String {
        &self.command
    }

    pub fn package(&self) -> Option<&String> {
        self.package.as_ref()
    }

    #[allow(dead_code)]
    pub fn with_dependency(mut self, key: InstallerKey) -> CargoCommand {
        self.dependencies.push(key);
        self
    }
}

impl Display for CargoCommand {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.command())
    }
}

impl Installable for CargoCommand {
    fn install(&self) -> anyhow::Result<()> {
        let package = self.package().unwrap_or(self.command());
        execute(
            Command::new(home_path(".cargo/bin/cargo"))
                .arg("install")
                .arg(package),
        )?;
        Ok(())
    }

    fn installed(&self) -> anyhow::Result<bool> {
        Ok(home_path(".cargo/bin").join(self.command()).exists())
    }

    fn dependencies(&self) -> &[InstallerKey] {
        self.dependencies.as_slice()
    }
}

pub struct Rust;

impl Installable for Rust {
    fn install(&self) -> anyhow::Result<()> {
        execute(
            Command::new("/bin/bash")
                .arg("-c")
                .arg("curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh"),
        )?;
        execute(
            Command::new(home_path(".cargo/bin/rustup"))
                .arg("component")
                .arg("add")
                .arg("rust-src"),
        )?;
        Ok(())
    }

    fn installed(&self) -> anyhow::Result<bool> {
        Ok(home_path(".cargo").exists())
    }

    fn dependencies(&self) -> &[InstallerKey] {
        Default::default()
    }
}

