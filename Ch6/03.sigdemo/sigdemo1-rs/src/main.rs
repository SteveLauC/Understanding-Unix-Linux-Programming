use std::thread::sleep;
use std::time::Duration;

use nix::sys::signal::{signal, SIGINT, SigHandler};
use libc::c_int;


extern "C" fn f(_signum: c_int) {
    println!("OUCH");
}

fn main() {
    unsafe{signal(SIGINT, SigHandler::Handler(f)).expect("can not get previous handler")};

    for _ in 0..5 {
        println!("hello");
        sleep(Duration::from_secs(1));
    }
}
