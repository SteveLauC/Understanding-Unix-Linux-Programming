use std::env;
use std::process;

extern crate libc;
use libc::{termios, tcgetattr, tcsetattr, ECHO, TCSANOW};

fn main() {
    let mut buf: termios = termios{
        c_iflag: 0,
        c_oflag: 0,
        c_cflag: 0,
        c_lflag: 0,
        c_line: 0,
        c_cc: [0;32],
        c_ispeed: 0,
        c_ospeed: 0,
    };

    let av: Vec<String> = env::args().collect();
    if av.len()==1 {
        eprintln!("usage: setecho [y|n]");
        process::exit(1);
    }


    if -1 == unsafe{tcgetattr(0, &mut buf as *mut termios)} {
        eprintln!("Can not fetch attribute for Stdin");
        std::process::exit(1);
    }

    if av[1].starts_with('y') {
        buf.c_lflag |= ECHO;
    }else{
        buf.c_lflag &= !ECHO;
    }
    
    if -1 == unsafe{tcsetattr(0, TCSANOW, &buf)} {
        eprintln!("Can not send modification back");
        process::exit(1);
    }
}
