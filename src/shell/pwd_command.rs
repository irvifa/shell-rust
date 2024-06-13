use std::env;

use super::command::Command;

pub struct PwdCommand;

impl Command for PwdCommand {
    fn execute(&self, args: &[String]) -> bool {
        if !args.is_empty() {
            eprintln!("pwd: too many arguments");
            return true;
        }

        match env::current_dir() {
            Ok(dir) => {
                println!("{}", dir.display());
            }
            Err(_) => {
                eprintln!("pwd: could not determine current directory");
            }
        }
        true
    }
}
