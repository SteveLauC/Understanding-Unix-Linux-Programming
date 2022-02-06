// Here is a little bug I cannot figure out:(
//
// let's assume the following test condition:
// $ .write0 /dev/pts/4
// world
// You typed `world` and wanted to quit
//
// In the C version of write0, 2 times of EOF is sufficient.
// Nevertheless, you need 3 times of it here.
// Seemingly, in Rust version of write0, 2 times of EOF to send `world` and the last one for the exit 
//
// idk the reason, `fgets` and `read_until` are both buffered
// If you know this, PR welcome.
use std::env::args;
use std::process;
use std::io::{stdin, Stdin, Write, BufRead, BufReader};
use std::fs::File;


fn main() {
    let av: Vec<String> = args().collect();

    // check args
    if av.len() != 2 {
        eprintln!("Usage: write0 ttyname");
        process::exit(-1);
    }
   
    let mut tty: File = match File::create(av[1].as_str()) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("Cannot open {}: {}", av[1].as_str(), e);
            process::exit(-1);
        }
    };
    
    let mut buf: Vec<u8> = Vec::new();
    let input: Stdin = stdin();
    let mut buf_input: BufReader<Stdin> = BufReader::new(input);

    while let Ok(n) = buf_input.read_until(10, &mut buf) {
        if n==0 {
            break;
        }

        // println!("one iteration executed, read {} bytes", n);
        if tty.write(buf.as_slice()).is_err() {
            break;
        }
        buf.clear();
    }
}
