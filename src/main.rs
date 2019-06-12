use std::io::{self, stdout, BufRead, Write};
use std::option::Option;
use std::process;

use nix::sys::wait::{waitpid, WaitPidFlag, WaitStatus};
use nix::unistd::Pid;

#[macro_use]
extern crate lazy_static;

mod builtins;
mod shell;
mod utility;

use shell::*;

fn display_prompt_ctrlc(exit_status: i32) {
    let cwd = std::env::current_dir().unwrap();

    println!();

    if let Some(dir) = cwd.file_name() {
        print!("{} -> ({})$ ", exit_status, dir.to_str().unwrap());
    } else {
        print!("{} -> ({})$ ", exit_status, cwd.to_str().unwrap());
    }

    if let Err(err) = stdout().flush() {
        eprintln!("{}", err);
    }
}

fn ctrlc_handler() {
    match waitpid(Pid::from_raw(-1), Some(WaitPidFlag::WNOHANG)) {
        Err(_) => display_prompt_ctrlc(1),
        Ok(WaitStatus::Exited(_, status)) => display_prompt_ctrlc(status),
        Ok(_) => println!(),
    }
}

fn main() {
    let mut sh: shell::Shell = shell::new_shell();
    let stdin = io::stdin();

    ctrlc::set_handler(ctrlc_handler).expect("Error setting Ctrl-C handler");

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
