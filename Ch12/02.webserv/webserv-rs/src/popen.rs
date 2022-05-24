use nix::sys::wait::wait;
use nix::unistd::{close, dup2, fork, pipe, ForkResult};
use std::ffi::OsStr;
use std::fs::File;
use std::io::BufReader;
use std::os::unix::io::{FromRawFd, RawFd};
use std::os::unix::prelude::CommandExt;
use std::process::Command;

#[derive(PartialEq)]
#[allow(dead_code)]
pub enum Type {
    Read,
    Write,
}

pub fn my_popen<S: AsRef<OsStr>>(command: S, op_type: Type) -> BufReader<File> {
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

pub fn my_pclose() {
    wait().unwrap();
}
