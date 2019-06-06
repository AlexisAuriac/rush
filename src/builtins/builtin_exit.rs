use crate::shell::Shell;

pub fn builtin_exit(sh: &mut Shell, command: &[String]) {
    if command.len() > 2 {
        eprintln!("exit: Expression Syntax.");
        sh.exit_status = 1;
        return;
    }

    if command.len() == 2 {
        match command[1].parse::<u64>() {
            Ok(n) => sh.exit_status = n as i32,
            Err(_) => {
                sh.exit_status = 1;
                eprintln!("exit: Expression Syntax.");
                return;
            }
        }
    }

    sh.stop = true;
}
