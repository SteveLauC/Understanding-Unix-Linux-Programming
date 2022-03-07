use std::io::{self, Write, stdin, Read};
use termios::{self, Termios, VMIN, tcsetattr, TCSANOW, ICANON};


enum Answer{
    Yes,
    No,
}


fn get_resonse() -> Answer {
    print!("Do you want another transaction(y/n)");
    io::stdout().flush().unwrap();

    for n in stdin().bytes().map(|x|x.expect("cannot read from stdin")) {
        if n == 'y' as u32 as u8 || n == 'Y' as u32 as u8 {
            return Answer::Yes;
        } else if n == 'n' as u32 as u8 || n == 'N' as u32 as u8 {
            return Answer::No;
        }else {
            println!("\ncannot understand {}, Please type y or n", char::from(n));
            continue;
        }
    }
    unreachable!()
}


fn set_crmode() {
    let mut ttyinfo: Termios = Termios::from_fd(0).expect("can not fetch configuration");

    ttyinfo.c_lflag &= !ICANON;
    ttyinfo.c_cc[VMIN] = 1;

    tcsetattr(0, TCSANOW, &ttyinfo).expect("cannot send the modification back to the kernel");
}



fn main() {
    let orig_mode: Termios = Termios::from_fd(0).expect("cannot get the orig mode");
    set_crmode();
    match get_resonse() {
        Answer::No => {
            tcsetattr(0, TCSANOW, &orig_mode).expect("cannot restore the original mode");
            std::process::exit(1);
        },
        Answer::Yes => tcsetattr(0, TCSANOW, &orig_mode).expect("cannot restore the original mode"),
    }
}
