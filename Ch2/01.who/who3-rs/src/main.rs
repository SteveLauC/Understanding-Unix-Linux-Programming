mod utmplib;

use chrono::{DateTime, Local};
use libc::{utmpx, USER_PROCESS};
use std::ffi;
use std::time::{Duration, UNIX_EPOCH};

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
    let mut lib: utmplib::UtmpLib = utmplib::UtmpLib::open();
    while let Some(ut_ins) = lib.next() {
        show_info(ut_ins);
    }
}
