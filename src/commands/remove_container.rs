use crate::command::Command;
use std::error::Error;

pub struct RemoveContainer {
    name: String,
}

impl Command for RemoveContainer {
    fn build(args: &[String]) -> RemoveContainer {
        RemoveContainer {
            name: args[1].to_string(),
        }
    }
    fn run(&self) -> Result<(), Box<dyn Error>> {
        println!("{}", self.name);
        Ok(())
    }
}
