use anyhow::Result;
use camino::Utf8PathBuf;
use clap::ValueEnum;

use InstallerKey::*;

use crate::system::{CargoCommand, DockerSymlinkInstaller, FishConfigInstaller, GitRepo, Group, HomebrewPackage, IdeavimConfigInstaller, NixPackageManager, register, StarshipConfigInstaller};
use crate::utils::{home_path, homebrew_path};

pub fn init() -> Result<()> {
    register(
        Docker,
        HomebrewPackage::new("docker", Utf8PathBuf::from("/usr/local/bin/docker"))
            .cask()
            .with_dependency(Homebrew)
            .with_dependency(DockerSymlink)

    )?;

    register(
        DockerSymlink,
        DockerSymlinkInstaller,
    )?;

    register(
        Tilt,
        HomebrewPackage::command("tilt")
            .with_dependency(Docker),
    )?;

    register(
        Kind,
        HomebrewPackage::command("kind")
            .with_dependency(Docker),
    )?;

    register(
        Just,
        HomebrewPackage::command("just")
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
            home_path(".config/nvim"),
        ).with_dependency(JetbrainsFont).with_dependency(Node),
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
        ).with_dependency(JetbrainsFont),
    )?;

    register(
        Exa,
        CargoCommand::new("exa"),
    )?;

    register(
        Zoxide,
        CargoCommand::new("zoxide"),
    )?;

    register(
        Kubernetes,
        Group::new("Kubernetes")
            .with_dependency(Kind)
            .with_dependency(Tilt),
    )?;

    register(
        Homebrew,
        crate::system::HomebrewPackageManager,
    )?;

    register(
        Zed,
        HomebrewPackage::command("zed")
            .cask(),
    )?;

    register(
        Fonts,
        Group::new("Fonts")
            .with_dependency(JetbrainsFont),
    )?;

    register(
        JetbrainsFont,
        HomebrewPackage::new(
            "font-jetbrains-mono-nerd-font",
            home_path("Library/Fonts/JetBrainsMonoNerdFont-Bold.ttf"),
        ).tap("homebrew/cask-fonts"),
    )?;

    register(
        JetbrainsToolbox,
        HomebrewPackage::new("jetbrains-toolbox", Utf8PathBuf::from("/Applications/JetBrains Toolbox.app"))
            .cask()
            .with_dependency(IdeavimConfig),
    )?;

    register(
        AraxisMerge,
        HomebrewPackage::new("araxis-merge", Utf8PathBuf::from("/Applications/Araxis Merge.app"))
            .cask(),
    )?;

    register(
        Zoom,
        HomebrewPackage::new("zoom", Utf8PathBuf::from("/Applications/zoom.us.app"))
            .cask(),
    )?;

    register(
        Rust,
        crate::system::Rust,
    )?;

    register(
        Fish,
        HomebrewPackage::command("fish")
            .with_dependency(FishConfig),
    )?;

    register(
        FishConfig,
        FishConfigInstaller
    )?;

    register(
        Lazygit,
        HomebrewPackage::command("lazygit")
    )?;

    register(
        Postman,
        HomebrewPackage::new("postman", Utf8PathBuf::from("/Applications/Postman.app"))
            .cask()
    )?;

    register(
       Protoc,
        HomebrewPackage::new("protobuf", homebrew_path("protoc"))
    )?;

    register(
        Ripgrep,
        HomebrewPackage::command("rg"),
    )?;

    register(
        Tailwind,
        HomebrewPackage::command("tailwindcss"),
    )?;

    register(
        Trunk,
        HomebrewPackage::command("trunk"),
    )?;

    register(
        Nix,
        NixPackageManager,
    )?;

    register(
        IdeavimConfig,
        IdeavimConfigInstaller
    )?;

    register(
        Karabiner,
        HomebrewPackage::new("karabiner-elements", Utf8PathBuf::from("/test")),
    )?;

    register(
        Maven,
        HomebrewPackage::new("maven", homebrew_path("mvn")),
    )?;

    register(
        Node,
        HomebrewPackage::command("node"),
    )?;

    register(
        Nushell,
        HomebrewPackage::new("nushell", homebrew_path("nu"))
    )?;

    register(
        Starship,
        CargoCommand::new("starship")
            .with_dependency(StarshipConfig)
            .with_dependency(Fonts)
    )?;

    register(
        StarshipConfig,
        StarshipConfigInstaller
    )?;

    Ok(())
}

#[derive(Clone, Debug, PartialOrd, PartialEq, Eq, Ord, Hash, ValueEnum)]
pub enum InstallerKey {
    AraxisMerge,
    Archetect,
    ArchetectRepo,
    Docker,
    DockerSymlink,
    Exa,
    Fish,
    FishConfig,
    Fonts,
    Homebrew,
    IdeavimConfig,
    JetbrainsFont,
    JetbrainsToolbox,
    Just,
    Karabiner,
    Kind,
    Kubernetes,
    Lazygit,
    Maven,
    Nix,
    Node,
    Neovim,
    NeovimConfig,
    Nushell,
    Postman,
    Protoc,
    Ripgrep,
    Rust,
    Starship,
    StarshipConfig,
    Tailwind,
    Tilt,
    Trunk,
    Wezterm,
    WeztermConfig,
    Zed,
    Zoom,
    Zoxide,
}
