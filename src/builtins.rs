use crate::shell::Shell;

mod builtin_cd;
mod builtin_env;
mod builtin_exit;
mod builtin_setenv;
mod builtin_unsetenv;

pub struct BuiltinFunction {
    pub func: fn(&mut Shell, &[&str]),
    pub name: &'static str,
}

pub static BUILTINS: &'static [&'static BuiltinFunction] = &[
    &BuiltinFunction {
        func: builtin_exit::builtin_exit,
        name: "exit",
    },
    &BuiltinFunction {
        func: builtin_env::builtin_env,
        name: "env",
    },
    &BuiltinFunction {
        func: builtin_setenv::builtin_setenv,
        name: "setenv",
    },
    &BuiltinFunction {
        func: builtin_unsetenv::builtin_unsetenv,
        name: "unsetenv",
    },
    &BuiltinFunction {
        func: builtin_cd::builtin_cd,
        name: "cd",
    },
];
