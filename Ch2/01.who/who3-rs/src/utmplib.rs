// hurdles:
//   Mutating a static variable is unsafe in rust cause it violates the axiom
//   so we have better move the static variables somewhere else.
// solution:
//   Encapsulate those variables into a struct

use std::io::Read;
use std::mem::zeroed;
use std::slice;
use std::{fs, mem, process};

use libc::utmpx;

const NRECS: usize = 16;
const UTSIZE: usize = mem::size_of::<utmpx>();

pub struct UtmpLib {
    buf: [utmpx; NRECS],
    cur_rec: usize,
    num_recs: usize,
    file: fs::File,
}

impl UtmpLib {
    pub fn open() -> Self {
        Self {
            buf: [unsafe { zeroed() }; NRECS],
            cur_rec: 0,
            num_recs: 0,
            file: fs::File::open("/var/run/utmp").expect("Can not open the utmp file"),
        }
    }

    fn reload(&mut self) -> usize {
        // construct a slice
        let buf = unsafe {
            slice::from_raw_parts_mut(
                &self.buf as *const utmpx as *const u8 as *mut u8,
                NRECS * UTSIZE,
            )
        };
        let amt_read: usize = match self.file.read(buf) {
            Ok(n) => n,
            Err(msg) => {
                eprintln!("Can not read from the utmp file, {}", msg);
                process::exit(-1);
            }
        };
        self.cur_rec = 0;
        self.num_recs = amt_read / UTSIZE;
        self.num_recs
    }

    pub fn next(&mut self) -> Option<&utmpx> {
        if self.cur_rec == self.num_recs && self.reload() == 0 {
            return None;
        }
        let p: &utmpx = self.buf.get(self.cur_rec).expect("cannot get pointer");
        self.cur_rec += 1;
        Some(p)
    }
}
