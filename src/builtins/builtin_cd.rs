use std::env::current_dir;
use std::path::{Path, PathBuf};

use crate::shell::Shell;

fn get_dest_home(sh: &Shell) -> Result<(PathBuf, String), String> {
    if !sh.env.contains_key("HOME") {
        return Err("cd: Can't change to home directory.".to_string());
    }

    let home = &sh.env["HOME"];

    return Ok((
        Path::new(home).to_path_buf(),
        "cd: Can't change to home directory.".to_string(),
    ));
}

fn get_dest_path(path_str: &String) -> Result<(PathBuf, String), String> {
    let path = Path::new(&path_str);

    if !path.exists() {
        return Err(format!("{}: No such file or directory.", path_str));
    } else if !path.is_dir() {
        return Err(format!("{}: Not a directory.", path_str));
    }

    return Ok((
        path.to_path_buf(),
        format!("{}: Not a directory.", path_str),
    ));
}

fn get_dest_back(sh: &Shell) -> Result<(PathBuf, String), String> {
    if !sh.env.contains_key("OLDPWD") {
        return Err(": No such file or directory.".to_string());
    }

    let oldpwd = &sh.env["OLDPWD"];

    return Ok((
        Path::new(oldpwd).to_path_buf(),
        format!("{}: No such file or directory.", oldpwd),
    ));
}

fn get_dest(sh: &Shell, command: &[String]) -> Result<(PathBuf, String), String> {
    return if command.len() > 2 {
        Err("cd: Too many arguments.".to_string())
    } else if command.len() == 1 {
        get_dest_home(sh)
    } else if command[1] == "-" {
        get_dest_back(sh)
    } else {
        get_dest_path(&command[1])
    };
}

fn update_pwd(sh: &mut Shell, new: &PathBuf, old: &PathBuf) {
    let new = new.to_str().unwrap().to_string();
    let old = old.to_str().unwrap().to_string();

    sh.env.insert("PWD".to_string(), new);
    sh.env.insert("OLDPWD".to_string(), old);
}

fn change_dir(sh: &mut Shell, dest: &PathBuf, err_msg: String) {
    let oldpwd = current_dir().unwrap();

    if let Err(_) = std::env::set_current_dir(dest) {
        eprintln!("{}", err_msg);
        sh.exit_status = 1;
        return;
    }

    update_pwd(sh, dest, &oldpwd);
}

pub fn builtin_cd(sh: &mut Shell, command: &[String]) {
    match get_dest(sh, &command) {
        Ok((dest, err)) => change_dir(sh, &dest, err),
        Err(err) => {
            eprintln!("{}", err);
            sh.exit_status = 1;
        }
    }
}
