use nix::libc::{pid_t, wait};
use nix::unistd::{fork, ForkResult};
use std::process;
use std::thread;
use std::time::Duration;

const DELAY: u64 = 10;

fn main() {
    println!("before: mypid is {}\n", process::id());

    match unsafe { fork() } {
        Ok(ForkResult::Parent { child }) => parent_code(child.as_raw()),
        Ok(ForkResult::Child) => child_code(DELAY),
        Err(msg) => eprintln!("Can not fork(): {}", msg),
    }
}

fn child_code(delay: u64) {
    println!(
        "child {} here. will sleep for {} seconds",
        process::id(),
        delay
    );
    thread::sleep(Duration::from_secs(delay));
    println!("child done. about to exit");
    process::exit(17);
}

fn parent_code(child_pid: pid_t) {
    // for the reason that nix has done some abstraction based on the original syscall
    // we use libc instead here.

    let mut child_status = 0;
    let wait_rv: pid_t = unsafe{wait(&mut child_status)};
    
    println!("done waiting for {}. Wait returned: {}\n", child_pid, wait_rv);
    let high_8: i32 = child_status >> 8;
    let low_7: i32 = child_status & 0x7f;
    let bit_7: i32 = (child_status & 0x80) >> 7;

    println!("status: exit = {}, sig = {}, core = {}", high_8, low_7, bit_7);
}
