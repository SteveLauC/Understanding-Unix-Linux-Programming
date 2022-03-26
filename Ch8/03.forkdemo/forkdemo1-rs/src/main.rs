use nix::unistd::{fork, ForkResult};
use nix::libc::pid_t;

use std::process;

fn main() {
    let mut ret_from_fork: pid_t = 0;
    println!("Before: my pid is {}", process::id());

    match unsafe { fork() } {
        Ok(ForkResult::Parent { child }) => {
            ret_from_fork = child.as_raw();
        },
        Err(msg) => {
            eprintln!("Can not create process: {}", msg);
        },
        _ => (),
    }

    println!("After: my pid is {}, fork() said {}", process::id(), ret_from_fork);
}
