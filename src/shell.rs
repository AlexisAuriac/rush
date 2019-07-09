use std::collections::HashMap;
use std::env;
use std::io::{stdout, Write};

pub struct Shell {
    pub tty: bool,
    pub env: HashMap<String, String>,
    pub exit_status: i32,
    pub stop: bool,
}

impl Shell {
    pub fn new() -> Shell {
        Shell {
            tty: atty::is(atty::Stream::Stdin),
            env: env::vars().collect(),
            exit_status: 0,
            stop: false,
        }
    }
}

fn print_prompt(status: i32, cwd: &str) {
    print!("{} -> ({})$ ", status, cwd);

    if let Err(err) = stdout().flush() {
        eprintln!("{}", err);
    }
}

pub fn display_prompt(sh: &Shell) {
    if !sh.tty {
        return;
    }

    let cwd = std::env::current_dir().unwrap();

    if let Some(dir) = cwd.file_name() {
        print_prompt(sh.exit_status, dir.to_str().unwrap());
    } else {
        print_prompt(sh.exit_status, cwd.to_str().unwrap());
    }
}
