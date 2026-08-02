use crate::command::Command;
use crate::docker::container::create_bc_docker_container;
use std::error::Error;

pub struct NewBCContainer {
    name: String,
    version: String,
    country: String, // TODO enum?
}

impl Command for NewBCContainer {
    fn build(args: &[String]) -> NewBCContainer {
        NewBCContainer {
            name: args[0].to_string(),
            version: args[1].to_string(),
            country: args[2].to_string(),
        }
    }
    fn run(&self) -> Result<(), Box<dyn Error>> {
        create_bc_docker_container(&self.name, &self.version, &self.country)?;
        Ok(())
    }
}
