use nix::unistd::{fork, ForkResult};

use crate::builtins::BUILTINS;
use crate::shell::Shell;

mod exec_child;
mod exec_parent;

use crate::handle_command::exec_child::exec_child;
use crate::handle_command::exec_parent::exec_parent;

fn split_command(line: &str) -> Vec<String> {
    line.split(' ')
        .map(|s| s.to_string())
        .filter(|s| s.trim() != "")
        .collect()
}

fn exec_command(sh: &mut Shell, command: &[String]) {
    match fork() {
        Ok(ForkResult::Parent { child }) => exec_parent(sh, child),
        Ok(ForkResult::Child) => exec_child(sh, command),
        Err(_) => println!("err"),
    }
}

pub fn handle_command(sh: &mut Shell, command: String) {
    let command = split_command(&command);

    if command.len() == 0 {
        return;
    }

    for builtin in BUILTINS {
        if command[0] == builtin.name {
            (builtin.func)(sh, &command);
        }
    }

    exec_command(sh, &command);
}
