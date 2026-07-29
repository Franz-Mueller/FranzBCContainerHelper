use crate::command::Command;
use std::error::Error;

pub struct NewContainer {
    name: String,
}

impl Command for NewContainer {
    fn build(args: &[String]) -> NewContainer {
        NewContainer {
            name: "bccontainer".to_string(),
        }
    }
    fn run(&self) -> Result<(), Box<dyn Error>> {
        Ok(())
    }
}
