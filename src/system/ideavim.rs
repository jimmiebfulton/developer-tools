use std::fs::OpenOptions;
use std::io::Write;
use anyhow::Context;
use camino::Utf8PathBuf;
use crate::installers::InstallerKey;
use crate::system::Installable;
use crate::utils::home_path;

pub struct IdeavimConfigInstaller;

impl IdeavimConfigInstaller {
    fn config_path() -> Utf8PathBuf {
        home_path(".ideavimrc")
    }
}

impl Installable for IdeavimConfigInstaller {
    fn install(&self) -> anyhow::Result<()> {
        let config_path = Self::config_path();
        let mut config_file = OpenOptions::new().write(true).create(true).open(&config_path)
            .context(format!("Error opening {config_path}"))?;
        config_file.write_all(include_bytes!("../../resource/ideavim/.ideavimrc"))
            .context(format!("Error writing {config_path}"))?;
        Ok(())
    }

    fn update(&self) -> anyhow::Result<()> {
        self.install()
    }


    fn installed(&self) -> anyhow::Result<bool> {
        Ok(Self::config_path().exists())
    }

    fn dependencies(&self) -> &[InstallerKey] {
        Default::default()
    }
}