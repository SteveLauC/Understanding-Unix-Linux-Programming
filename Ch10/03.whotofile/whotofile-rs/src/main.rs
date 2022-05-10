use nix::fcntl::{open, OFlag};
use nix::sys::stat::Mode;
use nix::unistd::{close, dup2, fork, ForkResult};
use std::os::unix::io::RawFd;
use std::os::unix::process::CommandExt;
use std::process::exit;
use std::process::Command;

fn main() {
    match unsafe { fork() } {
        Ok(res) => match res {
            ForkResult::Parent { child: _ } => (),
            ForkResult::Child => {
                redirect(1, "userlist");
                Command::new("who").exec();
            }
        },
        Err(_) => exit(1),
    }
}

fn redirect(new_fd: RawFd, file_name: &str) {
    let old_fd: RawFd = open(
        file_name,
        OFlag::O_CREAT | OFlag::O_RDWR | OFlag::O_TRUNC,
        Mode::from_bits(0o644_u32).unwrap(),
    )
    .unwrap();
    dup2(old_fd, new_fd).unwrap();
    close(old_fd).unwrap();
}
