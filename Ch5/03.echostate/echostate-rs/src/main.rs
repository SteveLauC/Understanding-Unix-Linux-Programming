extern crate libc;
use libc::{termios, tcgetattr, ECHO};

fn main() {
    let mut buf = termios{
        c_iflag: 0,
        c_oflag: 0,
        c_cflag: 0,
        c_lflag: 0,
        c_line: 0,
        c_cc: [0;32],
        c_ispeed: 0,
        c_ospeed: 0,
    };

    if -1 == unsafe{tcgetattr(0, &mut buf as *mut termios)} {
        eprintln!("Can not fetch attribute for Stdin");
        std::process::exit(1);
    }

    if buf.c_lflag & ECHO != 0 {
        println!("echo is on, since lits bit is 1");
    }else {
        println!("echo is OFF, since its bit is 0");
    }
}
