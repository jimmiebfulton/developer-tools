use std::fmt::{Display, Formatter};
use crate::install::homebrew::command::HomebrewCommandInstaller;
use crate::install::installer::{Installer};
use crate::install::installers::Fonts;

pub struct Neovim;

impl Display for Neovim {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Neovim")
    }
}

impl HomebrewCommandInstaller for Neovim {
    fn command(&self) -> &'static str {
        "nvim"
    }

    fn package(&self) -> &'static str {
        "nvim"
    }

    fn dependencies(&self) -> Vec<Box<dyn Installer>> {
        vec![Box::new(Fonts)]
    }
}