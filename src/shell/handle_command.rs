use std::ffi::CString;
use std::option::Option;

use nix::unistd::fork;
use nix::unistd::ForkResult;

use crate::shell::Shell;
use crate::utility::split_no_empty;

fn is_exec(path: &str) -> Result<(), String> {
    if let Err(err) = nix::unistd::access(path, nix::unistd::AccessFlags::X_OK) {
        return Err(String::from(err.as_errno().unwrap().desc()));
    }

    return Ok(());
}

impl Shell {
    fn wait_for_child(self: &mut Shell, child_pid: nix::unistd::Pid) {
        match nix::sys::wait::waitpid(child_pid, Option::None) {
            Ok(nix::sys::wait::WaitStatus::Exited(_, status)) => {
                self.exit_status = status;
            }
            Ok(_) => println!("other"),
            Err(err) => println!("{:?}", err),
        }
    }

    fn get_command_path(self: &Shell, command: &String) -> Result<CString, String> {
        if command.contains('/') {
            return match is_exec(command) {
                Ok(_) => Ok(CString::new(command.clone()).unwrap()),
                Err(err) => Err(err),
            };
        }

        let paths = self.env["PATH"].split(':');

        for p in paths {
            let full_command = format!("{}/{}", p, command);

            if let Ok(_) = is_exec(&full_command) {
                return Ok(CString::new(full_command.as_str()).unwrap());
            }
        }

        return Err(format!("{}: Command not found.", command).to_string());
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

        match self.get_command_path(&command[0]) {
            Ok(path) => {
                if let Err(err) = nix::unistd::execve(&path, &args, &cenv) {
                    println!("{}", err.as_errno().unwrap().desc());
                }
            }
            Err(err) => println!("{}", err),
        };
        std::process::exit(1);
    }

    fn exec_command(self: &mut Shell, command: &Vec<String>) {
        match fork() {
            Ok(ForkResult::Parent { child }) => self.wait_for_child(child),
            Ok(ForkResult::Child) => self.child_exec_command(&command),
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
