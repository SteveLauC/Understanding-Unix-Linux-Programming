use std::io::{stdout, Write};
use std::thread::sleep;
use std::time::Duration;
const NUM: usize =  5;

fn main() {
    print_msg("hello");
    print_msg("world\n");
}

fn print_msg(m: &str) {
    for _ in 0..NUM {
        print!("{}", m);
        stdout().flush().unwrap(); 
        sleep(Duration::from_secs(1)); 
    }
}
