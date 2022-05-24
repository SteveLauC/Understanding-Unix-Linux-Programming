use std::net::UdpSocket;
use std::env::args;

fn main() {
    let av: Vec<String> = args().collect(); 
    assert_eq!(av.len(), 4);

    let client: UdpSocket = UdpSocket::bind("0.0.0.0:0").unwrap();

    client.send_to(av[3].as_bytes(), format!("{}:{}", av[1], av[2])).unwrap();
}
