use crate::command::Command;
use crate::docker::container::{create_bc_docker_container, install_app_into_bc_container};
use std::error::Error;

pub struct NewBCContainerWithApps {
    name: String,
    version: String,
    country: String, // TODO enum?
    path: String,
}

impl Command for NewBCContainerWithApps {
    fn build(args: &[String]) -> NewBCContainerWithApps {
        NewBCContainerWithApps {
            name: args[0].to_string(),
            version: args[1].to_string(),
            country: args[2].to_string(),
            path: args[3].to_string(),
        }
    }
    fn run(&self) -> Result<(), Box<dyn Error>> {
        create_bc_docker_container(&self.name, &self.version, &self.country)?;
        install_app_into_bc_container(&self.name, &self.path)?;
        Ok(())
    }
}
