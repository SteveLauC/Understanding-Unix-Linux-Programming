use std::process::{Command, exit};
use std::os::unix::process::CommandExt;

use nix::unistd::{fork, ForkResult};
use nix::sys::signal::{signal, SigHandler, SIGINT, SIGQUIT};
use nix::sys::wait::{wait, WaitStatus};


pub fn execute(mut cmd: Command) -> Option<WaitStatus> {
    match unsafe{fork()} {
        Ok(ForkResult::Child) => {
            // enable default signal handling for SIGINT and SIGQUIT
            unsafe{
                let _ = signal(SIGINT, SigHandler::SigDfl);
                let _ = signal(SIGQUIT, SigHandler::SigDfl);
            }
            eprintln!("{}", cmd.exec());
            exit(-1);
        },
        Ok(ForkResult::Parent { child: _ }) => {
            return wait().ok(); 
        },
        Err(msg) => {
            eprintln!("fork() error: {:?}", msg);
            exit(-1);
        },
    }
}
