use crate::shell::Shell;

pub fn builtin_env(sh: &mut Shell, command: &[String]) {
    for (key, value) in sh.env.iter() {
        println!("{}={}", key, value);
    }

    sh.exit_status = 0;
}
