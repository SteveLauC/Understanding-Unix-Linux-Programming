use nix::unistd::{fork, ForkResult};
use std::process;

fn main() {
    println!("Before: my pid is {}", process::id());

    match unsafe { fork() } {
        Ok(ForkResult::Parent { child }) => {
            println!("After: my pid is {}, fork() said {}", process::id(), child);
        }

        Ok(ForkResult::Child) => {
            println!("After my pid is {}, fork() said {}", process::id(), 0);
        }
        Err(msg) => {
            eprintln!("Can not create process: {}", msg);
        }
    }
}
