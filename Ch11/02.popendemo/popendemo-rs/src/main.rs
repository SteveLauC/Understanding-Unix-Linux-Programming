/*
  C's `FILE` is buffered so we use `BufReader` here

  And `FILE *` constructed using `popen()` must be closed via `pclose()` for
  the reason that `wait()` will be called in `pclose()`'s implementation. So
  I just manually call `wait()` here to avoid zombie process
*/

use libc::{fileno, popen, FILE};
use nix::sys::wait::wait;
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
        });

    println!("{:?}", wait()); // manually call `wait()`
}
