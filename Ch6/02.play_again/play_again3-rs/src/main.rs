use std::io::{stdin, stdout, Read, Write};
use std::process;
use std::thread::sleep;
use std::time::Duration;

use libc::{c_int, fcntl, F_GETFL, F_SETFL, O_NONBLOCK};
use termios::{tcsetattr, Termios, ECHO, ICANON, TCSANOW, VMIN};

const TRIES: i32 = 3;
const SLEEPTIME: u64 = 2;

fn set_cr_noecho_mode() {
    let mut ttyinfo: Termios = match Termios::from_fd(0) {
        Ok(s) => s,
        Err(msg) => {
            eprintln!("Cannot fetch tty configuration: {}", msg);
            process::exit(1);
        }
    };

    // disable ICANON bit
    ttyinfo.c_lflag &= !ICANON;
    // disable ECHO bit
    ttyinfo.c_lflag &= !ECHO;
    // set minimum number of bytes been read to 1
    ttyinfo.c_cc[VMIN] = 1;

    // send the modification back to kernel
    tcsetattr(0, TCSANOW, &ttyinfo).expect("Can not send the modification back");
}

// put file descriptor 0 into non-blocking mode
fn set_non_blocking_mode() {
    let mut terflags: c_int = unsafe { fcntl(0, F_GETFL) };
    terflags |= O_NONBLOCK;
    unsafe {
        fcntl(0, F_SETFL, terflags);
    }
}

fn get_ok_chars() -> u8 {
    let mut n: u8 = 0;
    for i in stdin().bytes() {
        if i.is_err() {
            return n;
        }
        if n == 'Y' as u32 as u8 || n == 'y' as u32 as u8 {
            n = 'Y' as u32 as u8;
        }
        if n == 'N' as u32 as u8 || n == 'n' as u32 as u8 {
            n = 'N' as u32 as u8;
        }
    }
    n
}

fn get_response() -> u8 {
    print!("Do you want another transaction: (y/n)");
    stdout().flush().unwrap();

    for _ in 0..TRIES {
        sleep(Duration::from_secs(SLEEPTIME));
        let input: u8 = get_ok_chars();

        if input == 'Y' as u32 as u8 {
            return 0;
        }
        if input == 'N' as u32 as u8 {
            return 1;
        }

        if input == 0 {
            return 2;
        }
    }

    unreachable!()
}
fn main() {
    let orig_mode: Termios = Termios::from_fd(0).expect("Can not fetch original confuguration");
    let orig_flags: c_int = unsafe { fcntl(0, F_GETFL) };
    set_cr_noecho_mode();
    set_non_blocking_mode();
    let response: u8 = get_response();
    tcsetattr(0, TCSANOW, &orig_mode).expect("Can not send the original mode back");
    unsafe { fcntl(0, F_SETFL, orig_flags) };
    process::exit(response as i32);
}
