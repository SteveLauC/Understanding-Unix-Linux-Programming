use nix::fcntl::{open, OFlag};
use nix::sys::stat::Mode;
use nix::unistd::{close, dup2};
use std::io::stdin;
use std::os::unix::io::RawFd;

fn main() {
    let old_fd: RawFd = open("/etc/passwd", OFlag::O_RDONLY, Mode::empty()).unwrap();
    assert_eq!(dup2(old_fd, 0), Ok(0));
    close(old_fd).unwrap();

    let mut buf: String = String::with_capacity(100);
    stdin().read_line(&mut buf).unwrap();
    println!("{}", buf);
}
