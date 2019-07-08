use std::io::{self, BufRead};
use std::process::exit;

#[macro_use]
extern crate lazy_static;

mod builtins;
mod handle_command;
mod shell;

use handle_command::handle_command;
use shell::{display_prompt, Shell};

fn main() {
    let stdin = io::stdin();
    let mut sh = Shell::new();

    display_prompt(&sh);
    for line in stdin.lock().lines() {
        handle_command(&mut sh, line.unwrap());
        if sh.stop {
            exit(sh.exit_status);
        }
        display_prompt(&sh);
    }

    if sh.tty {
        println!("exit");
    }
    exit(sh.exit_status);
}
