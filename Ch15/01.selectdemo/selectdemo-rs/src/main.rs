use nix::libc::timeval;
use nix::sys::select::{select, FdSet};
use nix::sys::time::TimeVal;
use std::env::args;
use std::fs::File;
use std::io::stdout;
use std::io::Read;
use std::io::Write;
use std::os::unix::io::AsRawFd;
use std::process::exit;

fn main() {
    let av: Vec<String> = args().collect();
    if av.len() != 4 {
        eprintln!("usage: ./selectdemo file1 file2 timeout");
        exit(1);
    }

    let mut file1: File = File::open(av[1].as_str()).unwrap();
    let mut file2: File = File::open(av[2].as_str()).unwrap();
    let timeout: TimeVal = TimeVal::from(timeval {
        tv_sec: av[3].parse().unwrap(),
        tv_usec: 0,
    });
    let mut fd_set: FdSet = FdSet::new();

    loop {
        fd_set.clear();
        fd_set.insert(file1.as_raw_fd());
        fd_set.insert(file2.as_raw_fd());

        if let Ok(res) = select(
            fd_set.highest().unwrap() + 1,
            &mut fd_set,
            None,
            None,
            &mut timeout.clone(),
        ) {
            if res > 0 {
                if fd_set.contains(file1.as_raw_fd()) {
                    show_data(av[1].as_str(), &mut file1);
                }
                if fd_set.contains(file2.as_raw_fd()) {
                    show_data(av[2].as_str(), &mut file2);
                }
            } else {
                println!("no input after {} seconds", av[3].as_str());
            }
        }
    }
}

fn show_data(file_name: &str, file: &mut File) {
    println!("FILE: {}: ", file_name);

    let mut buf: [u8; 1024] = [0; 1024];

    if let Ok(n) = file.read(&mut buf) {
        stdout().write(&buf[0..n]).unwrap();
    }
    println!();
}
