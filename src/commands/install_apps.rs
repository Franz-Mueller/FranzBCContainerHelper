use crate::command::Command;
use crate::docker::container::install_app_into_bc_container;
use std::error::Error;

pub struct InstallBCApps {
    container_name: String,
    path: String,
}

impl Command for InstallBCApps {
    fn build(args: &[String]) -> InstallBCApps {
        InstallBCApps {
            container_name: args[0].to_string(),
            path: args[1].to_string(),
        }
    }
    fn run(&self) -> Result<(), Box<dyn Error>> {
        install_app_into_bc_container(&self.container_name, &self.path)?;
        Ok(())
    }
}
