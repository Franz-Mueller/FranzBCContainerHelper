use crate::command::Command;
use crate::commands::{new_container::NewContainer, remove_container::RemoveContainer};
use crate::errors::{NoCommand, UnknownCommand};
use std::error::Error;
use std::{env, process};

mod command;
mod commands;
mod docker;
mod errors;

const RED: &str = "\x1b[31m";
const RESET: &str = "\x1b[0m";

fn run(args: &[String]) -> Result<(), Box<dyn Error>> {
    if args.len() < 2 {
        return Err(NoCommand.into());
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
        n => Err(UnknownCommand {
            false_command: n.to_string(),
        }
        .into()),
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if let Err(e) = run(&args) {
        eprintln!("{RED}Application error: {e}{RESET}");
        process::exit(1);
    }
}
