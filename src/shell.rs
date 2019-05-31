use isatty;
use std::collections::hash_map::HashMap;
use std::env;
use std::io::{stdout, Write};
use std::process::Command;

use crate::utility::split_no_empty;

#[derive(Debug)]
pub struct Shell {
    pub env: Vec<(String, String)>,
    pub exit_status: i32,
    pub tty: bool,
    pub stop: bool,
}

use nix::unistd::fork;
use nix::unistd::ForkResult;

impl Shell {
    pub fn new() -> Shell {
        Shell {
            env: env::vars().collect(),
            exit_status: 0,
            tty: isatty::stdin_isatty(),
            stop: false,
        }
    }

    pub fn display_prompt(self: &Shell) {
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

    pub fn handle_command(self: &mut Shell, line: String) {
        let command = split_no_empty(&line);

        if command.len() == 0 {
            return;
        }

        for builtin in crate::builtins::BUILTINS.iter() {
            if command[0] == builtin.name {
                (builtin.f)(self, &command);
                return;
            }
        }

        match fork() {
            Ok(ForkResult::Parent { child }) => {
                println!("parent of {}", child);
                match nix::sys::wait::waitpid(Option(child, {})) {
                    nix::sys::wait::WaitStatus::Exited(_, status) => println!("exited"),
                }
            }
            Ok(ForkResult::Child) => {
                println!("child with pid {}", nix::unistd::getpid());
                std::process::exit(1);
            }
            Err(err) => println!("{:?}", err),
        }

        // Command::new("ls")
        // .env_clear()
        // .env("PATH", "/bin")
        // .spawn()
        // .expect("piss off");

        // let filtered_env: HashMap<String, String> = env::vars()
        //     .filter(|&(ref k, _)| k == "TERM" || k == "TZ" || k == "LANG" || k == "PATH")
        //     .collect();

        // let filtered_env: HashMap<String, String> = HashMap::new();

        // Command::new(&command[0])
        // .env("PATH", "/bin")
        // .env_clear()
        // .envs(&self.env)
        // .envs(&filtered_env)
        // .spawn()
        // .expect(&format!("{}: Command not found.", command[0]));
        // if let Err(err) = Command::new(&command[0])
        //     .args(&["hello"])
        //     .env("PATH", "/bin")
        //     .spawn()
        // {
        //     println!("{:?}", err);
        // }
        println!("end command");
    }
}
