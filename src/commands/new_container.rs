use crate::command::Command;
use crate::docker::container::create_bc_docker_container;
use std::error::Error;

pub struct NewContainer {
    name: String,
}

impl Command for NewContainer {
    fn build(args: &[String]) -> NewContainer {
        NewContainer {
            name: args[0].to_string(),
        }
    }
    fn run(&self) -> Result<(), Box<dyn Error>> {
        create_bc_docker_container(&self.name)?;
        Ok(())
    }
}
