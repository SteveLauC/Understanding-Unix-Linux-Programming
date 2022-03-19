use libc::c_int;
use nix::sys::signal::{signal, SigHandler, SIGINT, SIGQUIT};

use std::io::stdin;
use std::process::exit;
use std::thread::sleep;
use std::time::Duration;

extern "C" fn int_handler(signum: c_int) {
    println!("Recived signal {} .. waiting", signum);
    sleep(Duration::from_secs(2));
    println!("Leaving int_handler");
}

extern "C" fn quit_handler(signum: c_int) {
    println!("Recived signal {} .. waiting", signum);
    sleep(Duration::from_secs(2));
    println!("Leaving quit_handler");
}

fn main() {
    let mut buf: String = String::with_capacity(10);
    unsafe {
        let _ = signal(SIGINT, SigHandler::Handler(int_handler));
        let _ = signal(SIGQUIT, SigHandler::Handler(quit_handler));
    }
    let quit: String = String::from("quit\n");

    loop {
        stdin()
            .read_line(&mut buf)
            .expect("Can not read from stdin");
        println!("Debug: {}", buf);

        if buf == quit {
            exit(0);
        }
        buf.clear();
    }
}
