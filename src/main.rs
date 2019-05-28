use std::io::{self, BufRead};
use std::process;

mod builtins;
mod shell;
mod utility;

fn main() {
    let mut sh: shell::Shell = shell::Shell::new();
    let stdin = io::stdin();

    sh.display_prompt();
    for line in stdin.lock().lines() {
        sh.display_prompt();
        sh.handle_command(line.unwrap());
        if sh.stop {
            process::exit(sh.exit_status);
        }
    }
}
