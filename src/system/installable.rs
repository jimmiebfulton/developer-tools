use std::fmt::Display;

use anyhow::Result;

use crate::installers::InstallerKey;

pub trait Installable: Send + Sync {
    fn install(&self) -> Result<()>;

    fn installed(&self) -> Result<bool>;

    fn dependencies(&self) -> &[InstallerKey];
}
