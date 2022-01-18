use std::collections::VecDeque;
use std::env;
use std::fs::File;
use std::io::{self, Read, Write, BufReader};

const PAGELEN: i32 = 24;
const LINELEN: i32 = 511; // to make the behavior consistent with more01.c

fn main() {
    // convert Args into a vector
    let mut av: Vec<String> = env::args().collect();
    let ac: i32 = av.len() as i32;

    if ac == 1 {
        do_more(io::stdin().lock());
    } else {
        av.remove(0);
        for file_name in av.iter() {
            let error_msg: String = format!("can not open {}", file_name);
            let file: File = File::open(file_name).expect(&error_msg);
            let file_with_buffer: io::BufReader<File> = io::BufReader::new(file);
            do_more(file_with_buffer);
        }
    }
}

fn do_more<S: io::BufRead>(mut p: S) {
    let mut buf: String = String::new();
    let mut lines: i32 = 0; // how many lines have been read
    let mut line_queue: VecDeque<String> = VecDeque::new();
    let tty: File = File::open("/dev/tty").unwrap();

    while let Ok(bytes) = p.read_line(&mut buf) {
        // return when we reached EOF
        if bytes == 0 {
            return;
        }
        // Already read PAGELEN lines, ask for further instruction.
        if lines == PAGELEN {
            let buffered_tty: BufReader<&File> = BufReader::new(&tty);
            let reply: i32 = see_more(buffered_tty);
            if reply == 0 {
                break;
            } else {
                lines -= reply;
            }
        }

        // push lines to queue
        for slice in buf.as_bytes().chunks(LINELEN as usize) {
            line_queue.push_back(String::from_utf8_lossy(slice).into_owned());
        }

        // print as many lines as we can
        let limit: usize = if line_queue.len() < (PAGELEN - lines) as usize {
            line_queue.len()
        } else {
            (PAGELEN - lines) as usize
        };
        for _ in 0..limit {
            // we can warrant we have enough strings here, feel free to use unwrap()
            print!("{}", line_queue.pop_front().unwrap());
            io::stdout().flush().unwrap();
            lines += 1;
        }
        buf.clear();
    }
}

fn see_more(mut cmd: BufReader<&File>) -> i32 {
    print!("\x1b[93mmore?\x1b[0m");
    io::stdout().flush().unwrap();
    let mut char_buf: [u8; 1] = [0];

    // println!("debug: {}", selection);
    loop{
        cmd.read_exact(&mut char_buf).unwrap();
        match char_buf[0] as char {
            'q' => break 0,
            ' ' => {
                break PAGELEN;
            },
            '\n' => break 1,
            _ => {
               break 0;
            },
        }
    }
}
