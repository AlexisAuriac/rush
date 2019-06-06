use std::ffi::CString;
use std::option::Option;

use nix::unistd::fork;
use nix::unistd::ForkResult;

use crate::shell::Shell;

fn is_exec(path: &str) -> Result<(), String> {
    if let Err(err) = nix::unistd::access(path, nix::unistd::AccessFlags::X_OK) {
        return Err(String::from(err.as_errno().unwrap().desc()));
    }

    return Ok(());
}

fn wait_for_child(sh: &mut Shell, child_pid: nix::unistd::Pid) {
    match nix::sys::wait::waitpid(child_pid, Option::None) {
        Ok(nix::sys::wait::WaitStatus::Exited(_, status)) => {
            sh.exit_status = status;
        }
        Ok(_) => println!("other"),
        Err(err) => eprintln!("{}", err),
    }
}

fn get_command_path(sh: &Shell, command: &String) -> Result<CString, String> {
    if command.contains('/') {
        return match is_exec(command) {
            Ok(_) => Ok(CString::new(command.clone()).unwrap()),
            Err(err) => Err(err),
        };
    }

    if !sh.env.contains_key("PATH") {
        return Err(format!("{}: Command not found.", command).to_string());
    }

    let paths = sh.env["PATH"].split(':');

    for p in paths {
        let full_command = format!("{}/{}", p, command);

        if let Ok(_) = is_exec(&full_command) {
            return Ok(CString::new(full_command.as_str()).unwrap());
        }
    }

    return Err(format!("{}: Command not found.", command).to_string());
}

fn child_exec_command(sh: &mut Shell, command: &Vec<String>) {
    let args: Vec<CString> = command
        .iter()
        .map(|s| CString::new(s.clone()).unwrap())
        .collect();
    let args = &args[..];

    let cenv: Vec<CString> = sh
        .env
        .iter()
        .map(|(key, val)| CString::new(format!("{}={}", key, val)).unwrap())
        .collect();
    let cenv = &cenv[..];

    match get_command_path(sh, &command[0]) {
        Ok(path) => {
            if let Err(err) = nix::unistd::execve(&path, &args, &cenv) {
                eprintln!("{}", err.as_errno().unwrap().desc());
            }
        }
        Err(err) => eprintln!("{}", err),
    };
    std::process::exit(1);
}

pub fn exec_command(sh: &mut Shell, command: &Vec<String>) {
    match fork() {
        Ok(ForkResult::Parent { child }) => wait_for_child(sh, child),
        Ok(ForkResult::Child) => child_exec_command(sh, &command),
        Err(err) => eprintln!("{}", err),
    }
}
