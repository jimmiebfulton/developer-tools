use std::fmt::{Display, Formatter};
use crate::install::homebrew::command::HomebrewCommandInstaller;
use crate::install::installer::{Installer};
use crate::install::installers::docker::Docker;

pub struct Kind;

impl Display for Kind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Kind")
    }
}

impl HomebrewCommandInstaller for Kind {
    fn command(&self) -> &'static str {
        "kind"
    }

    fn package(&self) -> &'static str {
        "kind"
    }

    fn dependencies(&self) -> Vec<Box<dyn Installer>> {
        vec![Box::new(Docker)]
    }
}