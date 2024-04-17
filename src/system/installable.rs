use anyhow::Result;
use tracing::debug;

use crate::installers::InstallerKey;

pub trait Installable: Send + Sync {
    fn install(&self) -> Result<()>;

    fn update(&self) -> Result<()> {
        debug!("Calling Default Update");
        Ok(())
    }

    fn installed(&self) -> Result<bool>;

    fn dependencies(&self) -> &[InstallerKey];
}
