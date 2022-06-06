/*
   shm_rs2-rs does not remove the shared memory and semaphores

   So when you use `Ctrl-C` to interrupt the server and try to run it again
   you will get a complaint from `semget` due to the use of flag(IPC_EXCL)
*/
use chrono::{DateTime, Local};
use libc::{c_short, c_void, shmctl, IPC_CREAT, IPC_EXCL, IPC_RMID};
use libc::{sembuf, semctl, semget, semop, shmat, shmget, shmid_ds};
use std::ffi::CString;
use std::process::exit;
use std::ptr::null;
use std::thread::sleep;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

const SEM_UNDO: c_short = 0x1000;
const SETVAL: i32 = 16;

const TIME_MEM_KEY: i32 = 99;
const TIME_SEM_KEY: i32 = 9900;
const SEG_SIZE: usize = 100;

fn main() {
    let seg_id: i32 = unsafe { shmget(TIME_MEM_KEY, SEG_SIZE, IPC_CREAT | 0o777) };
    if seg_id == -1 {
        eprintln!("shmget");
        exit(1);
    }

    let mem_ptr: *mut c_void = unsafe { shmat(seg_id, null(), 0) };
    if mem_ptr == (-1_i32) as *mut c_void {
        eprintln!("shmat");
        exit(2);
    }

    let sem_set_id: i32 = unsafe { semget(TIME_SEM_KEY, 2, IPC_CREAT | IPC_EXCL | 0o666) };
    // let sem_set_id: i32 = unsafe { semget(TIME_SEM_KEY, 2, IPC_CREAT | 0o666) };
    if sem_set_id == -1 {
        eprintln!("semget");
        exit(3);
    }

    set_sem_value(sem_set_id, 0, 0);
    set_sem_value(sem_set_id, 1, 0);

    // run for a minute
    for _ in 0..60 {
        let now: CString = CString::new(show_time(
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        ))
        .unwrap();
        println!("\tshm_ts2 waiting for lock");
        wait_and_lock(sem_set_id);
        println!("\tshm_ts2 updating memory");
        unsafe { mem_ptr.copy_from(now.as_ptr() as *const c_void, now.as_bytes().len() + 1) };
        release_lock(sem_set_id);
        println!("\tshm_ts2 released lock");
        sleep(Duration::from_secs(1));
    }

    unsafe {
        shmctl(seg_id, IPC_RMID, null() as *const shmid_ds as *mut shmid_ds);
        semctl(sem_set_id, 0, IPC_RMID, null::<*const i32>());
    }
}

fn show_time(seconds: u64) -> String {
    let date: DateTime<Local> = DateTime::from(UNIX_EPOCH + Duration::from_secs(seconds));
    date.format("%b %e %H:%M:%S %Y").to_string()
}

fn set_sem_value(sem_set_id: i32, sem_num: i32, val: i32) {
    if unsafe { semctl(sem_set_id, sem_num, SETVAL, val) } == -1 {
        eprintln!("semctl");
        exit(4);
    }
}

fn wait_and_lock(sem_set_id: i32) {
    let mut actions: [sembuf; 2] = [
        sembuf {
            sem_num: 0,
            sem_flg: SEM_UNDO,
            sem_op: 0,
        },
        sembuf {
            sem_num: 1,
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
        sem_num: 1,
        sem_flg: SEM_UNDO,
        sem_op: -1,
    }];

    if unsafe { semop(sem_set_id, &mut actions as *mut sembuf, 1) } == -1 {
        eprintln!("semop: unlocking");
        exit(10);
    }
}
