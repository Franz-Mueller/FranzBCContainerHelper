use command::Command;
use commands::{new_container::NewContainer, remove_container::RemoveContainer};

use std::{env, error::Error};

mod command;
mod commands;

fn run() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    match args[1].as_str() {
        "NewContainer" => {
            let command = NewContainer::build(&args[2..])?;
            command.run()
        }
        "RemoveContainer" => {
            let command = RemoveContainer::build(&args[2..])?;
            command.run()
        }
        _ => Err("Unknown command".into()),
    }
}

fn main() {
    match run() {
        Ok(_) => {}
        Err(e) => {
            eprintln!("\x1b[1;31m{} {:#}", "Error", e);
            std::process::exit(1);
        }
    }
}
