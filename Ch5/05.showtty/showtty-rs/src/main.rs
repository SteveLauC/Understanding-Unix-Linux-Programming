use std::process;
use termios::os::linux::{
    speed_t, tcflag_t, B1200, B1800, B2400, B300, B38400, B4800, B600, B9600, BRKINT, ECHO, ECHOE,
    ICANON, ICRNL, IGNBRK, IGNCR, IGNPAR, INLCR, INPCK, ISIG, IXOFF, IXON, PARMRK, VERASE, VKILL,
};
use termios::{cfgetospeed, Termios};

struct FlagInfo<'a> {
    fl_value: tcflag_t,
    fl_name: &'a str,
}
impl<'a> FlagInfo<'a> {
    const fn new(value: tcflag_t, name: &'a str) -> Self {
        Self {
            fl_value: value,
            fl_name: name,
        }
    }
}

const INPUT_FLAGS: [FlagInfo; 10] = [
    FlagInfo::new(IGNBRK, "Ignore break condition"),
    FlagInfo::new(BRKINT, "Signal interrupt on break"),
    FlagInfo::new(IGNPAR, "Ignore chars with parity errors"),
    FlagInfo::new(PARMRK, "Mark parity errors"),
    FlagInfo::new(INPCK, "Enable input parity check"),
    FlagInfo::new(INLCR, "Map NL to CR on input"),
    FlagInfo::new(IGNCR, "Ignore CR"),
    FlagInfo::new(ICRNL, "Map CR to NL on input"),
    FlagInfo::new(IXON, "Enable start/stop output contrl"),
    FlagInfo::new(IXOFF, "Ebale start/stop input control"),
];

const LOCAL_FLAGS: [FlagInfo; 4] = [
    FlagInfo::new(ISIG, "Enable signals"),
    FlagInfo::new(ICANON, "Cannoical input(erase and kill)"),
    FlagInfo::new(ECHO, "Enable echo"),
    FlagInfo::new(ECHOE, "Echo ERASE as BS-SPACE-BS"),
];

fn show_baud(thespeed: speed_t) {
    match thespeed {
        B300 => println!("300"),
        B600 => println!("600"),
        B1200 => println!("1200"),
        B1800 => println!("1800"),
        B2400 => println!("2400"),
        B4800 => println!("4800"),
        B9600 => println!("9600"),
        B38400 => println!("38400"),
        _ => println!("fast"),
    }
}

fn show_flagset(thvalue: tcflag_t, thebitnames: &[FlagInfo]) {
    for flag in thebitnames {
        print!("{} is ", flag.fl_name);
        if thvalue & flag.fl_value != 0 {
            println!("ON");
        } else {
            println!("OFF");
        }
    }
}

fn show_some_flags(ttyp: &Termios) {
    show_flagset(ttyp.c_iflag, &INPUT_FLAGS);
    show_flagset(ttyp.c_lflag, &LOCAL_FLAGS);
}
fn main() {
    let ttyinfo: Termios = match Termios::from_fd(0) {
        Ok(s) => s,
        Err(msg) => {
            eprintln!("Cannot get params about stdin: {}", msg);
            process::exit(1);
        }
    };

    println!("OUTPUT BAUD RATE:");
    show_baud(cfgetospeed(&ttyinfo));
    print!("\n");

    println!("CONTROL CHARACTER:");
    println!(
        "The erase character is ascii {}, Ctrl - {}",
        ttyinfo.c_cc[VERASE],
        char::from(ttyinfo.c_cc[VERASE] + 'A' as u8 - 1)
    );

    println!(
        "The line kill character is ascii {}, Ctrl - {}",
        ttyinfo.c_cc[VKILL],
        char::from(ttyinfo.c_cc[VKILL] + 'A' as u8 - 1)
    );
    print!("\n");

    println!("INPUT FLAGSET AND LOCAL FLAFSET:");
    show_some_flags(&ttyinfo);
}
