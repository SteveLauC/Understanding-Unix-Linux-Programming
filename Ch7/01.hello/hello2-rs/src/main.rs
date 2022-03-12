use std::ffi::CString;
use curses_sys::*;

fn main() {
    unsafe{
        initscr();
        clear();
        let hello: CString = CString::new("Hello world").expect("can not create c compatible string");

        for i in 0..LINES {
            move_(i, i+1);
            if i%2 == 1 {
                standout();
            }
            addstr(hello.as_ptr());
            if i%2 == 1 {
                standend();
            }
        }

        refresh();
        getch();
        endwin();
    }
}
