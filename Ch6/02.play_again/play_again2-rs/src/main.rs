use std::process;
use std::io::{self, stdin, Read, Write};
use termios::{self, Termios, ECHO, ICANON, VMIN, TCSANOW, tcsetattr};

enum Answer{
    Yes,
    No,
}


fn get_response() -> Answer {
    print!("Do you want another transaction(y/n)");
    io::stdout().flush().unwrap(); 

    
    for n in stdin().bytes().map(|x| x.unwrap()) {
        if n == 'y' as u32 as u8 || n == 'Y' as u32 as u8 {
            return Answer::Yes;
        } else if n == 'n' as u32 as u8 || n == 'N' as u32 as u8 {
            return Answer::No;
        }  
    }

    unreachable!()
}

fn set_cr_noecho_mode() {
    let mut ttyinfo: Termios = Termios::from_fd(0).expect("can not fetch configuration");

    ttyinfo.c_lflag &= !ICANON;
    ttyinfo.c_lflag &= !ECHO;
    ttyinfo.c_cc[VMIN] = 1;

    tcsetattr(0, TCSANOW, &ttyinfo).expect("cannot send the modification back to the kernel");
}

fn main() {
    let orig_mode: Termios = Termios::from_fd(0).expect("cannot get the orig mode");
    set_cr_noecho_mode();
    match get_resonse() {
        Answer::No => {
            tcsetattr(0, TCSANOW, &orig_mode).expect("cannot restore the original mode");
            std::process::exit(1);
        },
        Answer::Yes => tcsetattr(0, TCSANOW, &orig_mode).expect("cannot restore the original mode"),
    }
}