use anyhow::Result;
use camino::Utf8PathBuf;
use clap::ValueEnum;

use InstallerKey::*;

use crate::system::{CargoCommand, GitRepo, Group, HomebrewPackage, NixPackageManager, register};
use crate::utils::home_path;

pub fn init() -> Result<()> {
    register(
        Docker,
        HomebrewPackage::new("docker", Utf8PathBuf::from("/usr/local/bin/docker"))
            .cask()
            .with_dependency(Homebrew)
    )?;

    register(
        Tilt,
        HomebrewPackage::command("tilt")
            .with_dependency(Homebrew)
            .with_dependency(Docker)
    )?;

    register(
        Kind,
        HomebrewPackage::command("kind")
            .with_dependency(Homebrew)
            .with_dependency(Docker)
    )?;

    register(
        Neovim,
        HomebrewPackage::command("nvim")
            .with_dependency(NeovimConfig),
    )?;

    register(
        NeovimConfig,
        GitRepo::new(
            "Neovim Config",
            "git@github.com:jimmiebfulton/conf-nvim-lazyvim.git",
            home_path(".config/nvim")
        ).with_dependency(JetbrainsFont)
    )?;

    register(
        Wezterm,
        HomebrewPackage::command("wezterm")
            .cask()
            .with_dependency(WeztermConfig),
    )?;

    register(
        WeztermConfig,
        GitRepo::new(
            "Wezterm Config",
            "git@github.com:jimmiebfulton/conf-wezterm.git",
            home_path(".config/wezterm"),
        ).with_dependency(JetbrainsFont)
   )?;

    register(
        Exa,
        CargoCommand::new("exa")
    )?;

    register(
        Zoxide,
        CargoCommand::new("zoxide")
    )?;

    register(
        Kubernetes,
        Group::new("Kubernetes")
            .with_dependency(Kind)
            .with_dependency(Tilt)
    )?;

    register(
        Homebrew,
        crate::system::HomebrewPackageManager,
    )?;

    register(
        Zed,
        HomebrewPackage::command("zed")
            .cask()
    )?;

    register(
        Fonts,
        Group::new("Fonts")
            .with_dependency(JetbrainsFont)
    )?;

    register(
        JetbrainsFont,
        HomebrewPackage::new(
            "font-jetbrains-mono-nerd-font",
            home_path("Library/Fonts/JetBrainsMonoNerdFont-Bold.ttf")
        )
    )?;

    register(
        JetbrainsToolbox,
        HomebrewPackage::new("jetbrains-toolbox", Utf8PathBuf::from("/Applications/JetBrains Toolbox.app"))
            .cask()
    )?;

    register(
        AraxisMerge,
        HomebrewPackage::new("araxis-merge", Utf8PathBuf::from("/Applications/Araxis Merge.app"))
            .cask()
    )?;

    register(
        Zoom,
        HomebrewPackage::new("zoom", Utf8PathBuf::from("/Applications/zoom.us.app"))
            .cask()
    )?;

    register(
        Rust,
        crate::system::Rust,
    )?;

    register(
        Fish,
        HomebrewPackage::command("fish")
    )?;

    register(
        Ripgrep,
        HomebrewPackage::command("rg")
    )?;

    register(
        Nix,
        NixPackageManager
    )?;

    register(
        Karabiner,
        HomebrewPackage::new("karabiner-elements", Utf8PathBuf::from("/test"))
    )?;

    Ok(())
}

#[derive(Clone, Debug, PartialOrd, PartialEq, Eq, Ord, Hash, ValueEnum)]
pub enum InstallerKey {
    AraxisMerge,
    Archetect,
    ArchetectRepo,
    Docker,
    Exa,
    Fish,
    Fonts,
    Homebrew,
    JetbrainsFont,
    JetbrainsToolbox,
    Karabiner,
    Kind,
    Kubernetes,
    Nix,
    Neovim,
    NeovimConfig,
    Ripgrep,
    Rust,
    Tilt,
    Wezterm,
    WeztermConfig,
    Zed,
    Zoom,
    Zoxide,
}

#[cfg(test)]
mod tests {
    use anyhow::Result;

    use crate::installers::init;
    use crate::installers::InstallerKey::Wezterm;
    use crate::system::install;

    #[test]
    fn test() -> Result<()> {
        init()?;
        install(&Wezterm)?;
        Ok(())
    }
}

