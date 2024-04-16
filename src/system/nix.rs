use std::process::Command;
use crate::install::installer::execute;
use crate::installers::InstallerKey;
use crate::system::Installable;

pub struct NixPackageManager;

impl Installable for NixPackageManager {
    fn install(&self) -> anyhow::Result<()> {
        execute(
            Command::new("/bin/bash")
                .arg("-c")
                .arg("sh <(curl -L https://nixos.org/nix/install)")
        )?;
        Ok(())
    }

    fn installed(&self) -> anyhow::Result<bool> {
        Ok(false)
    }

    fn dependencies(&self) -> &[InstallerKey] {
        Default::default()
    }
}