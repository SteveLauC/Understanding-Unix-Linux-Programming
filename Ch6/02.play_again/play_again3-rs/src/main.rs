use std::io::stdin;
use std::io::Read;
use std::thread::sleep;
use std::time::Duration;

use libc::{fcntl, F_GETFL, F_SETFL, O_NONBLOCK};





fn main() {
        unsafe{fcntl(0, F_SETFL, fcntl(0, F_GETFL)|O_NONBLOCK);}

        sleep(Duration::from_secs(4));
        for n in stdin().bytes().map(|x| x.unwrap()) {
            println!("{}", n);
        }


        unsafe{fcntl(0, F_SETFL, fcntl(0, F_GETFL)&!O_NONBLOCK);}
}
