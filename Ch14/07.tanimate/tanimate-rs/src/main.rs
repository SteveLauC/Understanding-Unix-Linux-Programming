/*
 * Only the last string will appear on the screen(Actually it is the first one cuz I used `swap_remove`)
 *
 * And the main thread will do nothing to the keyboard event
 *
 * :(
*/

use curses_sys::{
    addch, addstr, cbreak, clear, endwin, getch, initscr, move_, noecho, refresh, COLS, LINES,
};
use libc::strlen;
use rand::prelude::*;
use std::env::args;
use std::ffi::CString;
use std::process::exit;
use std::sync::{Arc, Mutex, MutexGuard};
use std::thread::{scope, sleep};
use std::time::Duration;

const MAXMSG: usize = 10;
const TUNIT: u64 = 20000;

#[derive(Default, Debug)]
struct PropSet {
    str: CString,
    row: i32,
    delay: u64,
    dir: i32,
}

fn main() {
    let mut av: Vec<String> = args().collect();
    if av.len() == 1 {
        println!("usage: tanimate strings..");
        exit(1);
    }
    av.swap_remove(0);

    let mut props: [PropSet; MAXMSG] = [(); MAXMSG].map(|_| PropSet::default());
    let num_msg: i32 = setup(av.len(), av, &mut props);
    let lock: Arc<Mutex<()>> = Arc::new(Mutex::new(()));
    // let mut threads: Vec<ScopedJoinHandle<_>> = Vec::new();

    for i in 0..num_msg {
        scope(|s| {
            /* threads.push(s.spawn(|_| {
                animate(&mut props[i as usize], &lock);
            })); */
            s.spawn(|| {
                animate(&mut props[i as usize], &lock);
            });
        });
    }

    loop {
        let c: i32 = unsafe { getch() };
        if c == 'Q' as i32 {
            break;
        }
        if c == ' ' as i32 {
            println!("debug: space");
            for i in 0..num_msg {
                props[i as usize].dir *= -1;
            }
        }
        if c >= '0' as i32 && c <= '9' as i32 {
            println!("debug: number {}", c);
            let idx: i32 = c - '0' as i32;
            if idx < num_msg {
                props[idx as usize].dir *= -1;
            }
        }
    }

    /* for item in threads {
        let raw_handle: RawPthread = item.as_pthread_t();
        unsafe{
            pthread_cancel(raw_handle);
        }
    } */
    unsafe {
        endwin();
    }
}

fn setup(n_strings: usize, strings: Vec<String>, props: &mut [PropSet; MAXMSG]) -> i32 {
    let num_msg = if n_strings > MAXMSG {
        MAXMSG
    } else {
        n_strings
    };

    let mut rng: ThreadRng = rand::thread_rng();
    for idx in 0..num_msg {
        props[idx].str = CString::new(strings[idx].as_str()).unwrap();
        props[idx].row = idx as i32;
        props[idx].delay = 1 + (rng.gen::<u64>() % 15);
        props[idx].dir = if rng.gen::<i32>() % 2 == 0 { -1 } else { 1 };
    }

    unsafe {
        initscr();
        cbreak();
        noecho();
        clear();
    }

    num_msg as i32
}

fn animate(arg: &mut PropSet, lock: &Arc<Mutex<()>>) {
    let len: usize = unsafe { strlen(arg.str.as_ptr()) } + 2;
    let mut col: i32 = rand::random::<i32>() % (unsafe { COLS } - len as i32 - 3);

    loop {
        sleep(Duration::from_micros(arg.delay * TUNIT));
        // critical section
        unsafe {
            let _guard: MutexGuard<_> = lock.lock().unwrap();
            move_(arg.row, col);
            addch(' ' as u32);
            addstr(arg.str.as_ptr());
            addch(' ' as u32);
            move_(LINES - 1, COLS - 1);
            refresh();
        }

        col += arg.dir;
        if col <= 0 && arg.dir == -1 {
            arg.dir = 1;
        }
        if col + len as i32 > unsafe { COLS } && arg.dir == 1 {
            arg.dir = -1;
        }
    }
}
