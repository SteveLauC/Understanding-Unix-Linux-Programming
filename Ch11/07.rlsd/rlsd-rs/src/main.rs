mod popen;

use nix::unistd::gethostname;
use popen::{my_pclose, my_popen, Type};
use std::fs::File;
use std::io::{BufRead, BufReader, Read, Write};
use std::net::{TcpListener, TcpStream};

const PORT_NUM: u16 = 15000;
const HOST_NAME_LEN: usize = 64;

fn main() {
    let mut hostname_buffer: [u8; HOST_NAME_LEN] = [0; HOST_NAME_LEN];

    for stream in TcpListener::bind(format!(
        "{}:{}",
        gethostname(&mut hostname_buffer).unwrap().to_str().unwrap(),
        PORT_NUM
    ))
    .unwrap()
    .incoming()
    {
        // read dir name
        let mut dir_name: String = String::with_capacity(100);
        let mut stream: TcpStream = stream.unwrap();
        let mut buffered_stream: BufReader<&TcpStream> = BufReader::new(&stream);
        buffered_stream.read_line(&mut dir_name).unwrap();

        // call popen to execute `ls`
        let mut child_process_reader: BufReader<File> = my_popen(format!("ls {}", dir_name), Type::Read);
        let mut results: String = String::with_capacity(300);
        child_process_reader.read_to_string(&mut results).unwrap();
        my_pclose();

        // send results back
        stream.write(results.as_bytes()).unwrap();
    }
}
