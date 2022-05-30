use std::env::args;
use std::process::exit;
use std::os::unix::net::UnixDatagram;

fn main() {
    let av: Vec<String> = args().collect();
    if av.len() != 2 {
        eprintln!("usage: ./logfilec \"message\"");
        exit(1);
    }

    let client: UnixDatagram = UnixDatagram::unbound().unwrap();

    client
        .send_to(av[1].as_bytes(), "/tmp/logfilesock")
        .unwrap();
}
