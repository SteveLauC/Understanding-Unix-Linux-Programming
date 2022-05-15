use nix::unistd::{close, dup2, fork, pipe, ForkResult};
use std::env::args;
use std::os::unix::io::RawFd;
use std::os::unix::prelude::CommandExt;
use std::process::{exit, Command};

fn main() {
    let av: Vec<String> = args().collect();
    if av.len() != 3 {
        eprintln!("usage: pipe2-rs cmd1 cmd2\n");
        exit(1);
    }
    let (read_end, write_end): (RawFd, RawFd) = pipe().unwrap();

    if let ForkResult::Child = unsafe { fork().unwrap() } {
        close(read_end).unwrap();
        dup2(write_end, 1).unwrap();
        close(write_end).unwrap();
        Command::new(av[1].as_str()).exec();
    }

    if let ForkResult::Child = unsafe { fork().unwrap() } {
        close(write_end).unwrap();
        dup2(read_end, 0).unwrap();
        close(read_end).unwrap();
        Command::new(av[2].as_str()).exec();
    }

    close(read_end).unwrap();
    close(write_end).unwrap();
}
