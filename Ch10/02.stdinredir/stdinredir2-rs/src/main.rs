use nix::unistd::{close, dup};
use nix::fcntl::{open, OFlag};
use nix::sys::stat::Mode;
use std::os::unix::io::RawFd;
use std::io::stdin;

fn main() {
    let fd: RawFd = open("/etc/passwd", OFlag::O_RDONLY, Mode::empty()).unwrap();
    close(0).unwrap();
    dup(fd).unwrap();
    close(fd).unwrap();
    
    let mut buf: String = String::with_capacity(100);
    stdin().read_line(&mut buf).unwrap();
    println!("{}", buf);
}
