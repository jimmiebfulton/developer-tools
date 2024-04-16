use std::fmt::{Display, Formatter};
use crate::install::homebrew::command::HomebrewCommandInstaller;
use crate::install::installer::{Installer};
use crate::install::installers::docker::Docker;

pub struct K9S;

impl Display for K9S {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "k9s")
    }
}

impl HomebrewCommandInstaller for K9S {
    fn command(&self) -> &'static str {
        "k9s"
    }

    fn package(&self) -> &'static str {
        "k9s"
    }

    fn dependencies(&self) -> Vec<Box<dyn Installer>> {
        vec![Box::new(Docker)]
    }
}