use isatty;
use std::env;
use std::io::{stdout, Write};

use crate::utility::split_no_empty;

#[derive(Debug)]
pub struct Shell {
    pub env: Vec<(String, String)>,
    pub exit_status: i32,
    pub tty: bool,
    pub stop: bool,
}

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

    pub fn handle_command(self: &mut Shell, line: String) {
        let command = split_no_empty(&line);

        if command.len() == 0 {
            return;
        }

        for builtin in crate::builtins::BUILTINS.iter() {
            if command[0] == builtin.name {
                (builtin.f)(self, &command);
                return;
            }
        }
    }
}
