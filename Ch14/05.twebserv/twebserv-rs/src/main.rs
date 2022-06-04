mod utility;

use crossbeam::scope;
use std::env::args;
use std::io::{BufRead, BufReader};
use std::net::{TcpListener, TcpStream};
use std::process::exit;
use utility::{hostname, process_request, Status};

fn main() {
    let av: Vec<String> = args().collect();
    if av.len() != 2 {
        eprintln!("usage: ./twebserv-rs port");
        exit(1);
    }

    let server: TcpListener =
        TcpListener::bind(format!("{}:{}", hostname(), av[1].parse::<u16>().unwrap())).unwrap();

    let mut status: Status = Status::setup();
    for request in server.incoming() {
        if let Ok(stream) = request {
            status.server_requests += 1;
            let mut buffered_reader: BufReader<&TcpStream> = BufReader::new(&stream);
            let mut header: String = String::new();
            buffered_reader.read_line(&mut header).unwrap();

            scope(|s| {
                s.spawn(|_| process_request(header, &stream, &status));
            })
            .unwrap();
        }
    }
}
