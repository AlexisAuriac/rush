use std::ffi::CString;

use nix::unistd::{access, execve, AccessFlags};

use crate::shell::Shell;

fn is_exec(path: &str) -> Result<(), String> {
    if let Err(err) = access(path, AccessFlags::X_OK) {
        return Err(err.as_errno().unwrap().desc().to_string());
    }

    return Ok(());
}

fn get_command_path(sh: &mut Shell, command: &String) -> Result<CString, String> {
    if command.contains('/') {
        return Ok(CString::new(command.clone()).unwrap());
    }

    if !sh.env.contains_key("PATH") {
        return Err(format!("{}: Command not found.", command));
    }

    let paths = sh.env["PATH"].split(':');

    for p in paths {
        let full_command = format!("{}/{}", p, command);

        if let Ok(_) = is_exec(&full_command) {
            return Ok(CString::new(full_command).unwrap());
        }
    }

    return Err(format!("{}: Command not found.", command));
}

pub fn exec_child(sh: &mut Shell, command: &[String]) {
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
            if let Err(err) = execve(&path, args, cenv) {
                eprintln!("{}", err);
            }
        }
        Err(err) => eprintln!("{}", err),
    }

    std::process::exit(0);
}
