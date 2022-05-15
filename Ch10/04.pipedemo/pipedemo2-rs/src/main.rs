// Rust code snippet

use nix::unistd::{fork, pipe, ForkResult}; // needs extern crate `nix`
use std::fs::File;
use std::io::{stdout, Read, Write};
use std::os::unix::io::{FromRawFd, RawFd};
use std::thread::sleep;
use std::time::Duration;

const CHILD_MSG: &str = "Child: I wanna cookie\n";
const PAR_MSG: &str = "Parent: testing...\n";

fn main() {
    let (read_end, write_end): (RawFd, RawFd) = pipe().unwrap();
    let mut buf: [u8; 100] = [0; 100];
    let mut read_end: File = unsafe { File::from_raw_fd(read_end) };
    let mut write_end: File = unsafe { File::from_raw_fd(write_end) };

    match unsafe { fork() } {
        Ok(res) => match res {
            ForkResult::Child => loop {
                write_end.write_all(CHILD_MSG.as_bytes()).expect("write");
                sleep(Duration::from_secs(5));
            },
            ForkResult::Parent { child: _ } => loop {
                write_end.write_all(PAR_MSG.as_bytes()).expect("write");
                sleep(Duration::from_secs(1));
                let n = read_end.read(&mut buf).unwrap();
                if n == 0 {
                    break;
                }
                // just write `n` bytes instead of writing the whole buffer to stdout
                // print!("{}", std::str::from_utf8(&buf).unwrap());
                stdout().write(&buf[0..n]).unwrap();
            },
        },
        _ => (),
    }
}
