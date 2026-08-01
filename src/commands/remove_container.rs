use crate::command::Command;
use crate::docker::container::remove_bc_docker_container;
use std::error::Error;

pub struct RemoveContainer {
    name: String,
}

impl Command for RemoveContainer {
    fn build(args: &[String]) -> RemoveContainer {
        RemoveContainer {
            name: args[0].to_string(),
        }
    }
    fn run(&self) -> Result<(), Box<dyn Error>> {
        remove_bc_docker_container(&self.name)?;
        Ok(())
    }
}
