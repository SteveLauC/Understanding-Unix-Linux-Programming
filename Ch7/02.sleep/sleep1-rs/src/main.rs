use nix::sys::signal::{signal, SIGALRM, SigHandler};
use nix::unistd::alarm::set;
use nix::unistd::pause;
use libc::c_int;

extern "C" fn wakeup(_signum: c_int) {
    println!("Alarm recieved from kernel");
}


fn main() {
    unsafe{signal(SIGALRM, SigHandler::Handler(wakeup)).expect("can not get previous SIGALRM handler");}
    println!("About to sleep for 4 seconds");
    let _ = set(4);   // ignore the return value of `set`
    pause();
    println!("Morning so soon");
}
