use std::collections::HashMap;

use super::command::Command;

pub struct TypeCommand<'a> {
    builtins: &'a HashMap<String, Box<dyn Command + 'a>>,
}

impl<'a> TypeCommand<'a> {
    pub fn new(builtins: &'a HashMap<String, Box<dyn Command + 'a>>) -> TypeCommand<'a> {
        TypeCommand { builtins }
    }
}

impl<'a> Command for TypeCommand<'a> {
    fn execute(&self, args: &[String]) -> bool {
        if args.len() != 1 {
            eprintln!("type: invalid number of arguments");
            return true;
        }

        let command_name = &args[0];

        if self.builtins.contains_key(command_name) {
            println!("{} is a shell builtin", command_name);
            return true;
        }

        match std::process::Command::new("which").arg(command_name).output() {
            Ok(output) => {
                if output.status.success() {
                    let path = String::from_utf8_lossy(&output.stdout);
                    print!("{} is {}", command_name, path);
                } else {
                    eprintln!("{}: not found", command_name);
                }
            }
            Err(_) => {
                eprintln!("type: could not execute 'which' command");
            }
        }
        true
    }
}
