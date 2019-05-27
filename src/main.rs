use std::io::{self, BufRead};
use std::process;

#[derive(Debug)]
struct Shell {
    // env: Vec<(String, String)>,
    // exit_status: i8,
    // tty: bool,
    stop: bool,
}

impl Shell {
    fn new() -> Shell {
        Shell {
            // env: Vec::new(),
            // exit_status: 0,
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

fn main() {
    let mut shell: Shell = Shell::new();

    let stdin = io::stdin();

    for line in stdin.lock().lines() {
        let line = line.unwrap();
        let command = split_command(&line);

        println!("{:?}", command);
    }
}
