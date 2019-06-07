use crate::shell::Shell;

fn cd_home(sh: &mut Shell) -> bool {
    if !sh.env.contains_key("HOME") {
        eprintln!("cd: Can't change to home directory.");
        return false;
    }

    let home = &sh.env["HOME"];
    let home = std::path::Path::new(&home);

    if let Err(_) = std::env::set_current_dir(home) {
        eprintln!("cd: Can't change to home directory.");
        return false;
    } else {
        return true;
    }
}

fn cd_back(sh: &mut Shell) -> bool {
    if !sh.env.contains_key("OLDPWD") {
        eprintln!(": No such file or directory.");
        return false;
    }

    let oldpwd_str = &sh.env["OLDPWD"];
    let oldpwd = std::path::Path::new(&oldpwd_str);

    if let Err(_) = std::env::set_current_dir(oldpwd) {
        eprintln!("{}: No such file or directory.", oldpwd_str);
        return false;
    } else {
        return true;
    }
}

fn cd_path(path_str: &str) -> bool {
    let path = std::path::Path::new(&path_str);

    if !path.exists() {
        eprintln!("{}: No such file or directory.", path_str);
        return false;
    } else if !path.is_dir() {
        eprintln!("{}: Not a directory.", path_str);
        return false;
    }

    if let Err(_) = std::env::set_current_dir(path) {
        eprintln!("{}: No such file or directory.", path_str);
        return false;
    } else {
        return true;
    }
}

fn update_pwd(sh: &mut Shell, oldcwd: &std::path::PathBuf) {
    let cwd = std::env::current_dir().unwrap();
    let cwd = cwd.to_str().unwrap().to_string();

    let oldcwd = oldcwd.to_str().unwrap().to_string();

    sh.env.insert(String::from("PWD"), cwd);
    sh.env.insert(String::from("OLDPWD"), oldcwd);
}

pub fn builtin_cd(sh: &mut Shell, command: &[String]) {
    let cwd = std::env::current_dir().unwrap();
    let successful: bool;

    if command.len() == 1 {
        successful = cd_home(sh);
    } else if command.len() > 2 {
        eprintln!("cd: Too many arguments.");
        successful = false;
    } else if command[1] == "-" {
        successful = cd_back(sh);
    } else {
        successful = cd_path(&command[1]);
    }

    if successful {
        update_pwd(sh, &cwd);
        sh.exit_status = 0;
    } else {
        sh.exit_status = 1;
    }
}
