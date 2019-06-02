use std::collections::HashMap;
use std::env;
use std::ffi::CString;
use std::io::{stdout, Write};

use nix::unistd::fork;
use nix::unistd::ForkResult;

use crate::utility::split_no_empty;

#[derive(Debug)]
pub struct Shell {
    pub env: HashMap<String, String>,
    pub exit_status: i32,
    pub tty: bool,
    pub stop: bool,
}

impl Shell {
    pub fn new() -> Shell {
        Shell {
            env: env::vars().collect(),
            exit_status: 0,
            tty: isatty::stdin_isatty(),
            stop: false,
        }
    }

    pub fn display_prompt(self: &Shell) {
        let cwd = std::env::current_dir().unwrap();

        if !self.tty {
            return;
        }

        if let Some(dir) = cwd.file_name() {
            if let Some(dir) = dir.to_str() {
                print!("{} -> ({})$ ", self.exit_status, dir);
                if let Err(err) = stdout().flush() {
                    println!("{:?}", err);
                }
            }
        }
    }

    fn wait_for_child(self: &mut Shell, child_pid: nix::unistd::Pid) {
        match nix::sys::wait::waitpid(child_pid, Option::None) {
            Ok(nix::sys::wait::WaitStatus::Exited(_, status)) => {
                self.exit_status = status;
            }
            Ok(_) => println!("other"),
            Err(err) => println!("{:?}", err),
        }
    }

    fn child_exec_command(self: &mut Shell, command: &Vec<String>) {
        let args: Vec<CString> = command
            .iter()
            .map(|s| CString::new(s.clone()).unwrap())
            .collect();
        let args = &args[..];

        let cenv: Vec<CString> = self
            .env
            .iter()
            .map(|(key, val)| CString::new(format!("{}={}", key, val)).unwrap())
            .collect();
        let cenv = &cenv[..];

        let path = CString::new(command[0].clone()).unwrap();

        match nix::unistd::execve(&path, &args, &cenv) {
            Ok(_) => unimplemented!(),
            Err(err) => println!("{:?}", err),
        };
    }

    fn exec_command(self: &mut Shell, command: &Vec<String>) {
        match fork() {
            Ok(ForkResult::Parent { child }) => self.wait_for_child(child),
            Ok(ForkResult::Child) => {
                self.child_exec_command(&command);
                std::process::exit(1);
            }
            Err(err) => println!("{:?}", err),
        }
    }

    pub fn handle_command(self: &mut Shell, line: String) {
        let command = split_no_empty(&line);

        if command.len() == 0 {
            return;
        }

        for builtin in crate::builtins::BUILTINS.iter() {
            if command[0] == builtin.name {
                (builtin.f)(self, &command);
                return;
            }
        }

        self.exec_command(&command);
    }
}
