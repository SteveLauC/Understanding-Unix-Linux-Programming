use curses_sys::{addstr, cbreak, clear, endwin, getch, initscr, move_, noecho, refresh, COLS};
use std::os::raw::c_int;
use std::sync::{Arc, Mutex, MutexGuard};
use std::thread::{sleep, spawn};
use std::time::Duration;

const MESSAGE: [u8; 6] = [72, 101, 108, 108, 111, 0];
const BLANK: [u8; 6] = [32, 32, 32, 32, 32, 0];
const LEN: i32 = 5;

struct Config {
    row: i32,
    col: i32,
    dir: i32,
    delay: i32,
}

impl Config {
    fn new() -> Self {
        Self {
            row: 10,
            col: 0,
            dir: 1,
            delay: 200,
        }
    }
}

fn main() {
    let config: Arc<Mutex<Config>> = Arc::new(Mutex::new(Config::new()));
    let mut ndelay: i32;
    let mut c: c_int;

    unsafe {
        let lock_gurad: MutexGuard<_> = config.lock().unwrap();
        initscr();
        cbreak();
        noecho();
        clear();
        move_(lock_gurad.row, lock_gurad.col);
        addstr(&MESSAGE as *const u8 as *const i8);
        refresh();
    }

    let config_clone: Arc<Mutex<Config>> = config.clone();
    spawn(move || {
        moving_msg(config_clone);
    });

    loop {
        ndelay = 0;
        unsafe {
            c = getch();
            if c == 'Q' as c_int {
                break;
            }
            if c == ' ' as c_int {
                let mut lock_guard: MutexGuard<_> = config.lock().unwrap();
                lock_guard.dir *= -1;
            }
            if c == 'f' as c_int {
                let lock_guard: MutexGuard<_> = config.lock().unwrap();
                if lock_guard.delay > 2 {
                    ndelay = lock_guard.delay / 2;
                }
            }
            if c == 's' as c_int {
                let lock_guard: MutexGuard<_> = config.lock().unwrap();
                ndelay = lock_guard.delay * 2;
            }
            if ndelay > 0 {
                let mut lock_guard: MutexGuard<_> = config.lock().unwrap();
                lock_guard.delay = ndelay;
            }
        }
    }

    unsafe {
        endwin();
    }
}

fn moving_msg(config: Arc<Mutex<Config>>) {
    loop {
        {
            let lock_guard: MutexGuard<_> = config.lock().unwrap();
            sleep(Duration::from_millis(lock_guard.delay as u64));
        }
        unsafe {
            {
                let mut lock_guard: MutexGuard<_> = config.lock().unwrap();

                move_(lock_guard.row, lock_guard.col);
                addstr(&BLANK as *const u8 as *const i8);
                lock_guard.col += lock_guard.dir;
                move_(lock_guard.row, lock_guard.col);
            }
            addstr(&MESSAGE as *const u8 as *const i8);
            refresh();

            {
                let mut lock_guard: MutexGuard<_> = config.lock().unwrap();
                if lock_guard.col <= 0 && lock_guard.dir == -1 {
                    lock_guard.dir = 1;
                }
            }

            {
                let mut lock_guard: MutexGuard<_> = config.lock().unwrap();
                if lock_guard.col + LEN >= COLS && lock_guard.dir == 1 {
                    lock_guard.dir = -1;
                }
            }
        }
    }
}
