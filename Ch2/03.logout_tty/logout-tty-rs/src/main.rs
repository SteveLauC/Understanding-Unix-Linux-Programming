// Rust has a much better tool for error handling, so let's define a `Enum`
// to represent the errors we may encounter.

use std::ffi::CStr;
use std::fs::File;
use std::io::{Read, Seek, SeekFrom, Write};
use std::time::{SystemTime, UNIX_EPOCH};
use std::{mem, slice};

use crate::LogoutError::*;
use libc::{utmpx, DEAD_PROCESS};

enum LogoutError {
    OpenError(String),
    ReadError(String),
    NotFound(String),
    SeekError(String),
    WriteError(String),
    CloseError(String),
}

fn logout_tty(line: &str) -> Result<(), LogoutError> {
    let mut rec: utmpx = unsafe { mem::zeroed() };
    let len: usize = mem::size_of::<utmpx>();
    let buf: &mut [u8] =
        unsafe { slice::from_raw_parts_mut(&rec as *const utmpx as *const u8 as *mut u8, len) };

    let mut utmp_file: File = match File::open("/var/run/utmp") {
        Ok(f) => f,
        Err(msg) => return Err(OpenError(msg.to_string())),
    };

    while utmp_file.read(buf).is_ok() {
        if unsafe { CStr::from_ptr(rec.ut_line.as_ptr()).to_str().unwrap() } == line {
            rec.ut_type = DEAD_PROCESS;
            rec.ut_tv.tv_sec = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("can not calculate time elapsed")
                .as_secs() as i32;
            rec.ut_tv.tv_usec = 0;

            if let Err(msg) = utmp_file.seek(SeekFrom::Current(-(len as i64))) {
                return Err(SeekError(msg.to_string()));
            }

            match utmp_file.write(buf) {
                Ok(n) => {
                    if n != len {
                        return Err(WriteError("cannot write so many bytes to it".to_string()));
                    }
                }
                Err(msg) => return Err(WriteError(msg.to_string())),
            }
        }
    }
    Ok(())
}

fn main() {}
