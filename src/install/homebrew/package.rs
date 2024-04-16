use std::fmt::Display;

use crate::install::installer::Installer;

pub trait HomebrewPackageInstaller: Display {
    fn package(&self) -> &'static str;
}

