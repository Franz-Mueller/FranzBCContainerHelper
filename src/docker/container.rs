use crate::errors::{FailedBCContainerCreation, FailedContainerRemoval, OnlyAvailableOnWindows};
use std::error::Error;
use std::process::Command;

pub fn create_bc_docker_container(
    // TODO pass struct?
    name: &str,
    version: &str,
    country: &str,
) -> Result<(), Box<dyn Error>> {
    let output = if cfg!(target_os = "windows") {
        let get_artifact_url_command =
            format!("(Get-BCArtifactUrl -version {version} -country {country} -select Latest)");

        Command::new("New-BcContainer")
            .arg("-accept_eula")
            .args(["-containerName", name])
            .args(["-artifactUrl", &get_artifact_url_command])
            .args(["-Credential", "(New-Object pscredential 'admin', (ConvertTo-SecureString -String 'P@ssword1' -AsPlainText -Force))"])
            .args(["auth", "NavUserPassword"])
            .output()?
    } else {
        return Err(OnlyAvailableOnWindows {
            command: "NewBCContainer".to_string(), // TODO move to better spot
        }
        .into());
    };

    if output.status.success() {
        Ok(())
    } else {
        Err(FailedBCContainerCreation {
            status: format!("{}", output.status),
            stderr: format!("{}", String::from_utf8_lossy(&output.stderr)),
        }
        .into())
    }
}

pub fn remove_bc_docker_container(name: &str) -> Result<(), Box<dyn Error>> {
    let output = if cfg!(target_os = "windows") {
        Command::new("Remove-BcContainer").arg(name).output()?
    } else {
        return Err(OnlyAvailableOnWindows {
            command: "NewBCContainer".to_string(), // TODO move to better spot
        }
        .into());
    };

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
