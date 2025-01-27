use crate::installers::InstallerKey;
use crate::system::Installable;
use crate::utils::home_path;
use anyhow::Context;
use camino::Utf8PathBuf;
use std::fs::OpenOptions;
use std::io::Write;

pub struct StarshipConfigInstaller;

impl StarshipConfigInstaller {
    fn starship_config_path() -> Utf8PathBuf {
        home_path(".config/starship.toml")
    }
}

impl Installable for StarshipConfigInstaller {
    fn install(&self) -> anyhow::Result<()> {
        let config_path = Self::starship_config_path();
        let mut config_file = OpenOptions::new()
            .write(true)
            .truncate(true)
            .open(Self::starship_config_path())
            .context(format!("Error opening {config_path}"))?;
        config_file
            .write_all(include_bytes!("../../resource/starship/starship.toml"))
            .context(format!("Error writing {config_path}"))?;
        Ok(())
    }

    fn update(&self) -> anyhow::Result<()> {
        self.install()
    }

    fn installed(&self) -> anyhow::Result<bool> {
        Ok(Self::starship_config_path().exists())
    }

    fn dependencies(&self) -> &[InstallerKey] {
        Default::default()
    }
}

