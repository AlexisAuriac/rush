use regex::Regex;

use crate::builtins::builtin_env::builtin_env;
use crate::shell::Shell;

fn error_setenv(command: &[String]) -> bool {
    if command.len() > 3 {
        eprintln!("setenv: Too many arguments.");
        return true;
    }

    lazy_static! {
        static ref RLETTER: Regex = Regex::new("^[a-zA-Z].*$").unwrap();
        static ref RKEY: Regex = Regex::new("^[a-zA-Z]\\w*$").unwrap();
    }

    let key = &command[1];

    if !RLETTER.is_match(&key) {
        eprintln!("setenv: Variable name must begin with a letter.");
        return true;
    } else if !RKEY.is_match(&key) {
        eprintln!("setenv: Variable name must contain alphanumeric characters.");
        return true;
    }

    return false;
}

pub fn builtin_setenv(sh: &mut Shell, command: &[String]) {
    if command.len() == 1 {
        builtin_env(sh, command);
        return;
    } else if error_setenv(&command) {
        sh.exit_status = 1;
        return;
    }

    if command.len() == 2 {
        sh.env.insert(command[1].clone(), "".to_string());
    } else {
        sh.env.insert(command[1].clone(), command[2].clone());
    }

    sh.exit_status = 0;
}
