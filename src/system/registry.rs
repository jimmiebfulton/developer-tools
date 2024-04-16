use std::collections::HashMap;
use std::sync::{Arc, Mutex, OnceLock, RwLock};

use anyhow::{anyhow, Result};
use tracing::{debug, info, trace};
use crate::installers::InstallerKey;

use crate::system::Installable;

fn registry() -> &'static Arc<RwLock<Registry>> {
    static REGISTRY: OnceLock<Arc<RwLock<Registry>>> = OnceLock::new();
    REGISTRY.get_or_init(|| Arc::new(RwLock::new(Registry::new())))
}

pub fn install(key: &InstallerKey) -> Result<()> {
    registry().read().unwrap().install(key)
}

pub fn installed(key: &InstallerKey) -> Result<bool> {
    registry().read().unwrap().installed(key)
}

pub fn init() -> Result<()> {
    crate::installers::init()
}

pub fn register<I: Installable + 'static>(key: InstallerKey, installer: I) -> Result<()> {
    registry().write().unwrap().register(key, Arc::new(installer))?;
    Ok(())
}

pub struct Registry {
    installers: HashMap<InstallerKey, Arc<dyn Installable>>
}

impl Registry {
    pub fn new() -> Self {
        Registry {
            installers: Default::default(),
        }
    }

    pub fn register(&mut self, key: InstallerKey, installable: Arc<dyn Installable>) -> Result<&mut Registry> {
        if self.installers.contains_key(&key) {
            return Err(anyhow!("Registry already contains installer for '{key:?}'"))
        }
        self.installers.insert(key, installable);
        Ok(self)
    }

    pub fn install(&self, key: &InstallerKey) -> Result<()> {
        match self.installers.get(key) {
            None => {
                return Err(anyhow!("'{:?}' not registered", key))
            }
            Some(installer) => {
                for dependency in installer.dependencies() {
                    self.install(dependency)?
                }
                debug!("Checking {key:?}:");
                if !installer.installed()? {
                    debug!("\t'{key:?} installing");
                    return installer.install();
                } else {
                    info!("\t'{key:?}' already installed");
                    Ok(())
                }
            }
        }
    }

    pub fn installed(&self, key: &InstallerKey) -> Result<bool> {
        match self.installers.get(key) {
            None => {
                return Err(anyhow!("'{:?}' not registered", key))
            }
            Some(installer) => {
                return installer.installed();
            }
        }
    }
}