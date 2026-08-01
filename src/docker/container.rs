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
        let status = output.status;
        let stderr = String::from_utf8_lossy(&output.stderr);
        Err(format!(
            "Could not create the container with status code: {status}\n{}",
            stderr.trim()
        )
        .into())
    }
}

pub fn remove_bc_docker_container(name: &str) -> Result<(), Box<dyn Error>> {
    let output = Command::new("docker").arg("rm").arg(name).output()?;

    if output.status.success() {
        Ok(())
    } else {
        let status = output.status;
        let stderr = String::from_utf8_lossy(&output.stderr);
        Err(format!(
            "Could not remove the container with status code: {status}\n{}",
            stderr.trim()
        )
        .into())
    }
}
