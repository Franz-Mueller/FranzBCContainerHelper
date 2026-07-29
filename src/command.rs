use std::error::Error;

pub trait Command {
    fn build(args: &[String]) -> Result<Self, Box<dyn Error>>
    where
        Self: Sized;
    fn run(&self) -> Result<(), Box<dyn Error>>;
}
