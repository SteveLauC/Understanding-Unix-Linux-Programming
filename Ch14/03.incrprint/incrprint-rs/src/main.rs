use std::thread::{sleep, spawn, JoinHandle};
use std::time::Duration;

const NUM: usize = 5;
static mut COUNTER: i32 = 0;

fn main() {
    let t1: JoinHandle<()> = spawn(print_count);

    for _ in 0..NUM {
        unsafe {
            COUNTER += 1;
        }
        sleep(Duration::from_secs(1));
    }

    t1.join().unwrap();
}

fn print_count() {
    for _ in 0..NUM {
        println!("counter = {}", unsafe { COUNTER });
        sleep(Duration::from_secs(1));
    }
}
