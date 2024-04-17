use std::fmt::{Display, Formatter};

use crate::installers::InstallerKey;
use crate::system::Installable;
use crate::system::registry::installed;

pub struct Group {
    name: String,
    dependencies: Vec<InstallerKey>,
}

impl Group {
    pub fn new<N: Into<String>>(name: N) -> Group {
        Group {
            name: name.into(),
            dependencies: Default::default(),
        }
    }

    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn with_dependency(mut self, key: InstallerKey) -> Group {
        self.dependencies.push(key);
        self
    }
}

impl Display for Group {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name())
    }
}

impl Installable for Group {
    fn install(&self) -> anyhow::Result<()> {
        Ok(())
    }

    fn installed(&self) -> anyhow::Result<bool> {
        for dependency in self.dependencies() {
            if !installed(dependency)? {
                return Ok(false);
            }
        }
        Ok(true)
    }

    fn dependencies(&self) -> &[InstallerKey] {
        self.dependencies.as_slice()
    }
}

