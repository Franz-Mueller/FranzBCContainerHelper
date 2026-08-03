use crate::command::Command;
use crate::commands::{
    install_apps::InstallBCApps, new_container::NewBCContainer,
    new_container_with_apps::NewBCContainerWithApps, remove_container::RemoveBCContainer,
};
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
        "NewBCContainer" => {
            let command = NewBCContainer::build(&args[2..]);
            command.run()
        }
        "RemoveBCContainer" => {
            let command = RemoveBCContainer::build(&args[2..]);
            command.run()
        }
        "InstallBCApps" => {
            let command = InstallBCApps::build(&args[2..]);
            command.run()
        }
        "NewBCContainerWithApps" => {
            let command = NewBCContainerWithApps::build(&args[2..]);
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
