use std::io::{stdout, Write};
use std::thread::sleep;
use std::thread::{spawn, JoinHandle};
use std::time::Duration;

const NUM: usize = 5;

fn main() {
    let handle1: JoinHandle<()> = spawn(||print_msg("hello"));
    let handle2: JoinHandle<()> = spawn(||print_msg("world\n"));

    handle1.join().unwrap();
    handle2.join().unwrap();
}

fn print_msg(m: &str) {
    for _ in 0..NUM {
        print!("{}", m);
        stdout().flush().unwrap();
        sleep(Duration::from_secs(1));
    }
}
