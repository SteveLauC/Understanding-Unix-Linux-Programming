//! In Rust, we don't have `FILE` type, so I am gonna make the signature
//! something like the following one:
//!
//! fn my_popen<S: AsRef<OsStr>>(command: S, op_type: Type) -> BufReader<File>
//!
//! To utilize the benefit of pattern matching, I will define a type `Type`
//! to represent the operation type
//!

use nix::sys::wait::wait;
use nix::unistd::{close, dup2, fork, pipe, ForkResult};
use std::ffi::OsStr;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::os::unix::io::{FromRawFd, RawFd};
use std::os::unix::prelude::CommandExt;
use std::process::Command;

#[derive(PartialEq)]
enum Type {
    Read,
    Write,
}

fn my_popen<S: AsRef<OsStr>>(command: S, op_type: Type) -> BufReader<File> {
    // get a pipe
    let (read_end, write_end): (RawFd, RawFd) = pipe().unwrap();

    if let Ok(res) = unsafe { fork() } {
        match res {
            ForkResult::Child => {
                if op_type == Type::Read {
                    close(read_end).unwrap();
                    dup2(write_end, 1).unwrap();
                } else {
                    close(write_end).unwrap();
                    dup2(read_end, 0).unwrap();
                }
                Command::new("bash")
                    .args(["-c", command.as_ref().to_str().unwrap()])
                    .exec();
            }
            ForkResult::Parent { child: _ } => {
                if op_type == Type::Read {
                    close(write_end).unwrap();
                    return BufReader::new(unsafe { File::from_raw_fd(read_end) });
                } else {
                    close(read_end).unwrap();
                    return BufReader::new(unsafe { File::from_raw_fd(write_end) });
                }
            }
        }
    }
    unreachable!()
}

fn my_pclose() {
    wait().unwrap();
}

fn main() {
    let command: &str = "who|sort";
    let op_type: Type = Type::Read;

    let buffered_file: BufReader<File> = my_popen(command, op_type);

    buffered_file
        .lines()
        .map(|res| res.unwrap())
        .for_each(|line_contents| {
            println!("{}", line_contents);
        });

    my_pclose();
}
