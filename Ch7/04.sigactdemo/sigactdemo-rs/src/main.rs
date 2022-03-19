/*
    for the weird bug of sigactdemo.c(see sigactdemo.c line 5), I just use `read_line` in the rust implementation instead of
    simulating `fgets()`'s behaviour.
*/
use libc::c_int;
use nix::sys::signal::{sigaction, SaFlags, SigAction, SigHandler, SigSet, SIGINT, SIGQUIT};
use std::io::stdin;
use std::process::exit;
use std::thread::sleep;
use std::time::Duration;

extern "C" fn int_handler(signum: c_int) {
    println!("Called with signal: {}", signum);
    sleep(Duration::from_secs(
        u64::try_from(signum).expect("Can not safely convert signum from i32 to u64 "),
    ));
    println!("done handling signal {}", signum);
}

fn main() {
    let mut x: String = String::with_capacity(10);
    let flag: SaFlags = SaFlags::SA_RESETHAND;
    let mut blocked: SigSet = SigSet::empty();
    blocked.add(SIGQUIT);
    let new_handler: SigAction = SigAction::new(SigHandler::Handler(int_handler), flag, blocked);

    if let Err(msg) = unsafe { sigaction(SIGINT, &new_handler) } {
        eprintln!("sigaction() failed for {}", msg);
        exit(1);
    } else {
        loop {
            stdin().read_line(&mut x).expect("Can not read from stdin");
            println!("input: {}", x);
            x.clear();
        }
    }
}
