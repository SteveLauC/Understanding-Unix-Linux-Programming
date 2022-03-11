use std::thread::sleep;
use std::time::Duration;

use nix::sys::signal::{signal, SIGINT, SigHandler};

fn main() {
    unsafe{
        signal(SIGINT, SigHandler::SigIgn).expect("can not get previous handler");
    }

    println!("You can not stop me");
    loop{
        println!("Haha");
        sleep(Duration::from_secs(1));
    }
}
