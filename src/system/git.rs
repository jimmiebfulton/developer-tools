use std::fmt::{Display, Formatter};
use std::process::Command;

use camino::Utf8PathBuf;

use crate::installers::InstallerKey;
use crate::system::Installable;
use crate::utils::execute;

pub struct GitRepo {
    name: String,
    url: String,
    directory: Utf8PathBuf,
    dependencies: Vec<InstallerKey>
}

impl GitRepo {
    pub fn new<N: Into<String>, U: Into<String>, D: Into<Utf8PathBuf>>(name: N, url: U, directory: D) -> GitRepo {
        GitRepo {
            name: name.into(),
            url: url.into(),
            directory: directory.into(),
            dependencies: Default::default(),
        }
    }

    pub fn with_dependency(mut self, key: InstallerKey) -> GitRepo {
        self.dependencies.push(key);
        self
    }
}

impl Display for GitRepo {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl Installable for GitRepo {
    fn install(&self) -> anyhow::Result<()> {
        execute(
            Command::new("git")
                .arg("clone")
                .arg(&self.url)
                .arg(self.directory.to_string())
        )?;
        Ok(())
    }

    fn installed(&self) -> anyhow::Result<bool> {
        Ok(self.directory.exists())
    }

    fn dependencies(&self) -> &[InstallerKey] {
        self.dependencies.as_slice()
    }
}