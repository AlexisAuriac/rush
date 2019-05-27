use std::io::{self, BufRead};

#[derive(Debug)]
struct Shell {
    // env: Vec<(String, String)>,
    exit_status: i8,
    // tty: bool,
    stop: bool,
}

impl Shell {
    fn new() -> Shell {
        Shell {
            // env: Vec::new(),
            exit_status: 0,
            // tty: true,
            stop: false,
        }
    }
}

fn split_command(line: &String) -> Vec<String> {
    line.split(' ')
        .map(|s| s.to_string())
        .filter(|s| s.trim() != "")
        .map(|s| s.to_string())
        .collect()
}

use std::io::stdout;
use std::io::Write;

fn display_prompt(shell: &Shell) {
    let cwd = std::env::current_dir().unwrap();

    if let Some(dir) = cwd.file_name() {
        if let Some(dir) = dir.to_str() {
            print!("{} -> ({})$ ", shell.exit_status, dir);
            if let Err(err) = stdout().flush() {
                println!("{:?}", err);
            }
        }
    }
}

fn main() {
    let mut shell: Shell = Shell::new();

    let stdin = io::stdin();

    display_prompt(&shell);
    for line in stdin.lock().lines() {
        display_prompt(&shell);

        let line = line.unwrap();
        let command = split_command(&line);

        // println!("{:?}", command);
    }
}
