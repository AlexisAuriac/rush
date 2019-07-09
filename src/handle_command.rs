use nix::unistd::{fork, ForkResult};

use crate::builtins::BUILTINS;
use crate::shell::Shell;

mod exec_child;
mod exec_parent;

use exec_child::exec_child;
use exec_parent::exec_parent;

fn split_command(command: &str) -> Vec<&str> {
    command.split_whitespace().collect()
}

fn exec_command(sh: &mut Shell, command: &[&str]) {
    match fork() {
        Ok(ForkResult::Parent { child }) => exec_parent(sh, child),
        Ok(ForkResult::Child) => exec_child(sh, command),
        Err(err) => eprintln!("{}", err),
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
            return;
        }
    }

    exec_command(sh, &command);
}
