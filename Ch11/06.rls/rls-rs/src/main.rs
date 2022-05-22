use std::env::args;
use std::io::{Read, Write};
use std::net::TcpStream;

const PORT_NUM: u16 = 15000;

fn main() {
    let av: Vec<String> = args().collect();
    assert_eq!(av.len(), 3);

    let mut stream: TcpStream = TcpStream::connect(format!("{}:{}", av[1], PORT_NUM)).unwrap();

    stream.write(av[2].as_bytes()).unwrap();
    stream.write(b"\n").unwrap();

    let mut buffer: String = String::with_capacity(100);
    stream.read_to_string(&mut buffer).unwrap();

    println!("{}", buffer);
}
