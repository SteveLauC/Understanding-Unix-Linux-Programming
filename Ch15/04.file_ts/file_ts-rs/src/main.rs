// `fcntl` relevant file lock type/constants are missing from `nix` crate
// so I use `flock` syscall here

use chrono::{DateTime, Local};
use nix::fcntl::{flock, FlockArg};
use std::env::args;
use std::fs::{self, File};
use std::io::{Seek, SeekFrom, Write};
use std::os::unix::io::{AsRawFd, RawFd};
use std::process::exit;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

fn main() {
    let av: Vec<String> = args().collect();
    if av.len() != 2 {
        eprintln!("usage: ./file_ts-rs filename");
        exit(1);
    }

    let mut time_file: File = fs::OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open(av[1].as_str())
        .unwrap();

    loop {
        let time_str: String = show_time(
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        );

        lock_operation(time_file.as_raw_fd(), FlockArg::LockExclusive);
        time_file.seek(SeekFrom::Start(0)).unwrap();
        time_file.write(time_str.as_bytes()).unwrap();
        lock_operation(time_file.as_raw_fd(), FlockArg::Unlock);
    }
}
fn show_time(seconds: u64) -> String {
    let date: DateTime<Local> = DateTime::from(UNIX_EPOCH + Duration::from_secs(seconds));
    date.format("%b %e %H:%M:%S %Y").to_string()
}

fn lock_operation(fd: RawFd, op: FlockArg) {
    flock(fd, op).unwrap();
}
