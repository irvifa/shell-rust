use std::io::{self, Write};

use super::command::Command;

pub struct EchoCommand;

impl Command for EchoCommand {
    fn execute(&self, args: &[String]) -> bool {
        let mut omit_newline = false;
        let mut start = 0;

        if !args.is_empty() && args[0] == "-n" {
            omit_newline = true;
            start = 1;
        }

        let output = args[start..].join(" ");
        if omit_newline {
            print!("{}", output);
            io::stdout().flush().unwrap();
        } else {
            println!("{}", output);
        }
        true
    }
}
