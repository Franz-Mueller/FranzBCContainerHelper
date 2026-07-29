use std::env;
use std::error::Error;
use std::result::Result;

pub mod command;
pub mod commands;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        panic!("Please enter a command");
    }

    match args[1].as_str() {
        "new_container" => {
            let command = NewContainer::build(&args[2..]);
            command.run();
        }
        "remove_container" => {
            let command = RemoveContainer::build(&args[2..]);
            command.run();
        }
        _ => {
            panic!("NOOOO");
        }
    }
}
