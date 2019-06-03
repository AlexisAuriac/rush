use std::collections::HashMap;
use std::env;
use std::io::{stdout, Write};

#[derive(Debug)]
pub struct Shell {
    pub env: HashMap<String, String>,
    pub exit_status: i32,
    pub tty: bool,
    pub stop: bool,
}

mod handle_command;

impl Shell {
    pub fn new() -> Shell {
        Shell {
            env: env::vars().collect(),
            exit_status: 0,
            tty: isatty::stdin_isatty(),
            stop: false,
        }
    }

    pub fn display_prompt(self: &Shell) {
        let cwd = std::env::current_dir().unwrap();

        if !self.tty {
            return;
        }

        if let Some(dir) = cwd.file_name() {
            if let Some(dir) = dir.to_str() {
                print!("{} -> ({})$ ", self.exit_status, dir);
                if let Err(err) = stdout().flush() {
                    println!("{:?}", err);
                }
            }
        }
    }
}
