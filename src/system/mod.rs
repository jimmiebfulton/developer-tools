pub use cargo::{CargoCommand, Rust};
pub use fish::FishConfigInstaller;
pub use git::GitRepo;
pub use group::Group;
pub use homebrew::{HomebrewPackageManager, HomebrewPackage};
pub use ideavim::IdeavimConfigInstaller;
pub use installable::Installable;
pub use nix::{NixPackageManager};
pub use registry::{install, register, update};
pub use starship::StarshipConfigInstaller;

mod installable;
mod homebrew;
mod registry;
mod cargo;
mod git;
mod group;
mod nix;
mod fish;
mod starship;
mod ideavim;

