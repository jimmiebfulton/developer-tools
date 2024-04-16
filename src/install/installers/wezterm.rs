use std::fmt::{Display, Formatter};

use crate::install::homebrew::command::HomebrewCommandInstaller;
use crate::install::installer::Installer;
use crate::install::installers::Fonts;

pub struct Wezterm;

impl Display for Wezterm {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Wezterm")
    }
}

impl HomebrewCommandInstaller for Wezterm {
    fn command(&self) -> &'static str {
        "wezterm"
    }

    fn package(&self) -> &'static str {
        "wezterm"
    }

    fn dependencies(&self) -> Vec<Box<dyn Installer>> {
        vec![Box::new(Fonts)]
    }
}