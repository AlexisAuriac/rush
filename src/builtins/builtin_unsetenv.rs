use crate::shell::Shell;

pub fn builtin_unsetenv(sh: &mut Shell, command: &[String]) {
    if command.len() == 1 {
        eprintln!("unsetenv: Too few arguments.");
        sh.exit_status = 1;
        return;
    }

    let (_, keys) = command.split_first().unwrap();

    for key in keys {
        sh.env.remove(key);
    }

    sh.exit_status = 0;
}
