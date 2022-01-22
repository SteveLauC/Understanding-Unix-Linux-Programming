use std::fs::File;
use std::io::Read;
use std::time::{Duration, UNIX_EPOCH};
use std::{ffi, mem, slice};

use chrono::{DateTime, Local};
use libc::{utmpx, USER_PROCESS};

fn show_time(seconds: i32) {
    let date: DateTime<Local> =
        DateTime::from(UNIX_EPOCH + Duration::from_secs(u64::try_from(seconds).unwrap()));
    let date_str: String = date.format("%b %e %H:%M:%S %Y").to_string();
    print!("{:<12}", date_str);
}

fn show_info(ut_buf_p: &utmpx) {
    if ut_buf_p.ut_type != USER_PROCESS {
        return;
    }
    unsafe {
        print!(
            "{:<8}",
            ffi::CStr::from_ptr(ut_buf_p.ut_user.as_ptr())
                .to_str()
                .unwrap()
        );
        print!(" ");
        print!(
            "{:<8}",
            ffi::CStr::from_ptr(ut_buf_p.ut_line.as_ptr())
                .to_str()
                .unwrap()
        );
        print!(" ");
    }
    show_time(ut_buf_p.ut_tv.tv_sec);
    print!(" ");
    unsafe {
        print!(
            "({})",
            ffi::CStr::from_ptr(ut_buf_p.ut_host.as_ptr())
                .to_str()
                .unwrap()
        );
    }
    println!();
}

fn main() {
    let mut utmp_file: File = File::open("/var/run/utmp").unwrap();
    let struct_size: usize = mem::size_of::<utmpx>();
    unsafe {
        let mut current_record: utmpx = mem::zeroed();
        let buffer: &mut [u8] =
            slice::from_raw_parts_mut(&mut current_record as *mut utmpx as *mut u8, struct_size);
        while utmp_file.read_exact(buffer).is_ok() {
            show_info(&current_record);
        }
    }
}
