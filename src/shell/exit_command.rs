use std::process;

use super::command::Command;

pub struct ExitCommand;

impl Command for ExitCommand {
    fn execute(&self, args: &[String]) -> bool {
        if args.len() == 1 {
            if let Ok(code) = args[0].parse::<i32>() {
                process::exit(code);
            } else {
                eprintln!("exit: invalid status code");
            }
        } else {
            eprintln!("exit: invalid number of arguments");
        }
        true
    }
}
