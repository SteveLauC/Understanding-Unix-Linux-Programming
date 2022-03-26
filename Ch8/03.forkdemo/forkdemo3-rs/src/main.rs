use nix::unistd::{fork, ForkResult};
use std::process;

fn main() {
    println!("Before: my pid = {}", process::id());

    match unsafe{fork()} {
        Ok(ForkResult::Parent { child: _}) => {
            println!("I am the parent process. my pid = {}", process::id());
        },
        Ok(ForkResult::Child) => {
            println!("I am the child process. my pid = {}", process::id());
        },
        Err(msg) => {
            eprintln!("fork(): {}", msg);
        }
    }
}
