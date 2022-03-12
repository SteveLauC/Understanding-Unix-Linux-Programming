use std::ffi::CString;
use std::os::raw::c_int;
use std::thread::sleep;
use std::time::Duration;

use curses_sys::*;

const LEFTEDGE: c_int = 10;
const RIGHTEDGE: c_int = 30;
const ROW: c_int = 10;

fn main() {
    let message: CString = CString::new("Hello").expect("can not create c compatible string");
    let blank: CString = CString::new("     ").expect("can not create c compatible string");
    let direction: c_int = 1;
    let mut position: c_int = LEFTEDGE;

    unsafe {
        initscr();
        clear();

        loop {
            move_(ROW, position);
            addstr(message.as_ptr());
            move_(LINES - 1, COLS - 1);
            refresh();
            sleep(Duration::from_secs(1));
            move_(ROW, position);
            addstr(blank.as_ptr());
            position += direction;
            if position >= RIGHTEDGE {
                position -= 1;
            }
            if position <= LEFTEDGE {
                position += 1;
            }
        }
    }
}
