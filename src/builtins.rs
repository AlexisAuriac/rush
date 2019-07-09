use crate::shell::Shell;

mod builtin_cd;
mod builtin_env;
mod builtin_exit;
mod builtin_setenv;
mod builtin_unsetenv;

pub struct Builtin {
    pub name: &'static str,
    pub func: fn(&mut Shell, &[&str]),
}

pub static BUILTINS: &'static [&'static Builtin] = &[
    &Builtin {
        name: "cd",
        func: builtin_cd::builtin_cd,
    },
    &Builtin {
        name: "env",
        func: builtin_env::builtin_env,
    },
    &Builtin {
        name: "exit",
        func: builtin_exit::builtin_exit,
    },
    &Builtin {
        name: "setenv",
        func: builtin_setenv::builtin_setenv,
    },
    &Builtin {
        name: "unsetenv",
        func: builtin_unsetenv::builtin_unsetenv,
    },
];
