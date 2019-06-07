use crate::shell::Shell;

fn cd_home(sh: &mut Shell) {
    if !sh.env.contains_key("HOME") {
        eprintln!("cd: Can't change to home directory.");
        sh.exit_status = 1;
        return;
    }

    let home = &sh.env["HOME"];
    let home = std::path::Path::new(&home);

    if let Err(_) = std::env::set_current_dir(home) {
        eprintln!("cd: Can't change to home directory.");
        sh.exit_status = 1;
    } else {
        sh.exit_status = 0;
    }
}

fn cd_back(sh: &mut Shell) {
    if !sh.env.contains_key("OLDPWD") {
        eprintln!(": No such file or directory.");
        sh.exit_status = 1;
        return;
    }

    let oldpwd_str = &sh.env["OLDPWD"];
    let oldpwd = std::path::Path::new(&oldpwd_str);

    if let Err(_) = std::env::set_current_dir(oldpwd) {
        eprintln!("{}: No such file or directory.\n", oldpwd_str);
        sh.exit_status = 1;
    } else {
        sh.exit_status = 0;
    }
}

fn cd_path(sh: &mut Shell, path_str: &String) {
    let path = std::path::Path::new(&path_str);

    if !path.exists() {
        eprintln!("{}: No such file or directory.", path_str);
        sh.exit_status = 1;
        return;
    } else if !path.is_dir() {
        eprintln!("{}: Not a directory.", path_str);
        sh.exit_status = 1;
        return;
    }

    if let Err(_) = std::env::set_current_dir(path) {
        eprintln!("{}: No such file or directory.\n", path_str);
        sh.exit_status = 1;
    } else {
        sh.exit_status = 0;
    }
}

pub fn builtin_cd(sh: &mut Shell, command: &[String]) {
    // let cwd = std::env::current_dir().unwrap();

    if command.len() == 1 {
        cd_home(sh);
        return;
    } else if command.len() > 2 {
        eprintln!("cd: Too many arguments.");
        sh.exit_status = 1;
        return;
    } else if command[1] == "-" {
        cd_back(sh);
    } else {
        cd_path(sh, &command[1]);
    }

    // update_pwd(oldpwd);
}
