use crate::errors::{FailedContainerCreation, FailedContainerRemoval};
use std::error::Error;
use std::process::Command;

pub fn create_bc_docker_container(name: &str) -> Result<(), Box<dyn Error>> {
    let output = Command::new("docker")
        .arg("run")
        .arg("--name")
        .arg(name)
        .arg("busybox")
        .output()?;

    if output.status.success() {
        Ok(())
    } else {
        Err(FailedContainerCreation {
            status: format!("{}", output.status),
            stderr: format!("{}", String::from_utf8_lossy(&output.stderr)),
        }
        .into())
    }
}

pub fn remove_bc_docker_container(name: &str) -> Result<(), Box<dyn Error>> {
    let output = Command::new("docker").arg("rm").arg(name).output()?;

    if output.status.success() {
        Ok(())
    } else {
        Err(FailedContainerRemoval {
            status: format!("{}", output.status),
            stderr: format!("{}", String::from_utf8_lossy(&output.stderr)),
        }
        .into())
    }
}
