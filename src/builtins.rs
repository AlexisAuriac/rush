use crate::shell::Shell;

mod builtin_env;
mod builtin_exit;

pub struct BuiltinFunction {
    pub f: fn(&mut Shell, &[String]),
    pub name: &'static str,
}

pub static BUILTINS: &'static [&'static BuiltinFunction] = &[
    &BuiltinFunction {
        f: builtin_exit::builtin_exit,
        name: "exit",
    },
    &BuiltinFunction {
        f: builtin_env::builtin_env,
        name: "env",
    },
];
