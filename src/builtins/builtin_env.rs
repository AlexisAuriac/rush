use crate::shell::Shell;

pub fn builtin_env(sh: &mut Shell, _command: &[&str]) {
    for (key, val) in sh.env.iter() {
        println!("{}={}", key, val);
    }

    sh.exit_status = 0;
}
