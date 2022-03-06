use std::env;
use std::process;

use termios::os::linux::{ECHO, TCSANOW};
use termios::Termios;

fn main() {
    let av: Vec<String> = env::args().collect();
    if av.len() == 1 {
        eprintln!("usage: setecho [y|n]");
        process::exit(1);
    }

    let mut buf: Termios = match Termios::from_fd(0) {
        Ok(s) => s,
        Err(msg) => {
            eprintln!("can not fetch terminal driver configuration: {}", msg);
            process::exit(1);
        }
    };

    if av[1].starts_with('y') {
        buf.c_lflag |= ECHO;
    } else {
        buf.c_lflag &= !ECHO;
    }

    if termios::tcsetattr(0, TCSANOW, &buf).is_err() {
        eprintln!("cannot send the modification back to the kernel");
        process::exit(1);
    }
}
