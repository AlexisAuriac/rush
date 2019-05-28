use isatty;
use std::io::stdout;
use std::io::Write;
use std::io::{self, BufRead};

#[derive(Debug)]
struct Shell {
    // env: Vec<(String, String)>,
    exit_status: i8,
    tty: bool,
    stop: bool,
}

impl Shell {
    fn new() -> Shell {
        Shell {
            // env: Vec::new(),
            exit_status: 0,
            tty: isatty::stdin_isatty(),
            stop: false,
        }
    }

    fn display_prompt(self: &Shell) {
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

fn split_command(line: &String) -> Vec<String> {
    line.split(' ')
        .map(|s| s.to_string())
        .filter(|s| s.trim() != "")
        .collect()
}

fn main() {
    let mut shell: Shell = Shell::new();

    let stdin = io::stdin();

    shell.display_prompt();
    for line in stdin.lock().lines() {
        shell.display_prompt();

        let line = line.unwrap();
        let command = split_command(&line);

        println!("{:?}", command);
    }
}
