//! execute.rs: executes the command

use crate::process::ProcessRes;
use std::os::unix::process::CommandExt;
use std::process::{exit, Command};

use nix::sys::signal::{signal, SigHandler, SIGINT, SIGQUIT};
use nix::sys::wait::wait;
use nix::unistd::{fork, ForkResult};

/// purpose: execute the command
///
/// arguments:
///     * `cmd`: command to be executed
///   
/// return: command executation result
pub fn execute(mut cmd: Command) -> ProcessRes {
    match unsafe { fork() } {
        Ok(ForkResult::Child) => {
            // enable default signal handling for SIGINT and SIGQUIT
            unsafe {
                let _ = signal(SIGINT, SigHandler::SigDfl);
                let _ = signal(SIGQUIT, SigHandler::SigDfl);
            }
            eprintln!("{}", cmd.exec());
            exit(-1);
        }
        Ok(ForkResult::Parent { child: _ }) => match wait() {
            Ok(_) => return ProcessRes::Success,
            Err(_) => return ProcessRes::Failure,
        },
        Err(msg) => {
            eprintln!("fork() error: {:?}", msg);
            exit(-1);
        }
    }
}
