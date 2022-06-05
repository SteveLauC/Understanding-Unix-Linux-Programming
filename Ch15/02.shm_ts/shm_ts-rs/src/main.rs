use std::ffi::CString;
use chrono::{DateTime, Local};
use libc::{c_void, shmat, shmctl, shmget, shmid_ds, IPC_CREAT, IPC_RMID, strlen};
use std::process::exit;
use std::ptr::null;
use std::{
    thread::sleep,
    time::{Duration, SystemTime, UNIX_EPOCH},
};
const TIME_MEM_KEY: i32 = 99;
const SEG_SIZE: usize = 100;

fn oops(msg: &str, x: i32) {
    eprintln!("{}", msg);
    exit(x);
}

fn main() {
    let seg_id: i32 = unsafe { shmget(TIME_MEM_KEY, SEG_SIZE, IPC_CREAT | 0o777) };
    if seg_id == -1 {
        oops("shmget", 1);
    }

    let mem_ptr: *mut c_void = unsafe { shmat(seg_id, null(), 0) };
    if mem_ptr == (-1_i32) as *mut c_void {
        oops("shmat", 2);
    }

    for _ in 0..60 {
        let now: u64 = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        let now_str: CString = CString::new(show_time(now)).unwrap();
        unsafe {
            now_str.as_ptr().copy_to(mem_ptr as *mut i8, strlen(now_str.as_ptr()));
        }
        sleep(Duration::from_secs(1));
    }

    unsafe { shmctl(seg_id, IPC_RMID, null() as *const shmid_ds as *mut shmid_ds) };
}

fn show_time(seconds: u64) -> String {
    let date: DateTime<Local> = DateTime::from(UNIX_EPOCH + Duration::from_secs(seconds));
    date.format("%b %e %H:%M:%S %Y").to_string()
}
