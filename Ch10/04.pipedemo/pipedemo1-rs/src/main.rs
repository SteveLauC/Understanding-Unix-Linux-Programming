use std::fs::File;
use nix::unistd::pipe;
use std::os::unix::io::{RawFd, FromRawFd};
use std::io::{stdin, Write, Read};

fn main() {
    let (read_end, write_end): (RawFd, RawFd) = pipe().unwrap();

    println!("get a pipe, its file descriptors are: {} & {}", read_end, write_end);

    let mut read_end: File = unsafe{ File::from_raw_fd(read_end)};
    let mut write_end: File = unsafe { File::from_raw_fd(write_end) };
    let mut buf: String = String::with_capacity(100);
    
    // read from stdin
    stdin().read_line(&mut buf).unwrap();
    
    // write to the write end
    write_end.write(buf.as_bytes()).unwrap();
    
    // wipe the buffer
    buf.clear();
    

    // read from the read end
    // read_to_string reads all the bytes until EOF 
    // However, only when the write_end is closed can we get the EOF from the read end
    // so we need to manually drop the write end here
    drop(write_end);
    read_end.read_to_string(&mut buf).unwrap();
    
    // prints it out
    println!("{}", buf);
}
