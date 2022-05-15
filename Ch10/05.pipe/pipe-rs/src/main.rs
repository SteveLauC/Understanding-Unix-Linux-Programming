// In the rust implementation, child-process executes cmd2 and
// parent-process executes cmd1

use nix::unistd::{dup2, fork, pipe, ForkResult};
use std::env::args;
use std::fs::File;
use std::os::unix::io::{AsRawFd, FromRawFd, RawFd};
use std::os::unix::process::CommandExt;
use std::process::{exit, Command};

fn main() {
    let av: Vec<String> = args().collect();
    if av.len() != 3 {
        eprintln!("usage: pipe-rs cmd1 cmd2");
        exit(1);
    }
    let (read_end, write_end): (RawFd, RawFd) = pipe().unwrap();
    let read_end: File = unsafe { File::from_raw_fd(read_end) };
    let write_end: File = unsafe { File::from_raw_fd(write_end) };

    match unsafe { fork() } {
        Err(_) => (),
        Ok(fork_res) => match fork_res {
            ForkResult::Child => {
                drop(write_end);
                dup2(read_end.as_raw_fd(), 0).unwrap();
                Command::new(av[2].as_str()).exec();
            }
            ForkResult::Parent { child: _ } => {
                drop(read_end);
                dup2(write_end.as_raw_fd(), 1).unwrap();
                Command::new(av[1].as_str()).exec();
            }
        },
    }
}
