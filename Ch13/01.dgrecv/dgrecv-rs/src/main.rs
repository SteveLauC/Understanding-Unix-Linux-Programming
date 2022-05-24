use std::net::UdpSocket;
use std::env::args;
use std::ffi::CStr;
use nix::unistd::gethostname;

fn main() {
    let av: Vec<String> = args().collect();
    assert_eq!(av.len(), 2);

    let mut hostname: [u8; 64] = [0; 64];
    let server: UdpSocket = UdpSocket::bind(format!("{}:{}", gethostname(&mut hostname).unwrap().to_str().unwrap(), av[1])).unwrap();

    let mut buf: [u8; 300] = [0; 300];
    while let Ok((size, addr)) = server.recv_from(&mut buf) {
        let msg = CStr::from_bytes_with_nul(&buf[..size+1]).unwrap();
        println!("dgrecv: got a message: {:?}", msg);
        println!("from: {}", addr);
        buf.fill(0);
    }
}
