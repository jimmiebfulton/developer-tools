use std::fs::File;
use std::io::Write;
use anyhow::Context;
use log::debug;
use crate::installers::InstallerKey;
use crate::system::Installable;
use crate::utils::home_path;

pub struct FishConfigInstaller;

impl Installable for FishConfigInstaller {
    fn install(&self) -> anyhow::Result<()> {
        let config_path = home_path(".config/fish/config.fish");
            let mut config_file = std::fs::OpenOptions::new().create(true).write(true).open(config_path.to_string())
                .context(format!("Error opening {config_path}"))?;
            debug!("Writing Fish Config File");
            config_file.write_all(include_bytes!("../../resource/fish/config.fish"))
                .context("Error writing to {config_path}")?;

        let local_path = home_path(".config/fish/local.fish");
        if !local_path.exists() {
            let mut local_file = File::create(local_path)?;
            debug!("Writing Fish Local File");
            local_file.write_all(include_bytes!("../../resource/fish/local.fish"))?;
        }
        Ok(())
    }

    fn update(&self) -> anyhow::Result<()> {
        self.install()
    }

    fn installed(&self) -> anyhow::Result<bool> {
        Ok(home_path(".config/fish/config.fish").exists())
    }



    fn dependencies(&self) -> &[InstallerKey] {
        Default::default()
    }
}