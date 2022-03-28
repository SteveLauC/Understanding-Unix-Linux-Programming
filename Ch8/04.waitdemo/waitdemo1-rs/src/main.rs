use nix::libc::pid_t;
use nix::sys::wait::{wait, WaitStatus};
use nix::unistd::{fork, ForkResult};
use std::process;
use std::thread;
use std::time::Duration;

const DELAY: u64 = 2;

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
    // let (wait_rv, _): (pid_t, _) = wait().expect("can not wait for the child");
    if let WaitStatus::Exited(wait_rv, _) = wait().expect("can not wait for the child") {
        println!(
            "done waiting for {}. wait() returned: {}",
            child_pid, wait_rv
        );
    }
}
