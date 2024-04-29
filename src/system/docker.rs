use std::process::Command;
use camino::Utf8Path;
use tracing::info;
use crate::installers::InstallerKey;
use crate::system::Installable;
use crate::utils::{execute, home_path};

pub struct DockerSymlinkInstaller;

impl Installable for DockerSymlinkInstaller {
    fn install(&self) -> anyhow::Result<()> {
        info!("Sudo to create Docker Symlink");
        let docker_socket = home_path("Library/Containers/com.docker.docker/Data/docker.raw.sock");
        execute(
            Command::new("sudo")
                .arg("ln")
                .arg("-s")
                .arg(docker_socket.as_str())
                .arg("/var/run/docker.sock")
        )?;
        Ok(())
    }

    fn installed(&self) -> anyhow::Result<bool> {
        Ok(Utf8Path::new("/var/run/docker.sock").exists())
    }

    fn dependencies(&self) -> &[InstallerKey] {
        Default::default()
    }
}