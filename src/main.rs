use isatty;
use std::io::stdout;
use std::io::Write;
use std::io::{self, BufRead};
use std::process;

#[derive(Debug)]
struct Shell {
    // env: Vec<(String, String)>,
    exit_status: i32,
    tty: bool,
    stop: bool,
}

fn builtin_exit(shell: &mut Shell, command: &[String]) {
    if command.len() > 2 {
        println!("exit: Expression Syntax.");
    }

    if command.len() == 2 {
        match command[1].parse::<u64>() {
            Ok(n) => shell.exit_status = n as i32,
            Err(_) => {
                shell.exit_status = 1;
                println!("exit: Expression Syntax.");
                return;
            }
        }
    }

    shell.stop = true;
}

struct BuiltinFunction {
    f: fn(&mut Shell, &[String]),
    name: &'static str,
}

static BUILTINS: &'static [&'static BuiltinFunction] = &[&BuiltinFunction {
    f: builtin_exit,
    name: "exit",
}];

impl Shell {
    fn new() -> Shell {
        Shell {
            // env: Vec::new(),
            exit_status: 0,
            tty: isatty::stdin_isatty(),
            stop: false,
        }
    }

    fn display_prompt(self: &Shell) {
        let cwd = std::env::current_dir().unwrap();

        if !self.tty {
            return;
        }

        if let Some(dir) = cwd.file_name() {
            if let Some(dir) = dir.to_str() {
                print!("{} -> ({})$ ", self.exit_status, dir);
                if let Err(err) = stdout().flush() {
                    println!("{:?}", err);
                }
            }
        }
    }

    fn handle_command(self: &mut Shell, line: String) {
        let command = split_command(&line);

        if command.len() == 0 {
            return;
        }

        for builtin in BUILTINS.iter() {
            if command[0] == builtin.name {
                (builtin.f)(self, &command);
                return;
            }
        }
    }
}

fn split_command(line: &String) -> Vec<String> {
    line.split(' ')
        .map(|s| s.to_string())
        .filter(|s| s.trim() != "")
        .collect()
}

fn main() {
    let mut shell: Shell = Shell::new();

    let stdin = io::stdin();

    shell.display_prompt();
    for line in stdin.lock().lines() {
        shell.display_prompt();
        shell.handle_command(line.unwrap());
        if shell.stop {
            process::exit(shell.exit_status);
        }
    }
}
