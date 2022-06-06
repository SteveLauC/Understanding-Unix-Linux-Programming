use libc::{c_void, shmat, shmdt, shmget};
use std::ffi::CStr;
use std::process::exit;
use std::ptr::null;

const TIME_MEM_KEY: i32 = 99;
const SEG_SIZE: usize = 100;

fn oops(msg: &str, x: i32) {
    eprintln!("{}", msg);
    exit(x);
}

fn main() {
    let seg_id: i32 = unsafe { shmget(TIME_MEM_KEY, SEG_SIZE, 0o777) };
    if seg_id == -1 {
        oops("shmget", 1);
    }

    let mem_ptr: *mut c_void = unsafe { shmat(seg_id, null(), 0) };
    if mem_ptr == -1_i32 as *mut c_void {
        oops("shmat", 2);
    }

    let now: &CStr = unsafe{CStr::from_ptr(mem_ptr as *mut i8)};
    println!("The time, direct from memory: {:?}", now);
    unsafe { shmdt(mem_ptr) };
}
