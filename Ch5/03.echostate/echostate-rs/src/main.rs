use std::process;
use termios::os::linux::ECHO;
use termios::Termios;

fn main() {
    let buf: Termios = match Termios::from_fd(0) {
        Ok(s) => s,
        Err(msg) => {
            eprintln!("Can not fetch the configuration: {}", msg);
            process::exit(1);
        }
    };

    if buf.c_lflag & ECHO != 0 {
        println!("echo is on since its bit is 1");
    } else {
        println!("echo is OFF since its bit is 0");
    }
}
