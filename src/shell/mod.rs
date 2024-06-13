pub mod command;
pub mod cd_command;
pub mod echo_command;
pub mod exit_command;
pub mod pwd_command;
pub mod type_command;

use std::collections::HashMap;
use std::io::{self, Write};

use command::Command;
use cd_command::CdCommand;
use echo_command::EchoCommand;
use exit_command::ExitCommand;
use pwd_command::PwdCommand;
use type_command::TypeCommand;

pub struct Shell<'a> {
    commands: HashMap<String, Box<dyn Command + 'a>>,
}

impl<'a> Shell<'a> {
    pub fn new() -> Shell<'a> {
        let mut shell = Shell {
            commands: HashMap::new(),
        };

        shell.commands.insert("exit".to_string(), Box::new(ExitCommand));
        shell.commands.insert("echo".to_string(), Box::new(EchoCommand));
        shell.commands.insert("cd".to_string(), Box::new(CdCommand));
        shell.commands.insert("pwd".to_string(), Box::new(PwdCommand));

        // Initialize TypeCommand separately to avoid borrowing issues
        let builtins_ref = &shell.commands as *const _;
        unsafe {
            let type_command = Box::new(TypeCommand::new(&*builtins_ref));
            shell.commands.insert("type".to_string(), type_command);
        }

        shell
    }

    pub fn run(&mut self) {
        let stdin = io::stdin();
        loop {
            print!("$ ");
            io::stdout().flush().unwrap();

            let mut input = String::new();
            stdin.read_line(&mut input).unwrap();
            let input = input.trim();

            if input.is_empty() {
                continue;
            }

            let command = self.tokenize(input);
            if !self.execute(&command) {
                self.run_external_command(&command);
            }
        }
    }

    fn tokenize(&self, input: &str) -> CommandStruct {
        let parts: Vec<&str> = input.split_whitespace().collect();
        CommandStruct {
            name: parts[0].to_string(),
            args: parts[1..].iter().map(|&s| s.to_string()).collect(),
        }
    }

    fn execute(&self, command: &CommandStruct) -> bool {
        if let Some(cmd) = self.commands.get(&command.name) {
            return cmd.execute(&command.args);
        }
        false
    }

    fn run_external_command(&self, command: &CommandStruct) {
        match std::process::Command::new(&command.name)
            .args(&command.args)
            .spawn()
        {
            Ok(mut child) => {
                child.wait().unwrap();
            }
            Err(_) => {
                eprintln!("{}: command not found", command.name);
            }
        }
    }
}

pub struct CommandStruct {
    pub name: String,
    pub args: Vec<String>,
}
