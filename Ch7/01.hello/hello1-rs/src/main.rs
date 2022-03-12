use std::ffi::CString;
use curses_sys::*;


fn main() {
    unsafe{
        initscr();
        clear();
        move_(10, 20);
        let c_str: CString = CString::new("Hello world").expect("can not create c compatible string");
        addstr(c_str.as_ptr());
        move_(LINES-1, 0);
        refresh();
        getch();
        endwin();
    }
}
