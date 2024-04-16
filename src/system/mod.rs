pub use cargo::{CargoCommand, Rust};
pub use git::GitRepo;
pub use group::Group;
pub use homebrew::{HomebrewPackageManager, HomebrewPackage};
pub use installable::Installable;
pub use registry::{install, register};
pub use nix::{NixPackageManager};

mod installable;
mod homebrew;
mod registry;
mod cargo;
mod git;
mod group;
mod nix;

