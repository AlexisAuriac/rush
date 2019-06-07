use crate::shell::Shell;

fn cd_home(sh: &mut Shell) {
    if !sh.env.contains_key("HOME") {
        eprintln!("cd: Can't change to home directory.");
        sh.exit_status = 1;
        return;
    }

    let home_str = &sh.env["HOME"];
    let home = std::path::Path::new(&home_str);

    if let Err(err) = std::env::set_current_dir(home) {
        eprintln!("{}", err);
        sh.exit_status = 1;
    } else {
        // update_pwd(home_str);
        sh.exit_status = 0;
    }
}

pub fn builtin_cd(sh: &mut Shell, command: &[String]) {
    if command.len() == 1 {
        cd_home(sh);
        return;
    }
}
