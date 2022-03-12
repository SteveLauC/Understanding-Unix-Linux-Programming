use std::ffi::CString;
use std::thread::sleep;
use std::time::Duration;

use curses_sys::*;

fn main() {
    unsafe{
        initscr();
        clear();
        let hello: CString = CString::new("Hello world").expect("can not create c compatible string");
        let empty: CString = CString::new("                   ").expect("can not create c compatible string");

        for i in 0..LINES {
            move_(i, i+1);
            if i%2 == 1 {
                standout();
            }
            addstr(hello.as_ptr());
            if i%2 == 1 {
                standend();
            }
            sleep(Duration::from_secs(1));
            refresh();
            move_(i, i+1);
            addstr(empty.as_ptr()) ;
        }

        endwin();
    }
}
