use crate::command::Command;
use std::error::Error;

pub struct RemoveContainer {
    name: String,
}

impl Command for RemoveContainer {
    fn build(args: &[String]) -> Result<Self, Box<dyn Error>>
    where
        Self: Sized,
    {
        Ok(Self {
            name: "bccontainer".to_string(),
        })
    }
    fn run(&self) -> Result<(), Box<dyn Error>> {
        Ok(())
    }
}
