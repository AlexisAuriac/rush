use std::io::{self, BufRead};
use std::process;

#[macro_use]
extern crate lazy_static;

mod builtins;
mod shell;
mod utility;

use shell::*;

fn main() {
    let mut sh: shell::Shell = shell::new_shell();
    let stdin = io::stdin();

    display_prompt(&sh);
    for line in stdin.lock().lines() {
        handle_command(&mut sh, line.unwrap());
        if sh.stop {
            process::exit(sh.exit_status);
        }
        display_prompt(&sh);
    }

    if sh.tty {
        println!("exit");
    }
    process::exit(sh.exit_status);
}
