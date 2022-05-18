/*
  C's `FILE` is buffered so we use `BufReader` here
*/

use libc::{fileno, popen, FILE};
use std::ffi::CString;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::os::unix::io::{FromRawFd, RawFd};

fn main() {
    let command: CString = CString::new("who|sort").unwrap();
    let mode: CString = CString::new("r").unwrap();
    let file: File;
    unsafe {
        let fp: *mut FILE = popen(command.as_ptr(), mode.as_ptr());
        let fd: RawFd = fileno(fp);
        file = File::from_raw_fd(fd);
    };

    let buffered_file: BufReader<File> = BufReader::new(file);

    buffered_file
        .lines()
        .map(|res| res.unwrap())
        .for_each(|line_contents| {
            println!("{}", line_contents);
        })
}
