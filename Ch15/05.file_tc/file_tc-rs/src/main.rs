use nix::fcntl::{flock, FlockArg};
use std::env::args;
use std::fs::File;
use std::io::Read;
use std::os::unix::io::{AsRawFd, RawFd};
use std::process::exit;
use std::str;

fn main() {
    let av: Vec<String> = args().collect();
    if av.len() != 2 {
        eprintln!("usage: ./file_ts-rs filename");
        exit(1);
    }

    let mut time_file: File = File::open(av[1].as_str()).unwrap();
    let mut buf: [u8; 30] = [0; 30];

    lock_operation(time_file.as_raw_fd(), FlockArg::LockShared);
    if let Ok(n) = time_file.read(&mut buf) {
        let res: &str = std::str::from_utf8(&buf[..n]).unwrap();
        println!("{}", res);
    }
    lock_operation(time_file.as_raw_fd(), FlockArg::Unlock);
}

fn lock_operation(fd: RawFd, op: FlockArg) {
    flock(fd, op).unwrap();
}
