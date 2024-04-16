use std::fmt::{Display, Formatter};
use crate::install::homebrew::command::HomebrewCommandInstaller;
use crate::install::installer::{InstallationGroup, Installer};
use crate::install::installers::k9s::K9S;
use crate::install::installers::kind::Kind;
use crate::install::installers::tilt::Tilt;

pub struct Kubernetes;

impl Display for Kubernetes {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Kubernetes")
    }
}

impl InstallationGroup for Kubernetes {
    fn dependencies(&self) -> Vec<Box<dyn Installer>> {
        vec![Box::new(Kind), Box::new(Tilt), Box::new(K9S)]
    }
}