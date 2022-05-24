mod popen;
mod utility;

use nix::sys::wait::{waitpid, WaitPidFlag};
use nix::unistd::{fork, gethostname, ForkResult, Pid};
use std::env::args;
use std::io::{BufRead, BufReader};
use std::net::{TcpListener, TcpStream};
use utility::{cannot_do, do_404, do_cat, do_exec, do_ls, ends_in_cgi, is_dir, not_exist};

fn main() {
    let av: Vec<String> = args().collect();
    assert_eq!(av.len(), 2);
    let mut host_name: [u8; 64] = [0; 64];

    for stream in TcpListener::bind(format!(
        "{}:{}",
        gethostname(&mut host_name).unwrap().to_str().unwrap(),
        av[1]
    ))
    .unwrap()
    .incoming()
    {
        let stream: TcpStream = stream.unwrap();
        let mut buffered_stream: BufReader<&TcpStream> = BufReader::new(&stream);
        let mut first_line_of_request_header: String = String::with_capacity(100);
        buffered_stream
            .read_line(&mut first_line_of_request_header)
            .unwrap();
        eprintln!("got a call: request = {}", first_line_of_request_header);

        process_request(first_line_of_request_header, &stream);
    }

    waitpid(Pid::from_raw(-1), Some(WaitPidFlag::WNOHANG)).unwrap();
}

fn process_request(header: String, client: &TcpStream) {
    if let ForkResult::Child = unsafe { fork().unwrap() } {
        // parse header
        let mut header: Vec<String> = header
            .split_whitespace()
            .map(|str| str.to_string())
            .collect();
        assert_eq!(header.len(), 3);

        header[1].remove(0); // remove the first `/`

        if header[0] != "GET" {
            cannot_do(client);
        } else if not_exist(&header[1]) {
            do_404(&header[1], client);
        } else if is_dir(&header[1]) {
            do_ls(&header[1], client);
        } else if ends_in_cgi(&header[1]) {
            do_exec(&header[1], client);
        } else {
            do_cat(&header[1], client);
        }
    }
}
