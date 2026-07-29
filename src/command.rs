use std::error::Error;

pub trait Command {
    fn build(args: &[String]) -> Self;
    fn run(&self) -> Result<(), Box<dyn Error>>;
}
