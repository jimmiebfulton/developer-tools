use std::fmt::{Display, Formatter};
use crate::install::homebrew::command::HomebrewCommandInstaller;
use crate::install::installer::{Installer};
use crate::install::installers::docker::Docker;

pub struct Tilt;

impl Display for Tilt {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Tilt")
    }
}

impl HomebrewCommandInstaller for Tilt {
    fn command(&self) -> &'static str {
        "tilt"
    }

    fn package(&self) -> &'static str {
        "tilt"
    }

    fn dependencies(&self) -> Vec<Box<dyn Installer>> {
        vec![Box::new(Docker)]
    }
}