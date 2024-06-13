use std::env;
use std::path::PathBuf;

use super::command::Command;

pub struct CdCommand;

impl Command for CdCommand {
    fn execute(&self, args: &[String]) -> bool {
        if args.len() != 1 {
            eprintln!("cd: invalid number of arguments");
            return true;
        }

        let path = &args[0];
        let target_path: PathBuf;

        if path == "~" {
            match env::var("HOME") {
                Ok(home_dir) => {
                    target_path = PathBuf::from(home_dir);
                }
                Err(_) => {
                    eprintln!("cd: HOME environment variable not set");
                    return true;
                }
            }
        } else {
            target_path = match PathBuf::from(path).canonicalize() {
                Ok(p) => p,
                Err(_) => {
                    eprintln!("cd: {}: No such file or directory", path);
                    return true;
                }
            };
        }

        if let Err(err) = env::set_current_dir(&target_path) {
            eprintln!("cd: {}: {}", path, err);
        }
        true
    }
}
