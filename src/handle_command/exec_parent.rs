use nix::sys::signal::Signal;
use nix::sys::wait::{waitpid, WaitStatus};
use nix::unistd::Pid;

use crate::shell::Shell;

struct ErrorSignal {
    signal: Signal,
    value: i32,
    mssg: &'static str,
}

static ERROR_SIGNALS: &'static [&'static ErrorSignal] = &[
    &ErrorSignal {
        signal: Signal::SIGFPE,
        value: 136,
        mssg: "Floating exception",
    },
    &ErrorSignal {
        signal: Signal::SIGSEGV,
        value: 139,
        mssg: "Segmentation fault",
    },
];

fn print_err_sig(sh: &mut Shell, sig: Signal, core_dumped: bool) {
    for err_sig in ERROR_SIGNALS {
        if err_sig.signal == sig {
            let core_dumped = if core_dumped { "(core dumped)" } else { "" };

            eprintln!("{} {}", err_sig.mssg, core_dumped);
            sh.exit_status = err_sig.value;
        }
    }
}

pub fn exec_parent(sh: &mut Shell, child_pid: Pid) {
    match waitpid(child_pid, Option::None) {
        Ok(WaitStatus::Exited(_, status)) => sh.exit_status = status,
        Ok(WaitStatus::Signaled(_, sig, core_dumped)) => print_err_sig(sh, sig, core_dumped),
        Ok(_) => unimplemented!(),
        Err(err) => eprintln!("{}", err),
    }
}
