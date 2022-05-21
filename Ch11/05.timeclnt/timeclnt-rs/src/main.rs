use std::env::args;
use std::io::Read;
use std::net::TcpStream;

fn main() {
    let av: Vec<String> = args().collect();
    assert_eq!(av.len(), 3);

    if let Ok(mut stream) = TcpStream::connect(format!("{}:{}", av[1], av[2])) {
        let mut buf: String = String::with_capacity(20);
        stream.read_to_string(&mut buf).unwrap();

        println!("{}", buf);
    }
}
