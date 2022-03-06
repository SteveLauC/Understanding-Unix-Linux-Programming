use libc::{self, cfgetospeed, speed_t, tcgetattr, termios};
use std::process;

struct FlagInfo<'a>{
    fl_value: libc::tcflag_t,
    fl_name: &'a str,
}
impl<'a> FlagInfo<'a> {
    const fn new(value: libc::tcflag_t, name: &'a str)->Self {
        Self{
            fl_value: value,
            fl_name: name,
        }
    }
}

const INPUT_FLAGS: [FlagInfo;10] = [
    FlagInfo::new(libc::IGNBRK, "Ignore break condition"),
    FlagInfo::new(libc::BRKINT, "Signal interrupt on break"),
    FlagInfo::new(libc::IGNPAR, "Ignore chars with parity errors"),
    FlagInfo::new(libc::PARMRK, "Mark parity errors"),
    FlagInfo::new(libc::INPCK, "Enable input parity check"),
    FlagInfo::new(libc::INLCR, "Map NL to CR on input"),
    FlagInfo::new(libc::IGNCR, "Ignore CR"),
    FlagInfo::new(libc::ICRNL, "Map CR to NL on input"),
    FlagInfo::new(libc::IXON, "Enable start/stop output contrl"),
    FlagInfo::new(libc::IXOFF, "Ebale start/stop input control"),
]; 

const LOCAL_FLAGS: [FlagInfo;4] = [
    FlagInfo::new(libc::ISIG, "Enable signals"),
    FlagInfo::new(libc::ICANON, "Cannoical input(erase and kill)"),
    FlagInfo::new(libc::ECHO, "Enable echo"),
    FlagInfo::new(libc::ECHOE, "Echo ERASE as BS-SPACE-BS"),
];

fn show_baud(thespeed: speed_t) {
    match thespeed {
        libc::B300 => println!("300"),
        libc::B600 => println!("600"),
        libc::B1200 => println!("1200"),
        libc::B1800 => println!("1800"),
        libc::B2400 => println!("2400"),
        libc::B4800 => println!("4800"),
        libc::B9600 => println!("9600"),
        libc::B38400 => println!("38400"),
        _ => println!("fast"),
    }
}

fn show_flagset(thvalue: libc::tcflag_t, thebitnames: &[FlagInfo]) {
    for flag in thebitnames {
        print!("{} is ", flag.fl_name);
        if thvalue & flag.fl_value != 0 {
            println!("ON");
        }else{
            println!("OFF");
        }
    }
}

fn show_some_flags(ttyp: &termios) {
    show_flagset(ttyp.c_iflag, &INPUT_FLAGS);
    show_flagset(ttyp.c_lflag, &LOCAL_FLAGS);

}
fn main() {
    let mut ttyinfo: termios = termios {
        c_iflag: 0,
        c_oflag: 0,
        c_cflag: 0,
        c_lflag: 0,
        c_line: 0,
        c_cc: [0; 32],
        c_ispeed: 0,
        c_ospeed: 0,
    };

    if (unsafe { tcgetattr(0, &mut ttyinfo as *mut termios) } == -1) {
        eprintln!("Cannot get params about stdin");
        process::exit(1);
    }

    println!("OUTPUT BAUD RATE:");
    show_baud(unsafe { cfgetospeed(&ttyinfo as *const termios) });
    print!("\n");

    println!("CONTROL CHARACTER:");
    println!(
        "The erase character is ascii {}, Ctrl - {}",
        ttyinfo.c_cc[libc::VERASE],
        char::from(ttyinfo.c_cc[libc::VERASE] + 'A' as u8 - 1)
    );

    println!(
        "The line kill character is ascii {}, Ctrl - {}",
        ttyinfo.c_cc[libc::VKILL],
        char::from(ttyinfo.c_cc[libc::VKILL] + 'A' as u8 - 1)
    );
    print!("\n");

    println!("INPUT FLAGSET AND LOCAL FLAFSET:");
    show_some_flags(&ttyinfo);
}
