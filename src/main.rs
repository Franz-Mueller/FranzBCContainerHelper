use crate::command::Command;
use crate::commands::{new_container::NewContainer, remove_container::RemoveContainer};
use std::error::Error;
use std::{env, process};

mod command;
mod commands;
mod docker;

fn run(args: &[String]) -> Result<(), Box<dyn Error>> {
    if args.len() < 2 {
        return Err("Please provide arguments.".into());
    }

    match args[1].as_str() {
        "NewContainer" => {
            let command = NewContainer::build(&args[2..]);
            command.run()
        }
        "RemoveContainer" => {
            let command = RemoveContainer::build(&args[2..]);
            command.run()
        }
        _ => Err("Command not found.".into()),
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if let Err(e) = run(&args) {
        eprint!("Application error: {e}");
        process::exit(1);
    }
}
