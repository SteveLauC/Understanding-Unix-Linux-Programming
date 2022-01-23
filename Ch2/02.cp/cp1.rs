use std::env;
use std::fs::{self, File, OpenOptions};
use std::io::{Read, Result, Write};
use std::os::unix::fs::OpenOptionsExt;
use std::process;

const BUFFERSIZE: usize = 4096;
const COPYMODE: u32 = 0o644;

fn main() {
    // buffer
    let mut buf: [u8; BUFFERSIZE] = [0; BUFFERSIZE];

    // prepare and check the arguments
    let av: Vec<String> = env::args().collect();
    if av.len() != 3 {
        eprintln!("usage: {} source destination", av[0].as_str());
        process::exit(-1);
    }

    // read(2) can NOT read from a directory, however std::io::Read::read can do this.
    // so I manually check it here.
    if fs::metadata(av[1].as_str()).unwrap().is_dir() {
        oops("is a directory", av[1].as_str());
    }

    let mut in_file: File = match File::open(av[1].as_str()) {
        Ok(f) => f,
        Err(msg) => oops(msg.to_string().as_str(), av[1].as_str()),
    };

    let mut out_file: File = match OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .mode(COPYMODE)
        .open(av[2].as_str())
    {
        Ok(f) => f,
        Err(msg) => oops(msg.to_string().as_str(), av[2].as_str()),
    };

    while let Ok(n_chars) = in_file.read(&mut buf) {
        // handle EOF
        if n_chars == 0 {
            break;
        }

        let write_res: Result<usize> = out_file.write(if n_chars < BUFFERSIZE {
			// The remaining size of file(av[1]) contents is less than BUFFERSIZE
			// chunk it to get a smaller slice, whose length is same as the remaining size
            buf.chunks(n_chars).next().unwrap()
        } else {
            &buf
        });
        match write_res {
            Ok(n) => {
                if n != n_chars {
                    oops(av[2].as_str(), "Write error to ");
                }
            }
            Err(msg) => {
                oops(msg.to_string().as_str(), av[2].as_str());
            }
        }
    }
}

fn oops(s1: &str, s2: &str) -> ! {
    eprintln!("Error: {} {}", s2, s1);
    process::exit(-1);
}
