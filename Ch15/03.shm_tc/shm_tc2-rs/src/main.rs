use libc::{c_short, c_void, sembuf, semget, semop, shmat, shmdt, shmget};
use std::ffi::CStr;
use std::process::exit;
use std::ptr::null;

const SEM_UNDO: c_short = 0x1000;
const TIME_MEM_KEY: i32 = 99;
const TIME_SEM_KEY: i32 = 9900;
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
    let sem_set_id: i32 = unsafe { semget(TIME_SEM_KEY, 2, 0) };

    if sem_set_id == -1 {
        oops("semget", 3);
    }

    wait_and_lock(sem_set_id);
    let now: &CStr = unsafe { CStr::from_ptr(mem_ptr as *mut i8) };
    println!("The time, direct from memory: {:?}", now);
    release_lock(sem_set_id);

    unsafe { shmdt(mem_ptr) };
}

fn wait_and_lock(sem_set_id: i32) {
    let mut actions: [sembuf; 2] = [
        sembuf {
            sem_num: 1,
            sem_flg: SEM_UNDO,
            sem_op: 0,
        },
        sembuf {
            sem_num: 0,
            sem_flg: SEM_UNDO,
            sem_op: 1,
        },
    ];
    if unsafe { semop(sem_set_id, &mut actions as *mut sembuf, 2) } == -1 {
        eprintln!("semop: locking");
        exit(10);
    }
}

fn release_lock(sem_set_id: i32) {
    let mut actions: [sembuf; 1] = [sembuf {
        sem_num: 0,
        sem_flg: SEM_UNDO,
        sem_op: -1,
    }];

    if unsafe { semop(sem_set_id, &mut actions as *mut sembuf, 1) } == -1 {
        eprintln!("semop: unlocking");
        exit(10);
    }
}
