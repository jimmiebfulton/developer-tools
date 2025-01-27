use std::collections::HashMap;
use std::sync::{Arc, OnceLock, RwLock};

use crate::installers::InstallerKey;
use anyhow::{anyhow, Result};
use tracing::{debug, info};

use crate::system::Installable;

fn registry() -> &'static Arc<RwLock<Registry>> {
    static REGISTRY: OnceLock<Arc<RwLock<Registry>>> = OnceLock::new();
    REGISTRY.get_or_init(|| Arc::new(RwLock::new(Registry::new())))
}

pub fn install(key: &InstallerKey) -> Result<()> {
    registry().read().unwrap().install(key)
}

pub fn update(key: &InstallerKey) -> Result<()> {
    registry().read().unwrap().update(key)
}

pub fn installed(key: &InstallerKey) -> Result<bool> {
    registry().read().unwrap().installed(key)
}

pub fn register<I: Installable + 'static>(key: InstallerKey, installer: I) -> Result<()> {
    registry()
        .write()
        .unwrap()
        .register(key, Arc::new(installer))?;
    Ok(())
}

pub struct Registry {
    installers: HashMap<InstallerKey, Arc<dyn Installable>>,
}

impl Registry {
    pub fn new() -> Self {
        Registry {
            installers: Default::default(),
        }
    }

    pub fn register(
        &mut self,
        key: InstallerKey,
        installable: Arc<dyn Installable>,
    ) -> Result<&mut Registry> {
        if self.installers.contains_key(&key) {
            return Err(anyhow!("Registry already contains installer for '{key:?}'"));
        }
        self.installers.insert(key, installable);
        Ok(self)
    }

    pub fn install(&self, key: &InstallerKey) -> Result<()> {
        match self.installers.get(key) {
            None => Err(anyhow!("'{:?}' not registered", key)),
            Some(installer) => {
                for dependency in installer.dependencies() {
                    self.install(dependency)?
                }
                debug!("Checking {key:?}:");
                if !installer.installed()? {
                    debug!("\t'{key:?} installing");
                    installer.install()
                } else {
                    info!("\t'{key:?}' already installed");
                    Ok(())
                }
            }
        }
    }

    pub fn update(&self, key: &InstallerKey) -> Result<()> {
        match self.installers.get(key) {
            None => Err(anyhow!("'{:?}' not registered", key)),
            Some(installer) => {
                info!("Updating '{key:?}'");
                installer.update()
            }
        }
    }

    pub fn installed(&self, key: &InstallerKey) -> Result<bool> {
        match self.installers.get(key) {
            None => Err(anyhow!("'{:?}' not registered", key)),
            Some(installer) => installer.installed(),
        }
    }
}

